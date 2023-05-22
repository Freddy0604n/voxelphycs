// TODO: Update high proton number elements names (e.g. 118)

#[derive(Debug)]
pub struct Atom {
    pub element: Element,
    pub neutrons: usize,
}

impl Atom {
    pub fn new(number: usize, isotope: usize) -> Atom {
        Atom {
            element: Self::get_element(number),
            neutrons: isotope - number,
        }
    }

    // gives the name of the element TODO: add the elements
    pub fn get_name(&self, short: bool) -> &str {
        use Element::*;
        match self.element {
            H => {
                if short {
                    "H"
                } else {
                    "Hydrogen"
                }
            }
            He => {
                if short {
                    "He"
                } else {
                    "Helium"
                }
            }
            Li => {
                if short {
                    "Li"
                } else {
                    "Lithium"
                }
            }
            Be => {
                if short {
                    "Be"
                } else {
                    "Beryllium"
                }
            }
            B => {
                if short {
                    "B"
                } else {
                    "Boron"
                }
            }
            C => {
                if short {
                    "C"
                } else {
                    "Carbon"
                }
            }
            N => {
                if short {
                    "N"
                } else {
                    "Nitrogen"
                }
            }
            O => {
                if short {
                    "O"
                } else {
                    "Oxygen"
                }
            }
            F => {
                if short {
                    "F"
                } else {
                    "Fluorine"
                }
            }
            Ne => {
                if short {
                    "Ne"
                } else {
                    "Neon"
                }
            }
            Na => {
                if short {
                    "Na"
                } else {
                    "Sodium"
                }
            }
            Mg => {
                if short {
                    "Mg"
                } else {
                    "Magnesium"
                }
            }
            Al => {
                if short {
                    "Al"
                } else {
                    "Aluminium"
                }
            }
            Si => {
                if short {
                    "Si"
                } else {
                    "Silicon"
                }
            }
            P => {
                if short {
                    "P"
                } else {
                    "Phosphorous"
                }
            }
            S => {
                if short {
                    "S"
                } else {
                    "Sulfur"
                }
            }
            Cl => {
                if short {
                    "Cl"
                } else {
                    "Chlorine"
                }
            }
            Ar => {
                if short {
                    "Ar"
                } else {
                    "Argon"
                }
            }
            K => {
                if short {
                    "K"
                } else {
                    "Potassium"
                }
            }
            Ca => {
                if short {
                    "Ca"
                } else {
                    "Calcium"
                }
            }
            Sc => {
                if short {
                    "Sc"
                } else {
                    "Scandium"
                }
            }
            Ti => {
                if short {
                    "Ti"
                } else {
                    "Titanium"
                }
            }
            V => {
                if short {
                    "V"
                } else {
                    "Vanadium"
                }
            }
            Cr => {
                if short {
                    "Cr"
                } else {
                    "Chromium"
                }
            }
            Mn => {
                if short {
                    "Mn"
                } else {
                    "Manganese"
                }
            }
            Fe => {
                if short {
                    "Fe"
                } else {
                    "Iron"
                }
            }
            Co => {
                if short {
                    "Co"
                } else {
                    "Cobalt"
                }
            }
            Ni => {
                if short {
                    "Ni"
                } else {
                    "Nickel"
                }
            }
            Cu => {
                if short {
                    "Cu"
                } else {
                    "Copper"
                }
            }
            Zn => {
                if short {
                    "Zn"
                } else {
                    "Zinc"
                }
            }
            Ga => {
                if short {
                    "Ga"
                } else {
                    "Gallium"
                }
            }
            Ge => {
                if short {
                    "Ge"
                } else {
                    "Germanium"
                }
            }
            As => {
                if short {
                    "As"
                } else {
                    "Arsenic"
                }
            }
            Se => {
                if short {
                    "Se"
                } else {
                    "Selenium"
                }
            }
            Br => {
                if short {
                    "Br"
                } else {
                    "Bromine"
                }
            }
            Kr => {
                if short {
                    "Kr"
                } else {
                    "Krypton"
                }
            }
            Rb => {
                if short {
                    "Rb"
                } else {
                    "Rubidium"
                }
            }
            Sr => {
                if short {
                    "Sr"
                } else {
                    "Strontium"
                }
            }
            Y => {
                if short {
                    "Y"
                } else {
                    "Ytterium"
                }
            }
            Zr => {
                if short {
                    "Zr"
                } else {
                    "Zirconium"
                }
            }
            Nb => {
                if short {
                    "Nb"
                } else {
                    "Niobium"
                }
            }
            Mo => {
                if short {
                    "Mo"
                } else {
                    "Molybdenum"
                }
            }
            Tc => {
                if short {
                    "Tc"
                } else {
                    "Technetium"
                }
            }
            Ru => {
                if short {
                    "Ru"
                } else {
                    "Ruthenium"
                }
            }
            Rh => {
                if short {
                    "Rh"
                } else {
                    "Rhodium"
                }
            }
            Pd => {
                if short {
                    "Pd"
                } else {
                    "Palladium"
                }
            }
            Ag => {
                if short {
                    "Ag"
                } else {
                    "Silver"
                }
            }
            Cd => {
                if short {
                    "Cd"
                } else {
                    "Cadmium"
                }
            }
            In => {
                if short {
                    "In"
                } else {
                    "Indium"
                }
            }
            Sn => {
                if short {
                    "Sn"
                } else {
                    "Tin"
                }
            }
            Sb => {
                if short {
                    "Sb"
                } else {
                    "Antimony"
                }
            }
            Te => {
                if short {
                    "Te"
                } else {
                    "Tellurium"
                }
            }
            I => {
                if short {
                    "I"
                } else {
                    "Iodine"
                }
            }
            Xe => {
                if short {
                    "Xe"
                } else {
                    "Xenon"
                }
            }
            Cs => {
                if short {
                    "Cs"
                } else {
                    "Caesium"
                }
            }
            Ba => {
                if short {
                    "Ba"
                } else {
                    "Barium"
                }
            }
            La => {
                if short {
                    "La"
                } else {
                    "Lanthanum"
                }
            }
            Ce => {
                if short {
                    "Ce"
                } else {
                    "Cerium"
                }
            }
            Pr => {
                if short {
                    "Pr"
                } else {
                    "Praseodymium"
                }
            }
            Nd => {
                if short {
                    "Nd"
                } else {
                    "Neodymium"
                }
            }
            Pm => {
                if short {
                    "Pm"
                } else {
                    "Promethium"
                }
            }
            Sm => {
                if short {
                    "Sm"
                } else {
                    "Samarium"
                }
            }
            Eu => {
                if short {
                    "Eu"
                } else {
                    "Europium"
                }
            }
            Gd => {
                if short {
                    "Gd"
                } else {
                    "Gadolinium"
                }
            }
            Tb => {
                if short {
                    "Tb"
                } else {
                    "Terbium"
                }
            }
            Dy => {
                if short {
                    "Dy"
                } else {
                    "Dysprosium"
                }
            }
            Ho => {
                if short {
                    "Ho"
                } else {
                    "Holmium"
                }
            }
            Er => {
                if short {
                    "Er"
                } else {
                    "Erbium"
                }
            }
            Tm => {
                if short {
                    "Tm"
                } else {
                    "Thulium"
                }
            }
            Yb => {
                if short {
                    "Yb"
                } else {
                    "Ytterbium"
                }
            }
            Lu => {
                if short {
                    "Lu"
                } else {
                    "Lutetium"
                }
            }
            Hf => {
                if short {
                    "Hf"
                } else {
                    "Hafnium"
                }
            }
            Ta => {
                if short {
                    "Ta"
                } else {
                    "Tantalum"
                }
            }
            W => {
                if short {
                    "W"
                } else {
                    "Tungsten"
                }
            } // met pijn in mijn hart :'(
            Re => {
                if short {
                    "Re"
                } else {
                    "Rhenium"
                }
            }
            Os => {
                if short {
                    "Os"
                } else {
                    "Osmium"
                }
            }
            Ir => {
                if short {
                    "Ir"
                } else {
                    "Iridium"
                }
            }
            Pt => {
                if short {
                    "Pt"
                } else {
                    "Platinum"
                }
            }
            Au => {
                if short {
                    "Au"
                } else {
                    "Gold"
                }
            }
            _ => panic!("This element has not been implemented yet in the get_short function"),
        }
    }

