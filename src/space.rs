use bevy::prelude::*;

pub use crate::XrActive;
pub use crate::XrLocal;

/// The defining [`Component`] for head entities,
/// Represents the origin transform of the xr runtime and all tracked xr entities are relative to this transform.
///
/// This transform can be moved to move the whole xr space.
///
/// This component should be spawned including a  [`SpatialBundle`] or similar and a [`XrActive`] component.
#[derive(Clone, Copy, Component)]
pub enum XrOrigin {
    /// For an origin that is positioned at the head of the person.
    View,
    /// For an origin that is positioned below the person who is seated or standing in place.
    Seat,
    /// For an origin that is positioned offset of a person at the center of a room.
    Room,
    /// For an origin without specified locality.
    Other,
}

mod notes {
    /// Is there other information obtainable of the space a xr session is taking place in?
    pub struct XrSpace;
    /// Intended for chaperone, guardian geometry. Could be a SpacialBundle with the index indication the index of the polygon vertx.
    pub struct XrBoundary(usize);
    /// Apple room scanning might find place here.
    pub struct XrRoom;
}
