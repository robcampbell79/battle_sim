use battle_sim::Board;
use battle_sim::Army;
use std::io;
use battle_sim::random_setup;
use battle_sim::setup;
use battle_sim::board_placement;

fn main() {

    let mut create: String;
    let random: bool;
    let mut arc: String;
    let mut calv: String;
    let mut inf: String;
    let mut arque: String;
    let mut cann: String;
    let mut error: u32 = 0;
    let mut col = String::new();
    let mut row = String::new();

    let mut board = Board::new();
    let mut army: Army;

    army = setup(0, 0, 0, 0, 0);

    board.show_board();

    println!("Do you wish to set up your army or do you want the system to do it for you?");
    println!("Please type system or random to use a system setup, otherwise type manual or me to do manual setup.");

    create = String::new();

    io::stdin().read_line(&mut create).expect("Invalid input.");

    match create.trim() {
        "system" | "System" | "SYSTEM" | "random" | "Random" | "RANDOM" | "rand" | "sys" => random = true,
        _ => random = false,
    }

    if random == true {
        army = random_setup();
        println!("Your generated army: {:?}", army);
    } else {
        println!("Please enter number of archers, not to exceed 500:");

        arc = String::new();

        io::stdin().read_line(&mut arc).expect("Invalid input.");

        let arc: u32 = match arc.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if arc > 500 {
            println!("You can't have that many, sorry.");

            error = error + 1;
        }

        println!("Please enter number of calvary, not to exceed 500:");

        calv = String::new();

        io::stdin().read_line(&mut calv).expect("Invalid input.");

        let calv: u32 = match calv.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if calv > 500 {
            println!("You can't have that many, sorry.");

            error = error + 1;
        }

        println!("Please enter number of infantry, not to exceed 500:");

        inf = String::new();

        io::stdin().read_line(&mut inf).expect("Invalid input.");

        let inf: u32 = match inf.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if inf > 500 {
            println!("You can't have that many, sorry.");

            error = error + 1;
        }

        println!("Please enter number of arquebusiers, not to exceed 500:");

        arque = String::new();

        io::stdin().read_line(&mut arque).expect("Invalid input.");

        let arque: u32 = match arque.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if arque > 500 {
            println!("You can't have that many, sorry.");

            error = error + 1;
        }

        println!("Please enter number of cannons, not to exceed 50:");

        cann = String::new();

        io::stdin().read_line(&mut cann).expect("Invalid input.");

        let cann: u32 = match cann.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if cann > 50 {
            println!("You can't have that many, sorry.");

            error = error + 1;
        }

        if error < 1 {
            army = setup(arc, calv, inf, arque, cann);
            println!("Your army: {:?}", army);
        } else {
            println!("Sorry, but there were errors.");
        }
    }

    println!("Time to place your army, column is 1-10 and row is a-o.");
    
    if army.archers > 0 {
        println!("Where do you want to place your archers?");
        println!("Row a-o");

        io::stdin().read_line(&mut row).expect("Invalid input.");

        println!("Column 1-10");

        io::stdin().read_line(&mut col).expect("Invalid input.");

        board_placement(&mut board, "archers".to_string(), 1, &mut row, &mut col);

        board.show_board();
    }

    if army.calvary > 0 {
        row = String::new();
        col = String::new();

        println!("Where do you want to place your calvary?");
        println!("Row a-o");

        io::stdin().read_line(&mut row).expect("Invalid input.");

        println!("Column 1-10");

        io::stdin().read_line(&mut col).expect("Invalid input.");

        board_placement(&mut board, "calvary".to_string(), 1, &mut row, &mut col);

        board.show_board();
    }

    if army.infantry > 0 {
        row = String::new();
        col = String::new();
        
        println!("Where do you want to place your infantry?");
        println!("Row a-o");

        io::stdin().read_line(&mut row).expect("Invalid input.");

        println!("Column 1-10");

        io::stdin().read_line(&mut col).expect("Invalid input.");

        board_placement(&mut board, "infantry".to_string(), 1, &mut row, &mut col);

        board.show_board();
    }

    if army.arquebusiers > 0 {
        row = String::new();
        col = String::new();

        println!("Where do you want to place your arquebusiers?");
        println!("Row a-o");

        io::stdin().read_line(&mut row).expect("Invalid input.");

        println!("Column 1-10");

        io::stdin().read_line(&mut col).expect("Invalid input.");

        board_placement(&mut board, "arquebusiers".to_string(), 1, &mut row, &mut col);

        board.show_board();
    }

    if army.cannons > 0 {
        row = String::new();
        col = String::new();
        
        println!("Where do you want to place your cannons?");
        println!("Row a-o");

        io::stdin().read_line(&mut row).expect("Invalid input.");

        println!("Column 1-10");

        io::stdin().read_line(&mut col).expect("Invalid input.");

        board_placement(&mut board, "cannons".to_string(), 1, &mut row, &mut col);

        board.show_board();
    }

    board.show_board();
}