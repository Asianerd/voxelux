use std::collections::HashMap;

use bevy::{prelude::*, render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages, render_resource::Face}};
use bevy_rapier3d::geometry::Collider;

use crate::{chunk::{self}, player::Player};

#[derive(Resource)]
pub struct Universe {
    pub chunks: HashMap<(i32, i32, i32), bool>,
    // pub chunks: Vec<(i32, i32, i32)>,

    pub load_distance: i32
}
impl Universe {
    pub fn new() -> Universe {
        Universe {
            chunks: HashMap::new(),
            load_distance: 10
        }
    }

    pub fn generate(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,

        mut universe: ResMut<Universe>,

        player: Query<&Transform, With<Player>>,

        // chunks: Query<(&Transform, &Chunk)>,

        asset_server: Res<AssetServer>
    ) {
        let chunk_pos = chunk::Chunk::tile_to_chunk_pos(player.get_single().unwrap().translation);

        for rx in -universe.load_distance..universe.load_distance {
            for ry in -universe.load_distance..universe.load_distance {
                for rz in -universe.load_distance..universe.load_distance {
                    let cx = chunk_pos.0 + rx;
                    let cy = chunk_pos.1 + ry;
                    let cz = chunk_pos.2 + rz;

                    if !universe.chunks.contains_key(&(cx, cy, cz)) {
                        universe.chunks.insert((cx, cy, cz), true);

                        let fx = cx as f32;
                        let fy = cy as f32;
                        let fz = cz as f32;

                        let c = chunk::Chunk::new((cx, cy, cz));
                        let mut m = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
                        c.update_mesh(&mut m, [[[None; 3]; 3]; 3]);
                        let (v, i) = c.generate_trimesh_data([[[None; 3]; 3]; 3]);

                        // create chunk
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(m),
                                material: materials.add(
                                    StandardMaterial {
                                        base_color_texture: Some(asset_server.load("atlas.png")),

                                        double_sided: false,
                                        cull_mode: Some(Face::Back),

                                        ..default()
                                    }
                                ),
                                transform: Transform::from_xyz(
                                    fx * chunk::CHUNK_SIZE as f32,
                                    fy * chunk::CHUNK_SIZE as f32,
                                    fz * chunk::CHUNK_SIZE as f32
                                    ),
                                ..default()
                            },
                            c,
                            Collider::trimesh(v, i)
                        ));
                    }
                }
            }
        }
    }
}
