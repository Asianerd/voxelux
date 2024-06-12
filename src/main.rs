use bevy::core_pipeline::bloom::BloomSettings;
use bevy_rapier3d::prelude::*;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    mesh::{
        Indices, VertexAttributeValues
    },
    render_resource::{
        PrimitiveTopology,
        WgpuFeatures
    },
    settings::{
        RenderCreation,
        WgpuSettings
    }
};
use bevy::pbr::wireframe::{WireframePlugin, WireframeConfig};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use entity::Entity;
use rand::Rng;
use universe::Universe;

mod utils;

mod universe;

mod camera;

mod block;
mod chunk;

mod entity;
mod player;

fn main() {
    // run with cargo run --features bevy/dynamic_linking

    App::new()
        .add_plugins(
            DefaultPlugins
            .set(ImagePlugin::default_nearest()) // texture sampling to pointclamp
        )
        // .add_plugins((
        //     DefaultPlugins.set(RenderPlugin {
        //         render_creation: RenderCreation::Automatic(WgpuSettings {
        //             features: WgpuFeatures::POLYGON_MODE_LINE,
        //             ..default()
        //         }),
        //     }),
        //     WireframePlugin,
        // ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::GREEN,
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Universe::new())

        .add_systems(Startup,
            (
                startup,
                universe::Universe::generate.after(startup)
            )
        )

        .add_systems(Update, 
            (
                quit_on_escape,
                player::Player::movement,
                player::Player::selection,
                camera::PlayerCamera::camera_movement
            )
        )

        .run();
}

fn quit_on_escape(
    mut e: ResMut<Events<bevy::app::AppExit>>,
    key_input: Res<ButtonInput<KeyCode>>
) {
    if key_input.pressed(KeyCode::Escape) {
        e.send(bevy::app::AppExit);
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(50.0, 2.0, 50.0)),
        material: materials.add(Color::GREEN),
        ..default()
    }).insert(Collider::cuboid(25., 1., 25.));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 1000.0,
            range:100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    player::Player::spawn(&mut commands, &mut meshes);
}