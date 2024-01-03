use bevy::prelude::*;

pub use crate::XrLocal;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

#[derive(Component)]
pub enum XrPointer {
    Head,
    Eye,
    Hand,
    Controller,
}

mod notes {
    use bevy::prelude::*;
    #[derive(Event)]
    struct Event {}
    struct Select(bool);
    struct Back(bool); //?
}
