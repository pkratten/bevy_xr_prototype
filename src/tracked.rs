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

/// Ideas to support other things in the future that could be seperate from XrTrackedObject.
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
