//! This file sould implement the bevy_input ButtonInput<> and Axis<> resources and other logic.
//! It would be desirable to abstract the existing Gamepad implementation to create generic axis and button implementations and settings.

use bevy::reflect::Reflect;

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
