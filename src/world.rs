mod atom;
use atom::*;

#[derive(Debug)]
pub struct World {
    pub dimensions: (u32, u32, u32),
    pub voxels: Vec<Voxel>,
}

impl World {
    pub fn new(size_x: u32, size_y: u32, size_z: u32) -> World {
        let mut voxel_vec: Vec<Voxel> = Vec::new();

        for i in 0..size_x {
            for j in 0..size_y {
                for k in 0..size_z {
                    voxel_vec.push(Voxel::new(i, j, k));
                }
            }
        }
        
        World {
            dimensions: (size_x, size_y, size_z),
            voxels: voxel_vec,
        }
    }
}

#[derive(Debug)]
pub struct Voxel {
    position: (u32, u32, u32),
}

impl Voxel {
    pub fn new (x: u32, y: u32, z: u32) -> Voxel {
        Voxel {
            position: (x, y, z)
        }
    }
}

