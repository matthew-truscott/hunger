use super::tribute;

pub struct Roster {
    tribute_vec: Vec<Box<tribute::Tribute>>,
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
    pub fn to_string(&mut self) -> String {
        let mut output: String = String::from("");
        for (i, item) in self.tribute_vec.iter().enumerate() {
            let tb: tribute::Tribute = *item.clone();
            let tb_contents: String = tb.to_string();
            output = format!("{}({}): {}\n", output, i, tb_contents);
        }
        output
    }
}

 