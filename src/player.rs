use std::collections::HashMap;

use bevy::{core_pipeline::bloom::BloomSettings, input::keyboard::KeyboardInput, prelude::*};
use bevy_rapier3d::{dynamics::{CoefficientCombineRule, Damping, LockedAxes, RigidBody, Velocity}, geometry::{Collider, Friction}, pipeline::QueryFilter, plugin::RapierContext};

use crate::{
    block,
    camera::PlayerCamera,
    chunk::{self, Chunk},
    entity::{self, Entity, EntityType, Species},
    universe::Universe,
    utils
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub can_jump: bool, // resets to true when touching ground
    pub targeted_block: Vec3,
    pub targeted_normal: Vec3,

    pub jump_vel: f32,
    pub sprint_multiplier: f32
}
impl Entity for Player {
    const ENTITY_TYPE: EntityType = EntityType::Player;
    const SPECIES: Species = Species::Player;

    fn new() -> Player {
        Player {
            speed: 2.0,
            // can_jump: false,
            can_jump: true,

            targeted_block: Vec3::ZERO,
            targeted_normal: Vec3::ZERO,

            jump_vel: 5.0,
            sprint_multiplier: 2.0
        }
    }
}

impl Player {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>
    ) {
        let cam = commands.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::FRAC_PI_2 * 0.8f32,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
            BloomSettings::NATURAL,
            PlayerCamera {}
        ))
        .id();

        let player_mesh = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
                transform: Transform::from_xyz(0.0, 0.4, 0.0),
                ..default()
            },
            Collider::capsule_y(0.5, 0.4),
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_Z
        )).id();
        
        commands.spawn((
            TransformBundle {
                local: Transform::from_xyz(0.0, 5.0, 0.0),
                ..default()
            },
            Player::new(),
            RigidBody::Dynamic,
            Velocity { ..default() },
            Friction {
                coefficient: 5.0,
                combine_rule: CoefficientCombineRule::Max
            },
            Damping {
                linear_damping: 1.0,
                angular_damping: 1.0
            },
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_Z
        ))
        .add_child(cam)
        .add_child(player_mesh);
    }


    pub fn movement(
        mut q: Query<(&mut Velocity, &Transform, &mut Player)>,
        key_input: Res<ButtonInput<KeyCode>>
    ) {
        let (mut vel, transform, mut player) = q.get_single_mut().unwrap();

        let mut final_vel = Vec3::ZERO;
        // dereferencing to a vec3 : https://docs.rs/bevy/latest/i686-pc-windows-msvc/bevy/math/primitives/struct.Direction3d.html#deref-methods-Vec3
        if key_input.pressed(KeyCode::KeyW) {
            final_vel += *transform.forward();
        }
        if key_input.pressed(KeyCode::KeyS) {
            final_vel += *transform.back();
        }
        if key_input.pressed(KeyCode::KeyA) {
            final_vel += *transform.left();
        }
        if key_input.pressed(KeyCode::KeyD) {
            final_vel += *transform.right();
        }
        final_vel = final_vel.normalize_or_zero();
        final_vel *= player.speed;

        if key_input.pressed(KeyCode::ShiftLeft) {
            final_vel *= player.sprint_multiplier;
        }

        if key_input.pressed(KeyCode::Space) && player.can_jump {
            final_vel.y += player.jump_vel;

            // player.can_jump = false;
        }


        vel.linvel.x = utils::reasonably_add_vec(vel.linvel.x, final_vel.x);
        vel.linvel.y = utils::reasonably_add_vec(vel.linvel.y, final_vel.y);
        vel.linvel.z = utils::reasonably_add_vec(vel.linvel.z, final_vel.z);
    }

    pub fn raycast_block_target(
        rapier_context: Res<RapierContext>,
        cam: Query<&GlobalTransform, With<Camera>>,
        mut p_rb: Query<(bevy::prelude::Entity, &mut Player)>,
        mut debug: Query<&mut Transform, (With<entity::Debug>, Without<Camera>)>,
    ) {
        let cam = cam.get_single().unwrap();
        let ray_pos = cam.translation();
        let ray_dir = cam.forward().into();
        let max_toi = 4.0;
        let solid = true;

        let (player_entity, mut player) = p_rb.get_single_mut().unwrap();

        let filter = QueryFilter::default().exclude_rigid_body(player_entity);
    
        // if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        //     let hit_point = ray_pos + ray_dir * toi;
        // }
    
        if let Some((_, intersection)) = rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
        {
            let mut hit_point = Vec3::new(
                utils::slightly_round_floats(intersection.point.x, None),
                utils::slightly_round_floats(intersection.point.y, None),
                utils::slightly_round_floats(intersection.point.z, None)
            );
            let hit_normal = intersection.normal;

            hit_point -= hit_normal * 0.5;

            hit_point.x = (hit_point.x + 0.5).round() - 0.5;
            hit_point.y = (hit_point.y + 0.5).round() - 0.5;
            hit_point.z = (hit_point.z + 0.5).round() - 0.5;

            debug.get_single_mut().unwrap().translation = hit_point;

            player.targeted_block = hit_point.clone();
            player.targeted_normal = hit_normal.clone();
        }
    }

    pub fn mouse_events(
        // mut commands: Commands,
        player: Query<&Player>,
        mouse_input: Res<ButtonInput<MouseButton>>,

        mut universe: ResMut<Universe>,
        mut chunk: Query<&mut Chunk>,
    ) {
        let player = player.get_single().unwrap();

        if mouse_input.just_pressed(MouseButton::Left) {
            let chunk_pos = Chunk::real_to_chunk(player.targeted_block);
            chunk::Chunk::replace_block(
                chunk_pos,
                &mut universe,
                &mut chunk,
                block::Block {
                    species: block::Species::Air
                },
                chunk::Chunk::real_to_tile(player.targeted_block)
            );
        }
        if mouse_input.just_pressed(MouseButton::Right) {
            let chunk_pos = Chunk::real_to_chunk(player.targeted_block + player.targeted_normal);
            chunk::Chunk::replace_block(
                chunk_pos,
                &mut universe,
                &mut chunk,
                block::Block {
                    species: block::Species::Dirt
                },
                chunk::Chunk::real_to_tile(player.targeted_block + player.targeted_normal)
            );
        }
    }
}
