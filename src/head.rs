//! [`Component`]s for declaring entities which deffine the head and eyes in an xr application.

use bevy::prelude::*;

use crate::space::XrOrigin;
use crate::XrActive;
use crate::XrLocal;

/// The defining [`Component`] for head entities,
/// Represents the bone transform of the head.
///
/// Head entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component)]
pub struct XrHead;

/// The defining [`Component`] for headset entities,
/// Represents the transform of the headset.
///
/// Headset entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`].
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component)]
pub struct XrHeadset;

pub use crate::handedness::XrHandedness;
pub use crate::handedness::XrLeft;
pub use crate::handedness::XrRight;

/// The defining [`Component`] for eye entities,
/// Represents the transform an eye.
///
/// Eye entities should be indexed per handedness. Meaning there should be two [`XrEye`] entities per index.
///
/// Head entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`]. TODO: Check if this is always the case or if eyes are sometimes parented to the head.
///
/// This component should be spawned including a  [`Camera3dBundle`] or similar and should be marked by a [`XrHandedness`] and one of the [`XrLeft`] or [`XrRight`] components.
#[derive(Component)]
pub struct XrEye(pub usize);
