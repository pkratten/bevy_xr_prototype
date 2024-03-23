use bevy::input::gamepad::{AxisSettings, ButtonAxisSettings, ButtonSettings};
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;

pub mod prelude {
    pub use super::AnalogInput;
    pub use super::DigitalInput;
    pub use super::DigitalInputState;
    pub use super::XrController;
    pub use super::XrControllerAxis;
    pub use super::XrControllerAxisType;
    pub use super::XrControllerInputType;
    pub use super::XrControllerPress;
    pub use super::XrControllerTouch;
    pub use super::XrControllers;
}

pub use crate::controller::XrController;

pub type AnalogInputSettings = AxisSettings;
pub type DigitalAnalogInputSettings = ButtonAxisSettings;
pub type DigitalInputSettings = ButtonSettings;

pub type DigitalInput<T> = ButtonInput<T>;
pub type AnalogInput<T> = Axis<T>;

pub struct XrControllerInputPlugin;

impl Plugin for XrControllerInputPlugin {
    fn build(&self, app: &mut App) {
        // gamepad
        app.add_event::<XrControllerStateChangedEvent>()
            .add_event::<XrControllerTouchChangedEvent>()
            .add_event::<XrControllerTouchInputEvent>()
            .add_event::<XrControllerPressChangedEvent>()
            .add_event::<XrControllerPressInputEvent>()
            .add_event::<XrControllerAxisChangedEvent>()
            .add_event::<XrControllerEvent>()
            //.add_event::<GamepadRumbleRequest>()
            .init_resource::<XrControllerSettings>()
            .init_resource::<XrControllers>()
            .init_resource::<DigitalInput<XrControllerTouch>>()
            .init_resource::<DigitalInput<XrControllerPress>>()
            .init_resource::<AnalogInput<XrControllerAxis>>()
            .init_resource::<AnalogInput<XrControllerTouch>>()
            .init_resource::<AnalogInput<XrControllerPress>>()
            .add_systems(
                PreUpdate,
                (
                    xr_controller_event_system,
                    xr_controller_state_system.after(xr_controller_event_system),
                    xr_controller_touch_event_system
                        .after(xr_controller_event_system)
                        .after(xr_controller_state_system),
                    xr_controller_press_event_system
                        .after(xr_controller_event_system)
                        .after(xr_controller_state_system),
                    xr_controller_axis_event_system
                        .after(xr_controller_event_system)
                        .after(xr_controller_state_system),
                )
                    .in_set(InputSystem),
            );

        app.register_type::<XrController>()
            .register_type::<XrControllerState>()
            .register_type::<XrControllerInputType>()
            .register_type::<XrControllerTouch>()
            .register_type::<XrControllerTouchInputEvent>()
            .register_type::<XrControllerPress>()
            .register_type::<XrControllerPressInputEvent>()
            .register_type::<XrControllerAxisType>()
            .register_type::<XrControllerAxis>()
            .register_type::<XrControllerSettings>()
            .register_type::<DigitalInputState>();
    }
}

/// A type of a [`XrControllerTouch`] or [`XrControllerPress`].
///
/// ## Usage
///
/// This is used to determine which button has changed its value when receiving a
/// [`XrControllerTouchChangedEvent`] and [`XrControllerPressChangedEvent`]. It is also used in the [`XrControllerTouch`] and [`XrControllerPress`]
/// which in turn is used to create the [`DigitalInput<XrControllerTouch>`] and [`DigitalInput<XrControllerPress>`] or
/// [`Axis<XrControllerTouch>`] and [`Axis<XrControllerPress>`] `bevy` resources.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
pub enum XrControllerInputType {
    AorX, // this is better in case of web xr, feedback needed.
    BorY,
    //X,
    //Y,
    Stick,
    Pad,
    Trigger,
    Grip,
    Bumper, // Or Shoulder?
    Option,
    System,
    Other(u8),
}

