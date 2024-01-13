use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

#[derive(Component)]
pub enum XrHand {
    Forearm,
    Wrist,
    Palm,
    ThumbMetacarpal,
    ThumbProximal,
    ThumbDistal,
    ThumbTip,
    IndexMetacarpal,
    IndexProximal,
    IndexIntermediate,
    IndexDistal,
    IndexTip,
    MiddleMetacarpal,
    MiddleProximal,
    MiddleIntermediate,
    MiddleDistal,
    MiddleTip,
    RingMetacarpal,
    RingProximal,
    RingIntermediate,
    RingDistal,
    RingTip,
    LittleMetacarpal,
    LittleProximal,
    LittleIntermediate,
    LittleDistal,
    LittleTip,
}

#[derive(Component)]
pub struct XrHandJointRadius(pub f32);

pub mod hand_part {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Forearm;

    #[derive(Component)]
    pub struct Wrist;

    #[derive(Component)]
    pub struct Palm;
}

pub mod finger {
    use bevy::prelude::*;

    #[derive(Component)]
    pub enum Finger {
        Thumb,
        Index,
        Middle,
        Ring,
        Little,
    }

    pub trait FingerTrait {
        fn finger() -> Finger;
    }

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
    pub enum FingerJoint {
        Metacarpal,
        ProximalPhalanx,
        IntermediatePhalanx,
        DistalPhalanx,
        Tip,
    }

    pub trait FingerJointTrait {
        fn finger_joint() -> FingerJoint;
    }

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

//
// IntoEnum
//

use crate::IntoEnum;
use finger::*;
use finger_joint::*;
use hand_part::*;

// Hand

impl IntoEnum<XrHand> for Forearm {
    fn into_enum() -> XrHand {
        XrHand::Forearm
    }
}

impl IntoEnum<XrHand> for Wrist {
    fn into_enum() -> XrHand {
        XrHand::Wrist
    }
}

impl IntoEnum<XrHand> for Palm {
    fn into_enum() -> XrHand {
        XrHand::Palm
    }
}

// Thumb

impl IntoEnum<XrHand> for (Thumb, Metacarpal) {
    fn into_enum() -> XrHand {
        XrHand::ThumbMetacarpal
    }
}

impl IntoEnum<XrHand> for (Thumb, ProximalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::ThumbProximal
    }
}

impl IntoEnum<XrHand> for (Thumb, DistalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::ThumbDistal
    }
}

impl IntoEnum<XrHand> for (Thumb, Tip) {
    fn into_enum() -> XrHand {
        XrHand::ThumbTip
    }
}

// Index

impl IntoEnum<XrHand> for (Index, Metacarpal) {
    fn into_enum() -> XrHand {
        XrHand::IndexMetacarpal
    }
}

impl IntoEnum<XrHand> for (Index, ProximalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::IndexProximal
    }
}

impl IntoEnum<XrHand> for (Index, IntermediatePhalanx) {
    fn into_enum() -> XrHand {
        XrHand::IndexIntermediate
    }
}

impl IntoEnum<XrHand> for (Index, DistalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::IndexDistal
    }
}

impl IntoEnum<XrHand> for (Index, Tip) {
    fn into_enum() -> XrHand {
        XrHand::IndexTip
    }
}

// Middle

impl IntoEnum<XrHand> for (Middle, Metacarpal) {
    fn into_enum() -> XrHand {
        XrHand::MiddleMetacarpal
    }
}

impl IntoEnum<XrHand> for (Middle, ProximalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::MiddleProximal
    }
}

impl IntoEnum<XrHand> for (Middle, IntermediatePhalanx) {
    fn into_enum() -> XrHand {
        XrHand::MiddleIntermediate
    }
}

impl IntoEnum<XrHand> for (Middle, DistalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::MiddleDistal
    }
}

impl IntoEnum<XrHand> for (Middle, Tip) {
    fn into_enum() -> XrHand {
        XrHand::MiddleTip
    }
}

// Ring

impl IntoEnum<XrHand> for (Ring, Metacarpal) {
    fn into_enum() -> XrHand {
        XrHand::RingMetacarpal
    }
}

impl IntoEnum<XrHand> for (Ring, ProximalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::RingProximal
    }
}

impl IntoEnum<XrHand> for (Ring, IntermediatePhalanx) {
    fn into_enum() -> XrHand {
        XrHand::RingIntermediate
    }
}

impl IntoEnum<XrHand> for (Ring, DistalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::RingDistal
    }
}

impl IntoEnum<XrHand> for (Ring, Tip) {
    fn into_enum() -> XrHand {
        XrHand::RingTip
    }
}

// Little

impl IntoEnum<XrHand> for (Little, Metacarpal) {
    fn into_enum() -> XrHand {
        XrHand::LittleMetacarpal
    }
}

impl IntoEnum<XrHand> for (Little, ProximalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::LittleProximal
    }
}

impl IntoEnum<XrHand> for (Little, IntermediatePhalanx) {
    fn into_enum() -> XrHand {
        XrHand::LittleIntermediate
    }
}

impl IntoEnum<XrHand> for (Little, DistalPhalanx) {
    fn into_enum() -> XrHand {
        XrHand::LittleDistal
    }
}

impl IntoEnum<XrHand> for (Little, Tip) {
    fn into_enum() -> XrHand {
        XrHand::LittleTip
    }
}

// Thumb swapped

