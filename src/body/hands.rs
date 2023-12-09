use bevy::prelude::*;

//this shouldn't be a component, instead this should be a bundle
#[derive(Component)]
pub struct HandJoint {
    hand: Hand,
    finger: Finger,
    finger_joint: FingerJoint,
    limb_joint: LimbJoint,
}

pub enum Hand {
    Left,
    Right,
}

pub enum Finger {
    None,
    Thumb,
    Index,
    Middle,
    Ring,
    Little,
}

pub enum FingerJoint {
    None,
    Metacarpal,
    ProximalPhalanx,
    IntermediatePhalanx,
    DistalPhalanx,
    Tip,
}

pub enum LimbJoint {
    None,
    Palm,
    Wrist,
    Forearm,
}
