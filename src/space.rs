use bevy::prelude::*;

use crate::XrLocal;

#[derive(Clone, Copy, Component)]
pub enum XrOrigin {
    View,
    Seat,
    Room,
}

mod notes {

    pub struct XrSpace;
    pub struct XrBoundary;
}
