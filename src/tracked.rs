use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

/// The defining [`Component`] for any tracked entities,
/// Represents the transform of an object such as a headset, controller or puck.
///
/// Tracked entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`]..
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
pub struct XrTrackedObject(pub u8);

#[derive(Bundle)]
pub struct XrTrackedObjectBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    xr_tracked_object: XrTrackedObject,
}

impl XrTrackedObjectBundle {
    pub fn default(index: u8) -> XrTrackedObjectBundle {
        XrTrackedObjectBundle {
            name: Name::new("XrTrackedObject_".to_string() + &index.to_string()),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            xr_tracked_object: XrTrackedObject(index),
        }
    }
}

/// Ideas to support other things in the future that could be seperate from XrTrackedObject.
#[cfg(notes)]
mod notes {
    /// Vuforia?
    struct XrTrackedMesh;
    struct XrTrackedImage;
    struct XrTrackedMarker;
    struct XrTrackedAnchor;
    /// Apple?
    struct XrTrackedFurniture;
    /// ARCore? Apple?
    struct XrTrackedWall;
    struct XrTrackedWindow;
    struct XrTrackedDoor;
}
