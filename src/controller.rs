use bevy::prelude::*;

pub use crate::space::XrOrigin;
use crate::IntoEnum;
pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::Handedness;
pub use crate::handedness::LeftHanded;
pub use crate::handedness::RightHanded;

pub use crate::tracked::XrTrackedObject;

use crate::handedness::HandednessMarker;

/// The defining [`Component`] for entities that represent controllers.
/// Represents the transform of a controller.
///
/// The hand of the controller is defined by the [`XrHandedness`] and either the [`XrLeft`] or [`XrRight`] components.
///
/// Controller entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`] and [`XrTrackedObject`] component.
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
pub enum XrController {
    Right,
    Left,
    Other(u8),
}

#[derive(Bundle)]
pub struct XrControllerBundle<Handed: HandednessMarker> {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    handedness: Handed,
    handedness_enum: Handedness,
    xr_controller: XrController,
    xr_tracked_object: XrTrackedObject,
}

impl<Handed: HandednessMarker + IntoEnum<XrController>> XrControllerBundle<Handed> {
    pub fn default(index: u8) -> Self {
        let handedness = Handed::default();
        let name = "XrController_".to_string() + handedness.reflect_type_ident().unwrap();
        XrControllerBundle {
            name: Name::new(name),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            handedness,
            handedness_enum: Handed::into_enum(),
            xr_controller: Handed::into_enum(),
            xr_tracked_object: XrTrackedObject(index),
        }
    }
}

#[derive(Bundle)]
pub struct XrControllerHandlessBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    xr_controller: XrController,
    xr_tracked_object: XrTrackedObject,
}

impl XrControllerHandlessBundle {
    pub fn default(index: u8) -> XrControllerHandlessBundle {
        XrControllerHandlessBundle {
            name: Name::new("XrController_Other(".to_string() + &index.to_string() + ")"),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            xr_controller: XrController::Other(index),
            xr_tracked_object: XrTrackedObject(index),
        }
    }
}

impl IntoEnum<XrController> for LeftHanded {
    fn into_enum() -> XrController {
        XrController::Left
    }
}

impl IntoEnum<XrController> for RightHanded {
    fn into_enum() -> XrController {
        XrController::Right
    }
}
