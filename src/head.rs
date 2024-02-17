//! [`Component`]s for declaring entities which deffine the head and eyes in an xr application.

use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;
pub use crate::XrView;

/// The defining [`Component`] for head entities,
/// Represents the bone transform of the head.
///
/// Head entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component)]
pub struct XrHead;

/// The defining [`Component`] for headset entities,
/// Represents the transform of the headset.
///
/// Headset entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component)]
pub struct XrHeadset;

#[derive(Bundle)]
pub struct XrHeadsetBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_headset: XrHeadset,
    xr_local: XrLocal,
    xr_active: XrActive,
}

impl XrHeadsetBundle {
    pub fn default() -> XrHeadsetBundle {
        XrHeadsetBundle {
            name: Name::new("XrOrigin"),
            spatial_bundle: SpatialBundle::default(),
            xr_headset: XrHeadset,
            xr_local: XrLocal,
            xr_active: XrActive(true),
        }
    }
}

pub use crate::handedness::Handedness;
pub use crate::handedness::LeftHanded;
pub use crate::handedness::RightHanded;

use crate::handedness::HandednessMarker;

/// The defining [`Component`] for eye entities,
/// Represents the transform an eye.
///
/// Eye entities should be indexed per handedness. Meaning there should be two [`XrEye`] entities per index.
///
/// Head entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`]. TODO: Check if this is always the case or if eyes are sometimes parented to the head.
///
/// This component should be spawned including a  [`Camera3dBundle`] or similar and should be marked by a [`XrHandedness`] and one of the [`XrLeft`] or [`XrRight`] components.
#[derive(Component)]
pub struct XrEye(pub u8);

#[derive(Bundle)]
pub struct XrEyeBundle<Handed: HandednessMarker> {
    name: Name,
    camera_bundle: Camera3dBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    xr_view: XrView,
    handedness: Handed,
    handedness_enum: Handedness,
    xr_eye: XrEye,
}

impl<Handed: HandednessMarker> XrEyeBundle<Handed> {
    pub fn default(index: u8) -> XrEyeBundle<Handed> {
        let handedness = Handed::default();
        let name = "XrEye_".to_string()
            + handedness.reflect_type_ident().unwrap()
            + "_"
            + &index.to_string();
        XrEyeBundle {
            name: Name::new(name),
            camera_bundle: Camera3dBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            xr_view: XrView(index),
            handedness,
            handedness_enum: Handed::into_enum(),
            xr_eye: XrEye(index),
        }
    }
}
