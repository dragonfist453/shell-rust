use super::Command;
use std::{fmt, os::unix::process::CommandExt, process};

pub struct ExternalCmd {
    pub cmd_name: String,
    pub path: String,
    pub args: Vec<String>,
}

impl Command for ExternalCmd {
    fn name(&self) -> &str {
        &self.path
    }

    fn run(&self) {
        let result = process::Command::new(&self.path)
            .arg0(&self.cmd_name)
            .args(self.args.iter())
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
