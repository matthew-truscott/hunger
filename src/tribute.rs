use std::sync::atomic::{AtomicUsize, Ordering};
use serde::ser::{Serialize, Serializer, SerializeStruct};

static TRIBUTE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq)]
pub enum Gender {
    M,
    F,
    A,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Gender::M => "M",
            Gender::F => "F",
            Gender::A => "A",
        }
    }
    pub fn from_str(s: &str) -> Result<Gender, ()> {
        match s {
            "M" => Ok(Gender::M),
            "F" => Ok(Gender::F),
            "A" => Ok(Gender::A),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tribute {
    pub name: String,
    pub id: usize,
    pub alive: bool,
    pub available: bool,
    pub deathday: i32,
    pub killcount: i32,
    pub gender: Gender,
    pub gender_label_nominative: String,
    pub gender_label_accusative: String,
    pub gender_label_genitive: String,
    pub gender_label_reflexitive: String,
    pub avatar: Option<String>
}

impl Serialize for Tribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Roster", 1)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("gender", &self.gender.as_str())?;
        state.serialize_field("genN", &self.gender_label_nominative.as_str())?;
        state.serialize_field("genA", &self.gender_label_accusative.as_str())?;
        state.serialize_field("genG", &self.gender_label_genitive.as_str())?;
        state.serialize_field("genS", &self.gender_label_reflexitive.as_str())?;
        state.skip_field("id")?;
        state.skip_field("alive")?;
        state.skip_field("available")?;
        state.skip_field("deathday")?;
        state.skip_field("killcount")?;
        state.end()
    }
}

impl Tribute {
    pub fn new() -> Tribute {
        let tribute_id = TRIBUTE_COUNTER.fetch_add(1, Ordering::SeqCst);
        Tribute{name:String::from("null"), id:tribute_id, alive:false, available:false, deathday:0, killcount:0, gender:Gender::A,
            gender_label_nominative:String::from("null"), gender_label_accusative:String::from("null"), 
            gender_label_genitive:String::from("null"), gender_label_reflexitive:String::from("null"), avatar:None}
    }
    pub fn from_data(name: &str, gen: &str, avatar: &str) -> Tribute {
        let tribute_id = TRIBUTE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let gen_result = match Gender::from_str(gen) {
            Ok(v) => v,
            Err(_) => Gender::A
        };
        Tribute{name:String::from(name), id:tribute_id, alive:true, available:true, deathday:0, killcount:0, gender:gen_result,
            gender_label_nominative:String::from("null"), gender_label_accusative:String::from("null"), 
            gender_label_genitive:String::from("null"), gender_label_reflexitive:String::from("null"), avatar:Some(avatar.to_string())}
    }
    pub fn to_string(self) -> String {
        format!("Name: {}, ID: {}", self.name, self.id)
    }
}