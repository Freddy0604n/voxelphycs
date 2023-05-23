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
                        composition: Composition::propane(),
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
    pub fn propane() -> Composition {
        Composition {
            molecules: vec![Molecule {
                atoms: vec![
                    Atom::new(6, 6), // C
                    Atom::new(1, 1), // H
                    Atom::new(1, 1), // H
                    Atom::new(1, 1), // H
                    Atom::new(6, 6), // C
                    Atom::new(1, 1), // H
                    Atom::new(1, 1), // H
                    Atom::new(6, 6), // C
                    Atom::new(1, 1), // H
                    Atom::new(1, 1), // H
                    Atom::new(1, 1), // H
                ],
                connections: vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 3]],
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
        // TODO: Add more configurations than three single connections
        /*
        This works following these steps:
        - select the script to run with a match statement according to amount of secondary atoms and the position in the molecule
        - each script contains a format!() macro with a hardcoded string that fits the primary atom
        */
        // BUG: This script generates data that is not easily printable. The lines should be added to each other.
        let mut result: Vec<String> = vec![];
        let mut index = 0;
        for connection in self.connections.iter() {
            let main_atom = self.atoms[index].get_name(true).to_string();
            match connection.len() {
                //TODO: add the option 2 & 3
                0 => {
                    if index == self.connections.len() - 1 {
                        result.append(&mut vec![format!("{}    ", main_atom)]);
                    } else {
                        result.append(&mut vec![format!("{} -  ", main_atom)]);
                    }
                }
                1 => {
                    // this checks whether the secondary atom should be added to the side
                    if index != 0 && index != self.connections.len() - 1 {
                        result.append(&mut vec![format!(
                            r"
                                {}   
                                |   
                                {} - ",
                            self.atoms[index + connection[0]].get_name(true),
                            main_atom
                        )
                        .to_string()]);
                    } else if index == 0 && index == self.connections.len() - 1 {
                        result.append(&mut vec![format!(
                            "{} - {}",
                            self.atoms[index + connection[0]].get_name(true),
                            main_atom
                        )])
                    } else if index == 0 {
                        result.append(&mut vec![format!(
                            "{} - {} - ",
                            self.atoms[index + connection[0]].get_name(true),
                            main_atom
                        )
                        .to_string()]);
                    }
                    //TODO: last atom exception
                }
                2 => {
                    result.append(&mut vec![format!(
                        "
                        {}    
                        |
                        {} - 
                        |    
                        {}    ",
                        self.atoms[index + connection[0]].get_name(true),
                        main_atom,
                        self.atoms[index + connection[1]].get_name(true)
                    )
                    .to_string()]);
                }
                3 => {
                    if index == 0 {
                        result.append(&mut vec![format!(
                            r"
                                {}   
                                |    
                            {} - {} - 
                                |   
                                {}
                            ",
                            self.atoms[index + connection[0]].get_name(true),
                            self.atoms[index + connection[1]].get_name(true),
                            main_atom,
                            self.atoms[index + connection[1]].get_name(true)
                        )
                        .to_string()]);
                    }
                }
                _ => return vec![String::from("test2")],
            }
            index += 1;
        }
        return result;
    }
}
