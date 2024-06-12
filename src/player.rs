use bevy::{core_pipeline::bloom::BloomSettings, input::keyboard::KeyboardInput, prelude::*};
use bevy_rapier3d::{dynamics::{CoefficientCombineRule, Damping, LockedAxes, RigidBody, Velocity}, geometry::{Collider, Friction}};

use crate::{camera::PlayerCamera, chunk::Chunk, entity::{Entity, EntityType, Species}, universe::Universe, utils::reasonably_add_vec};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub can_jump: bool, // resets to true when touching ground

    pub jump_vel: f32,
    pub sprint_multiplier: f32
}
impl Entity for Player {
    const ENTITY_TYPE: EntityType = EntityType::Player;
    const SPECIES: Species = Species::Player;

    fn new() -> Player {
        Player {
            speed: 5.0,
            // can_jump: false,
            can_jump: true,


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
                transform: Transform::from_xyz(0.0, 0.9, 0.0),
                ..default()
            },
            BloomSettings::NATURAL,
            PlayerCamera {}
        ))
        .id();

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                ..default()
            },
            Player::new(),
            Collider::capsule_y(0.5, 0.4),
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
        .add_child(cam);
    }

    pub fn selection(
        mut u: ResMut<Universe>,
        mut p: Query<(&Transform, &mut Player)>,
        mut c: Query<&mut Chunk>,

        key_input: Res<ButtonInput<KeyCode>>
    ) {
        let (player_transform, mut player) = p.get_single_mut().unwrap();
        let chunk_pos = Chunk::tile_to_chunk_pos(player_transform.translation);

        match u.chunks.get_mut(&chunk_pos) {
            Some(chunk_entity) => {
                let c = c.get_mut(*chunk_entity).unwrap();

                let block = c.get_at(
                    player_transform.translation.x as i32,
                    player_transform.translation.y as i32 - 1,
                    player_transform.translation.z as i32,
                    &[[[None; 3]; 3]; 3]
                );

                println!("{block:?} in {chunk_pos:?}");
            },
            None => {}
        }
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
        final_vel *= player.speed;

        if key_input.pressed(KeyCode::ShiftLeft) {
            final_vel *= player.sprint_multiplier;
        }

        if key_input.pressed(KeyCode::Space) && player.can_jump {
            final_vel.y += player.jump_vel;

            // player.can_jump = false;
        }


        vel.linvel.x = reasonably_add_vec(vel.linvel.x, final_vel.x);
        vel.linvel.y = reasonably_add_vec(vel.linvel.y, final_vel.y);
        vel.linvel.z = reasonably_add_vec(vel.linvel.z, final_vel.z);
    }
}

