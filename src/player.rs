use crate::{entity::{Entity, EntityType}, camera::PlayerCamera};
use bevy::{
    prelude::*,
    core_pipeline::bloom::BloomSettings
};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
}
impl Player {
    pub fn movement(
        // q_cam: Query<&Transform, With<PlayerCamera>>,
        mut q: Query<(&mut Velocity, &Transform, &Player)>,
        key_input: Res<Input<KeyCode>>
        // time: Res<Time>
    ) {
        let (mut velocity, transform, player) = q.get_single_mut().unwrap();

        if key_input.just_pressed(KeyCode::Space) {
            velocity.linvel.y = 5.0;
            // only set if touching the ground
        }

        let mut final_vel = Vec3::ZERO;
        if key_input.pressed(KeyCode::W) {
            final_vel += transform.forward();
        }
        if key_input.pressed(KeyCode::S) {
            final_vel += transform.back();
        }
        if key_input.pressed(KeyCode::A) {
            final_vel += transform.left();
        }
        if key_input.pressed(KeyCode::D) {
            final_vel += transform.right();
        }
        if final_vel == Vec3::ZERO {
            return;
        }
        final_vel.y = 0.;
        final_vel = final_vel.normalize();
        velocity.linvel.x = (final_vel * player.speed).x;
        velocity.linvel.z = (final_vel * player.speed).z;
    }

    pub fn spawn_player(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        // let cam = commands.spawn(Camera3dBundle {
        //     camera: Camera {
        //         hdr: true,
        //         ..default()
        //     },
        //     transform: Transform::from_xyz(0.0, 0.5, 0.0)/*.looking_at(Vec3::ZERO, Vec3::Y)*/,
        //     ..default()
        // })
        let cam = commands.spawn(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::FRAC_PI_2 * 0.8f32,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0)/*.looking_at(Vec3::ZERO, Vec3::Y)*/,
            ..default()
        })
        .insert(
            BloomSettings::NATURAL
        )
        .insert(
            PlayerCamera {}
        ).id();

        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Capsule {
                radius:2.0,
                depth:1.0,
                ..default()
            }.into()),
            transform: Transform::from_xyz(-5.0, 5.0, 0.0),
            ..default()
        })
        .insert(Player::new())
        .insert(Collider::capsule_y(0.5, 0.4))
        .insert(RigidBody::Dynamic)
        .insert(Velocity{..default()})
        .insert(Friction {
            coefficient: 5.0,
            combine_rule: CoefficientCombineRule::Max
        })
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0
        })
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z | LockedAxes::ROTATION_LOCKED_Y)
        .add_child(cam);
    }
}
impl Entity for Player {
    const ENTITY_TYPE: EntityType = EntityType::Player;

    fn new() -> Self {
        Player {
            speed: 4.0
        }
    }
}

