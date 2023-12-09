use bevy::prelude::*;

enum ControllerInput {
    A,
    B,
    X,
    Y,
    Stick,
    Pad,
    Trigger,
    Grip,
    Shoulder,
    Option,
    System,
    Other(usize),
}

enum ControllerHand {
    Left,
    Right,
}

enum InputState {
    None,
    Touched,
    Pressed,
}

enum InputValue {
    None,
    Boolean(bool),
    Analog(f32),
}
