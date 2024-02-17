use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;
pub use crate::XrView;

/// The defining [`Component`] for window entities,
/// Represents the transform of a window such as a smartphone using webxr.
///
/// See bundle for intended use.
#[derive(Component)]
pub struct XrWindow(pub u8);

#[derive(Bundle)]
pub struct XrWindowBundle {
    name: Name,
    camera_bundle: Camera3dBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    xr_view: XrView,
    xr_window: XrWindow,
}

impl XrWindowBundle {
    pub fn default(index: u8) -> XrWindowBundle {
        XrWindowBundle {
            name: Name::new("XrWindow_".to_string() + &index.to_string()),
            camera_bundle: Camera3dBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            xr_view: XrView(index),
            xr_window: XrWindow(index),
        }
    }
}
