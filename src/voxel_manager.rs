use crate::mesh::Mesh;
use nalgebra::{Vector2, Vector3, VectorView3};

pub static VOXEL_WIDTH : f32 = 0.2;

pub struct VoxelManager {
    pub voxels: Vec<Vec<Vec<bool>>>,
    pub length: usize,
    pub width: usize,
    pub height: usize
}

impl VoxelManager {
    pub fn new(length: usize, width: usize, height: usize) -> Self{
        let mut voxels : Vec<Vec<Vec<bool>>> = (0..length).into_iter().map(|x| {
            (0..width).into_iter().map(|y| {
                (0..height).map(|z| false).collect()
            }).collect()
        }).collect();

        Self {
            voxels,
            length,
            width,
            height
        }
    }

    pub fn update(&mut self) {
        for x in 0..self.length {
            for z in 0..self.width {
                for y in 0..self.height {
                    if self.voxels[x][y][z] && y != 0 && !self.voxels[x][y-1][z] {
                        self.voxels[x][y][z] = false;
                        self.voxels[x][y-1][z] = true;
                    }
                }
            }
        }
    }

    pub fn get_mesh(&self, gl: &eframe::glow::Context ) -> Mesh{
        let mut verts: Vec<Vector3<f32>> = Vec::new();

        for x in 0..self.length {
            for z in 0..self.width {
                for y in 0..self.height {
                    if !self.voxels[x][y][z] {
                        continue;
                    }

                    // println!("Found a true at {:?}", (x, y, z));


                    if (x+1 < self.width && !self.voxels[x+1][y][z]) || x+1 == self.width{
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        verts.append(&mut vec![
                            Vector3::new(x + 1.0, y, z),
                            Vector3::new(x + 1.0, y + 1.0, z),
                            Vector3::new(x + 1.0, y, z + 1.0),
                            
                            Vector3::new(x + 1.0, y + 1.0, z),
                            Vector3::new(x + 1.0, y + 1.0, z + 1.0),
                            Vector3::new(x + 1.0, y, z + 1.0),
                        ]);
                    }

                    if (x > 0 && !self.voxels[x-1][y][z]) || x == 0 {
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        verts.append(&mut vec![
                            Vector3::new(x, y, z),
                            Vector3::new(x, y + 1.0, z),
                            Vector3::new(x, y, z + 1.0),
                            
                            Vector3::new(x, y + 1.0, z),
                            Vector3::new(x, y + 1.0, z + 1.0),
                            Vector3::new(x, y, z + 1.0),
                        ]);
                    }

                    if (y > 0 && !self.voxels[x][y-1][z]) || y == 0 {
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        verts.append(&mut vec![
                            Vector3::new(x, y, z),
                            Vector3::new(x + 1.0, y, z),
                            Vector3::new(x, y, z + 1.0),
                            
                            Vector3::new(x + 1.0, y, z),
                            Vector3::new(x + 1.0, y, z + 1.0),
                            Vector3::new(x, y, z + 1.0),
                        ]);
                    }

                    if (y+1 < self.height && !self.voxels[x][y+1][z]) || y+1 == self.height{
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        verts.append(&mut vec![
                            Vector3::new(x, y + 1.0, z),
                            Vector3::new(x + 1.0, y + 1.0, z),
                            Vector3::new(x, y + 1.0, z + 1.0),
                            
                            Vector3::new(x + 1.0, y + 1.0, z),
                            Vector3::new(x + 1.0, y + 1.0, z + 1.0),
                            Vector3::new(x, y + 1.0, z + 1.0),
                        ]);
                    }

                    if (z+1 < self.length && !self.voxels[x][y][z+1]) || z+1 == self.length{
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        verts.append(&mut vec![
                            Vector3::new(x, y, z + 1.0),
                            Vector3::new(x , y + 1.0, z+ 1.0),
                            Vector3::new(x + 1.0, y, z + 1.0),
                            
                            Vector3::new(x, y + 1.0, z + 1.0),
                            Vector3::new(x + 1.0, y + 1.0, z + 1.0),
                            Vector3::new(x + 1.0, y, z + 1.0),
                        ]);
                    }

                    if (z > 0 && !self.voxels[x][y][z-1]) || z == 0 {
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        verts.append(&mut vec![
                            Vector3::new(x, y, z),
                            Vector3::new(x, y + 1.0, z),
                            Vector3::new(x + 1.0, y, z),
                            
                            Vector3::new(x, y + 1.0, z),
                            Vector3::new(x + 1.0, y + 1.0, z),
                            Vector3::new(x + 1.0, y, z),
                        ]);
                    }
                }
            }
        }

        verts.iter_mut().for_each(|x| *x = *x * VOXEL_WIDTH);

        let count = verts.len();

        let uvs = (0..count).into_iter().map(|x| {
            Vector2::new(0.0, 0.0)
        }).collect();

        Mesh::new(gl, verts, (0..count).into_iter().map(|x| x as u32).collect(), uvs, false)
    }


