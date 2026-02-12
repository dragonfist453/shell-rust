use crate::command::Command;
use std::env::current_dir;
use std::fmt;

pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn run(&self) {
        println!("{}", current_dir().unwrap().display());
    }
}

impl fmt::Display for Pwd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}