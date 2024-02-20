//! This file sould implement the bevy_input ButtonInput<> and Axis<> resources and other logic.
//! It would be desirable to abstract the existing Gamepad implementation to create generic axis and button implementations and settings.

use bevy::input::gamepad::{AxisSettings, ButtonAxisSettings, ButtonSettings};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::{input::ButtonState, reflect::Reflect};

pub use crate::controller::XrController;

/// A type of a [`XrControllerTouch`] or [`XrControllerPress`].
///
/// ## Usage
///
/// This is used to determine which button has changed its value when receiving a
/// [`XrControllerTouchChangedEvent`] and [`XrControllerPressChangedEvent`]. It is also used in the [`XrControllerTouch`] and [`XrControllerPress`]
/// which in turn is used to create the [`ButtonInput<XrControllerTouch>`] and [`ButtonInput<XrControllerPress>`] or
/// [`Axis<XrControllerTouch>`] and [`Axis<XrControllerPress>`] `bevy` resources.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
pub enum XrControllerInputType {
    A, // would AorX and BorY be better?
    B,
    X,
    Y,
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
}

///
/// Input
///  

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
/// It is used as the generic `T` value of an [`ButtonInput`] and [`Axis`] to create `bevy` resources. These
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
pub struct XrControllerTouchType {
    /// The gamepad on which the button is located on.
    pub xr_controller: XrController,
    /// The type of the button.
    pub input_type: XrControllerInputType,
}

impl XrControllerTouchType {
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
pub struct XrControllerTouchInput {
    /// The xr controller touch assigned to the event.
    pub touch: XrControllerTouchType,
    /// The state of the touch.
    pub state: ButtonState,
}

/// A press input of a [`XrController`].
///
/// ## Usage
///
/// It is used as the generic `T` value of an [`ButtonInput`] and [`Axis`] to create `bevy` resources. These
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
pub struct XrControllerPressType {
    /// The gamepad on which the button is located on.
    pub xr_controller: XrController,
    /// The type of the button.
    pub input_type: XrControllerInputType,
}

impl XrControllerPressType {
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
pub struct XrControllerPressInput {
    /// The xr controller press assigned to the event.
    pub touch: XrControllerTouchType,
    /// The state of the press.
    pub state: ButtonState,
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
/// [`XrControllerAxis`]. If no user defined [`ButtonSettings`], [`AxisSettings`], or [`ButtonAxisSettings`]
/// are defined, the default settings of each are used as a fallback accordingly.
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct XrControllerSettings {
    /// The default touch settings.
    pub default_touch_settings: ButtonSettings,
    // The default press settings.
    pub default_press_settings: ButtonSettings,
    /// The default axis settings.
    pub default_axis_settings: AxisSettings,
    /// The default touch axis settings.
    pub default_touch_axis_settings: ButtonAxisSettings,
    /// The default press axis settings.
    pub default_press_axis_settings: ButtonAxisSettings,
    /// The user defined touch settings.
    pub touch_settings: HashMap<XrControllerTouchType, ButtonSettings>,
    /// The user defined press settings.
    pub press_settings: HashMap<XrControllerPressType, ButtonSettings>,
    /// The user defined axis settings.
    pub axis_settings: HashMap<XrControllerAxis, AxisSettings>,
    /// The user defined touch axis settings.
    pub touch_axis_settings: HashMap<XrControllerTouchType, ButtonAxisSettings>,
    /// The user defined touch axis settings.
    pub press_axis_settings: HashMap<XrControllerPressType, ButtonAxisSettings>,
}

impl XrControllerSettings {
    /// TODO: Adjust doc
    /// Returns the [`ButtonSettings`] of the `button`.
    ///
    /// If no user defined [`ButtonSettings`] are specified the default [`ButtonSettings`] get returned.
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
    pub fn get_touch_settings(&self, touch: XrControllerTouchType) -> &ButtonSettings {
        self.touch_settings
            .get(&touch)
            .unwrap_or(&self.default_touch_settings)
    }