/// A type of a [`XrControllerAxis`].
///
/// ## Usage
///
/// This is used to determine which axis has changed its value when receiving a
/// [`XrControllerAxisChangedEvent`]. It is also used in the [`XrControllerAxis`]
/// which in turn is used to create the [`Axis<XrControllerAxis>`] `bevy` resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum XrControllerAxisType {
    StickX,
    StickY,
    PadX,
    PadY,
    Other(u8),
}

///
/// Input
///  

/// The current input state of an element
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum DigitalInputState {
    /// The button or input is down.
    Down,
    /// The button or input is up.
    Up,
}

impl DigitalInputState {
    /// Is this button pressed?
    pub fn is_pressed(&self) -> bool {
        matches!(self, DigitalInputState::Down)
    }
}

/// Metadata associated with a [`XrController`].
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerInfo {
    /// The name of the controller.
    pub name: String,
}

/// A collection of connected [`XrController`]s.
///
/// ## Usage
///
/// It is stored in a `bevy` resource which tracks all of the currently connected [`XrController`]s.
///
/// ## Updating
///
/// The [`XrController`]s are registered and deregistered in the [`xr_controller_state_system`]
/// whenever a [`XrControllerStateEvent`] is received.
#[derive(Resource, Default, Debug)]
pub struct XrControllers {
    /// The collection of the connected [`XrController`]s.
    xr_controllers: HashMap<XrController, XrControllerInfo>,
}

impl XrControllers {
    /// Returns `true` if the `gamepad` is connected.
    pub fn contains(&self, xr_controller: XrController) -> bool {
        self.xr_controllers.contains_key(&xr_controller)
    }

    /// Returns an iterator over registered [`XrController`]s in an arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = XrController> + '_ {
        self.xr_controllers.keys().copied()
    }

    /// The name of the xr_controller if this one is connected.
    pub fn name(&self, xr_controller: XrController) -> Option<&str> {
        self.xr_controllers
            .get(&xr_controller)
            .map(|g| g.name.as_str())
    }

    /// Registers the `xr_controller`, marking it as connected.
    fn register(&mut self, xr_controller: XrController, info: XrControllerInfo) {
        self.xr_controllers.insert(xr_controller, info);
    }

    /// Deregisters the `xr_controller`, marking it as disconnected.
    fn deregister(&mut self, xr_controller: XrController) {
        self.xr_controllers.remove(&xr_controller);
    }
}

/// A touch input of a [`XrController`].
///
/// ## Usage
///
/// It is used as the generic `T` value of an [`DigitalInput`] and [`Axis`] to create `bevy` resources. These
/// resources store the data of the inputs of a xr controller and can be accessed inside of a system.
///
/// ## Updating
///
/// The xr controller input resources needs to be updated by a xr platform crate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerTouch {
    /// The gamepad on which the button is located on.
    pub xr_controller: XrController,
    /// The type of the button.
    pub input_type: XrControllerInputType,
}

impl XrControllerTouch {
    pub fn new(xr_controller: XrController, input_type: XrControllerInputType) -> Self {
        Self {
            xr_controller,
            input_type,
        }
    }
}

/// A xr controller touch input event.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerTouchInputEvent {
    /// The xr controller touch assigned to the event.
    pub touch: XrControllerTouch,
    /// The state of the touch.
    pub state: DigitalInputState,
}

/// A press input of a [`XrController`].
///
/// ## Usage
///
/// It is used as the generic `T` value of an [`DigitalInput`] and [`Axis`] to create `bevy` resources. These
/// resources store the data of the inputs of a xr controller and can be accessed inside of a system.
///
/// ## Updating
///
/// The xr controller input resources needs to be updated by a xr platform crate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerPress {
    /// The gamepad on which the button is located on.
    pub xr_controller: XrController,
    /// The type of the button.
    pub input_type: XrControllerInputType,
}

impl XrControllerPress {
    pub fn new(xr_controller: XrController, input_type: XrControllerInputType) -> Self {
        Self {
            xr_controller,
            input_type,
        }
    }
}