    // returns the periodic number of the element
    pub fn get_number(&self) -> usize {
        use Element::*;
        match self.element {
            H => 1,
            He => 2,
            Li => 3,
            Be => 4,
            B => 5,
            C => 6,
            N => 7,
            O => 8,
            F => 9,
            Ne => 10,
            Na => 11,
            Mg => 12,
            Al => 13,
            Si => 14,
            P => 15,
            S => 16,
            Cl => 17,
            Ar => 18,
            K => 19,
            Ca => 20,
            Sc => 21,
            Ti => 22,
            V => 23,
            Cr => 24,
            Mn => 25,
            Fe => 26,
            Co => 27,
            Ni => 28,
            Cu => 29,
            Zn => 30,
            Ga => 31,
            Ge => 32,
            As => 33,
            Se => 34,
            Br => 35,
            Kr => 36,
            Rb => 37,
            Sr => 38,
            Y => 39,
            Zr => 40,
            Nb => 41,
            Mo => 42,
            Tc => 43,
            Ru => 44,
            Rh => 45,
            Pd => 46,
            Ag => 47,
            Cd => 48,
            In => 49,
            Sn => 50,
            Sb => 51,
            Te => 52,
            I => 53,
            Xe => 54,
            Cs => 55,
            Ba => 56,
            La => 57,
            Ce => 58,
            Pr => 59,
            Nd => 60,
            Pm => 61,
            Sm => 62,
            Eu => 63,
            Gd => 64,
            Tb => 65,
            Dy => 66,
            Ho => 67,
            Er => 68,
            Tm => 69,
            Yb => 70,
            Lu => 71,
            Hf => 72,
            Ta => 73,
            W => 74,
            Re => 75,
            Os => 76,
            Ir => 77,
            Pt => 78,
            Au => 79,
            Hg => 80,
            Tl => 81,
            Pb => 82,
            Bi => 83,
            Po => 84,
            At => 85,
            Rn => 86,
            Fr => 87,
            Ra => 88,
            Ac => 89,
            Th => 90,
            Pa => 91,
            U => 92,
            Np => 93,
            Pu => 94,
            Am => 95,
            Cm => 96,
            Bk => 97,
            Cf => 98,
            Es => 99,
            Fm => 100,
            Md => 101,
            No => 102,
            Lr => 103,
            Rf => 104,
            Db => 105,
            Sg => 106,
            Bh => 107,
            Hs => 108,
            Mt => 109,
            Ds => 110,
            Rg => 111,
            Cn => 112,
            Nh => 113,
            Fl => 114,
            Mc => 115,
            Lv => 116,
            Ts => 117,
            Og => 118,
            _ => panic!("Weird element"),
        }
    }

