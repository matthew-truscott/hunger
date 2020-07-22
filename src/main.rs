/// Main: Hunger Game Simulator v0.1
/// Author: Akharis Ren
/// 
/// This is the main file
/// 

extern crate rand;

mod tribute;
mod roster;
mod game;
mod img;

use std::io;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::env;
use serde_json::{Value};

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
    };
    input
}

fn read_file(filename: String) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
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
fn interactive_roster_creation(game_roster: &mut roster::Roster) -> i32 {
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
        status = create_roster_from_scratch(game_roster);
    }
    else if number == 2 {
        status = create_roster_from_files(game_roster);
    }
    else if number == 3 {
        status = 1 
    }

    status
}

fn create_roster_from_scratch(game_roster: &mut roster::Roster) -> i32 {
    let mut status: i32 = 0;
    let mut stage: i32 = 0;

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
            for _ in 0..number {
                let test_tribute = tribute::Tribute::new();
                game_roster.add_tribute(Box::new(test_tribute));
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
                println!("{}", game_roster.to_string());
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

fn find_data_directory() -> PathBuf {
    env::current_dir()
        .expect("Cannot access current directory")
        .as_path().join(Path::new("data"))
}

fn create_roster_from_files(game_roster: &mut roster::Roster) -> i32 {
    let mut status: i32 = 0;
    let datadir = find_data_directory();
    let mut stage: i32 = 0;

    loop {
        if stage == 0 {
            println!("Enter file to read (default: roster.json)");
            let choice = read_input();
            let mut slice_choice = choice.trim();
            if slice_choice.ends_with(".json") {
                slice_choice = &slice_choice[..slice_choice.len()-5];
            }
            
            let roster_path = Path::new(slice_choice);
            let mut abs_pathbuf = datadir.join(roster_path);
            abs_pathbuf.set_extension("json");

            if ! abs_pathbuf.exists() {
                println!("{} doesn't exist! defaulting...", abs_pathbuf.display());
                abs_pathbuf = datadir.join("roster.json");
            }

            if ! abs_pathbuf.exists() {
                println!("default file doesn't exist! PANIC");
            }

            println!("{} found!", abs_pathbuf.display());

            // now read as json
            let data = fs::read_to_string(abs_pathbuf)
                .expect("Something went wrong reading the file");

            let mut v: Value = Value::Null;
            match serde_json::from_str(data.as_str()) {
                Ok(result) => v = result,
                Err(e) => println!("Error {}", e)
            }

            // now read v for number of tributes
            let number_of_tributes = v["number_of_tributes"].as_i64().unwrap();

            println!("{}", number_of_tributes);

            // the tributes should now be accessed correctly 
            for i in 1..number_of_tributes+1 {
                let test_tribute = tribute::Tribute::from_data(
                    v[format!("{}", i)]["name"].as_str().unwrap(),
                    v[format!("{}", i)]["gender"].as_str().unwrap(),
                    v[format!("{}", i)]["avatar"].as_str().unwrap());
                game_roster.add_tribute(Box::new(test_tribute));
            }
            stage = 1;
        }
        else if stage == 1 {
            // review creation and make changes if necessary
            println!("To review roster, enter `review`, for other commands, enter `help`.");
            let choice: String = read_input();
            // check choices, if invalid selection, continue
            if choice.trim() == String::from("review") {
                println!("Review");
                println!("{}", game_roster.to_string());
            }
            else if choice.trim() == String::from("continue") {
                status = 0;
                break;
            }
            else {
                println!("Invalid input");
                continue;
            }
        }
    }

    status
}

fn run_interactive() {
    let mut number = 0;
    let mut game_roster: roster::Roster = roster::Roster::new();
    loop {
        if number == 0 {
            welcome_text();
        }
        else if number == 1 {
            let status = interactive_roster_creation(&mut game_roster);
            if status == 1 {
                number = 0;
            }
            else if status == 0 {
                number = 384;
            }
        }
        else if number == 2 {
            break;
        }
        else {
            println!("invalid selection\n");
        }
        if number == 384 {
            // roster complete, run simulation!
            // TODO maybe add a way to check the game settings
            let status = game::gameloop(&mut game_roster);
        }

        number = parse_choice();
    }
}

fn main() {
    // By default, run from file
    let datadir = find_data_directory();
    let mut game_abs_pathbuf = datadir.join("game.json");
    if ! game_abs_pathbuf.exists() {
        println!("game file doesn't exist! PANIC");
    }

    // now read as json
    let game_data = fs::read_to_string(game_abs_pathbuf)
        .expect("Something went wrong reading the file");

    let mut v: Value = Value::Null;
    match serde_json::from_str(game_data.as_str()) {
        Ok(result) => v = result,
        Err(e) => println!("Error {}", e)
    }

    // TODO move into its own class
    let lconsole: u64 = v["io"]["lconsole"].as_u64().unwrap();
    let lfile: u64 = v["io"]["lfile"].as_u64().unwrap();
    let limages: u64 = v["io"]["limages"].as_u64().unwrap();

    let mut game_roster: roster::Roster = roster::Roster::new();

    let mut roster_abs_pathbuf = datadir.join("roster.json");
    if ! roster_abs_pathbuf.exists() {
        println!("default file doesn't exist! PANIC");
    }

    // now read as json
    let roster_data = fs::read_to_string(roster_abs_pathbuf)
        .expect("Something went wrong reading the file");

    let mut v: Value = Value::Null;
    match serde_json::from_str(roster_data.as_str()) {
        Ok(result) => v = result,
        Err(e) => println!("Error {}", e)
    }

    // now read v for number of tributes
    let number_of_tributes = v["number_of_tributes"].as_i64().unwrap();

    println!("{}", number_of_tributes);

    // the tributes should now be accessed correctly 
    for i in 1..number_of_tributes+1 {
        let test_tribute = tribute::Tribute::from_data(
            v[format!("{}", i)]["name"].as_str().unwrap(),
            v[format!("{}", i)]["gender"].as_str().unwrap(),
            v[format!("{}", i)]["avatar"].as_str().unwrap());
        game_roster.add_tribute(Box::new(test_tribute));
    }

    // create thumbnails before to save time
    img::init_thumbs(&game_roster);

    let status = game::gameloop(&mut game_roster);  
}
