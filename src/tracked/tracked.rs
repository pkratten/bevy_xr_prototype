use bevy::prelude::*;

enum TrackedObject {
    Headset,
    LeftController,
    RightController,
    Other(usize),
}
