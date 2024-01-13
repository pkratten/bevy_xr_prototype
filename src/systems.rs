use bevy::prelude::*;

use crate::{
    handedness::{XrHandedness, XrLeft},
    hands::{
        finger::*,
        finger_joint::{FingerJoint, Metacarpal, ProximalPhalanx},
        hand_part::Palm,
        XrHand, XrHandJointRadius,
    },
    tracked::XrOrigin,
    IntoEnum, XrActive, XrLocal,
};

pub fn draw_hand_gizmos(
    joint: Query<(&GlobalTransform, Option<&XrHandJointRadius>), With<XrHand>>,
    mut gizmos: Gizmos,
) {
    for (transform, radius) in joint.iter() {
        let radius = {
            if let Some(radius) = radius {
                radius.0
            } else {
                0.007
            }
        };
        gizmos.circle(
            transform.translation(),
            transform.up(),
            radius,
            Color::WHITE,
        );
        gizmos.line(
            transform.translation(),
            transform.translation() + transform.forward() * radius,
            Color::BLUE,
        );
        gizmos.line(
            transform.translation(),
            transform.translation() + transform.right() * radius,
            Color::RED,
        );
        gizmos.line(
            transform.translation(),
            transform.translation() + transform.up() * radius,
            Color::GREEN,
        );
    }
}

pub fn substitute_local_palm<Handedness: Component + IntoEnum<XrHandedness>>(
    origin: Query<(Entity, &GlobalTransform), (With<XrLocal>, With<XrOrigin>)>,
    palm: Query<(Entity, &Transform, &XrActive), (With<XrLocal>, With<Handedness>, With<Palm>)>,
    joints: Query<
        (&GlobalTransform, &XrActive),
        (
            With<XrLocal>,
            With<Handedness>,
            With<Finger>,
            Without<Thumb>,
            Or<(With<Metacarpal>, With<ProximalPhalanx>)>,
        ),
    >,
    mut commands: Commands,
) {
    let test = Handedness::into_enum();

    todo!()
}

//example
fn test<Finger: Component + IntoEnum<Finger>, Joint: Component + IntoEnum<FingerJoint>>()
where
    (Finger, Joint): IntoEnum<XrHand>,
{
    let test = <(Finger, Joint)>::into_enum();
}