/// A xr controller press input event.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerPressInputEvent {
    /// The xr controller press assigned to the event.
    pub press: XrControllerPress,
    /// The state of the press.
    pub state: DigitalInputState,
}

/// An axis of a [`XrController`].
///
/// ## Usage
///
/// It is used as the generic `T` value of an [`Axis`] to create `bevy` resources. These
/// resources store the data of the axes of a xr_controller and can be accessed inside of a system.
///
/// ## Updating
///
/// The gamepad axes resources are updated inside a xr platform crate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerAxis {
    /// The gamepad on which the axis is located on.
    pub xr_controller: XrController,
    /// The type of the axis.
    pub axis_type: XrControllerAxisType,
}

impl XrControllerAxis {
    pub fn new(xr_controller: XrController, axis_type: XrControllerAxisType) -> Self {
        Self {
            xr_controller,
            axis_type,
        }
    }
}

/// Settings for all [`XrController`]s.
///
/// ## Usage
///
/// It is used to create a `bevy` resource that stores the settings of every [`XrControllerTouchType`], [`XrControllerPressType`] and
/// [`XrControllerAxis`]. If no user defined [`DigitalInputSettings`], [`AnalogInputSettings`], or [`DigitalAnalogInputSettings`]
/// are defined, the default settings of each are used as a fallback accordingly.
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct XrControllerSettings {
    /// The default touch settings.
    pub default_touch_settings: DigitalInputSettings,
    // The default press settings.
    pub default_press_settings: DigitalInputSettings,
    /// The default axis settings.
    pub default_axis_settings: AnalogInputSettings,
    /// The default touch axis settings.
    pub default_touch_axis_settings: DigitalAnalogInputSettings,
    /// The default press axis settings.
    pub default_press_axis_settings: DigitalAnalogInputSettings,
    /// The user defined touch settings.
    pub touch_settings: HashMap<XrControllerTouch, DigitalInputSettings>,
    /// The user defined press settings.
    pub press_settings: HashMap<XrControllerPress, DigitalInputSettings>,
    /// The user defined axis settings.
    pub axis_settings: HashMap<XrControllerAxis, AnalogInputSettings>,
    /// The user defined touch axis settings.
    pub touch_axis_settings: HashMap<XrControllerTouch, DigitalAnalogInputSettings>,
    /// The user defined touch axis settings.
    pub press_axis_settings: HashMap<XrControllerPress, DigitalAnalogInputSettings>,
}

impl XrControllerSettings {
    /// TODO: Adjust doc
    /// Returns the [`DigitalInputSettings`] of the `button`.
    ///
    /// If no user defined [`DigitalInputSettings`] are specified the default [`DigitalInputSettings`] get returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_input::gamepad::{GamepadSettings, GamepadButton, Gamepad, GamepadButtonType};
    /// #
    /// # let settings = GamepadSettings::default();
    /// let button = GamepadButton::new(Gamepad::new(1), GamepadButtonType::South);
    /// let button_settings = settings.get_button_settings(button);
    /// ```
    pub fn get_touch_settings(&self, touch: XrControllerTouch) -> &DigitalInputSettings {
        self.touch_settings
            .get(&touch)
            .unwrap_or(&self.default_touch_settings)
    }

    /// TODO: Adjust doc
    /// Returns the [`DigitalInputSettings`] of the `button`.
    ///
    /// If no user defined [`DigitalInputSettings`] are specified the default [`DigitalInputSettings`] get returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_input::gamepad::{GamepadSettings, GamepadButton, Gamepad, GamepadButtonType};
    /// #
    /// # let settings = GamepadSettings::default();
    /// let button = GamepadButton::new(Gamepad::new(1), GamepadButtonType::South);
    /// let button_settings = settings.get_button_settings(button);
    /// ```
    pub fn get_press_settings(&self, press: XrControllerPress) -> &DigitalInputSettings {
        self.press_settings
            .get(&press)
            .unwrap_or(&self.default_press_settings)
    }

