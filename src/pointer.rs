use bevy::prelude::*;

pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::Handedness;
pub use crate::handedness::LeftHanded;
pub use crate::handedness::RightHanded;

/// The defining [`Component`] for pointer entities,
/// Represents the transform for a variety of pointers that can be used for raycasting. Pointing at things in xr has become so universal that it should be covered in this crate. Which pointer to interact with should be decided at runtime and depending on pointer availability and interaction. A good example is the hololens and the mixed reality toolkit.
///
/// A [`XrPointer`] can be attached to all relevant tracked entities and can be more specified by other [`Component`] markers such as handedness or controller.
///
/// As the [`Component`] is an enum it is easy to iterate over the pointers on interaction. Interaction could be triggered by various actions such as a button or the pinch guesture.
///
/// This component should be spawned with another xr component or parented to another xr component including a [`XrActive`].
#[derive(Component)]
pub enum XrPointer {
    Head,
    Eye,
    Hand,
    Controller,
    Window,
    Other(usize),
}

/// Still need to figure out how to implement the interaction. Either as an event or as an input.
mod notes {
    use bevy::prelude::*;
    #[derive(Event)]
    struct Event {}
    struct Select(bool);
    struct Back(bool); //?
}
