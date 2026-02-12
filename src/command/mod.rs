use std::fmt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

mod echo;
mod exit;
mod external_cmd;
mod type_cmd;
mod unknown;

pub trait Command: fmt::Display {
    fn name(&self) -> &str;
    fn run(&self);
    fn is_builtin(&self) -> bool {
        true
    }
}

pub fn find_in_path(arg: &str) -> Option<String> {
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

pub fn parse(input: &str) -> Box<dyn Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Box::new(unknown::Unknown {
            name: String::new(),
        });
    }
    let args = parts[1..].join(" ");
    match parts[0] {
        "echo" => Box::new(echo::Echo { text: args }),
        "type" => Box::new(type_cmd::TypeCmd { args }),
        "exit" => Box::new(exit::Exit),
        other => {
            if let Some(path) = find_in_path(other) {
                Box::new(external_cmd::ExternalCmd { path, args })
            } else {
                Box::new(unknown::Unknown {
                    name: other.to_string(),
                })
            }
        }
    }
}