    //TODO : add all elements
    fn get_element(number: usize) -> Element {
        use Element::*;
        match number {
            0 => panic!("Use of element with number 0"),
            1 => H,
            2 => He,
            3 => Li,
            4 => Be,
            5 => B,
            6 => C,
            7 => N,
            8 => O,
            9 => F,
            10 => Ne,
            11 => Na,
            12 => Mg,
            13 => Al,
            14 => Si,
            15 => P,
            16 => S,
            17 => Cl,
            18 => Ar,
            19 => K,
            20 => Ca,
            21 => Sc,
            22 => Ti,
            23 => V,
            24 => Cr,
            25 => Mn,
            26 => Fe,
            27 => Co,
            28 => Ni,
            29 => Cu,
            30 => Zn,
            31 => Ga,
            32 => Ge,
            33 => As,
            34 => Se,
            35 => Br,
            36 => Kr,
            37 => Rb,
            38 => Sr,
            39 => Y,
            40 => Zr,
            41 => Nb,
            42 => Mo,
            43 => Tc,
            44 => Ru,
            45 => Rh,
            46 => Pd,
            47 => Ag,
            48 => Cd,
            49 => In,
            50 => Sn,
            51 => Sb,
            52 => Te,
            53 => I,
            54 => Xe,
            55 => Cs,
            56 => Ba,
            57 => La,
            58 => Ce,
            59 => Pr,
            60 => Nd,
            61 => Pm,
            62 => Sm,
            63 => Eu,
            64 => Gd,
            65 => Tb,
            66 => Dy,
            67 => Ho,
            68 => Er,
            69 => Tm,
            70 => Yb,
            71 => Lu,
            72 => Hf,
            73 => Ta,
            74 => W,
            75 => Re,
            76 => Os,
            77 => Ir,
            78 => Pt,
            79 => Au,
            80 => Hg,
            81 => Tl,
            82 => Pb,
            83 => Bi,
            84 => Po,
            85 => At,
            86 => Rn,
            87 => Fr,
            88 => Ra,
            89 => Ac,
            90 => Th,
            91 => Pa,
            92 => U,
            93 => Np,
            94 => Pu,
            95 => Am,
            96 => Cm,
            97 => Bk,
            98 => Cf,
            99 => Es,
            100 => Fm,
            101 => Md,
            102 => No,
            103 => Lr,
            104 => Rf,
            105 => Db,
            106 => Sg,
            107 => Bh,
            108 => Hs,
            109 => Mt,
            110 => Ds,
            111 => Rg,
            112 => Cn,
            113 => Nh,
            114 => Fl,
            115 => Mc,
            116 => Lv,
            117 => Ts,
            118 => Og,
            _ => panic!("Use of undecleared element"),
        }
    }
}

// all the elements according to BiNaS
#[derive(Debug)]
pub enum Element {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}