impl IntoEnum<XrHand> for (Metacarpal, Thumb) {
    fn into_enum() -> XrHand {
        XrHand::ThumbMetacarpal
    }
}

impl IntoEnum<XrHand> for (ProximalPhalanx, Thumb) {
    fn into_enum() -> XrHand {
        XrHand::ThumbProximal
    }
}

impl IntoEnum<XrHand> for (DistalPhalanx, Thumb) {
    fn into_enum() -> XrHand {
        XrHand::ThumbDistal
    }
}

impl IntoEnum<XrHand> for (Tip, Thumb) {
    fn into_enum() -> XrHand {
        XrHand::ThumbTip
    }
}

// Index swapped

impl IntoEnum<XrHand> for (Metacarpal, Index) {
    fn into_enum() -> XrHand {
        XrHand::IndexMetacarpal
    }
}

impl IntoEnum<XrHand> for (ProximalPhalanx, Index) {
    fn into_enum() -> XrHand {
        XrHand::IndexProximal
    }
}

impl IntoEnum<XrHand> for (IntermediatePhalanx, Index) {
    fn into_enum() -> XrHand {
        XrHand::IndexIntermediate
    }
}

impl IntoEnum<XrHand> for (DistalPhalanx, Index) {
    fn into_enum() -> XrHand {
        XrHand::IndexDistal
    }
}

impl IntoEnum<XrHand> for (Tip, Index) {
    fn into_enum() -> XrHand {
        XrHand::IndexTip
    }
}

// Middle swapped

impl IntoEnum<XrHand> for (Metacarpal, Middle) {
    fn into_enum() -> XrHand {
        XrHand::MiddleMetacarpal
    }
}

impl IntoEnum<XrHand> for (ProximalPhalanx, Middle) {
    fn into_enum() -> XrHand {
        XrHand::MiddleProximal
    }
}

impl IntoEnum<XrHand> for (IntermediatePhalanx, Middle) {
    fn into_enum() -> XrHand {
        XrHand::MiddleIntermediate
    }
}

impl IntoEnum<XrHand> for (DistalPhalanx, Middle) {
    fn into_enum() -> XrHand {
        XrHand::MiddleDistal
    }
}

impl IntoEnum<XrHand> for (Tip, Middle) {
    fn into_enum() -> XrHand {
        XrHand::MiddleTip
    }
}

// Ring swapped

impl IntoEnum<XrHand> for (Metacarpal, Ring) {
    fn into_enum() -> XrHand {
        XrHand::RingMetacarpal
    }
}

impl IntoEnum<XrHand> for (ProximalPhalanx, Ring) {
    fn into_enum() -> XrHand {
        XrHand::RingProximal
    }
}

impl IntoEnum<XrHand> for (IntermediatePhalanx, Ring) {
    fn into_enum() -> XrHand {
        XrHand::RingIntermediate
    }
}

impl IntoEnum<XrHand> for (DistalPhalanx, Ring) {
    fn into_enum() -> XrHand {
        XrHand::RingDistal
    }
}

impl IntoEnum<XrHand> for (Tip, Ring) {
    fn into_enum() -> XrHand {
        XrHand::RingTip
    }
}

// Little swapped

impl IntoEnum<XrHand> for (Metacarpal, Little) {
    fn into_enum() -> XrHand {
        XrHand::LittleMetacarpal
    }
}

impl IntoEnum<XrHand> for (ProximalPhalanx, Little) {
    fn into_enum() -> XrHand {
        XrHand::LittleProximal
    }
}

impl IntoEnum<XrHand> for (IntermediatePhalanx, Little) {
    fn into_enum() -> XrHand {
        XrHand::LittleIntermediate
    }
}

impl IntoEnum<XrHand> for (DistalPhalanx, Little) {
    fn into_enum() -> XrHand {
        XrHand::LittleDistal
    }
}

impl IntoEnum<XrHand> for (Tip, Little) {
    fn into_enum() -> XrHand {
        XrHand::LittleTip
    }
}

// Finger

impl IntoEnum<Finger> for Thumb {
    fn into_enum() -> Finger {
        Finger::Thumb
    }
}

impl IntoEnum<Finger> for Index {
    fn into_enum() -> Finger {
        Finger::Index
    }
}

impl IntoEnum<Finger> for Middle {
    fn into_enum() -> Finger {
        Finger::Middle
    }
}

impl IntoEnum<Finger> for Ring {
    fn into_enum() -> Finger {
        Finger::Ring
    }
}

impl IntoEnum<Finger> for Little {
    fn into_enum() -> Finger {
        Finger::Little
    }
}

// Finger Joints

impl IntoEnum<FingerJoint> for Metacarpal {
    fn into_enum() -> FingerJoint {
        FingerJoint::Metacarpal
    }
}

impl IntoEnum<FingerJoint> for ProximalPhalanx {
    fn into_enum() -> FingerJoint {
        FingerJoint::ProximalPhalanx
    }
}

impl IntoEnum<FingerJoint> for IntermediatePhalanx {
    fn into_enum() -> FingerJoint {
        FingerJoint::IntermediatePhalanx
    }
}

impl IntoEnum<FingerJoint> for DistalPhalanx {
    fn into_enum() -> FingerJoint {
        FingerJoint::DistalPhalanx
    }
}

impl IntoEnum<FingerJoint> for Tip {
    fn into_enum() -> FingerJoint {
        FingerJoint::Tip
    }
}
