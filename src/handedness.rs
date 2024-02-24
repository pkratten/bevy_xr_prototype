use bevy::prelude::*;

use crate::IntoEnum;

/// The defining [`Component`] for entities that belong to one side of the body.
/// Represents the left or right hand side.
///
/// As the amount of controllers can surpass the amount of hands the other [`XrHandedness::Other`] option is available including an index.
///
/// This [`Component`] allows for querying entities of both hands while getting the handedness information. For only querying one side [`XrLeft`] and [`XrRight`] components are available.
///
/// This component should be spawned with entities that belong to one side of the body.
#[derive(Component)]
pub enum Handedness {
    Right,
    Left,
}

pub trait HandednessMarker: Component + Reflect + IntoEnum<Handedness> + Default + Sized
where
    Self: Sized,
{
}

/// The defining [`Component`] for entities that belong to left side of the body.
/// Represents the left hand side.
///
/// This [`Component`] allows for querying entities of the left hand side. For querying both sides at the same time the [`XrHandedness`] component is available.
///
/// This component should be spawned with entities that belong to the left side of the body.
#[derive(Component, Reflect, Default)]
pub struct LeftHanded;

/// The defining [`Component`] for entities that belong to right side of the body.
/// Represents the right hand side.
///
/// This [`Component`] allows for querying entities of the right hand side. For querying both sides at the same time the [`XrHandedness`] component is available.
///
/// This component should be spawned with entities that belong to the right side of the body.
#[derive(Component, Reflect, Default)]
pub struct RightHanded;

//
// Traits
//

impl HandednessMarker for LeftHanded {}

impl HandednessMarker for RightHanded {}

impl IntoEnum<Handedness> for LeftHanded {
    fn into_enum() -> Handedness {
        Handedness::Left
    }
}

impl IntoEnum<Handedness> for RightHanded {
    fn into_enum() -> Handedness {
        Handedness::Right
    }
}

pub trait HandedTransform<Handed, Output> {
    fn inward(&self, handedness: Handed) -> Output;
    fn outward(&self, handedness: Handed) -> Output;
}

impl HandedTransform<Handedness, Direction3d> for Transform {
    fn inward(&self, handedness: Handedness) -> Direction3d {
        match handedness {
            Handedness::Left => self.right(),
            Handedness::Right => self.left(),
        }
    }
    fn outward(&self, handedness: Handedness) -> Direction3d {
        match handedness {
            Handedness::Left => self.left(),
            Handedness::Right => self.right(),
        }
    }
}

impl HandedTransform<LeftHanded, Direction3d> for Transform {
    fn inward(&self, _handedness: LeftHanded) -> Direction3d {
        self.right()
    }
    fn outward(&self, _handedness: LeftHanded) -> Direction3d {
        self.left()
    }
}

impl HandedTransform<RightHanded, Direction3d> for Transform {
    fn inward(&self, _handedness: RightHanded) -> Direction3d {
        self.left()
    }
    fn outward(&self, _handedness: RightHanded) -> Direction3d {
        self.right()
    }
}

impl HandedTransform<Handedness, Vec3> for GlobalTransform {
    fn inward(&self, handedness: Handedness) -> Vec3 {
        match handedness {
            Handedness::Left => self.right(),
            Handedness::Right => self.left(),
        }
    }
    fn outward(&self, handedness: Handedness) -> Vec3 {
        match handedness {
            Handedness::Left => self.left(),
            Handedness::Right => self.right(),
        }
    }
}

impl HandedTransform<LeftHanded, Vec3> for GlobalTransform {
    fn inward(&self, _handedness: LeftHanded) -> Vec3 {
        self.right()
    }
    fn outward(&self, _handedness: LeftHanded) -> Vec3 {
        self.left()
    }
}

impl HandedTransform<RightHanded, Vec3> for GlobalTransform {
    fn inward(&self, _handedness: RightHanded) -> Vec3 {
        self.left()
    }
    fn outward(&self, _handedness: RightHanded) -> Vec3 {
        self.right()
    }
}

#[cfg(notes)]
mod notes {
    /// This component is debatable as I don't know if openxr or other runtimes supply this information.
    pub struct XrMainHand;
    /// This component is debatable as I don't know if openxr or other runtimes supply this information.
    pub struct XrOffHand;
}
