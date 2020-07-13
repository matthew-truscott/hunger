use super::tribute;

use serde::ser::{Serialize, Serializer, SerializeMap};
use serde_json::Value;
use serde_json::json;

pub struct Roster {
    tribute_vec: Vec<Box<tribute::Tribute>>,
}

impl Serialize for Roster {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.tribute_vec.len()))?;
        for v in self.tribute_vec.iter() {
            let k = v.id;
            map.serialize_entry(&k, &v)?;
        }
        map.end()
        //let mut state = serializer.serialize_struct("Roster", 1)?;
        //state.serialize_field("tribute_vec", &self.tribute_vec)?;
        //state.end()
    }
}

impl Roster {
    // static constructor
    pub fn new() -> Roster {
        // initialize empty array
        let tempvec = Vec::<Box<tribute::Tribute>>::new();
        Roster{tribute_vec:tempvec}
    }
    pub fn add_tribute(&mut self, tb: Box<tribute::Tribute>) {
        self.tribute_vec.push(tb);
    }
    pub fn to_string(&self) -> String {
        let mut output: String = String::from("");
        for (i, item) in self.tribute_vec.iter().enumerate() {
            let tb: tribute::Tribute = *item.clone();
            let tb_contents: String = tb.to_string();
            output = format!("{}({}): {}\n", output, i, tb_contents);
        }
        output
    }
    pub fn len(&self) -> usize {
        self.tribute_vec.len()
    }
    pub fn n_alive(&self) -> i32 {
        let mut n: i32 = 0;
        for item in self.tribute_vec.iter() {
            if item.alive {
                n += 1;
            }
        }
        n
    }
    pub fn n_available(&self) -> i32 {
        let mut n: i32 = 0;
        for item in self.tribute_vec.iter() {
            if item.available {
                n += 1;
            }
        }
        n
    }
    pub fn get_alive(&self, i: usize) -> bool {
        self.tribute_vec[i].alive
    }
    pub fn get_available(&self, i: usize) -> bool {
        self.tribute_vec[i].available
    }
    pub fn get_name(&self, i: usize) -> String {
        let s: String = self.tribute_vec[i].name.clone();
        s
    }
    pub fn get_avatar(&self, i: usize) -> Option<String> {
        let o: Option<String> = self.tribute_vec[i].avatar.clone();
        o
    }
    pub fn kill(&mut self, i: usize, day: i32) {
        self.tribute_vec[i].alive = false;
        self.tribute_vec[i].available = false;
        self.tribute_vec[i].deathday = day;
    }
    pub fn add_kill(&mut self, i: usize) {
        self.tribute_vec[i].killcount += 1;
    }
    pub fn set_unavailable(&mut self, i: usize) {
        self.tribute_vec[i].available = false;
    }
    pub fn activate(&mut self) {
        for item in self.tribute_vec.iter_mut() {
            if item.alive {
                item.available = true
            }
            else {
                item.available = false
            }
        }
    }
    pub fn count_dead_on_day(&self, day: i32) -> i32 {
        let mut n: i32 = 0;
        for item in self.tribute_vec.iter() {
            if item.deathday == day {
                n += 1;
            }
        }
        n
    }
    pub fn death_summary_on_day(&self, day: i32) -> String {
        let mut output: String = String::from("The following tributes have died today: \n");
        for item in self.tribute_vec.iter() {
            if item.deathday == day {
                output = format!("{}{}\n", output, item.name);
            }
        }
        output
    }
    pub fn default_gender_setup(&mut self) {
        for item in self.tribute_vec.iter_mut() {
            match item.gender {
                tribute::Gender::M => {
                    item.gender_label_nominative = String::from("he");
                    item.gender_label_accusative = String::from("him");
                    item.gender_label_genitive = String::from("his");
                    item.gender_label_reflexitive = String::from("himself");
                }
                tribute::Gender::F => {
                    item.gender_label_nominative = String::from("she");
                    item.gender_label_accusative = String::from("her");
                    item.gender_label_genitive = String::from("her");
                    item.gender_label_reflexitive = String::from("herself");
                }
                tribute::Gender::A => {
                    item.gender_label_nominative = String::from("they");
                    item.gender_label_accusative = String::from("them");
                    item.gender_label_genitive = String::from("their");
                    item.gender_label_reflexitive = String::from("themselves");
                }
            }
        }
    }
    pub fn serialize_tribute(&self, i: usize) -> Value {
        json!(self.tribute_vec[i])
    }
    pub fn game_summary(&self) -> String {
        let mut output: String = String::from("Simulation Complete: \n");
        output = format!("{}Name                 Kills   Died    \n", output);
        output = format!("{}-------------------------------------\n", output);
        for item in self.tribute_vec.iter() {
            let mut died = item.deathday.to_string();
            if died == "0".to_string() {
                died = "Survivor".to_string();
            }
            output = format!("{}{:20} {:7} {:7}\n", output,
                item.name, item.killcount.to_string(), died);
        }
        output
    }
}

 