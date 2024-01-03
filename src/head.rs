use bevy::prelude::*;

pub use crate::XrLocal;

#[derive(Component)]
pub struct XrHead;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

#[derive(Component)]
pub struct XrEye;