    pub fn get_bounding_box(&self, gl: &eframe::glow::Context ) -> Mesh{
        let (width, height, length) = (self.width as f32, self.height as f32, self.length as f32);

        let mut verts = cube_wireframe_from_points(Vector3::new(0.0, 0.0, 0.0), Vector3::new(width, length, height));

        verts.iter_mut().for_each(|x| *x = *x * VOXEL_WIDTH);


        let uvs = (0..24).into_iter().map(|x| {
            Vector2::new(0.0, 0.0)
        }).collect();

        Mesh::new(gl, verts, (0..24).into_iter().map(|x| x as u32).collect(), uvs, false)
    }

    pub fn get_ghost_mesh(&self, gl: &eframe::glow::Context, pos: Vector3<f32>, dir: Vector3<f32>) -> (Option<Mesh>, Option<(usize, usize)>) {
        let (mut minx, mut miny, mut minz, mut mindepth) = (u32::MAX, u32::MAX, u32::MAX, f32::MAX);

        for x in 0..self.width {
            for z in 0..self.length {
                match self.ray_box_intersection(pos, dir, x as u32, -19, z as u32) {
                    Some(depth) => {
                        if depth < mindepth {
                            (minx, miny, minz, mindepth) = (x as u32, 19, z as u32, depth)
                        };
                    },
                    None => (),
                }
            }
        }

        if minx == u32::MAX {
            return (None, None);
        }
        
        let (x, y, z) = (minx as f32, miny as f32, minz as f32);

        let verts = cube_verts_from_points(Vector3::new(x * VOXEL_WIDTH, y * VOXEL_WIDTH, z * VOXEL_WIDTH), Vector3::new((x+1.0) * VOXEL_WIDTH, (y+1.0) * VOXEL_WIDTH, (z+1.0) * VOXEL_WIDTH));

        let uvs = (0..36).into_iter().map(|x| {
            Vector2::new(0.0, 0.0)
        }).collect();

        (Some(Mesh::new(gl, verts, (0..36).into_iter().map(|x| x as u32).collect(), uvs, false)), Some((x as usize, z as usize)))        // }
    }

