use bevy::prelude::*;

use super::hands::Finger;

#[derive(Event)]
enum GestureHand {
    Open,
    Flat,
    Fist,
    Pinch(Finger),
    ThumbsUp,
    Point,
    MiddleFinger,
    Ok,
    Rocking,
    SmallFinger,
    Grip,
}

struct Confidence(f32);
