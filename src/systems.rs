use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::{
    handedness::HandednessMarker,
    hands::{
        finger::*,
        finger_joint::{Metacarpal, ProximalPhalanx},
        hand_joint::{Palm, Wrist},
        Hand, HandJointBundle, HandJointRadius,
    },
    XrActive, XrLocal,
};

pub fn draw_hand_gizmos(
    joint: Query<(&GlobalTransform, Option<&HandJointRadius>), With<Hand>>,
    mut gizmos: Gizmos,
) {
    for (transform, radius) in joint.iter() {
        let radius = {
            if let Some(HandJointRadius(Some(radius))) = radius {
                *radius
            } else {
                0.0055
            }
        };

        let (scale, _rotation, translation) = transform.to_scale_rotation_translation();

        let radius = radius * scale.length();

        gizmos.circle(
            translation,
            Direction3d::new_unchecked(transform.forward()),
            radius,
            Color::WHITE,
        );
        gizmos.line(
            translation,
            translation + transform.forward() * radius,
            Color::BLUE,
        );
        gizmos.line(
            translation,
            translation + transform.right() * radius,
            Color::RED,
        );
        gizmos.line(
            translation,
            translation + transform.up() * radius,
            Color::GREEN,
        );
    }
}

/// TODO: Make rotation average of Metacarpals. Tweak position by filtering the joints further.
pub fn substitute_local_palm<Handedness: HandednessMarker>(
    mut palm: Query<
        (Entity, &mut Transform, &mut XrActive),
        (With<XrLocal>, With<Handedness>, With<Palm>),
    >,

    wrist: Query<
        (Entity, &GlobalTransform, &XrActive),
        (With<XrLocal>, With<Handedness>, With<Wrist>, Without<Palm>),
    >,
    joints: Query<
        (&GlobalTransform, &XrActive),
        (
            With<XrLocal>,
            With<Handedness>,
            Without<Thumb>,
            Without<Little>,
            Or<(With<Metacarpal>, With<ProximalPhalanx>)>,
            Without<Palm>,
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

            match palm.get_single_mut() {
                Ok((_, mut transform, mut active)) => {
                    transform.translation = translation;
                    transform.rotation = rotation;
                    active.0 = true;
                }
                Err(QuerySingleError::MultipleEntities(_)) => {
                    let mut palms = palm.iter();
                    palms.next();
                    for (entity, _, _) in palms {
                        commands.entity(entity).despawn();
                    }
                }
                Err(QuerySingleError::NoEntities(_)) => {
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
                    // TODO: Debatable
                    commands.entity(wrist).add_child(entity);
                }
            }
        }
    }

    for (_, _, mut active) in palm.iter_mut() {
        active.0 = false;
    }
}
