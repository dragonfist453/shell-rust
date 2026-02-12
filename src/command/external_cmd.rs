use std::{fmt, process};

use super::Command;

pub struct ExternalCmd {
    pub path: String,
    pub args: String,
}

impl Command for ExternalCmd {
    fn name(&self) -> &str {
        &self.path
    }

    fn run(&self) {
        let result = process::Command::new(&self.path)
            .args(self.args.split_whitespace())
            .status();

        if let Err(_) = result {
            println!("{}: command failed", self.path);
        }
    }

    fn is_builtin(&self) -> bool {
        false
    }
}

impl fmt::Display for ExternalCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