    pub fn ray_box_intersection(&self, pos: Vector3<f32>, dir: Vector3<f32>, x: u32, y: i32, z: u32) -> Option<f32> {
        // let pos = Vector3::new(-2.0, 2.0, 2.0);
        // let dir = Vector3::new(1.0, 0.01, 0.01).normalize();

        let (t0, t1) = (0.8, 100.0);
        let bounds = [Vector3::new(x as f32 * VOXEL_WIDTH, y as f32 * VOXEL_WIDTH, z as f32 * VOXEL_WIDTH), 
        Vector3::new((x as f32 + 1.0) * VOXEL_WIDTH, (y as f32 + 1.0) * VOXEL_WIDTH, (z as f32 + 1.0) * VOXEL_WIDTH)];
        
        let [mut tmin, mut tmax, tymin, tymax, tzmin, tzmax] : [f32; 6];
        tmin = (bounds[if dir.x >= 0.0 {0} else {1}].x - pos.x) / dir.x;
        tmax = (bounds[if dir.x >= 0.0 {1} else {0}].x - pos.x) / dir.x;

        tymin = (bounds[if dir.y >= 0.0 {0} else {1}].y - pos.y) / dir.y;
        tymax = (bounds[if dir.y >= 0.0 {1} else {0}].y - pos.y) / dir.y;

        if tmin > tymax || tymin > tmax {
            return None;
        };

        if tymin > tmin {
            tmin = tymin;
        };

        if tymax < tmax {
            tmax = tymax;
        };
        
        tzmin = (bounds[if dir.z >= 0.0 {0} else {1}].z - pos.z) / dir.z;
        tzmax = (bounds[if dir.z >= 0.0 {1} else {0}].z - pos.z) / dir.z;

        if tmin > tzmax || tzmin > tmax {
            return None;
        };

        if tzmin > tmin {
            tmin = tzmin;
        };
        if tzmax < tmax {
            tmax = tzmax;
        };

        if tmin < t1 && tmax > t0 {
            Some(tmin)
        } else {
            None
        }
    }
}



fn cube_wireframe_from_points(p1: Vector3<f32>, p2: Vector3<f32>) -> Vec<Vector3<f32>> {
    let (x, y, z) = (p1.x, p1.y, p1.z);
    let (width, height, length) = (p2.x, p2.y, p2.z);

    vec![
        Vector3::new(x, y, z), Vector3::new(width, y, z),
        Vector3::new(x, y, z), Vector3::new(x, height, z),
        Vector3::new(x, y, z), Vector3::new(x, y, length),
        Vector3::new(width, y, z), Vector3::new(width, height, z),
        Vector3::new(width, y, z), Vector3::new(width, y, length),
        Vector3::new(x, height, z), Vector3::new(width, height, z),
        Vector3::new(x, height, z), Vector3::new(x, height, length),
        Vector3::new(x, y, length), Vector3::new(width, y, length),
        Vector3::new(x, y, length), Vector3::new(x, height, length),
        Vector3::new(width, height, length), Vector3::new(x, height, length),
        Vector3::new(width, height, length), Vector3::new(width, y, length),
        Vector3::new(width, height, length), Vector3::new(width, height, z),
    ]
}

fn cube_verts_from_points(p1: Vector3<f32>, p2: Vector3<f32>) -> Vec<Vector3<f32>> {
    let (x1, y1, z1) = (p1.x, p1.y, p1.z);
    let (x2, y2, z2) = (p2.x, p2.y, p2.z);

    vec![
        Vector3::new(x1, y1, z1), Vector3::new(x2, y1, z1), Vector3::new(x2, y2, z1),
        Vector3::new(x1, y1, z1), Vector3::new(x2, y2, z1), Vector3::new(x1, y2, z1),

        Vector3::new(x1, y1, z2), Vector3::new(x2, y1, z2), Vector3::new(x2, y2, z2),
        Vector3::new(x1, y1, z2), Vector3::new(x2, y2, z2), Vector3::new(x1, y2, z2),

        Vector3::new(x1, y1, z1), Vector3::new(x1, y2, z1), Vector3::new(x1, y2, z2),
        Vector3::new(x1, y1, z1), Vector3::new(x1, y2, z2), Vector3::new(x1, y1, z2),

        Vector3::new(x2, y1, z1), Vector3::new(x2, y2, z1), Vector3::new(x2, y2, z2),
        Vector3::new(x2, y1, z1), Vector3::new(x2, y2, z2), Vector3::new(x2, y1, z2),

        Vector3::new(x1, y2, z1), Vector3::new(x2, y2, z1), Vector3::new(x2, y2, z2),
        Vector3::new(x1, y2, z1), Vector3::new(x2, y2, z2), Vector3::new(x1, y2, z2),

        Vector3::new(x1, y1, z1), Vector3::new(x2, y1, z1), Vector3::new(x2, y1, z2),
        Vector3::new(x1, y1, z1), Vector3::new(x2, y1, z2), Vector3::new(x1, y1, z2),
    ]
}