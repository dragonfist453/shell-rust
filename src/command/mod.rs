use std::fmt;
use std::str::FromStr;

mod run;
use run::{echo, exit, handle_unknown_command, type_command};

pub enum Command {
    Echo(String),
    Type(String),
    Exit,
    Unknown(String),
}

impl Command {
    pub fn name(&self) -> &str {
        match self {
            Command::Echo(_) => "echo",
            Command::Type(_) => "type",
            Command::Exit => "exit",
            Command::Unknown(cmd) => cmd,
        }
    }

    pub fn run(&self) {
        match self {
            Command::Echo(text) => echo(text),
            Command::Type(text) => type_command(&text.parse().unwrap()),
            Command::Exit => exit(),
            Command::Unknown(cmd) => handle_unknown_command(cmd),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name();
        write!(f, "{}", name)
    }
}

impl FromStr for Command {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(Command::Unknown(String::new()));
        }
        let args = parts[1..].join(" ");
        Ok(match parts[0] {
            "echo" => Command::Echo(args),
            "type" => Command::Type(args),
            "exit" => Command::Exit,
            other => Command::Unknown(other.to_string()),
        })
    }
}