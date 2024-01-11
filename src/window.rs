use bevy::prelude::*;

use crate::space::XrOrigin;
use crate::XrActive;
use crate::XrLocal;

/// The defining [`Component`] for window entities,
/// Represents the transform of a window such as a smartphone using webxr.
///
/// Window entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`Camera3dBundle`] or similar.
#[derive(Component)]
pub struct XrWindow(pub usize);
