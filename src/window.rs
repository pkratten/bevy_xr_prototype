use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

/// The defining [`Component`] for window entities,
/// Represents the transform of a window such as a smartphone using webxr.
///
/// Window entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`Camera3dBundle`] or similar.
#[derive(Component)]
pub struct XrWindow(pub usize);
