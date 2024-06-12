use std::{collections::HashMap, f32};

use bevy::{prelude::*, render::mesh::Indices};
use bevy_rapier3d::geometry::Collider;

use crate::{block::{self, Block}, universe::Universe};

use rand::Rng;

pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_SIZE: usize = 8;
// 8*8*8 block size

#[derive(Component)]
pub struct Chunk {
    pub position: (i32, i32),
    // x z y
    pub blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT],

    pub requires_update: bool,
    pub requires_collider_update: bool
}
impl Chunk {
    // pub fn new(position: (i32, i32, i32)) -> Chunk {
    //     Chunk {
    //         position,
    //         blocks: [[[Block::new(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
    //     }
    // }

    pub fn update(&mut self) {
        
    }

    pub fn new(pos: (i32, i32)) -> Chunk {
        let mut b: [[[block::Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT] = [[[block::Block::new(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];
        // wp : x, z
        let wp = (
            pos.0 * CHUNK_SIZE as i32,
            pos.1 * CHUNK_SIZE as i32,
        );

        let mut rng = rand::thread_rng();

        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let wx = wp.0 + x as i32;
                    let wz = wp.1 + z as i32;

                    let wy = y as i32;

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

                    if rng.gen_bool(0.95) {
                        continue;
                    }

                    // if ((pos.0 * 2) + pos.1) != (wy - 5) {
                    //     continue;
                    // }

                    // if y != 3 {
                    //     continue;
                    // }

                    // b[y][x][z] = if rng.gen_bool(0.5) { block::Block { species: block::Species::Stone } } else { block::Block { species: block::Species::Dirt } };
                    b[y][x][z] = block::Block { species: block::Species::Stone };
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

    pub fn replace_block(&mut self, b: Block, pos: (i32, i32, i32)) {
        self.blocks[pos.1 as usize][pos.0 as usize][pos.2 as usize] = b;
        self.requires_update = true;
        self.requires_collider_update = true;
    }
    
    // #region get at
    pub fn get_at(&self, x: i32, y: i32, z: i32, neighbours: &[[Option<[[[block::Species; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]>; 3]; 3]) -> block::Species {
        let cs = CHUNK_SIZE as i32;

        let mut ux = x as usize;
        let mut uy = y as usize;
        let mut uz = z as usize;

        // 0 1 2    2 2 2
        // 0 1 2    1 1 1
        // 0 1 2    0 0 0
        let mut nx = 1usize;
        // let mut ny = 1usize;
        let mut nz = 1usize;

        let mut y_neighbour_flag = false;

        // if (y >= cs) || (x >= cs) || (z >= cs) || (y < 0) || (x < 0) || (z < 0) {
        //     // oob
        //     return block::Species::Air;
        // }

        // run checks for x and y axis only

        let mut oob = false;

        if y >= (CHUNK_HEIGHT as i32) {
            oob = true;
            y_neighbour_flag = true;
            uy = 0;
        } else if y < 0 {
            oob = true;
            y_neighbour_flag = true;
            uy = CHUNK_HEIGHT - 1;
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
            // println!("not oob {uy} : {ux} : {uz}");
            return self.blocks[uy][ux][uz].species;
        }

        if y_neighbour_flag {
            // println!("oob : y = {y}");

            let r = neighbours[nx][nz];
            if r.is_none() {
                return block::Species::Air;
            }
            return r.unwrap()[uy][ux][uz];
        }
        // println!("oob {uy} : {ux} : {uz}");
        // println!("\t{y} : {x} : {z}");
        block::Species::Air
    }

    pub fn get_at_without_check(&self, x: i32, y: i32, z: i32) -> block::Species {
        self.blocks[y as usize][x as usize][z as usize].species
    }

    pub fn get_at_decide(&self, x: i32, y: i32, z: i32, neighbours: &[[Option<[[[block::Species; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]>; 3]; 3], edge: bool, t: &mut block::Species) -> block::Species {
        if edge {
            *t = self.get_at(x, y, z, neighbours);
            return *t;
        }
        *t = self.get_at_without_check(x, y, z);
        *t
    }
    // #endregion

    // #region mesh generation
    pub fn update_all(
        mut commands: Commands,
        universe: ResMut<Universe>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut q: Query<(Entity, &mut Chunk, &mut Collider)>,
        mut handle_query: Query<&Handle<Mesh>>
    ) {
        let mut neighbours: HashMap<(i32, i32), [[[block::Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]> = HashMap::new();

        // let entities = universe.chunks.values().collect::<Vec<&Entity>>();
    
        let mut neighbour_fetching = q.iter_many(universe.chunks.values().collect::<Vec<&Entity>>());
        while let Some((entity, chunk, _)) = neighbour_fetching.fetch_next() {
            neighbours.insert(
                (
                    chunk.position.0,
                    chunk.position.1,
                ),
                chunk.blocks.clone()
            );
        }
    
        let mut q_all = q.iter_many_mut(universe.chunks.values().collect::<Vec<&Entity>>());
    
        while let Some((entity, mut chunk, mut collider)) = q_all.fetch_next() {
            if !chunk.requires_update {
                continue;
            }
            chunk.requires_update = false;
    
            let mesh_handle = handle_query.get_mut(entity).unwrap();
            let mesh = meshes.get_mut(mesh_handle.id()).unwrap();
    
            // get neighbours
            let mut n: [[Option<[[[block::Species; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]>; 3]; 3] = [[None; 3]; 3];
            for (i, ni) in vec![
                // ((0, 0, 1), (1usize, 1usize, 2usize)),
                // ((0, 0, -1), (1usize, 1usize, 0usize)),
                // ((0, 1, 0), (1usize, 2usize, 1usize)),
                // ((0, -1, 0), (1usize, 0usize, 1usize)),
                // ((1, 0, 0), (2usize, 1usize, 1usize)),
                // ((-1, 0, 0), (0usize, 1usize, 1usize)),

                ((0, 1), (1usize, 2usize)),
                ((0, -1), (1usize, 0usize)),
                ((1, 0), (2usize, 1usize)),
                ((-1, 0), (0usize, 1usize))
            ] {
                let target = (
                    chunk.position.0 + i.0,
                    chunk.position.1 + i.1,
                );
    
                n[ni.0][ni.1] = match neighbours.get(&target) {
                    Some(i) => {
                        Some(
                            (*i)
                            .map(
                                |first|
                                first
                                .map(
                                    |second|
                                    second
                                    .map(
                                        |i| i.species
                                    )
                                )
                            )
                        )
                    },
                    None => None
                }
    
                // if neighbours.get(&target).is_some() { // redundant?
                //     continue;
                // }
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

    pub fn generate_trimesh_data(&self, neighbours: [[Option<[[[block::Species; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]>; 3]; 3]) -> (Vec<Vec3>, Vec<[u32; 3]>) {
        let mut vertices: Vec<Vec3> = vec![];
        let mut indices: Vec<[u32; 3]> = vec![];

        let mut index_count: u32 = 0;
            
        // culling done
        for y in 0..(CHUNK_HEIGHT as i32) {
            let y_edge = (y == 0) || ((y + 1) == (CHUNK_HEIGHT as i32));
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
                    
                    if self.get_at(x, y, z, &neighbours) != block::Species::Air {
                        continue;
                    }

                    let is_edge = y_edge || x_edge || z_edge;
                    let mut t = block::Species::Air;

                    if self.get_at_decide(x + 1, y, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_trimesh_attributes([[x1, yf, z1], [x1, y1, z1], [x1, y1, zf], [x1, yf, zf]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x, y, z + 1, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_trimesh_attributes([[xf, yf, z1], [xf, y1, z1], [x1, y1, z1], [x1, yf, z1]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x - 1, y, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_trimesh_attributes([[xf, yf, zf], [xf, y1, zf], [xf, y1, z1], [xf, yf, z1]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x, y, z - 1, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_trimesh_attributes([[x1, yf, zf], [x1, y1, zf], [xf, y1, zf], [xf, yf, zf]], &mut vertices, &mut indices, &mut index_count)
                    }

                    if self.get_at_decide(x, y + 1, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_trimesh_attributes([[x1, y1, z1], [xf, y1, z1], [xf, y1, zf], [x1, y1, zf]], &mut vertices, &mut indices, &mut index_count);
                    }

                    if self.get_at_decide(x, y - 1, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_trimesh_attributes([[x1, yf, zf], [xf, yf, zf], [xf, yf, z1], [x1, yf, z1]], &mut vertices, &mut indices, &mut index_count);
                    }
                }
            }
        }

        (vertices, indices)
    }

    fn generate_mesh_data(&self, neighbours: [[Option<[[[block::Species; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]>; 3]; 3]) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 2]>) {
        // mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        // mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);

        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut uv: Vec<[f32; 2]> = vec![];

        let mut index_count: u32 = 0;
            
        // culling done
        for y in 0..(CHUNK_HEIGHT as i32) {
            let y_edge = (y == 0) || ((y + 1) == (CHUNK_HEIGHT as i32));
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
                    
                    if self.get_at(x, y, z, &neighbours) != block::Species::Air {
                        continue;
                    }

                    let is_edge = y_edge || x_edge || z_edge;
                    let mut t = block::Species::Dirt;

                    if self.get_at_decide(x + 1, y, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_attributes([[x1, yf, z1], [x1, y1, z1], [x1, y1, zf], [x1, yf, zf]], [-1.0, 0.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 1);
                    }

                    if self.get_at_decide(x, y, z + 1, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_attributes([[xf, yf, z1], [xf, y1, z1], [x1, y1, z1], [x1, yf, z1]], [0.0, 0.0, 1.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 2);
                    }

                    if self.get_at_decide(x - 1, y, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_attributes([[xf, yf, zf], [xf, y1, zf], [xf, y1, z1], [xf, yf, z1]], [-1.0, 0.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 3);
                    }

                    if self.get_at_decide(x, y, z - 1, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_attributes([[x1, yf, zf], [x1, y1, zf], [xf, y1, zf], [xf, yf, zf]], [0.0, 0.0, -1.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 4);
                    }

                    if self.get_at_decide(x, y + 1, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_attributes([[x1, y1, z1], [xf, y1, z1], [xf, y1, zf], [x1, y1, zf]], [0.0, -1.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 5);
                    }

                    if self.get_at_decide(x, y - 1, z, &neighbours, is_edge, &mut t) != block::Species::Air {
                        Chunk::add_attributes([[x1, yf, zf], [xf, yf, zf], [xf, yf, z1], [x1, yf, z1]], [0.0, 1.0, 0.0], &mut vertices, &mut normals, &mut indices, &mut index_count, t, &mut uv, 6);
                    }
                }
            }
        }

        (vertices, normals, indices, uv)
    }

    pub fn update_mesh(&self, mesh: &mut Mesh, neighbours: [[Option<[[[block::Species; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]>; 3]; 3]) {
        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);

        let (vertices, normals, indices, uv) = self.generate_mesh_data(neighbours);

        // greedy meshing

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.insert_indices(Indices::U32(indices));
    }

    fn add_attributes(
        v: [[f32; 3]; 4],
        n: [f32; 3],

        vertices: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        
        indices: &mut Vec<u32>,
        index_count: &mut u32,

        t: block::Species,
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

        // if t == block::Species::Stone {
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

    fn fetch_uv(t: block::Species, side: i32) -> [[f32; 2]; 4] {
        let offset: f32 = t as i32 as f32;

        // let y = offset / (block::TOTAL_SPECIES as f32);
        // let increment = 1f32 / (block::TOTAL_SPECIES as f32);

        let total = block::TOTAL_SPECIES as f32;

        let start = (offset / total) + (1f32 / total) * ((side as f32 - 1f32) / 6 as f32);
        let end = (offset / total) + (1f32 / total) * ((side as f32) / 6 as f32);

        // let increment = (1 as f32 / block::TOTAL_SPECIES as f32) as f32;
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
    // #endregion

    // #region utils
    // pub fn real_to_tile(i: i32, size: i32) -> i32 {
    //     if i.signum() == -1 {
    //         return ((i % size) + size) % size;
    //     }
    //     i % size
    // }

    pub fn real_to_tile_single(y: f32) -> i32 {
        // if only it was that easy
        // return SIZE - ((x.abs() + 1.0) % SIZE);
        // return (x.abs() + 1.0) as i32;
        let fsize = CHUNK_SIZE as f32;
        let isize = CHUNK_SIZE as i32;
        if y < 0f32 {
            let x = y - 1.0;
            return ((x - f32::trunc(x / fsize) * fsize) as i32 + isize) % (isize);
        }
        (y as i32) % isize
    }

    pub fn real_to_tile(p: Vec3) -> (i32, i32, i32) {
        (
            Chunk::real_to_tile_single(p.x),
            p.y as i32,
            Chunk::real_to_tile_single(p.z)
        )
    }

    pub fn snap_axis_to_chunk<T: Into<f32>>(i: T) -> i32 {
        f32::floor(i.into() / CHUNK_SIZE as f32) as i32
    }

    pub fn real_to_chunk(pos: Vec3) -> (i32, i32) {
        (
            Chunk::snap_axis_to_chunk(pos.x),
            Chunk::snap_axis_to_chunk(pos.z)
        )
    }
    // #endregion
}

pub enum ChunkState {
    Loaded,
    Unloaded
}