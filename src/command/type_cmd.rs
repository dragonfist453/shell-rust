use std::fmt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use super::{parse, Command};

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

fn find_in_path(arg: &str) -> Option<String> {
    let path_var = std::env::var("PATH").ok()?;
    for dir in path_var.split(':') {
        let full_path = Path::new(dir).join(arg);
        if let Ok(metadata) = full_path.metadata() {
            if metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                return Some(full_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}

impl fmt::Display for TypeCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
