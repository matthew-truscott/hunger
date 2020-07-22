use std::io;
use std::fs;
use rand::Rng;
use rand::seq::SliceRandom;
use serde_json::{Value, Map, json};
use tinytemplate::TinyTemplate;
use std::process; // temporary

use super::roster;
use super::img;

#[derive(PartialEq)]
enum RoundType {
    BLOODBATH,
    FEAST,
    ARENA,
    DAY,
    NIGHT,
    FALLEN,
    NONE
}

impl RoundType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            RoundType::BLOODBATH => "bloodbath",
            RoundType::FEAST => "feast",
            RoundType::ARENA => "arena",
            RoundType::DAY => "day",
            RoundType::NIGHT => "night",
            RoundType::FALLEN => "fallen",
            RoundType::NONE => "none",
        }
    }
}

pub fn gameloop(game_roster: &mut roster::Roster) -> i32 {
    let status: i32 = 0;

    let mut day: i32 = 1;
    let mut n_alive: i32;
    let mut days_since_last_event: i32 = 0;
    let mut consecutive_rounds_without_deaths: i32 = 0;

    let mut bloodbath_passed: bool = false;
    let mut day_passed: bool = false;
    let mut fallen_passed: bool = false;
    let mut night_passed: bool = false;

    let mut rng = rand::thread_rng();

    game_roster.default_gender_setup();

    // read in the events...
    let data = fs::read_to_string("data/events.json")
        .expect("Something went wrong reading the file");

    let mut events: Value = Value::Null;
    match serde_json::from_str(data.as_str()) {
        Ok(result) => events = result,
        Err(e) => println!("Error {}", e)
    }
    // let mut eventlog

    let mut imgidx: u32 = 0;
    loop {
        let mut tt = TinyTemplate::new();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => println!("error: {}", error),
        };
        // do nothing with the input for now
        // next step

        n_alive = game_roster.n_alive();
        if n_alive < 2 {
            break
        }

        if night_passed {
            day += 1;
            days_since_last_event += 1;
            day_passed = false;
            fallen_passed = false;
            night_passed = false;
        }

        let mut feast_chance: f64 = 100.0 * (days_since_last_event as f64).powf(2.0);
        feast_chance /= 55.0;
        feast_chance += 9.0 / 55.0;

        let mut fatality_factor: i32 = rng.gen_range(2, 4) + consecutive_rounds_without_deaths;

        let step_type: RoundType;
        if (day == 1) && !(bloodbath_passed) {
            step_type = RoundType::BLOODBATH;
            fatality_factor += 2;
            bloodbath_passed = true;
        }
        else if !(day_passed) && rng.gen_range(0.0, 100.0) < feast_chance {
            step_type = RoundType::FEAST;
            days_since_last_event = 0;
            fatality_factor += 2;
        }
        else if days_since_last_event > 0 && rng.gen_range(1, 20) == 1 {
            step_type = RoundType::ARENA;
            days_since_last_event = 0;
            fatality_factor += 1;
        }
        else if !(day_passed) {
            step_type = RoundType::DAY;
            day_passed = true;
        }
        else if day_passed && !(fallen_passed) {
            step_type = RoundType::FALLEN;
            fallen_passed = true;
        }
        else {
            step_type = RoundType::NIGHT;
            night_passed = true;
        }

        let event_key = step_type.as_str();

        if step_type == RoundType::FALLEN {
            println!(
                "{} cannon shots can be heard from the distance.",
                game_roster.count_dead_on_day(day));
            if game_roster.count_dead_on_day(day) == 0 {
                consecutive_rounds_without_deaths += 1;
            }
            else {
                consecutive_rounds_without_deaths = 0;
                println!("{}", game_roster.death_summary_on_day(day));
            }
            continue;
        }

        let event: &Map<String, Value>;
        // for now just get the event title
        if step_type == RoundType::ARENA {
            event = events[event_key]
                .as_array().unwrap()
                .choose(&mut rand::thread_rng()).unwrap()
                .as_object().unwrap();
        }
        else {
            let event_result = events[event_key].as_object();
            match event_result {
                Some(e) => event = e,
                None => {
                    println!("no match for events[{}]", event_key);
                    // pick something that works
                    event = events["bloodbath"].as_object().unwrap();
                }
            }
        }

        let mut template = tt.add_template("title_tmp", event["title"].as_str().unwrap());
        match template {
            Ok(_) => (),
            Err(e) => println!("template error.\n {}", e)
        }

        let title_map = json!({
            "0".to_string(): day
        });
        
        let rendered = tt.render("title_tmp", &title_map);

        match rendered {
            Ok(r) => println!("{}", r),
            Err(e) => println!("rendering error.\n {}", e)
        }
        
        game_roster.activate();
        let mut action_members: Vec<usize> = Vec::with_capacity(game_roster.n_available() as usize);

        while game_roster.n_available() > 0 {
            let f: i32 = rng.gen_range(0, 10);
            let action: &Value;
            action_members.clear();


            if f < fatality_factor && n_alive > 1 {
                // time to die
                action = event["fatal"].as_array().unwrap()
                    .choose(&mut rand::thread_rng()).unwrap();
                if action["killed"].as_array().unwrap().len() >= n_alive as usize {
                    // not enough alive to satisfy event
                    continue;
                }
            }
            else {
                action = event["nonfatal"].as_array().unwrap()
                    .choose(&mut rand::thread_rng()).unwrap();
            }

            let mut action_tributes: i64 = action["tributes"].as_i64().unwrap();
            let action_number = action_tributes as usize;
            if action_tributes > game_roster.n_available().into() {
                // not enough available to satisfy event
                continue;
            }

            // UNSAFE, need force exit from loop
            while action_tributes > 0 {
                let idx: usize = rng.gen_range(0, game_roster.len());
                if game_roster.get_available(idx) {
                    action_members.push(idx);
                    game_roster.set_unavailable(idx);
                    action_tributes -= 1;
                }
            }

            //println!("ACTION MEMBERS");
            //for v in action_members.iter() {
            //    println!("{}", v);
            //}

            match action["killed"].as_array() {
                Some(killed_array) => {
                    match action["killer"].as_array() {
                        Some(killer_array) => {
                            for kr in killer_array {
                                let kr_res = kr.as_u64();
                                match kr_res {
                                    Some(_) => game_roster.add_kill(action_members[kr.as_u64().unwrap() as usize]),
                                    None => () // no killer
                                }
                            }
                        }
                        None => ()
                    }
                    for kd in killed_array {
                        game_roster.kill(action_members[kd.as_u64().unwrap() as usize], day)
                    }
                }
                None => ()
            }

            template = tt.add_template("msg_tmp", action["msg"].as_str().unwrap());
            match template {
                Ok(_) => (),
                Err(e) => println!("template error.\n {}", e)
            }

            // println!("action number: {}", action_number);
            // println!("message: {}", action["msg"].as_str().unwrap());

            let context_map: Map<String, Value> = (0..action_number)
                .map(|i| (i.to_string(), game_roster.serialize_tribute(action_members[i]).into()))
                .collect::<Map<String, Value>>();

            // println!("CONTEXT MAP");
            // for (key, val) in context_map.iter() {
            //    println!("{}: {}", key, val)
            // }

            let rendered = tt.render("msg_tmp", &context_map);

            match rendered {
                Ok(r) => {
                    println!("{}", r);
                    img::image(r, game_roster, &action_members, &imgidx);
                }
                Err(e) => println!("rendering error.\n {}", e)
            }

            imgidx += 1;       
            
            // process::exit(0x0000)

        }

    }

    // Simulation complete, print details
    println!("{}", game_roster.game_summary());

    status
}