    /// TODO: Adjust doc
    /// Returns the [`AnalogInputSettings`] of the `axis`.
    ///
    /// If no user defined [`AnalogInputSettings`] are specified the default [`AnalogInputSettings`] get returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_input::gamepad::{GamepadSettings, GamepadAxis, Gamepad, GamepadAxisType};
    /// #
    /// # let settings = GamepadSettings::default();
    /// let axis = GamepadAxis::new(Gamepad::new(1), GamepadAxisType::LeftStickX);
    /// let axis_settings = settings.get_axis_settings(axis);
    /// ```
    pub fn get_axis_settings(&self, axis: XrControllerAxis) -> &AnalogInputSettings {
        self.axis_settings
            .get(&axis)
            .unwrap_or(&self.default_axis_settings)
    }

    /// TODO: Adjust docs.
    /// Returns the [`DigitalAnalogInputSettings`] of the `button`.
    ///
    /// If no user defined [`DigitalAnalogInputSettings`] are specified the default [`DigitalAnalogInputSettings`] get returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_input::gamepad::{GamepadSettings, GamepadButton, Gamepad, GamepadButtonType};
    /// #
    /// # let settings = GamepadSettings::default();
    /// let button = GamepadButton::new(Gamepad::new(1), GamepadButtonType::South);
    /// let button_axis_settings = settings.get_button_axis_settings(button);
    /// ```
    pub fn get_touch_axis_settings(&self, touch: XrControllerTouch) -> &DigitalAnalogInputSettings {
        self.touch_axis_settings
            .get(&touch)
            .unwrap_or(&self.default_touch_axis_settings)
    }

    /// TODO: Adjust docs.
    /// Returns the [`DigitalAnalogInputSettings`] of the `button`.
    ///
    /// If no user defined [`DigitalAnalogInputSettings`] are specified the default [`DigitalAnalogInputSettings`] get returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_input::gamepad::{GamepadSettings, GamepadButton, Gamepad, GamepadButtonType};
    /// #
    /// # let settings = GamepadSettings::default();
    /// let button = GamepadButton::new(Gamepad::new(1), GamepadButtonType::South);
    /// let button_axis_settings = settings.get_button_axis_settings(button);
    /// ```
    pub fn get_press_axis_settings(&self, press: XrControllerPress) -> &DigitalAnalogInputSettings {
        self.press_axis_settings
            .get(&press)
            .unwrap_or(&self.default_press_axis_settings)
    }
}

/// The status of a xr controller.
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum XrControllerState {
    /// The xr controller is tracking.
    Tracking(XrControllerInfo),
    /// The xr controller is connected.
    Connected(XrControllerInfo),
    /// The xr controller is disconnected.
    Disconnected,
}

/// A xr controller state changed event. Created when the state of a xr controller
/// is changed and when a xr controller is disconnected.
#[derive(Event, Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerStateChangedEvent {
    /// The xr controller whose status changed.
    pub xr_controller: XrController,
    /// The change in the xr controllers state.
    pub state: XrControllerState,
}

impl XrControllerStateChangedEvent {
    /// Creates a [`XrControllerStateChangedEvent`].
    pub fn new(xr_controller: XrController, state: XrControllerState) -> Self {
        Self {
            xr_controller,
            state,
        }
    }

    /// Is the xr controller tracking?
    pub fn tracking(&self) -> bool {
        matches!(self.state, XrControllerState::Tracking(_))
    }

    /// Is the xr controller connected?
    pub fn connected(&self) -> bool {
        matches!(self.state, XrControllerState::Connected(_))
    }

    /// Is the xr controller disconnected?
    pub fn disconnected(&self) -> bool {
        !(self.connected() | self.tracking())
    }
}

