use std::fmt;

use super::Command;

pub struct Echo {
    pub text: String,
}

impl Command for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self) {
        println!("{}", self.text);
    }
}

impl fmt::Display for Echo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
