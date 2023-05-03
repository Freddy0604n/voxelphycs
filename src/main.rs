/* (c) Frank Loijenga

    This program will be a voxel physics engine, maybe as PWS, maybe as a personal project.
    In its current state it should not be used for anything.

    Starting development: 04/05/2023 00:37    :)
*/

fn main() {
    let mut world = World::new(10, 10, 10);
    println!("{:?}", world);
    println!("{}", world.voxels.len());
}

#[derive(Debug)]
struct World {
    dimensions: (u32, u32, u32),
    voxels: Vec<Voxel>,
}

impl World {
    pub fn new(x: u32, y: u32, z: u32) -> World {
        let mut voxel_vec: Vec<Voxel> = Vec::new();

        for i in 0..x {
            for j in 0..y {
                for k in 0..z {
                    voxel_vec.push(Voxel {
                        position: (i, j, k),
                        colour: (0, 0, 0, 255),
                    });
                }
            }
        }
        World {
            dimensions: (x, y, z),
            voxels: voxel_vec,
        }
    }
}

#[derive(Debug)]
struct Voxel {
    position: (u32, u32, u32),
    colour: (u8, u8, u8, u8),
}
