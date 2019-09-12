use std::sync::atomic::{AtomicUsize, Ordering};

static TRIBUTE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
pub struct Tribute {
    pub name: String,
    pub id: usize,
    pub alive: bool,
}

impl Tribute {
     pub fn new() -> Tribute {
         let tribute_id = TRIBUTE_COUNTER.fetch_add(1, Ordering::SeqCst);
         Tribute{name:String::from("null"), id:tribute_id, alive:false}
     }
     pub fn to_string(self) -> String {
         format!("Name: {}, ID: {}", self.name, self.id)
     }
}