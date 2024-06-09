use bevy::{
    prelude::*,
    render::{
        render_resource::{
            PrimitiveTopology,
            Face
        },
        mesh::Indices
    }, utils::hashbrown::HashMap
};
use bevy_rapier3d::geometry::Collider;
use rand::prelude::*;

use crate::block::{Block, BlockType, self};

#[derive(Resource)]
pub struct Universe {
    pub entities: Vec<Entity>,
    // pub chunks: Vec<Chunk>
}
impl Universe {
    pub fn new() -> Universe {
        Universe {
            entities: vec![],
            // chunks: vec![]
        }
    }

    // pub fn update_colliders(
    //     universe: ResMut<Universe>,
    //     mut q: Query<(&mut Collider, Entity, &mut Chunk)>,
    //     mut meshes: ResMut<Assets<Mesh>>
    // ) {
    //     let mut neighbours: HashMap<(i32, i32, i32), [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]> = HashMap::new();
    
    //     let mut neighbour_fetching = q.iter_many(&universe.entities);
    //     while let Some((entity, chunk)) = neighbour_fetching.fetch_next() {
    //         neighbours.insert(
    //             (
    //                 chunk.position.0,
    //                 chunk.position.1,
    //                 chunk.position.2
    //             ),
    //             chunk.blocks.clone()
    //         );
    //     }

    //     let mut q_iter = q.iter_many_mut(&universe.entities);

    //     while let Some((mut collider, entity, mut chunk)) = q_iter.fetch_next() {
    //         // collider.
    //         // update collider based on mesh of chunk

    //         if !chunk.requires_collider_update {
    //             continue;
    //         }
    //         chunk.requires_collider_update = false;

    //         collider.replace_if_neq(chunk.generate_trimesh_data(neighbours));
    //     }
    // }

