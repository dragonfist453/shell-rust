use std::fmt;

use super::{parse, Command, find_in_path};

pub struct TypeCmd {
    pub args: String,
}

impl Command for TypeCmd {
    fn name(&self) -> &str {
        "type"
    }

    fn run(&self) {
        let command = parse(&self.args);
        if command.is_builtin() {
            println!("{} is a shell builtin", command);
        } else if let Some(path) = find_in_path(&self.args) {
            println!("{} is {}", self.args, path);
        } else {
            println!("{}: not found", command);
        }
    }
}

impl fmt::Display for TypeCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
