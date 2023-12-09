use bevy::prelude::*;

enum TrackedObject{
    LeftController,
    RightController,
    Other(usize),
}