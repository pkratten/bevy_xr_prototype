use bevy::prelude::*;

pub mod controller;
pub mod handedness;
pub mod hands;
pub mod head;
pub mod pointer;
pub mod space;
pub mod tracked;
pub mod window;

#[derive(Component)]
pub struct XrLocal;
