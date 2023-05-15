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

    // gives the short name of the element TODO: add the elements
    pub fn get_short(&self) -> &str {
        match self.element {
            Element::H => "H",
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
    Uut,
    Fl,
    Uup,
    Lv,
    Uus,
    Uuo,
}
