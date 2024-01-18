use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::Handedness;
pub use crate::handedness::HandednessMarker;
pub use crate::handedness::LeftHanded;
pub use crate::handedness::RightHanded;

#[derive(Component, Reflect)]
pub enum Hand {
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
pub struct HandJointRadius(pub Option<f32>);

pub mod hand_joint {
    use bevy::prelude::*;

    use crate::IntoEnum;

    use super::Hand;

    pub trait HandJointMarker: Component + Reflect + IntoEnum<Hand> + Default
    where
        Self: Sized,
    {
    }

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

    pub trait FingerMarker: Component + Reflect + IntoEnum<Finger> + Default + Sized
    where
        Self: Sized,
    {
    }

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

    pub trait FingerJointMarker: Component + Reflect + IntoEnum<FingerJoint> + Default
    where
        Self: Sized,
    {
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
pub struct HandJointBundle<Handed: HandednessMarker, HandJoint: HandJointMarker> {
    pub name: Name,
    pub spatial_bundle: SpatialBundle,
    pub xr_local: XrLocal,
    pub xr_active: XrActive,
    pub handedness: Handed,
    pub handedness_enum: Handedness,
    pub hand_joint: HandJoint,
    pub hand: Hand,
    pub hand_joint_radius: HandJointRadius,
}

impl<Handed: HandednessMarker, HandJoint: HandJointMarker> Default
    for HandJointBundle<Handed, HandJoint>
{
    fn default() -> Self {
        let handedness = Handed::default();
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
            handedness_enum: Handed::into_enum(),
            hand_joint,
            hand: HandJoint::into_enum(),
            hand_joint_radius: HandJointRadius(None),
        }
    }
}

#[derive(Bundle)]
pub struct FingerJointBundle<
    Handed: HandednessMarker,
    Finger: FingerMarker,
    Joint: FingerJointMarker,
> where
    (Finger, Joint): IntoEnum<Hand>,
{
    pub name: Name,
    pub spatial_bundle: SpatialBundle,
    pub xr_local: XrLocal,
    pub xr_active: XrActive,
    pub handedness: Handed,
    pub handedness_enum: Handedness,
    pub finger: Finger,
    pub finger_enum: finger::Finger,
    pub joint: Joint,
    pub joint_enum: FingerJoint,
    pub hand: Hand,
    pub hand_joint_radius: HandJointRadius,
}

impl<Handed: HandednessMarker, Finger: FingerMarker, Joint: FingerJointMarker> Default
    for FingerJointBundle<Handed, Finger, Joint>
where
    (Finger, Joint): IntoEnum<Hand>,
{
    fn default() -> Self {
        let handedness = Handed::default();
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
            handedness_enum: Handed::into_enum(),
            finger,
            finger_enum: Finger::into_enum(),
            joint,
            joint_enum: Joint::into_enum(),
            hand: <(Finger, Joint)>::into_enum(),
            hand_joint_radius: HandJointRadius(None),
        }
    }
}

use finger::*;
use finger_joint::*;
use hand_joint::*;

///
/// GenericTraits
///

impl HandJointMarker for Forearm {}

impl HandJointMarker for Wrist {}

impl HandJointMarker for Palm {}

impl FingerMarker for Thumb {}

impl FingerMarker for Index {}

impl FingerMarker for Middle {}

impl FingerMarker for Ring {}

impl FingerMarker for Little {}

impl FingerJointMarker for Metacarpal {}

impl FingerJointMarker for ProximalPhalanx {}

impl FingerJointMarker for IntermediatePhalanx {}

impl FingerJointMarker for DistalPhalanx {}

impl FingerJointMarker for Tip {}

///
/// IntoEnum
///
///
use crate::IntoEnum;

// Hand

impl IntoEnum<Hand> for Forearm {
    fn into_enum() -> Hand {
        Hand::Forearm
    }
}

impl IntoEnum<Hand> for Wrist {
    fn into_enum() -> Hand {
        Hand::Wrist
    }
}

impl IntoEnum<Hand> for Palm {
    fn into_enum() -> Hand {
        Hand::Palm
    }
}

// Thumb

impl IntoEnum<Hand> for (Thumb, Metacarpal) {
    fn into_enum() -> Hand {
        Hand::ThumbMetacarpal
    }
}

impl IntoEnum<Hand> for (Thumb, ProximalPhalanx) {
    fn into_enum() -> Hand {
        Hand::ThumbProximal
    }
}

impl IntoEnum<Hand> for (Thumb, DistalPhalanx) {
    fn into_enum() -> Hand {
        Hand::ThumbDistal
    }
}

impl IntoEnum<Hand> for (Thumb, Tip) {
    fn into_enum() -> Hand {
        Hand::ThumbTip
    }
}

// Index

impl IntoEnum<Hand> for (Index, Metacarpal) {
    fn into_enum() -> Hand {
        Hand::IndexMetacarpal
    }
}

impl IntoEnum<Hand> for (Index, ProximalPhalanx) {
    fn into_enum() -> Hand {
        Hand::IndexProximal
    }
}

impl IntoEnum<Hand> for (Index, IntermediatePhalanx) {
    fn into_enum() -> Hand {
        Hand::IndexIntermediate
    }
}

impl IntoEnum<Hand> for (Index, DistalPhalanx) {
    fn into_enum() -> Hand {
        Hand::IndexDistal
    }
}

impl IntoEnum<Hand> for (Index, Tip) {
    fn into_enum() -> Hand {
        Hand::IndexTip
    }
}

// Middle

impl IntoEnum<Hand> for (Middle, Metacarpal) {
    fn into_enum() -> Hand {
        Hand::MiddleMetacarpal
    }
}

impl IntoEnum<Hand> for (Middle, ProximalPhalanx) {
    fn into_enum() -> Hand {
        Hand::MiddleProximal
    }
}

impl IntoEnum<Hand> for (Middle, IntermediatePhalanx) {
    fn into_enum() -> Hand {
        Hand::MiddleIntermediate
    }
}

impl IntoEnum<Hand> for (Middle, DistalPhalanx) {
    fn into_enum() -> Hand {
        Hand::MiddleDistal
    }
}

impl IntoEnum<Hand> for (Middle, Tip) {
    fn into_enum() -> Hand {
        Hand::MiddleTip
    }
}

// Ring

impl IntoEnum<Hand> for (Ring, Metacarpal) {
    fn into_enum() -> Hand {
        Hand::RingMetacarpal
    }
}

impl IntoEnum<Hand> for (Ring, ProximalPhalanx) {
    fn into_enum() -> Hand {
        Hand::RingProximal
    }
}

impl IntoEnum<Hand> for (Ring, IntermediatePhalanx) {
    fn into_enum() -> Hand {
        Hand::RingIntermediate
    }
}

impl IntoEnum<Hand> for (Ring, DistalPhalanx) {
    fn into_enum() -> Hand {
        Hand::RingDistal
    }
}

impl IntoEnum<Hand> for (Ring, Tip) {
    fn into_enum() -> Hand {
        Hand::RingTip
    }
}

// Little

impl IntoEnum<Hand> for (Little, Metacarpal) {
    fn into_enum() -> Hand {
        Hand::LittleMetacarpal
    }
}

impl IntoEnum<Hand> for (Little, ProximalPhalanx) {
    fn into_enum() -> Hand {
        Hand::LittleProximal
    }
}

impl IntoEnum<Hand> for (Little, IntermediatePhalanx) {
    fn into_enum() -> Hand {
        Hand::LittleIntermediate
    }
}

impl IntoEnum<Hand> for (Little, DistalPhalanx) {
    fn into_enum() -> Hand {
        Hand::LittleDistal
    }
}

impl IntoEnum<Hand> for (Little, Tip) {
    fn into_enum() -> Hand {
        Hand::LittleTip
    }
}

// Thumb swapped

impl IntoEnum<Hand> for (Metacarpal, Thumb) {
    fn into_enum() -> Hand {
        Hand::ThumbMetacarpal
    }
}

impl IntoEnum<Hand> for (ProximalPhalanx, Thumb) {
    fn into_enum() -> Hand {
        Hand::ThumbProximal
    }
}

impl IntoEnum<Hand> for (DistalPhalanx, Thumb) {
    fn into_enum() -> Hand {
        Hand::ThumbDistal
    }
}

impl IntoEnum<Hand> for (Tip, Thumb) {
    fn into_enum() -> Hand {
        Hand::ThumbTip
    }
}

// Index swapped

impl IntoEnum<Hand> for (Metacarpal, Index) {
    fn into_enum() -> Hand {
        Hand::IndexMetacarpal
    }
}

impl IntoEnum<Hand> for (ProximalPhalanx, Index) {
    fn into_enum() -> Hand {
        Hand::IndexProximal
    }
}

impl IntoEnum<Hand> for (IntermediatePhalanx, Index) {
    fn into_enum() -> Hand {
        Hand::IndexIntermediate
    }
}

impl IntoEnum<Hand> for (DistalPhalanx, Index) {
    fn into_enum() -> Hand {
        Hand::IndexDistal
    }
}

impl IntoEnum<Hand> for (Tip, Index) {
    fn into_enum() -> Hand {
        Hand::IndexTip
    }
}

// Middle swapped

impl IntoEnum<Hand> for (Metacarpal, Middle) {
    fn into_enum() -> Hand {
        Hand::MiddleMetacarpal
    }
}

impl IntoEnum<Hand> for (ProximalPhalanx, Middle) {
    fn into_enum() -> Hand {
        Hand::MiddleProximal
    }
}

impl IntoEnum<Hand> for (IntermediatePhalanx, Middle) {
    fn into_enum() -> Hand {
        Hand::MiddleIntermediate
    }
}

impl IntoEnum<Hand> for (DistalPhalanx, Middle) {
    fn into_enum() -> Hand {
        Hand::MiddleDistal
    }
}

impl IntoEnum<Hand> for (Tip, Middle) {
    fn into_enum() -> Hand {
        Hand::MiddleTip
    }
}

// Ring swapped

impl IntoEnum<Hand> for (Metacarpal, Ring) {
    fn into_enum() -> Hand {
        Hand::RingMetacarpal
    }
}

impl IntoEnum<Hand> for (ProximalPhalanx, Ring) {
    fn into_enum() -> Hand {
        Hand::RingProximal
    }
}

impl IntoEnum<Hand> for (IntermediatePhalanx, Ring) {
    fn into_enum() -> Hand {
        Hand::RingIntermediate
    }
}

impl IntoEnum<Hand> for (DistalPhalanx, Ring) {
    fn into_enum() -> Hand {
        Hand::RingDistal
    }
}

impl IntoEnum<Hand> for (Tip, Ring) {
    fn into_enum() -> Hand {
        Hand::RingTip
    }
}

// Little swapped

impl IntoEnum<Hand> for (Metacarpal, Little) {
    fn into_enum() -> Hand {
        Hand::LittleMetacarpal
    }
}

impl IntoEnum<Hand> for (ProximalPhalanx, Little) {
    fn into_enum() -> Hand {
        Hand::LittleProximal
    }
}

impl IntoEnum<Hand> for (IntermediatePhalanx, Little) {
    fn into_enum() -> Hand {
        Hand::LittleIntermediate
    }
}

impl IntoEnum<Hand> for (DistalPhalanx, Little) {
    fn into_enum() -> Hand {
        Hand::LittleDistal
    }
}

impl IntoEnum<Hand> for (Tip, Little) {
    fn into_enum() -> Hand {
        Hand::LittleTip
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
