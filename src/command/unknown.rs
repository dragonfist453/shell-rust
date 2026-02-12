use std::fmt;

use super::Command;

pub struct Unknown {
    pub name: String,
}

impl Command for Unknown {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) {
        println!("{}: not found", self.name);
    }

    fn is_builtin(&self) -> bool {
        false
    }
}

impl fmt::Display for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
