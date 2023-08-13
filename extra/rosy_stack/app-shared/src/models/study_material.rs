pub struct input_text { // Represents the input study material form
    pub text: String,
    pub breadth: Breadth,
    pub parsing: Parsing,
    pub sender: String,
    pub login: bool,
}

pub enum Breadth {
    All,
    Upper,
    Lower,
    Bottom,
}

pub enum Parsing {
    Words,
    Pairs,
    Trees,
}

impl input_text {
    pub fn test_example() {}

    pub fn length(&self) -> u64 {
        self.text.length()
    }

}