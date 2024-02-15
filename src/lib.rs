//! # Draft for an universal abstraction crate for the elemental xr components for enabling cross-platform xr development with bevy.
//!
//! The notes modules are only intended as notes of possible features that might never be implemented, but are recorded to more meaningfully define the components that are implemented.
//!
//! TODO: Whats currently missing is the axis definition of hand joints. It should be either the openxr standard or a decision made from the different implementations such as unity, meta, windows mr, apple or ultraleap.
//!
//! TODO: Bundles for easy spawning.

use bevy::prelude::*;

pub mod controller;
pub mod controller_input;
pub mod handedness;
pub mod hands;
pub mod head;
pub mod pointer;
pub mod post_process_flip_y;
pub mod space;
pub mod systems;
pub mod tracked;
pub mod window;

/// This [`Resource`] defines the type of xr experience.
#[derive(Resource)]
pub enum XrMode {
    /// VR defines an experience where the viewer does not get to see or interact with the real world around him visually.
    VR,
    /// AR defines an experience where the viewer does get to see or interact with the real world around him.
    AR,
    /// None defines any situations without a xr experience such as the inline session of webxr or an openxr session without a headset.
    None,
}

/// The defining [`Component`] for xr entities which are controlled by the local runtime.
///
///
/// This marker represents the xr entities which belong to the local device, or player.
///
/// By exluding the component all components of this crate can be used for other entities that might use similae setups such as remote players.
///
/// This component should be spawned with every entity that is managed by the xr platform specific crate.
#[derive(Component)]
pub struct XrLocal;

/// The defining [`Component`] which indicates that the entity is currently tracked.
///
///
/// When tracked objects loose tracking or a camera is not requested by the runtime anymore this component is set to false.
///
/// This component enables systems to know if the current state is active.
///
/// This component should be spawned with every entity that is managed by the xr platform specific crate.
///
/// TODO: This could be an enum specifing the state of the xr object.
#[derive(Component)]
pub struct XrActive(pub bool);

/// The defining [`Component`] which indicates that the entity is a xr managed view.
///
/// This component should be spawned with every entity that is managed by the xr platform and has a camera that renders a tracked view such as an [`XrEye`] or an [`XrWindow`].
///
/// The index of the view should be recorded in this component.
#[derive(Component)]
pub struct XrView(pub u8);

pub trait IntoEnum<T> {
    fn into_enum() -> T;
}
