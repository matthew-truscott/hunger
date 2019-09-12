/// Main: Hunger Game Simulator v0.1
/// Author: Akharis Ren
/// 
/// This is the main file
/// 

extern crate rand;

mod tribute;
mod roster;

use std::io;

use rand::Rng;

fn welcome_text() {
    println!("-----------------------------------------");
    println!("Welcome to the Hunger Game Simulator v0.1");
    println!("Author: Akharis Ren"); 
    println!("-----------------------------------------");
    println!("Select an option (1/2)");
    println!("1. Create roster");
    println!("2. Exit");
}

/// Reads input from stdin and returns a String
/// 
/// * `Return`: type{String}
fn read_input() -> String {

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(error) => println!("error: {}", error),
    }
    input
}

fn parse_choice() -> i32 {
    let choice = read_input()
            .trim()
            .parse::<i32>();
    let number = match choice {
        Ok(number) => number,
        Err(_) => 0,
    };
    number
}

fn parse_positive_integer(upper_limit: i32) -> i32 {
    let mut limit = upper_limit;
    if upper_limit < 0 {
        // invalid! assume that we have an unlimited input mode
        limit = i32::max_value();
    }

    let choice = read_input()
            .trim()
            .parse::<i32>();
    let mut number = match choice {
        Ok(number) => number,
        Err(_) => 0,
    };

    if number < 0 || number > limit {
        // error value assumed to be -1
        number = -1;
    }
    number
}

/// Interactive Roster Creation
fn interactive_roster_creation() -> i32 {
    let mut status: i32 = 0;

    println!("----------------------------------");
    println!("Interative roster creation module.");
    println!("----------------------------------");
    println!("Select an option (1/2/3)");
    println!("1. Create from scratch");
    println!("2. Create from files");
    println!("3. Go back");

    let number = parse_choice();

    if number == 1 {
        status = create_roster_from_scratch();
    }
    else if number == 2 {
        status = create_roster_from_files();
    }
    else if number == 3 {
        status = 1 
    }

    status
}

fn create_roster_from_scratch() -> i32 {
    let mut status: i32 = 0;
    let mut stage: i32 = 0;
    let mut test_roster = roster::Roster::new();

    println!("---------------------------");
    println!("Create roster from scratch.");
    println!("---------------------------");

    loop {
        
        if stage == 0 {
            // stage = 0 -> select tribute number
            println!("How many tributes? (enter a valid integer)");

            // limit is 24, eventually set this value in a settings container
            let mut number = parse_positive_integer(24);
            if number == -1 {
                // error!
                println!("Invalid input, valid numerical range (0-{})", 24);
                continue;
            }
            else if number == 0 {
                // technically invalid but assume we want to randomize
                println!("Input: 0, assume RANDOM (0-{})", 24);
                let mut rng = rand::thread_rng();
                number = rng.gen_range(1, 25);
                println!("Added {} tributes", number);
            }
            else {
                println!("Added!");
            }
            // now add tributes
            let test_roster_ref = &mut test_roster;
            for _ in 0..number {
                let test_tribute = tribute::Tribute::new();
                test_roster_ref.add_tribute(Box::new(test_tribute));
            }

            // proceed to the next stage
            stage = 1
        }
        else if stage == 1 {
            // review creation and make changes if necessary
            println!("To review roster, enter `review`, for other commands, enter `help`.");
            let choice: String = read_input();
            // check choices, if invalid selection, continue
            if choice.trim() == String::from("review") {
                println!("Review");
                let test_roster_ref = &mut test_roster;
                println!("{}", test_roster_ref.to_string());
            }
            else {
                println!("Invalid input");
                continue;
            }
        }
        else {
            break
        }


    }

    status
}

fn create_roster_from_files() -> i32 {
    let mut status: i32 = 0;
    status
}

fn main() {
    let mut number = 0;
    loop {
        if number == 0 {
            welcome_text();
        }
        else if number == 1 {
            let status = interactive_roster_creation();
            if status == 1 {
                number = 0;
            }
        }
        else if number == 2 {
            break;
        }
        else {
            println!("invalid selection\n");
        }

        number = parse_choice();
    }
}
