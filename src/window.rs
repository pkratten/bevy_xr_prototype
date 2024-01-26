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
pub struct XrWindow(pub u8);

#[derive(Bundle)]
pub struct XrWindowBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
}

impl XrWindowBundle {
    pub fn default(index: u8) -> XrWindowBundle {
        XrWindowBundle {
            name: Name::new("XrWindow ".to_string() + &index.to_string()),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
        }
    }
}
