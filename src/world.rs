#[derive(Debug)]
pub struct World {
    pub dimensions: (u32, u32, u32),
    pub voxels: Vec<Voxel>,
}

impl World {
    pub fn new(x: u32, y: u32, z: u32) -> World {
        let mut voxel_vec: Vec<Voxel> = Vec::new();

        for i in 0..x {
            for j in 0..y {
                for k in 0..z {
                    voxel_vec.push(Voxel {
                        position: (i, j, k),
                        composition: Composition::air(),
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
pub struct Voxel {
    position: (u32, u32, u32),
    composition: Composition,
}

impl Voxel {
    //TODO: make vxprint print all the molecules with their concentration
    pub fn vxprint(&self) {
        // should print the voxels position and composition.
        // the composition should be in molecule drawings.
        println!(
            "Voxel: {}, {}, {}",
            self.position.0, self.position.1, self.position.2
        );
    }
}

#[derive(Debug)]
pub struct Composition {
    molecules: Vec<Molecule>,
    concentration: Vec<u8>,
}

impl Composition {
    // air returns 100% hydrogen, should be changed
    pub fn air() -> Composition {
        Composition {
            molecules: vec![Molecule {
                atoms: vec![Atom::H, Atom::H],
                connections: vec![vec![1]],
            }],
            concentration: vec![255],
        }
    }
}

/*Every atom gets its own connection vector. This vector contains the connections between the following atoms.
Example:
Say atoms is [C, H, H, H, C, H, H, C, H, H, H]
and connections is [[1, 2, 3], [1, 2], [1, 2, 3]]
then the molecule is
     H   H   H
     |   |   |
 H - C - C - C - H
     |   |   |
     H   H   H
*/

#[derive(Debug)]
pub struct Molecule {
    atoms: Vec<Atom>,
    connections: Vec<Vec<u8>>,
}

impl Molecule {
/*    pub fn as_drawing_string(&self) -> String {
        // formats the molecule as a drawing in the String format for debug purposes
    }*/
}

//TODO: Add all atoms
#[derive(Debug)]
pub enum Atom {
    C,
    N,
    H,
}
