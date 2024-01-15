use bevy::prelude::*;

use crate::{
    handedness::XrGenericHandedness,
    hands::{
        finger::*,
        finger_joint::{FingerJoint, GenericFingerJoint, Metacarpal, ProximalPhalanx},
        hand_joint::{Palm, Wrist},
        HandJointBundle, XrHand, XrHandJointRadius,
    },
    IntoEnum, XrActive, XrLocal,
};

pub fn draw_hand_gizmos(
    joint: Query<(&GlobalTransform, Option<&XrHandJointRadius>), With<XrHand>>,
    mut gizmos: Gizmos,
) {
    for (transform, radius) in joint.iter() {
        let radius = {
            if let Some(XrHandJointRadius(Some(radius))) = radius {
                *radius
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

pub fn substitute_local_palm<Handedness: XrGenericHandedness>(
    wrist: Query<
        (Entity, &GlobalTransform, &XrActive),
        (With<XrLocal>, With<Handedness>, With<Wrist>),
    >,
    mut palm: Query<(&mut Transform, &mut XrActive), (With<XrLocal>, With<Handedness>, With<Palm>)>,
    joints: Query<
        (&GlobalTransform, &XrActive),
        (
            With<XrLocal>,
            With<Handedness>,
            Without<Thumb>,
            Or<(With<Metacarpal>, With<ProximalPhalanx>)>,
        ),
    >,
    mut commands: Commands,
) {
    if let Ok((wrist, wrist_transform, wrist_active)) = wrist.get_single() {
        if wrist_active.0 & !joints.is_empty() & joints.iter().all(|(_, active)| active.0) {
            // Calculate average for palm transform:

            let count = joints.iter().count() as f32;
            let translation = joints
                .iter()
                .map(|(transform, _)| transform.reparented_to(wrist_transform).translation)
                .sum::<Vec3>()
                / count;
            let rotation = joints
                .iter()
                .map(|(transform, _)| transform.reparented_to(wrist_transform).rotation)
                .sum::<Quat>()
                / count;
            let rotation = rotation.normalize();

            // Update or spawn palm entity.

            if let Ok((mut transform, mut active)) = palm.get_single_mut() {
                transform.translation = translation;
                transform.rotation = rotation;
                active.0 = true;
            } else {
                let entity = commands
                    .spawn(HandJointBundle::<Handedness, Palm> {
                        spatial_bundle: SpatialBundle {
                            transform: Transform::from_translation(translation)
                                .with_rotation(rotation),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                commands.entity(wrist).add_child(entity);
            }
        }
    }

    for (_, mut active) in palm.iter_mut() {
        active.0 = false;
    }
}

//example
fn test<Finger: GenericFinger, Joint: GenericFingerJoint>()
where
    (Finger, Joint): IntoEnum<XrHand>,
{
    let test = <(Finger, Joint)>::into_enum();
}
