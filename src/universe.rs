use std::{collections::HashMap, ops::{Deref, DerefMut}};

use bevy::{prelude::*, render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages, render_resource::Face}};
use bevy_rapier3d::geometry::Collider;

use crate::{chunk::{self, Chunk}, player::Player};

#[derive(Default, Resource)]
pub struct Universe {
    pub chunks: HashMap<(i32, i32), Entity>,
    // pub chunks: Vec<(i32, i32, i32)>,

    pub load_distance: i32
}
impl Universe {
    pub fn new() -> Universe {
        Universe {
            chunks: HashMap::new(),
            load_distance: 5
        }
    }

    pub fn generate(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,

        mut universe: ResMut<Universe>,

        player: Query<&Transform, With<Player>>,

        mut chunk_query: Query<&mut Chunk>,

        asset_server: Res<AssetServer>
    ) {
        let chunk_pos = chunk::Chunk::real_to_chunk(player.get_single().unwrap().translation);

        for rx in -universe.load_distance..universe.load_distance {
            for rz in -universe.load_distance..universe.load_distance {
                let cx = chunk_pos.0 + rx;
                let cz = chunk_pos.1 + rz;

                if !universe.chunks.contains_key(&(cx, cz)) {
                    let fx = cx as f32;
                    let fz = cz as f32;

                    let mut c = chunk::Chunk::new((cx, cz));
                    let mut m = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
                    c.update_mesh(&mut m, [[None; 3]; 3]);
                    let (v, i) = c.generate_trimesh_data([[None; 3]; 3]);

                    // create chunk
                    let c = commands.spawn((
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
                                0.0,
                                // fy * chunk::CHUNK_SIZE as f32,
                                fz * chunk::CHUNK_SIZE as f32
                                ),
                            ..default()
                        },
                        c,
                        Collider::trimesh(v, i)
                    ))
                    .id();

                    universe.chunks.insert((cx, cz), c);

                    // chunk::Chunk::update_neighbours((cx, cz), &mut universe, &mut chunk_query);

                    // println!("{:?}", universe.chunks);
                }
            }
        }
    }

    pub fn update_chunks() {

    }
}
// impl Deref for Universe {
//     type Target = Universe;

//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }

// impl DerefMut for Universe {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self
//     }
// }
