use bevy::prelude::*;

use crate::space::XrOrigin;
use crate::XrActive;
use crate::XrLocal;

/// The defining [`Component`] for any tracked entities,
/// Represents the transform of an object such as a headset, controller or puck.
///
/// Tracked entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`]..
#[derive(Component)]
enum XrTrackedObject {
    Headset,
    LeftController,
    RightController,
    Other(usize),
}

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