/// Handles [`XrControllerStateChangedEvent`]s and updates gamepad resources.
///
/// Updates the [`XrControllers`] resource and resets and/or initializes
/// the [`Axis<XrControllerTouch>`], [`DigitalInput<XrControllerTouch>`], [`Axis<XrControllerPress>`], [`DigitalInput<XrControllerPress>`] and [`Axis<XrControllerAxis>`] resources.
///
/// ## Note
///
/// Whenever a [`XrControllers`] connects or disconnects, an information gets printed to the console using the [`info!`] macro.
pub fn xr_controller_state_system(
    mut xr_controllers: ResMut<XrControllers>,
    mut connection_events: EventReader<XrControllerStateChangedEvent>,
    mut axis: ResMut<AnalogInput<XrControllerAxis>>,
    mut touch_axis: ResMut<AnalogInput<XrControllerTouch>>,
    mut touch_input: ResMut<DigitalInput<XrControllerTouch>>,
    mut press_axis: ResMut<AnalogInput<XrControllerPress>>,
    mut press_input: ResMut<DigitalInput<XrControllerPress>>,
) {
    for connection_event in connection_events.read() {
        let xr_controller = connection_event.xr_controller;

        match &connection_event.state {
            XrControllerState::Connected(info) | XrControllerState::Tracking(info) => {
                if !xr_controllers.contains(xr_controller) {
                    xr_controllers.register(xr_controller, info.clone());
                    info!("{:?} Connected", xr_controller);

                    for touch_type in &ALL_INPUT_TYPES {
                        let input = XrControllerTouch::new(xr_controller, *touch_type);
                        touch_input.reset(input);
                        touch_axis.set(input, 0.0);
                    }

                    for press_type in &ALL_INPUT_TYPES {
                        let input = XrControllerPress::new(xr_controller, *press_type);
                        press_input.reset(input);
                        press_axis.set(input, 0.0);
                    }

                    for axis_type in &ALL_AXIS_TYPES {
                        axis.set(XrControllerAxis::new(xr_controller, *axis_type), 0.0);
                    }
                }
            }
            XrControllerState::Disconnected => {
                xr_controllers.deregister(xr_controller);
                info!("{:?} Disconnected", xr_controller);

                for touch_type in &ALL_INPUT_TYPES {
                    let input = XrControllerTouch::new(xr_controller, *touch_type);
                    touch_input.reset(input);
                    touch_axis.remove(input);
                }

                for press_type in &ALL_INPUT_TYPES {
                    let input = XrControllerPress::new(xr_controller, *press_type);
                    press_input.reset(input);
                    press_axis.remove(input);
                }

                for axis_type in &ALL_AXIS_TYPES {
                    axis.remove(XrControllerAxis::new(xr_controller, *axis_type));
                }
            }
        }
    }
}

/// An array of every [`XrControllerInputType`] variant.
const ALL_INPUT_TYPES: [XrControllerInputType; 9] = [
    XrControllerInputType::AorX, // would AorX and BorY be better?
    XrControllerInputType::BorY,
    //XrControllerInputType::X,
    //XrControllerInputType::Y,
    XrControllerInputType::Stick,
    XrControllerInputType::Pad,
    XrControllerInputType::Trigger,
    XrControllerInputType::Grip,
    XrControllerInputType::Bumper, // Or Shoulder?
    XrControllerInputType::Option,
    XrControllerInputType::System,
];

/// An array of every [`XrControllerAxisType`] variant.
const ALL_AXIS_TYPES: [XrControllerAxisType; 4] = [
    XrControllerAxisType::StickX,
    XrControllerAxisType::StickY,
    XrControllerAxisType::PadX,
    XrControllerAxisType::PadY,
];

/// Xr controller event for when the "value" on the axis changes
/// by an amount larger than the threshold defined in [`XrControllerSettings`].
#[derive(Event, Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerAxisChangedEvent {
    /// The xr controller on which the axis is triggered.
    pub xr_controller: XrController,
    /// The type of the triggered axis.
    pub axis_type: XrControllerAxisType,
    /// The value of the axis.
    pub value: f32,
}

