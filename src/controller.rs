use bevy::prelude::*;

pub use crate::XrLocal;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

#[derive(Component)]
pub struct XrController;

mod notes {
    enum ControllerInput {
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

    enum InputState {
        None,
        Touched,
        Pressed,
    }

    //Needs rework
    enum InputValue {
        None,
        Boolean(bool),
        Analog(f32),
    }
}
