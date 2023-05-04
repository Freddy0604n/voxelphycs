/* (c) Frank Loijenga

    This program will be a voxel physics engine, maybe as PWS, maybe as a personal project.
    In its current state it should not be used for anything.

    Starting development: 04/05/2023 00:37    :)
*/

mod world;
use world::World;

fn main() {
    let mut world = World::new(10, 10, 10);
    for voxel in world.voxels {
        voxel.vxprint();
    }
}
