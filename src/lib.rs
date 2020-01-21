pub struct Board {
    pub tiles: [Box<[String]>; 15],
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
                Box::new(["#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string(), "#".to_string()]),
            ],
        }
    }

    pub fn show_board(&self) {
        println!("   1   2   3   4   5   6   7   8   9   10");
        println!("a {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[0][0], self.tiles[0][1], self.tiles[0][2], self.tiles[0][3], self.tiles[0][4], self.tiles[0][5], self.tiles[0][6], self.tiles[0][7], self.tiles[0][8], self.tiles[0][9]);
        println!("b {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[1][0], self.tiles[1][1], self.tiles[1][2], self.tiles[1][3], self.tiles[1][4], self.tiles[1][5], self.tiles[1][6], self.tiles[1][7], self.tiles[1][8], self.tiles[1][9]);
        println!("c {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[2][0], self.tiles[2][1], self.tiles[2][2], self.tiles[2][3], self.tiles[2][4], self.tiles[2][5], self.tiles[2][6], self.tiles[2][7], self.tiles[2][8], self.tiles[2][9]);
        println!("d {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[3][0], self.tiles[3][1], self.tiles[3][2], self.tiles[3][3], self.tiles[3][4], self.tiles[3][5], self.tiles[3][6], self.tiles[3][7], self.tiles[3][8], self.tiles[3][9]);
        println!("e {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[4][0], self.tiles[4][1], self.tiles[4][2], self.tiles[4][3], self.tiles[4][4], self.tiles[4][5], self.tiles[4][6], self.tiles[4][7], self.tiles[4][8], self.tiles[4][9]);
        println!("f {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[5][0], self.tiles[5][1], self.tiles[5][2], self.tiles[5][3], self.tiles[5][4], self.tiles[5][5], self.tiles[5][6], self.tiles[5][7], self.tiles[5][8], self.tiles[5][9]);
        println!("g {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[6][0], self.tiles[6][1], self.tiles[6][2], self.tiles[6][3], self.tiles[6][4], self.tiles[6][5], self.tiles[6][6], self.tiles[6][7], self.tiles[6][8], self.tiles[6][9]);
        println!("h {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[7][0], self.tiles[7][1], self.tiles[7][2], self.tiles[7][3], self.tiles[7][4], self.tiles[7][5], self.tiles[7][6], self.tiles[7][7], self.tiles[7][8], self.tiles[7][9]);
        println!("i {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[8][0], self.tiles[8][1], self.tiles[8][2], self.tiles[8][3], self.tiles[8][4], self.tiles[8][5], self.tiles[8][6], self.tiles[8][7], self.tiles[8][8], self.tiles[8][9]);
        println!("j {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[9][0], self.tiles[9][1], self.tiles[9][2], self.tiles[9][3], self.tiles[9][4], self.tiles[9][5], self.tiles[9][6], self.tiles[9][7], self.tiles[9][8], self.tiles[9][9]);
        println!("k {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[10][0], self.tiles[10][1], self.tiles[10][2], self.tiles[10][3], self.tiles[10][4], self.tiles[10][5], self.tiles[10][6], self.tiles[10][7], self.tiles[10][8], self.tiles[10][9]);
        println!("l {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[11][0], self.tiles[11][1], self.tiles[11][2], self.tiles[11][3], self.tiles[11][4], self.tiles[11][5], self.tiles[11][6], self.tiles[11][7], self.tiles[11][8], self.tiles[11][9]);
        println!("m {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[12][0], self.tiles[12][1], self.tiles[12][2], self.tiles[12][3], self.tiles[12][4], self.tiles[12][5], self.tiles[12][6], self.tiles[12][7], self.tiles[12][8], self.tiles[12][9]);
        println!("n {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[13][0], self.tiles[13][1], self.tiles[13][2], self.tiles[13][3], self.tiles[13][4], self.tiles[13][5], self.tiles[13][6], self.tiles[13][7], self.tiles[13][8], self.tiles[13][9]);
        println!("o {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[14][0], self.tiles[14][1], self.tiles[14][2], self.tiles[14][3], self.tiles[14][4], self.tiles[14][5], self.tiles[14][6], self.tiles[14][7], self.tiles[14][8], self.tiles[14][9]);
    }
}

pub struct Unit {
    pub power: u32,
    pub range: u32,
    pub defense: u32,
    pub speed: u32,
}

impl Unit {
    pub fn new(power: u32, range: u32, defense: u32, speed: u32) -> Unit {
        Unit {power: power, range: range, defense: defense, speed: speed}
    }
}

pub struct Army {
    archers: String,
    calvary: String,
    infantry: String,
    arquebusiers: String,
    cannons: String,
}

impl Army {
    pub fn new(ar: u32, calv: u32, inf: u32, arque: u32, can: u32) -> Army {

        let new_archers: String;
        let new_calvary: String;
        let new_infantry: String;
        let new_arquebusiers: String;
        let new_cannons: String;

        if ar > 0 {
            new_archers = "A".to_string();
        } else {
            new_archers = "#".to_string();
        }
        if calv > 0 {
            new_calvary = "$".to_string();
        } else {
            new_calvary = "#".to_string();
        }
        if inf > 0 {
            new_infantry = "R".to_string();
        } else {
            new_infantry = "#".to_string();
        }
        if arque > 0 {
            new_arquebusiers = "V".to_string();
        } else {
            new_arquebusiers = "#".to_string();
        }
        if can > 0 {
            new_cannons = "%".to_string();
        } else {
            new_cannons = "#".to_string();
        }
        Army {archers: new_archers, calvary: new_calvary, infantry: new_infantry, arquebusiers: new_arquebusiers, cannons: new_cannons}
    }

    //pub fn move
}

pub fn Unit_Stats(archers: u32, calvary: u32, infantry: u32, arquebusiers: u32, cannons: u32) -> Vec<Unit> {
    let mut units: Vec<Unit> = Vec::new();

    let mut archer_unit: Unit;
    let mut calvary_unit: Unit;
    let mut infantry_unit: Unit;
    let mut arquebusiers_unit: Unit;
    let mut cannon_unit: Unit;

    if archers > 0 {
        archer_unit = Unit::new(1, 5, 1, 3);
    } else {
        archer_unit = Unit::new(0, 0, 0, 0);
    }

    units.push(archer_unit);

    if calvary > 0 {
        calvary_unit = Unit::new(5, 2, 5, 10);
    } else {
        calvary_unit = Unit::new(0, 0, 0, 0);
    }

    units.push(calvary_unit);

    if infantry > 0 {
        infantry_unit = Unit::new(3, 1, 6, 2);
    } else {
        infantry_unit = Unit::new(0, 0, 0, 0);
    }

    units.push(infantry_unit);

    if arquebusiers > 0 {
        arquebusiers_unit = Unit::new(6, 7, 1, 3);
    } else {
        arquebusiers_unit = Unit::new(0, 0, 0, 0);
    }

    units.push(arquebusiers_unit);

    if cannons > 0 {
        cannon_unit = Unit::new(10, 10, 1, 1);
    } else {
        cannon_unit = Unit::new(0, 0, 0, 0);
    }

    units.push(cannon_unit);

    units

}