    pub fn update_meshes(
        mut commands: Commands,
        universe: ResMut<Universe>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut q: Query<(Entity, &mut Chunk, &mut Collider)>,
        mut handle_query: Query<&Handle<Mesh>>
    ) {
        let mut neighbours: HashMap<(i32, i32, i32), [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]> = HashMap::new();
    
        let mut neighbour_fetching = q.iter_many(&universe.entities);
        while let Some((entity, chunk, _)) = neighbour_fetching.fetch_next() {
            neighbours.insert(
                (
                    chunk.position.0,
                    chunk.position.1,
                    chunk.position.2
                ),
                chunk.blocks.clone()
            );
        }
    
        let mut q_all = q.iter_many_mut(&universe.entities);
    
        while let Some((entity, mut chunk, mut collider)) = q_all.fetch_next() {
            if !chunk.requires_update {
                continue;
            }
            chunk.requires_update = false;
    
            let mesh_handle = handle_query.get_mut(entity).unwrap();
            let mesh = meshes.get_mut(mesh_handle.id()).unwrap();
    
            // get neighbours
            let mut n: [[[Option<&[[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>; 3]; 3]; 3] = [[[None; 3]; 3]; 3];
            for (i, ni) in vec![
                ((0, 0, 1), (1usize, 1usize, 2usize)),
                ((0, 0, -1), (1usize, 1usize, 0usize)),
                ((0, 1, 0), (1usize, 2usize, 1usize)),
                ((0, -1, 0), (1usize, 0usize, 1usize)),
                ((1, 0, 0), (2usize, 1usize, 1usize)),
                ((-1, 0, 0), (0usize, 1usize, 1usize)),
            ] {
                let target = (
                    chunk.position.0 + i.0,
                    chunk.position.1 + i.1,
                    chunk.position.2 + i.2,
                );
    
                n[ni.0][ni.1][ni.2] = neighbours.get(&target);
    
                if neighbours.get(&target).is_some() { // redundant?
                    continue;
                }
            }
    
            chunk.update_mesh(mesh, n);

            let (v, i) = chunk.generate_trimesh_data(n);
            commands.entity(entity).insert(Collider::trimesh(v, i));
            // collider.replace_if_neq(Collider::trimesh(v, i));
            // println!("Updated chunk");
            // 6 neighbours
            // up down front back left right
        }
    }
}

pub const CHUNK_SIZE: usize = 4;

#[derive(Component)]
pub struct Chunk {
    pub position: (i32, i32, i32),
    pub blocks: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub requires_update: bool,
    pub requires_collider_update: bool
}
impl Chunk {
    pub fn new(pos: (i32, i32, i32)) -> Chunk {
        let mut b: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = [[[BlockType::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let wp = (
            pos.0 * CHUNK_SIZE as i32,
            pos.1 * CHUNK_SIZE as i32,
            pos.2 * CHUNK_SIZE as i32
        );

        let mut rng = rand::thread_rng();

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let wx = wp.0 + x as i32;
                    let wy = wp.1 + y as i32;
                    let wz = wp.2 + z as i32;

                    // let mut l = 0.;

                    // for i in vec![
                    //     20.0 * ((wx + wz) as f32 * 2.0 * std::f32::consts::PI / 200.0).sin() + 20.0,
                    //     20.0 * ((wx -wz) as f32 * 2.0 * std::f32::consts::PI / 200.0).cos() + 10.0,
                    // ] {
                    //     if i > l {
                    //         l = i;
                    //     }
                    // }

                    // if wy as f32 >= l {
                    //     continue;
                    // }

                    if rng.gen_bool(0.9) {
                        continue;
                    }

                    b[y][x][z] = if rng.gen_bool(0.5) { BlockType::Stone } else { BlockType::Dirt };
                }
            }
        }

        Chunk {
            position: pos,
            blocks: b,
            requires_update: true,
            requires_collider_update: true
        }
    }

    fn get_at(&self, x: i32, y: i32, z: i32, neighbours: &[[[Option<&[[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>; 3]; 3]; 3]) -> BlockType {
        let cs = CHUNK_SIZE as i32;

        let mut ux = x as usize;
        let mut uy = y as usize;
        let mut uz = z as usize;

        let mut nx = 1usize;
        let mut ny = 1usize;
        let mut nz = 1usize;

        // if (y >= cs) || (x >= cs) || (z >= cs) || (y < 0) || (x < 0) || (z < 0) {
        //     // oob
        //     return BlockType::Air;
        // }

        // run checks for x and y axis only

        let mut oob = false;

        if y >= cs {
            oob = true;
            ny = 2;
            uy = 0;
        } else if y < 0 {
            oob = true;
            ny = 0;
            uy = CHUNK_SIZE - 1;
        }

        if x >= cs {
            oob = true;
            nx = 2;
            ux = 0;
        } else if x < 0 {
            oob = true;
            nx = 0;
            ux = CHUNK_SIZE - 1;
        }

        if z >= cs {
            oob = true;
            nz = 2;
            uz = 0;
        } else if z < 0 {
            oob = true;
            nz = 0;
            uz = CHUNK_SIZE - 1;
        }

        if !oob {
            return self.blocks[uy][ux][uz];
        }

        let r = neighbours[nx][ny][nz];
        if r.is_none() {
            return BlockType::Air;
        }

        r.unwrap()[uy][ux][uz]
    }

    fn get_at_without_check(&self, x: i32, y: i32, z: i32) -> BlockType {
        self.blocks[y as usize][x as usize][z as usize]
    }

    fn get_at_decide(&self, x: i32, y: i32, z: i32, neighbours: &[[[Option<&[[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>; 3]; 3]; 3], edge: bool, t: &mut BlockType) -> BlockType {
        if edge {
            *t = self.get_at(x, y, z, neighbours);
            return *t;
        }
        *t = self.get_at_without_check(x, y, z);
        *t
    }

    pub fn generate_trimesh_data(&self, neighbours: [[[Option<&[[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>; 3]; 3]; 3]) -> (Vec<Vec3>, Vec<[u32; 3]>) {
        let mut vertices: Vec<Vec3> = vec![];
        let mut indices: Vec<[u32; 3]> = vec![];

        let mut index_count: u32 = 0;
            
        // culling done
        for y in 0..(CHUNK_SIZE as i32) {
            let y_edge = (y == 0) || ((y + 1) == (CHUNK_SIZE as i32));
            let yf = y as f32;
            let y1 = yf + 1.;

            for x in 0..(CHUNK_SIZE as i32) {
                let x_edge = (x == 0) || ((x + 1) == (CHUNK_SIZE as i32));
                let xf = x as f32;
                let x1 = xf + 1.;

                for z in 0..(CHUNK_SIZE as i32) {
                    let z_edge = (z == 0) || ((z + 1) == (CHUNK_SIZE as i32));
                    let zf = z as f32;
                    let z1 = zf + 1.;
                    
                    if self.get_at(x, y, z, &neighbours) != BlockType::Air {
                        continue;
                    }

                    let is_edge = y_edge || x_edge || z_edge;
                    let mut t = BlockType::Air;

                    if self.get_at_decide(x + 1, y, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_trimesh_attributes([[x1, yf, z1], [x1, y1, z1], [x1, y1, zf], [x1, yf, zf]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x, y, z + 1, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_trimesh_attributes([[xf, yf, z1], [xf, y1, z1], [x1, y1, z1], [x1, yf, z1]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x - 1, y, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_trimesh_attributes([[xf, yf, zf], [xf, y1, zf], [xf, y1, z1], [xf, yf, z1]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x, y, z - 1, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_trimesh_attributes([[x1, yf, zf], [x1, y1, zf], [xf, y1, zf], [xf, yf, zf]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x, y + 1, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_trimesh_attributes([[x1, y1, z1], [xf, y1, z1], [xf, y1, zf], [x1, y1, zf]], &mut vertices, &mut indices, &mut index_count);
                    }

                    if self.get_at_decide(x, y - 1, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_trimesh_attributes([[x1, yf, zf], [xf, yf, zf], [xf, yf, z1], [x1, yf, z1]], &mut vertices, &mut indices, &mut index_count);
                    }
                }
            }
        }

        (vertices, indices)
    }

    fn generate_mesh_data(&self, neighbours: [[[Option<&[[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>; 3]; 3]; 3]) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 2]>) {
        // mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        // mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);

        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut uv: Vec<[f32; 2]> = vec![];

        let mut index_count: u32 = 0;
            
        // culling done
        for y in 0..(CHUNK_SIZE as i32) {
            let y_edge = (y == 0) || ((y + 1) == (CHUNK_SIZE as i32));
            let yf = y as f32;
            let y1 = yf + 1.;

            for x in 0..(CHUNK_SIZE as i32) {
                let x_edge = (x == 0) || ((x + 1) == (CHUNK_SIZE as i32));
                let xf = x as f32;
                let x1 = xf + 1.;

                for z in 0..(CHUNK_SIZE as i32) {
                    let z_edge = (z == 0) || ((z + 1) == (CHUNK_SIZE as i32));
                    let zf = z as f32;
                    let z1 = zf + 1.;
                    
                    if self.get_at(x, y, z, &neighbours) != BlockType::Air {
                        continue;
                    }

                    let is_edge = y_edge || x_edge || z_edge;
                    let mut t = BlockType::Dirt;

                    if self.get_at_decide(x + 1, y, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_attributes([[x1, yf, z1], [x1, y1, z1], [x1, y1, zf], [x1, yf, zf]], [-1.0, 0.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 1);
                    }

                    if self.get_at_decide(x, y, z + 1, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_attributes([[xf, yf, z1], [xf, y1, z1], [x1, y1, z1], [x1, yf, z1]], [0.0, 0.0, 1.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 2);
                    }

                    if self.get_at_decide(x - 1, y, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_attributes([[xf, yf, zf], [xf, y1, zf], [xf, y1, z1], [xf, yf, z1]], [-1.0, 0.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 3);
                    }

                    if self.get_at_decide(x, y, z - 1, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_attributes([[x1, yf, zf], [x1, y1, zf], [xf, y1, zf], [xf, yf, zf]], [0.0, 0.0, -1.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 4);
                    }

                    if self.get_at_decide(x, y + 1, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_attributes([[x1, y1, z1], [xf, y1, z1], [xf, y1, zf], [x1, y1, zf]], [0.0, -1.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 5);
                    }

                    if self.get_at_decide(x, y - 1, z, &neighbours, is_edge, &mut t) != BlockType::Air {
                        Chunk::add_attributes([[x1, yf, zf], [xf, yf, zf], [xf, yf, z1], [x1, yf, z1]], [0.0, 1.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 6);
                    }
                }
            }
        }

        (vertices, normals, indices, uv)

        // greedy meshing

        // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        // mesh.set_indices(Some(Indices::U32(indices)));
    }

    pub fn update_mesh(&self, mesh: &mut Mesh, neighbours: [[[Option<&[[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>; 3]; 3]; 3]) {
        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);

        let (vertices, normals, indices, uv) = self.generate_mesh_data(neighbours);

        // let mut vertices: Vec<[f32; 3]> = vec![];
        // let mut normals: Vec<[f32; 3]> = vec![];
        // let mut indices: Vec<u32> = vec![];

        // let mut index_count: u32 = 0;
            
        // // culling done
        // for y in 0..(CHUNK_SIZE as i32) {
        //     let y_edge = (y == 0) || ((y + 1) == (CHUNK_SIZE as i32));
        //     let yf = y as f32;
        //     let y1 = yf + 1.;

        //     for x in 0..(CHUNK_SIZE as i32) {
        //         let x_edge = (x == 0) || ((x + 1) == (CHUNK_SIZE as i32));
        //         let xf = x as f32;
        //         let x1 = xf + 1.;

        //         for z in 0..(CHUNK_SIZE as i32) {
        //             let z_edge = (z == 0) || ((z + 1) == (CHUNK_SIZE as i32));
        //             let zf = z as f32;
        //             let z1 = zf + 1.;
                    
        //             if self.get_at(x, y, z, &neighbours) != BlockType::Air {
        //                 continue;
        //             }

        //             let is_edge = y_edge || x_edge || z_edge;

        //             if self.get_at_decide(x + 1, y, z, &neighbours, is_edge) != BlockType::Air {
        //                 Chunk::add_attributes([[x1, yf, z1], [x1, y1, z1], [x1, y1, zf], [x1, yf, zf]], [-1.0, 0.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count)
        //             }

        //             if self.get_at_decide(x, y, z + 1, &neighbours, is_edge) != BlockType::Air {
        //                 Chunk::add_attributes([[xf, yf, z1], [xf, y1, z1], [x1, y1, z1], [x1, yf, z1]], [0.0, 0.0, 1.0], &mut vertices, &mut normals, &mut indices, &mut index_count)
        //             }

        //             if self.get_at_decide(x - 1, y, z, &neighbours, is_edge) != BlockType::Air {
        //                 Chunk::add_attributes([[xf, yf, zf], [xf, y1, zf], [xf, y1, z1], [xf, yf, z1]], [-1.0, 0.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count)
        //             }

        //             if self.get_at_decide(x, y, z - 1, &neighbours, is_edge) != BlockType::Air {
        //                 Chunk::add_attributes([[x1, yf, zf], [x1, y1, zf], [xf, y1, zf], [xf, yf, zf]], [0.0, 0.0, -1.0], &mut vertices, &mut normals, &mut indices, &mut index_count)
        //             }

        //             if self.get_at_decide(x, y + 1, z, &neighbours, is_edge) != BlockType::Air {
        //                 Chunk::add_attributes([[x1, y1, z1], [xf, y1, z1], [xf, y1, zf], [x1, y1, zf]], [0.0, -1.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count);
        //             }

        //             if self.get_at_decide(x, y - 1, z, &neighbours, is_edge) != BlockType::Air {
        //                 Chunk::add_attributes([[x1, yf, zf], [xf, yf, zf], [xf, yf, z1], [x1, yf, z1]], [0.0, 1.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count);
        //             }
        //         }
        //     }
        // }

        // greedy meshing

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.set_indices(Some(Indices::U32(indices)));
    }

    fn add_attributes(
        v: [[f32; 3]; 4],
        n: [f32; 3],

        vertices: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        
        indices: &mut Vec<u32>,
        index_count: &mut u32,

        t: BlockType,
        uv: &mut Vec<[f32; 2]>,
        side: i32
    ) {
        vertices.push(v[0]);
        vertices.push(v[1]);
        vertices.push(v[2]);
        vertices.push(v[3]);

        normals.push(n);
        normals.push(n);
        normals.push(n);
        normals.push(n);

        // if t == BlockType::Stone {
        //     println!("This is stone");
        // }

        for u in Chunk::fetch_uv(t, side) {
            uv.push(u);
        }

        // 1  2

        // 0  3

        // 0,1,2
        // 0,2,3

        for i in vec![
            0u32, 1u32, 2u32,
            0u32, 2u32, 3u32
        ] {
            indices.push(*index_count + i);
        }

        *index_count += 4;
    }

    fn fetch_uv(t: BlockType, side: i32) -> [[f32; 2]; 4] {
        let offset: f32 = block::type_to_i32(t) as f32;

        // let y = offset / (block::TOTAL_TYPES as f32);
        // let increment = 1f32 / (block::TOTAL_TYPES as f32);

        let start = (offset / (block::TOTAL_TYPES as f32)) + (1f32 / (block::TOTAL_TYPES as f32)) * ((side as f32 - 1f32) / 6 as f32);
        let end = (offset / (block::TOTAL_TYPES as f32)) + (1f32 / (block::TOTAL_TYPES as f32)) * ((side as f32) / 6 as f32);

        // let increment = (1 as f32 / block::TOTAL_TYPES as f32) as f32;
        // let y = offset * increment * ((side as f32) / 6f32);

        // [[0.0, y + increment], [0.0, y], [1.0, y], [1.0, y + increment]]
        [[1.0, end], [1.0, start], [0.0, start], [0.0, end]]

        // [[0.0; 2]; 4]
    }

    fn add_trimesh_attributes(v: [[f32; 3]; 4], vertices: &mut Vec<Vec3>, indices: &mut Vec<[u32; 3]>, index_count: &mut u32) {
        vertices.push(Vec3::from_array(v[0]));
        vertices.push(Vec3::from_array(v[1]));
        vertices.push(Vec3::from_array(v[2]));
        vertices.push(Vec3::from_array(v[3]));

        indices.push(
            [
                0u32 + *index_count,
                1u32 + *index_count,
                2u32 + *index_count
            ],
        );
        indices.push(
            [
                0u32 + *index_count,
                2u32 + *index_count,
                3u32 + *index_count
            ],
        );

        *index_count += 4;
    }
}
