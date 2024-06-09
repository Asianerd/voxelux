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
use bevy::input::mouse::MouseMotion;
use rand::Rng;

use chunk::{Universe, Chunk};
use entity::Entity;

mod camera;
mod chunk;
mod block;
mod entity;
mod player;

fn main() {
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::GREEN,
        })
        .insert_resource(Universe::new())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))

        .add_systems(Startup, startup)

        .add_systems(Update, quit_on_escape)
        .add_systems(Update, camera::camera_movement)
        .add_systems(Update, player::Player::movement)
        .add_systems(Update, chunk::Universe::update_meshes)
        // .add_systems(Update, chunk::Universe::update_colliders)

        .run();
}

fn quit_on_escape(
    mut e: ResMut<Events<bevy::app::AppExit>>,
    key_input: Res<Input<KeyCode>>
) {
    if key_input.pressed(KeyCode::Escape) {
        e.send(bevy::app::AppExit);
    }
}

// fn load_world(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {

// }

fn startup(
    mut commands: Commands,
    mut universe: ResMut<chunk::Universe>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(50.0, 2.0, 50.0).into()),
        material: materials.add(Color::GREEN.into()),
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

    let gen_size = 2;

    for y in -gen_size..gen_size {
        for x in -gen_size..gen_size {
            for z in -gen_size..gen_size {
    // for y in -1..1 {
    //     for x in -1..1 {
    //         for z in -1..1 {
                let c = chunk::Chunk::new((x, y, z));
                let mut m = Mesh::new(PrimitiveTopology::TriangleList);
                c.update_mesh(&mut m, [[[None; 3]; 3]; 3]);
                let (v, i) = c.generate_trimesh_data([[[None; 3]; 3]; 3]);

                // let t = ColorMaterial
                // let t = TextureSampleType::
                // ImageSampler::Descriptor(ImageSamplerDescriptor {})

                universe.entities.push(commands.spawn(
                    PbrBundle {
                        mesh: meshes.add(m),
                        material: materials.add(
                            StandardMaterial {
                                base_color_texture: Some(asset_server.load("atlas.png")),
                                // base_color: Color::WHITE,
                                // double_sided: true,
                                // cull_mode:None,
                                
                                ..default()
                            }
                        ),
                        transform: Transform::from_xyz(
                            x as f32 * chunk::CHUNK_SIZE as f32,
                            y as f32 * chunk::CHUNK_SIZE as f32,
                            z as f32 * chunk::CHUNK_SIZE as f32
                        ),
                        ..default()
                    }
                )
                .insert(c)
                .insert(Collider::trimesh(v, i)).id());

                // commands.spawn(
                //     Collider::trimesh(v, i),
                // );
            }
        }
    }

    // let gltf = asset_server.load("building.glb#Scene0");

    // commands.spawn(SceneBundle {
    //     scene: gltf,
    //     ..default()
    // });

    player::Player::spawn_player(&mut commands, &mut meshes);

    // let texture_handle: Handle<Image> = asset_server.load("city.png");

    // commands.spawn(PbrBundle {
    //     transform: Transform::from_xyz(0.0, 2.0, 0.0),
    //     mesh: meshes.add(shape::Cube::new(2.0).into()),
    //     material: materials.add(StandardMaterial{
    //         emissive: Color::WHITE,
    //         emissive_texture: Some(texture_handle.clone()),
    //         base_color_texture: Some(texture_handle.clone()),
    //         ..default()
    //     }),
    //     ..default()
    // });

    // let a = commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         color: Color::WHITE,
    //         intensity: 3000f32,
    //         range:500f32,
    //         ..default()
    //     },
    //     ..default()
    // }).id();

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Capsule{
    //         radius:1.0f32,
    //         depth:0f32,
    //         ..default()
    //     }.into()),
    //     material: materials.add(StandardMaterial{
    //         base_color: Color::YELLOW,
    //         emissive: Color::WHITE,
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(0.0, 3.0, 0.0),
    //     ..default()
    // })
    // .insert(Collider::ball(1.0f32))
    // .add_child(a);




    // let world_size = 2;
    // for x in -world_size..world_size {
    //     for y in -world_size..world_size {
    //         for z in -world_size..world_size {
    //             universe.entities.push(
    //                 commands.spawn(PbrBundle {
    //                     ..default()
    //                 })
    //                 .insert(
    //                     chunk::Chunk::new((x, y, z))
    //                 )
    //                 .id()
    //             );
    //         }
    //     }
    // }
}
