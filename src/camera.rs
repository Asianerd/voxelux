use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::player::Player;

#[derive(Component)]
pub struct PlayerCamera {
}

impl PlayerCamera {
    pub fn camera_movement(
        mut cam: Query<&mut Transform, With<PlayerCamera>>,
        mut p: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    
        mut motion_evr: EventReader<MouseMotion>,
        key_input: Res<ButtonInput<KeyCode>>
    ) {
        let mut camera = cam.get_single_mut().unwrap();
        let mut player = p.get_single_mut().unwrap();
    
        let mut mouse_result = Vec2::ZERO;
        if !key_input.pressed(KeyCode::KeyE) {
            for e in motion_evr.read() {
                mouse_result = e.delta;
            }
        }
    
        mouse_result *= 0.5;
        player.rotate_y(mouse_result.x / 180.0 * -1.0); // rotating relative to y plane
        camera.rotate_local_x(mouse_result.y / 180.0 * -1.0); // rotating relative to local x plane
    }
    
    // depreciated
    pub fn primitive_camera_movement(
        mut cam: Query<&mut Transform, With<PlayerCamera>>,
        // mut l: Query<(&mut Transform, &mut SpotLight), (With<SpotLight>, Without<Camera3d>)>,
        mut motion_evr: EventReader<MouseMotion>,
        key_input: Res<ButtonInput<KeyCode>>
    ) {
        let speed = 0.1 * (if key_input.pressed(KeyCode::ShiftLeft) { 5.0 } else { 1.0 });
    
        let mut camera = cam.get_single_mut().unwrap();
        // let (mut light, mut spotlight) = l.get_single_mut().unwrap();
    
        let mut mouse_result = Vec2::ZERO;
        if !key_input.pressed(KeyCode::KeyE) {
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
        if key_input.pressed(KeyCode::KeyA) {
            result = *camera.left();
            result.y = 0.0;
            result = result.normalize();
        }
        if key_input.pressed(KeyCode::KeyD) {
            result = *camera.right();
            result.y = 0.0;
            result = result.normalize();
        }
        camera.translation += result * speed;
        // TODO: normalize vector
    
        if key_input.pressed(KeyCode::KeyW) {
            result = *camera.forward();
            result.y = 0.0;
            result = result.normalize();
        }
        if key_input.pressed(KeyCode::KeyS) {
            result = *camera.back();
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
    }
}