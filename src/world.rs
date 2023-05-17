mod atom;
use atom::*;

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
        for i in self.composition.molecules[0].as_drawing_string() {
            print!("{}", i);
        }
        print!("\n");
    }
}

#[derive(Debug)]
pub struct Composition {
    molecules: Vec<Molecule>,
    concentration: Vec<u8>,
}

impl Composition {
    //TODO: air returns 100% hydrogen, should be changed
    pub fn air() -> Composition {
        Composition {
            molecules: vec![Molecule {
                atoms: vec![Atom::new(1, 1), Atom::new(1, 1)],
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
    connections: Vec<Vec<usize>>,
}

impl Molecule {
    pub fn n_main_atoms(&self) -> usize {
        // returns the number of main atoms
        let mut secondary_atoms = 0;
        for i in &self.connections {
            secondary_atoms += i.len();
        }
        return self.atoms.len() - secondary_atoms;
    }
    pub fn as_drawing_string(&self) -> Vec<String> {
        // formats the molecule as a drawing in the String format for debug purposes
        // formats are the different drawings for different configurations TODO: there are more possible configurations

        /*
        TODO:
        This system needs to be changed to a system using the format!() macro.
        This will work as the following:
        - select the script to run with a match statement according to amount of secondary atoms and the position in the molecule
        - each script contains a format!() macro with a hardcoded string that fits the primary atom
        */
        let mut result: Vec<String> = vec![];
        let mut index = 0;
        for connection in self.connections.iter() {
            //TODO: optimisation possible by assigning the short names to local variables first
            match connection.len() {
                //TODO: add the option 2
                0 => {
                    if index != self.connections.len() - 1 {
                        result.append(&mut vec![
                            format!("{} -  ", self.atoms[index].get_short()).to_string()
                        ]);
                    }
                    //TODO: add first and last atom exceptions
                }
                1 => {
                    // this checks whether the secondary atom should be added to the side
                    if index != 0 && index != self.connections.len() - 1 {
                        result.append(&mut vec![format!(
                            r"
                                {}   
                                |   
                                {} - ",
                            self.atoms[index + connection[0]].get_short(),
                            self.atoms[index].get_short()
                        )
                        .to_string()]);
                    } else if index == 0 {
                        //BUG: if the first atom is also the last line is still present
                        result.append(&mut vec![format!(
                            "{} - {} - ",
                            self.atoms[index + connection[0]].get_short(),
                            self.atoms[index].get_short()
                        )
                        .to_string()]);
                    }
                    //TODO: last atom exception
                }
                _ => return vec![String::from("test2")],
            }
            index += 1;
        }
        return result;
    }
}
