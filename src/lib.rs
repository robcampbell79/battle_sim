extern crate rand;

use rand::Rng;

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

#[derive(Debug)]
pub struct Army {
    pub archers: u32,
    pub calvary: u32,
    pub infantry: u32,
    pub arquebusiers: u32,
    pub cannons: u32,
}

impl Army {
    pub fn new(arch: u32, calv: u32, inf: u32, arqu: u32, cann: u32) -> Army {
        Army {archers: arch, calvary: calv, infantry: inf, arquebusiers: arqu, cannons: cann}
    }
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

//Just a random message
//just another test message

pub fn setup(archer: u32, calvary: u32, infantry: u32, arquebusiers: u32, cannons: u32) -> Army {
    let arch = archer;
    let calv = calvary;
    let inf = infantry;
    let arque = arquebusiers;
    let cann = cannons;

    let army = Army::new(arch, calv, inf, arque, cann);

    army
}

pub fn random_setup() -> Army {
    let archer: u32 = rand::thread_rng().gen_range(100, 501);
    let calvary: u32 = rand::thread_rng().gen_range(100, 501);
    let infantry: u32 = rand::thread_rng().gen_range(100, 501);
    let arquebusiers: u32 = rand::thread_rng().gen_range(100, 501);
    let mut cannons: u32 = rand::thread_rng().gen_range(0, 51);

    if cannons < 2 {
        cannons = 0;
    }

    let army = Army::new(archer, calvary, infantry, arquebusiers, cannons);
    
    army
}

pub fn auto_companies(unit: String, amount: u32) -> Vec<u32> {
    let mut companies = Vec::new();

    let divisor: u32;

    if unit == "cannons" {
        
        divisor = 10;

        let company: u32 = amount / divisor;

        let mut extra = amount % divisor;

        companies.push(company);

        while company > 0 {

            if company == 1 {
                extra += 10;
                companies.push(extra);
            } else {
                companies.push(10);
            }

        }
    } else {

        divisor = 100;

        let company: u32 = amount / divisor;
    
        let mut extra = amount % divisor;
    
        companies.push(company);
    
        while company > 0 {
    
            if company == 1 {
                extra += 100;
                companies.push(extra);
            } else {
                companies.push(100);
            }
    
        }
    }


    companies

}

pub fn manual_companies(unit: String, amount: u32, divisor: u32) -> Vec<u32> {
    let mut companies = Vec::new();

    if divisor % 10 != 0 {

        panic!("The divisor must be divisible by 10.");

    } else {

        if divisor < amount {

            if unit == "cannons" {
        
                let company: u32 = amount / divisor;
        
                let mut extra = amount % divisor;
        
                companies.push(company);
        
                while company > 0 {
        
                    if company == 1 {
                        extra += 10;
                        companies.push(extra);
                    } else {
                        companies.push(10);
                    }
        
                }
            } else {
        
                let company: u32 = amount / divisor;
            
                let mut extra = amount % divisor;
            
                companies.push(company);
            
                while company > 0 {
            
                    if company == 1 {
                        extra += 100;
                        companies.push(extra);
                    } else {
                        companies.push(100);
                    }
            
                }
            }
        } else {

            panic!("The divisor must be less than the amount of soldiers in your unit.");
        }

    }



    companies

}

pub fn army_tokens() -> Vec<String> {

        let mut tokens = Vec::new();

        let new_archers: String;
        let new_calvary: String;
        let new_infantry: String;
        let new_arquebusiers: String;
        let new_cannons: String;

        new_archers = "A".to_string();

        tokens.push(new_archers);

        new_calvary = "$".to_string();

        tokens.push(new_calvary);

        new_infantry = "R".to_string();

        tokens.push(new_infantry);

        new_arquebusiers = "V".to_string();

        tokens.push(new_arquebusiers);

        new_cannons = "%".to_string();

        tokens.push(new_cannons);

        tokens
        
    }

pub fn board_placement(board: &mut Board, army: String, companies: u32, row: &str, column: &str) {

    println!("r: {}, c: {}", row, column);

    let tokens = army_tokens();

    let row_num = row.trim();
    let r: usize;

    match row_num {
        "a" => r = 0,
        "b" => r = 1,
        "c" => r = 2,
        "d" => r = 3,
        "e" => r = 4,
        "f" => r = 5,
        "g" => r = 6,
        "h" => r = 7,
        "i" => r = 8,
        "j" => r = 9,
        "k" => r = 10,
        "l" => r = 11,
        "m" => r = 12,
        "n" => r = 13,
        "o" => r = 14,
        _ => r = 15,
    }

    let mut c: usize = match column.trim().parse() {
            Ok(usize) => usize,
            Err(_) => 12,
    };

    if c < 11 && c > 0 {
        c -= 1;
    } else {
        println!("Invalid column.");
    }

    if army == "archers" {
        if board.tiles[r][c] != "#" {
            println!("This space is already occupied by an unit.");
        } else {
            board.tiles[r][c] = tokens[0].to_string();
        }
    } 
    else if army == "calvary" {
        if board.tiles[r][c] != "#" {
            println!("This space is already occupied by an unit.");
        } else {
            board.tiles[r][c] = tokens[1].to_string();
        }
    } 
    else if army == "infantry" {
        if board.tiles[r][c] != "#" {
            println!("This space is already occupied by an unit.");
        } else {
            board.tiles[r][c] = tokens[2].to_string();
        }
    } 
    else if army == "arquebusiers" {
        if board.tiles[r][c] != "#" {
            println!("This space is already occupied by an unit.");
        } else {
            board.tiles[r][c] = tokens[3].to_string();
        }
    } 
    else if army == "cannons" {
        if board.tiles[r][c] != "#" {
            println!("This space is already occupied by an unit.");
        } else {
            board.tiles[r][c] = tokens[4].to_string();
        }
    } 
    else {
        println!("Invalid input.");
    }
}

// pub fn turn(actions: Unit, ) {

// }