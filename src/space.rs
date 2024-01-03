use bevy::prelude::*;

#[derive(Component)]
pub enum XrOrigin {
    View,
    Seat,
    Room,
}

mod notes {

    pub struct XrSpace;
    pub struct XrBoundary;
}