impl XrControllerAxisChangedEvent {
    /// Creates a [`XrControllerAxisChangedEvent`].
    pub fn new(xr_controller: XrController, axis_type: XrControllerAxisType, value: f32) -> Self {
        Self {
            xr_controller,
            axis_type,
            value,
        }
    }
}

/// Uses [`XrControllerAxisChangedEvent`]s to update the relevant [`DigitalInput`] and [`Axis`] values.
pub fn xr_controller_axis_event_system(
    mut xr_controller_axis: ResMut<AnalogInput<XrControllerAxis>>,
    mut axis_events: EventReader<XrControllerAxisChangedEvent>,
) {
    for axis_event in axis_events.read() {
        let axis = XrControllerAxis::new(axis_event.xr_controller, axis_event.axis_type);
        xr_controller_axis.set(axis, axis_event.value);
    }
}

/// Xr controller event for when the "value" (amount of closeness) of a touch
/// changes by an amount larger than the threshold defined in [`XrControllerSettings`].
#[derive(Event, Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerTouchChangedEvent {
    /// The gamepad on which the button is triggered.
    pub xr_controller: XrController,
    /// The type of the triggered button.
    pub touch_type: XrControllerInputType,
    /// The value of the button.
    pub value: f32,
}

impl XrControllerTouchChangedEvent {
    /// Creates a [`XrControllerTouchChangedEvent`].
    pub fn new(xr_controller: XrController, touch_type: XrControllerInputType, value: f32) -> Self {
        Self {
            xr_controller,
            touch_type,
            value,
        }
    }
}

/// Uses [`XrControllerTouchChangedEvent`]s to update the relevant [`XrControllerTouchInput`] and [`Axis`] values.
pub fn xr_controller_touch_event_system(
    mut touch_changed_events: EventReader<XrControllerTouchChangedEvent>,
    mut touch_input: ResMut<DigitalInput<XrControllerTouch>>,
    mut touch_input_events: EventWriter<XrControllerTouchInputEvent>,
    //settings: Res<XrControllerSettings>,
) {
    for touch_event in touch_changed_events.read() {
        let touch = XrControllerTouch::new(touch_event.xr_controller, touch_event.touch_type);
        let value = touch_event.value;
        //let touch_property = settings.get_touch_settings(touch);

        //if touch_property.is_released(value) {
        if value < 0.1 {
            // Check if button was previously pressed
            if touch_input.pressed(touch) {
                touch_input_events.send(XrControllerTouchInputEvent {
                    touch,
                    state: DigitalInputState::Up,
                });
            }
            // We don't have to check if the button was previously pressed here
            // because that check is performed within Input<T>::release()
            touch_input.release(touch);
        }
        //else if touch_property.is_pressed(value) {
        else if value > 0.1 {
            // Check if button was previously not pressed
            if !touch_input.pressed(touch) {
                touch_input_events.send(XrControllerTouchInputEvent {
                    touch,
                    state: DigitalInputState::Down,
                });
            }
            touch_input.press(touch);
        };
    }
}

/// Xr controller event for when the "value" (amount of pressure) of a press
/// changes by an amount larger than the threshold defined in [`XrControllerSettings`].
#[derive(Event, Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct XrControllerPressChangedEvent {
    /// The gamepad on which the button is triggered.
    pub xr_controller: XrController,
    /// The type of the triggered button.
    pub press_type: XrControllerInputType,
    /// The value of the button.
    pub value: f32,
}

impl XrControllerPressChangedEvent {
    /// Creates a [`XrControllerPressChangedEvent`].
    pub fn new(xr_controller: XrController, press_type: XrControllerInputType, value: f32) -> Self {
        Self {
            xr_controller,
            press_type,
            value,
        }
    }
}