    /// TODO: Adjust doc
    /// Returns the [`ButtonSettings`] of the `button`.
    ///
    /// If no user defined [`ButtonSettings`] are specified the default [`ButtonSettings`] get returned.
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
    pub fn get_press_settings(&self, press: XrControllerPressType) -> &ButtonSettings {
        self.press_settings
            .get(&press)
            .unwrap_or(&self.default_press_settings)
    }

    /// TODO: Adjust doc
    /// Returns the [`AxisSettings`] of the `axis`.
    ///
    /// If no user defined [`AxisSettings`] are specified the default [`AxisSettings`] get returned.
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
    pub fn get_axis_settings(&self, axis: XrControllerAxis) -> &AxisSettings {
        self.axis_settings
            .get(&axis)
            .unwrap_or(&self.default_axis_settings)
    }

    /// TODO: Adjust docs.
    /// Returns the [`ButtonAxisSettings`] of the `button`.
    ///
    /// If no user defined [`ButtonAxisSettings`] are specified the default [`ButtonAxisSettings`] get returned.
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
    pub fn get_touch_axis_settings(&self, touch: XrControllerTouchType) -> &ButtonAxisSettings {
        self.touch_axis_settings
            .get(&touch)
            .unwrap_or(&self.default_touch_axis_settings)
    }

    /// TODO: Adjust docs.
    /// Returns the [`ButtonAxisSettings`] of the `button`.
    ///
    /// If no user defined [`ButtonAxisSettings`] are specified the default [`ButtonAxisSettings`] get returned.
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
    pub fn get_press_axis_settings(&self, press: XrControllerPressType) -> &ButtonAxisSettings {
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

/// A Gamepad connection event. Created when a connection to a gamepad
/// is established and when a gamepad is disconnected.
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
    /// Creates a [`GamepadConnectionEvent`].
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

    /// Is the xr_controller connected?
    pub fn connected(&self) -> bool {
        matches!(self.state, XrControllerState::Connected(_))
    }

    /// Is the gamepad disconnected?
    pub fn disconnected(&self) -> bool {
        !(self.connected() | self.tracking())
    }
}

/// Handles [`GamepadConnectionEvent`]s and updates gamepad resources.
///
/// Updates the [`Gamepads`] resource and resets and/or initializes
/// the [`Axis<GamepadButton>`] and [`ButtonInput<GamepadButton>`] resources.
///
/// ## Note
///
/// Whenever a [`Gamepad`] connects or disconnects, an information gets printed to the console using the [`info!`] macro.
pub fn xr_controller_state_system(
    mut gamepads: ResMut<XrControllers>,
    mut connection_events: EventReader<XrControllerStateChangedEvent>,
    mut axis: ResMut<Axis<XrControllerAxis>>,
    mut touch_axis: ResMut<Axis<XrControllerTouchType>>,
    mut touch_input: ResMut<ButtonInput<XrControllerTouchType>>,
    mut press_axis: ResMut<Axis<XrControllerPressType>>,
    mut press_input: ResMut<ButtonInput<XrControllerPressType>>,
) {
    for connection_event in connection_events.read() {
        let gamepad = connection_event.gamepad;

        if let GamepadConnection::Connected(info) = &connection_event.connection {
            gamepads.register(gamepad, info.clone());
            info!("{:?} Connected", gamepad);

            for button_type in &ALL_BUTTON_TYPES {
                let gamepad_button = GamepadButton::new(gamepad, *button_type);
                button_input.reset(gamepad_button);
                button_axis.set(gamepad_button, 0.0);
            }
            for axis_type in &ALL_AXIS_TYPES {
                axis.set(GamepadAxis::new(gamepad, *axis_type), 0.0);
            }
        } else {
            gamepads.deregister(gamepad);
            info!("{:?} Disconnected", gamepad);

            for button_type in &ALL_BUTTON_TYPES {
                let gamepad_button = GamepadButton::new(gamepad, *button_type);
                button_input.reset(gamepad_button);
                button_axis.remove(gamepad_button);
            }
            for axis_type in &ALL_AXIS_TYPES {
                axis.remove(GamepadAxis::new(gamepad, *axis_type));
            }
        }
    }
}
