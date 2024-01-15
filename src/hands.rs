use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::XrGenericHandedness;
pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

#[derive(Component, Reflect)]
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

#[derive(Component, Reflect, Default)]
pub struct XrHandJointRadius(pub Option<f32>);

pub mod hand_joint {
    use bevy::prelude::*;

    use crate::IntoEnum;

    use super::XrHand;

    pub trait GenericHandJoint: Component + Reflect + IntoEnum<XrHand> + Default {}

    #[derive(Component, Reflect, Default)]
    pub struct Forearm;

    #[derive(Component, Reflect, Default)]
    pub struct Wrist;

    #[derive(Component, Reflect, Default)]
    pub struct Palm;
}

pub mod finger {
    use bevy::prelude::*;

    use crate::IntoEnum;

    #[derive(Component, Reflect)]
    pub enum Finger {
        Thumb,
        Index,
        Middle,
        Ring,
        Little,
    }

    pub trait GenericFinger: Component + Reflect + IntoEnum<Finger> + Default {}

    #[derive(Component, Reflect, Default)]
    pub struct Thumb;

    #[derive(Component, Reflect, Default)]
    pub struct Index;

    #[derive(Component, Reflect, Default)]
    pub struct Middle;

    #[derive(Component, Reflect, Default)]
    pub struct Ring;

    #[derive(Component, Reflect, Default)]
    pub struct Little;
}

pub mod finger_joint {
    use bevy::prelude::*;

    use crate::IntoEnum;

    #[derive(Component, Reflect)]
    pub enum FingerJoint {
        Metacarpal,
        ProximalPhalanx,
        IntermediatePhalanx,
        DistalPhalanx,
        Tip,
    }

    pub trait GenericFingerJoint: Component + Reflect + IntoEnum<FingerJoint> + Default {}

    pub trait FingerJointTrait {
        fn finger_joint() -> FingerJoint;
    }

    #[derive(Component, Reflect, Default)]
    pub struct Metacarpal;

    #[derive(Component, Reflect, Default)]
    pub struct ProximalPhalanx;

    #[derive(Component, Reflect, Default)]
    pub struct IntermediatePhalanx;

    #[derive(Component, Reflect, Default)]
    pub struct DistalPhalanx;

    #[derive(Component, Reflect, Default)]
    pub struct Tip;
}

///
/// Bundles
///

#[derive(Bundle)]
pub struct HandJointBundle<Handedness: XrGenericHandedness, HandJoint: GenericHandJoint> {
    pub name: Name,
    pub spatial_bundle: SpatialBundle,
    pub xr_local: XrLocal,
    pub xr_active: XrActive,
    pub handedness: Handedness,
    pub handedness_enum: XrHandedness,
    pub hand_joint: HandJoint,
    pub hand: XrHand,
    pub hand_joint_radius: XrHandJointRadius,
}

impl<Handedness: XrGenericHandedness, HandJoint: GenericHandJoint> Default
    for HandJointBundle<Handedness, HandJoint>
{
    fn default() -> Self {
        let handedness = Handedness::default();
        let hand_joint = HandJoint::default();
        let name = "XrHand_".to_string()
            + handedness.reflect_type_ident().unwrap()
            + hand_joint.reflect_type_ident().unwrap();
        HandJointBundle {
            name: Name::new(name),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            handedness,
            handedness_enum: Handedness::into_enum(),
            hand_joint,
            hand: HandJoint::into_enum(),
            hand_joint_radius: XrHandJointRadius(None),
        }
    }
}

#[derive(Bundle)]
pub struct FingerJointBundle<
    Handedness: XrGenericHandedness,
    Finger: GenericFinger,
    Joint: GenericFingerJoint,
> where
    (Finger, Joint): IntoEnum<XrHand>,
{
    pub name: Name,
    pub spatial_bundle: SpatialBundle,
    pub xr_local: XrLocal,
    pub xr_active: XrActive,
    pub handedness: Handedness,
    pub handedness_enum: XrHandedness,
    pub finger: Finger,
    pub finger_enum: finger::Finger,
    pub joint: Joint,
    pub joint_enum: FingerJoint,
    pub hand: XrHand,
    pub hand_joint_radius: XrHandJointRadius,
}

impl<Handedness: XrGenericHandedness, Finger: GenericFinger, Joint: GenericFingerJoint> Default
    for FingerJointBundle<Handedness, Finger, Joint>
where
    (Finger, Joint): IntoEnum<XrHand>,
{
    fn default() -> Self {
        let handedness = Handedness::default();
        let finger = Finger::default();
        let joint = Joint::default();
        let name = "XrHand_".to_string()
            + handedness.reflect_type_ident().unwrap()
            + finger.reflect_type_ident().unwrap()
            + joint.reflect_type_ident().unwrap();
        FingerJointBundle {
            name: Name::new(name),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            handedness,
            handedness_enum: Handedness::into_enum(),
            finger,
            finger_enum: Finger::into_enum(),
            joint,
            joint_enum: Joint::into_enum(),
            hand: <(Finger, Joint)>::into_enum(),
            hand_joint_radius: XrHandJointRadius(None),
        }
    }
}

use finger::*;
use finger_joint::*;
use hand_joint::*;

///
/// GenericTraits
///

impl GenericHandJoint for Forearm {}

impl GenericHandJoint for Wrist {}

impl GenericHandJoint for Palm {}

impl GenericFinger for Thumb {}

impl GenericFinger for Index {}

impl GenericFinger for Middle {}

impl GenericFinger for Ring {}

impl GenericFinger for Little {}

impl GenericFingerJoint for Metacarpal {}

impl GenericFingerJoint for ProximalPhalanx {}

impl GenericFingerJoint for IntermediatePhalanx {}

impl GenericFingerJoint for DistalPhalanx {}

impl GenericFingerJoint for Tip {}

///
/// IntoEnum
///
///
use crate::IntoEnum;

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
