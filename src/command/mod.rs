use std::fmt;

mod echo;
mod exit;
mod type_cmd;
mod unknown;

pub trait Command: fmt::Display {
    fn name(&self) -> &str;
    fn run(&self);
    fn is_builtin(&self) -> bool {
        true
    }
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
        other => Box::new(unknown::Unknown {
            name: other.to_string(),
        }),
    }
}