/// Uses [`XrControllerPressChangedEvent`]s to update the relevant [`XrControllerpressInput`] and [`Axis`] values.
pub fn xr_controller_press_event_system(
    mut press_changed_events: EventReader<XrControllerPressChangedEvent>,
    mut press_input: ResMut<DigitalInput<XrControllerPress>>,
    mut press_input_events: EventWriter<XrControllerPressInputEvent>,
    //settings: Res<XrControllerSettings>,
) {
    for press_event in press_changed_events.read() {
        let press = XrControllerPress::new(press_event.xr_controller, press_event.press_type);
        let value = press_event.value;
        //let press_property = settings.get_press_settings(press);

        //if press_property.is_released(value) {
        if value < 0.8 {
            // Check if button was previously pressed
            if press_input.pressed(press) {
                press_input_events.send(XrControllerPressInputEvent {
                    press,
                    state: DigitalInputState::Up,
                });
            }
            // We don't have to check if the button was previously pressed here
            // because that check is performed within Input<T>::release()
            press_input.release(press);
        }
        //else if press_property.is_pressed(value) {
        else if value > 0.8 {
            // Check if button was previously not pressed
            if !press_input.pressed(press) {
                press_input_events.send(XrControllerPressInputEvent {
                    press,
                    state: DigitalInputState::Down,
                });
            }
            press_input.press(press);
        };
    }
}

/// A gamepad event.
///
/// This event type is used over the [`GamepadConnectionEvent`],
/// [`GamepadButtonChangedEvent`] and [`GamepadAxisChangedEvent`] when
/// the in-frame relative ordering of events is important.
#[derive(Event, Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum XrControllerEvent {
    /// A xr controller state has changed.
    State(XrControllerStateChangedEvent),
    /// A touch of the xr controller has been triggered.
    Touch(XrControllerTouchChangedEvent),
    /// A touch of the xr controller has been triggered.
    Press(XrControllerPressChangedEvent),
    /// An axis of the gamepad has been triggered.
    Axis(XrControllerAxisChangedEvent),
}

impl From<XrControllerStateChangedEvent> for XrControllerEvent {
    fn from(value: XrControllerStateChangedEvent) -> Self {
        Self::State(value)
    }
}

impl From<XrControllerTouchChangedEvent> for XrControllerEvent {
    fn from(value: XrControllerTouchChangedEvent) -> Self {
        Self::Touch(value)
    }
}

impl From<XrControllerPressChangedEvent> for XrControllerEvent {
    fn from(value: XrControllerPressChangedEvent) -> Self {
        Self::Press(value)
    }
}

impl From<XrControllerAxisChangedEvent> for XrControllerEvent {
    fn from(value: XrControllerAxisChangedEvent) -> Self {
        Self::Axis(value)
    }
}

/// Splits the [`GamepadEvent`] event stream into it's component events.
pub fn xr_controller_event_system(
    mut xr_controller_events: EventReader<XrControllerEvent>,
    mut state_events: EventWriter<XrControllerStateChangedEvent>,
    mut touch_events: EventWriter<XrControllerTouchChangedEvent>,
    mut press_events: EventWriter<XrControllerPressChangedEvent>,
    mut axis_events: EventWriter<XrControllerAxisChangedEvent>,
    mut touch_input: ResMut<DigitalInput<XrControllerTouch>>,
    mut press_input: ResMut<DigitalInput<XrControllerPress>>,
) {
    touch_input.bypass_change_detection().clear();
    press_input.bypass_change_detection().clear();
    for gamepad_event in xr_controller_events.read() {
        match gamepad_event {
            XrControllerEvent::State(connection_event) => {
                state_events.send(connection_event.clone());
            }
            XrControllerEvent::Touch(touch_event) => {
                touch_events.send(touch_event.clone());
            }
            XrControllerEvent::Press(press_event) => {
                press_events.send(press_event.clone());
            }
            XrControllerEvent::Axis(axis_event) => {
                axis_events.send(axis_event.clone());
            }
        }
    }
}
