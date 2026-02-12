use crate::command::Command;
use std::env::set_current_dir;
use std::fmt;
use std::path::PathBuf;

pub struct Cd {
    pub path: String,
}

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self) {
        let path = if self.path == "~" {
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home),
                Err(_) => return,
            }
        } else {
            PathBuf::from(&self.path)
        };
        if let Err(..) = set_current_dir(path) {
            eprintln!("cd: {}: No such file or directory", self.path);
        }
    }
}

impl fmt::Display for Cd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}