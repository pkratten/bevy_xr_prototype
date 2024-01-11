use bevy::prelude::*;

use super::hands::Finger;

/// It would be nice to have a generalised gesture detection as events or input. As gesture detection is ongoing and has a confidence it is unclear if events or inputs are the right way to go. Further thinking and reading should be done here.
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
