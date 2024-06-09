use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::player::Player;

#[derive(Component)]
pub struct PlayerCamera {
    
}

pub fn camera_movement(
    mut cam: Query<&mut Transform, With<PlayerCamera>>,
    mut p: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,

    mut motion_evr: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>
) {
    let mut camera = cam.get_single_mut().unwrap();
    let mut player = p.get_single_mut().unwrap();

    let mut mouse_result = Vec2::ZERO;
    if !key_input.pressed(KeyCode::E) {
        for e in motion_evr.read() {
            mouse_result = e.delta;
        }
    }

    mouse_result *= 0.5;
    player.rotate_y(mouse_result.x / 180.0 * -1.0); // rotating relative to y plane
    camera.rotate_local_x(mouse_result.y / 180.0 * -1.0); // rotating relative to local x plane
}

pub fn primitive_camera_movement(
    mut cam: Query<&mut Transform, With<PlayerCamera>>,
    // mut l: Query<(&mut Transform, &mut SpotLight), (With<SpotLight>, Without<Camera3d>)>,
    mut motion_evr: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>
) {
    let speed = 0.1 * (if key_input.pressed(KeyCode::ShiftLeft) { 5.0 } else { 1.0 });

    let mut camera = cam.get_single_mut().unwrap();
    // let (mut light, mut spotlight) = l.get_single_mut().unwrap();

    let mut mouse_result = Vec2::ZERO;
    if !key_input.pressed(KeyCode::E) {
        for e in motion_evr.read() {
            mouse_result = e.delta;
        }
    }
    // assume camera is pointing in Z axis
    // Y
    // .
    // .
    // O  .  .  . X
    //    .
    //       .
    //         Z (out of screen)

    mouse_result *= 0.5;
    camera.rotate_y(mouse_result.x / 180.0 * -1.0); // rotating relative to y plane
    camera.rotate_local_x(mouse_result.y / 180.0 * -1.0); // rotating relative to local x plane
    // camera.rotation.x = camera.rotation.x.clamp(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);
    // println!("{:?}", camera.rotation.x);
    // println!("Mouse vec : {mouse_result:?}");

    let mut result: Vec3 = Vec3::ZERO;
    if key_input.pressed(KeyCode::A) {
        result = camera.left();
        result.y = 0.0;
        result = result.normalize();
    }
    if key_input.pressed(KeyCode::D) {
        result = camera.right();
        result.y = 0.0;
        result = result.normalize();
    }
    camera.translation += result * speed;
    // TODO: normalize vector

    if key_input.pressed(KeyCode::W) {
        result = camera.forward();
        result.y = 0.0;
        result = result.normalize();
    }
    if key_input.pressed(KeyCode::S) {
        result = camera.back();
        result.y = 0.0;
        result = result.normalize();
    }
    camera.translation += result * speed;

    if key_input.pressed(KeyCode::Space) {
        camera.translation.y += speed;
    }
    if key_input.pressed(KeyCode::ControlLeft) {
        camera.translation.y -= speed;
    }

    // light.translation = camera.translation.clone();
    // light.translation += camera.back();
    // light.rotation = camera.rotation.clone();

    // if key_input.pressed(KeyCode::Up) {
    //     // spotlight.intensity += 100.0;
    //     spotlight.outer_angle += 0.01;
    //     spotlight.inner_angle = spotlight.outer_angle * 0.5;
    // }
    // if key_input.pressed(KeyCode::Down) {
    //     // spotlight.intensity -= 100.0;
    //     spotlight.outer_angle -= 0.01;
    //     spotlight.inner_angle = spotlight.outer_angle * 0.5;
    // }

    // let _r = light.right().clone();
    // let _d = light.down().clone() * 0.5;

    // light.translation += _r;
    // light.translation += _d;

    // println!("{} {}", spotlight.outer_angle, spotlight.inner_angle);

    // println!("{:?}", camera.translation);
}
