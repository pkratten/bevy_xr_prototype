use bevy::prelude::*;

#[derive(Component)]
pub enum XrHandedness {
    Right,
    Left,
    Other(usize),
}

#[derive(Component)]
pub struct XrRight;

#[derive(Component)]
pub struct XrLeft;
