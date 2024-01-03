use bevy::prelude::*;

pub use crate::XrLocal;

#[derive(Component)]
pub struct XrHand;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

pub mod hand_part {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Forearm;
    #[derive(Component)]
    pub struct Wrist;
    #[derive(Component)]
    pub struct Palm;
    #[derive(Component)]
    pub struct Finger;
}

pub mod finger {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Thumb;
    #[derive(Component)]
    pub struct Index;
    #[derive(Component)]
    pub struct Middle;
    #[derive(Component)]
    pub struct Ring;
    #[derive(Component)]
    pub struct Little;
}

pub mod finger_joint {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Metacarpal;
    #[derive(Component)]
    pub struct ProximalPhalanx;
    #[derive(Component)]
    pub struct IntermediatePhalanx;
    #[derive(Component)]
    pub struct DistalPhalanx;
    #[derive(Component)]
    pub struct Tip;
}
