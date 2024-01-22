use bevy::prelude::*;

pub use crate::space::XrOrigin;
pub use crate::XrActive;
pub use crate::XrLocal;

pub use crate::handedness::Handedness;
use crate::handedness::HandednessMarker;
pub use crate::handedness::LeftHanded;
pub use crate::handedness::RightHanded;

use crate::tracked::XrTrackedObject;

/// The defining [`Component`] for entities that represent controllers.
/// Represents the transform of a controller.
///
/// The hand of the controller is defined by the [`XrHandedness`] and either the [`XrLeft`] or [`XrRight`] components.
///
/// Controller entities should be parented to a [`XrOrigin`] entity and include a [`XrActive`] and [`XrTrackedObject`] component.
///
/// This component should be spawned including a  [`SpatialBundle`] or similar.
#[derive(Component)]
pub struct XrController;

#[derive(Bundle)]
pub struct XrControllerBundle<Handed: HandednessMarker> {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    handedness: Handed,
    handedness_enum: Handedness,
    xr_controller: XrController,
    xr_tracked_object: XrTrackedObject,
}

impl<Handed: HandednessMarker> Default for XrControllerBundle<Handed> {
    fn default() -> Self {
        let handedness = Handed::default();
        let name = "XrController_".to_string() + handedness.reflect_type_ident().unwrap();
        XrControllerBundle {
            name: Name::new(name),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            handedness,
            handedness_enum: Handed::into_enum(),
            xr_controller: XrController,
            xr_tracked_object: XrTrackedObject::LeftController,
        }
    }
}

#[derive(Bundle)]
pub struct XrControllerHandlessBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    xr_local: XrLocal,
    xr_active: XrActive,
    xr_controller: XrController,
    xr_tracked_object: XrTrackedObject,
}

impl XrControllerHandlessBundle {
    pub fn default(index: usize) -> XrControllerHandlessBundle {
        XrControllerHandlessBundle {
            name: Name::new("XrController_Other(".to_string() + &index.to_string() + ")"),
            spatial_bundle: SpatialBundle::default(),
            xr_local: XrLocal,
            xr_active: XrActive(true),
            xr_controller: XrController,
            xr_tracked_object: XrTrackedObject::Other(index),
        }
    }
}

/// This enum is entended for the bevy_input crate that still needs to be implemented below. The intention is to have an Controller_Input and an Controller_Touched variant to cover the information presented by most xr hardware.
pub enum XrControllerInput {
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

mod notes {
    // This enum is not going to find use as the bevy_input crate doesn't cover three state input.

    enum InputState {
        None,
        Touched,
        Pressed,
    }
    /// This enum is covered by the button and axis options of the bevy_input crate.

    //Needs rework
    enum InputValue {
        None,
        Boolean(bool),
        Analog(f32),
    }
}
