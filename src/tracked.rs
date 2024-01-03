use bevy::prelude::*;

#[derive(Component)]
enum XrTrackedObject {
    Headset,
    LeftController,
    RightController,
    Other(usize),
}

mod notes {
    struct XrTrackedMesh;
    struct XrTrackedImage;
    struct XrTrackedMarker;
    struct XrTrackedAnchor;
    struct XrTrackedFurniture;
    struct XrTrackedWall;
    struct XrTrackedWindow;
    struct XrTrackedDoor;
}
