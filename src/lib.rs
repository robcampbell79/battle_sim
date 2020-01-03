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
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[0][0], self.tiles[0][1], self.tiles[0][2], self.tiles[0][3], self.tiles[0][4], self.tiles[0][5], self.tiles[0][6], self.tiles[0][7], self.tiles[0][8], self.tiles[0][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[1][0], self.tiles[1][1], self.tiles[1][2], self.tiles[1][3], self.tiles[1][4], self.tiles[1][5], self.tiles[1][6], self.tiles[1][7], self.tiles[1][8], self.tiles[1][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[2][0], self.tiles[2][1], self.tiles[2][2], self.tiles[2][3], self.tiles[2][4], self.tiles[2][5], self.tiles[2][6], self.tiles[2][7], self.tiles[2][8], self.tiles[2][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[3][0], self.tiles[3][1], self.tiles[3][2], self.tiles[3][3], self.tiles[3][4], self.tiles[3][5], self.tiles[3][6], self.tiles[3][7], self.tiles[3][8], self.tiles[3][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[4][0], self.tiles[4][1], self.tiles[4][2], self.tiles[4][3], self.tiles[4][4], self.tiles[4][5], self.tiles[4][6], self.tiles[4][7], self.tiles[4][8], self.tiles[4][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[5][0], self.tiles[5][1], self.tiles[5][2], self.tiles[5][3], self.tiles[5][4], self.tiles[5][5], self.tiles[5][6], self.tiles[5][7], self.tiles[5][8], self.tiles[5][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[6][0], self.tiles[6][1], self.tiles[6][2], self.tiles[6][3], self.tiles[6][4], self.tiles[6][5], self.tiles[6][6], self.tiles[6][7], self.tiles[6][8], self.tiles[6][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[7][0], self.tiles[7][1], self.tiles[7][2], self.tiles[7][3], self.tiles[7][4], self.tiles[7][5], self.tiles[7][6], self.tiles[7][7], self.tiles[7][8], self.tiles[7][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[8][0], self.tiles[8][1], self.tiles[8][2], self.tiles[8][3], self.tiles[8][4], self.tiles[8][5], self.tiles[8][6], self.tiles[8][7], self.tiles[8][8], self.tiles[8][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[9][0], self.tiles[9][1], self.tiles[9][2], self.tiles[9][3], self.tiles[9][4], self.tiles[9][5], self.tiles[9][6], self.tiles[9][7], self.tiles[9][8], self.tiles[9][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[10][0], self.tiles[10][1], self.tiles[10][2], self.tiles[10][3], self.tiles[10][4], self.tiles[10][5], self.tiles[10][6], self.tiles[10][7], self.tiles[10][8], self.tiles[10][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[11][0], self.tiles[11][1], self.tiles[11][2], self.tiles[11][3], self.tiles[11][4], self.tiles[11][5], self.tiles[11][6], self.tiles[11][7], self.tiles[11][8], self.tiles[11][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[12][0], self.tiles[12][1], self.tiles[12][2], self.tiles[12][3], self.tiles[12][4], self.tiles[12][5], self.tiles[12][6], self.tiles[12][7], self.tiles[12][8], self.tiles[12][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[13][0], self.tiles[13][1], self.tiles[13][2], self.tiles[13][3], self.tiles[13][4], self.tiles[13][5], self.tiles[13][6], self.tiles[13][7], self.tiles[13][8], self.tiles[13][9]);
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.tiles[14][0], self.tiles[14][1], self.tiles[14][2], self.tiles[14][3], self.tiles[14][4], self.tiles[14][5], self.tiles[14][6], self.tiles[14][7], self.tiles[14][8], self.tiles[14][9]);
    }
}

pub struct Army {
    pub archers: u32,
    pub calvary: u32,
    pub infantry: u32,
    pub arquebusiers: u32,
    pub cannons:u32,
}

impl Army {
    pub fn new(archers: u32, calvary: u32, infantry: u32, arquebusiers: u32, cannons: u32) -> Army {
        Army {archers: archers, calvary: calvary, infantry: infantry, arquebusiers: arquebusiers, cannons: cannons}
    }

    //pub fn move
}