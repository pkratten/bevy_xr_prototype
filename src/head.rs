use bevy::prelude::*;

pub use crate::XrLocal;

#[derive(Component)]
pub struct XrHead;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

// TODO: OpenXR allows for multiple views per eye! WebXr probably too.
#[derive(Component)]
pub struct XrEye;
