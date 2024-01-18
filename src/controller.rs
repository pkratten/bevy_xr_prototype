use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::Handedness;
pub use crate::handedness::LeftHanded;
pub use crate::handedness::RightHanded;

/// The defining [`Component`] for entities that represent controllers.
/// Represents the transform of a controller.
///
/// The hand of the controller is defined by the [`XrHandedness`] and either the [`XrLeft`] or [`XrRight`] components.
///
/// Controller entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component)]
pub struct XrController;

/// This enum is entended for the bevy_input crate that still needs to be implemented below. The intention is to have an Controller_Input and an Controller_Touched variant to cover the information presented by most xr hardware.
pub enum XrControllerInput {
    A,
    B,
    X,
    Y,
    Stick,
    Pad,
    Trigger,
    Grip,
    Shoulder,
    Option,
    System,
    Other(usize),
}

mod notes {
    // This enum is not going to find use as the bevy_input crate doesn't cover three state input.

    enum InputState {
        None,
        Touched,
        Pressed,
    }
    /// This enum is covered by the button and axis options of the bevy_input crate.

    //Needs rework
    enum InputValue {
        None,
        Boolean(bool),
        Analog(f32),
    }
}
