
use crate::mesh::Mesh;
use nalgebra::{Vector2, Vector3};

pub struct VoxelManager {
    voxels: Vec<Vec<Vec<bool>>>,
    length: usize,
    width: usize,
    height: usize
}

impl VoxelManager {
    pub fn new(length: usize, width: usize, height: usize) -> Self{
        let mut voxels : Vec<Vec<Vec<bool>>> = (0..length).into_iter().map(|x| {
            (0..width).into_iter().map(|y| {
                (0..height).map(|z| false).collect()
            }).collect()
        }).collect();

        voxels[0][0][0] = true;
        voxels[0][1][0] = true;
        voxels[0][2][0] = true;

        Self {
            voxels,
            length,
            width,
            height
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

                    println!("Found a true");

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

        let count = verts.len();

        let uvs = (0..count).into_iter().map(|x| {
            Vector2::new(0.0, 0.0)
        }).collect();

        Mesh::new(gl, verts, (0..count).into_iter().map(|x| x as u32).collect(), uvs, false)
    }
}