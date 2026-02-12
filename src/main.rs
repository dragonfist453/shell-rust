use std::fmt;
#[allow(unused_imports)]
use std::io::{self, Write};
enum Command {
    Echo(String),
    Type(String),
    Exit,
    Unknown(String),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Echo(_) => write!(f, "echo"),
            Command::Type(_) => write!(f, "type"),
            Command::Exit => write!(f, "exit"),
            Command::Unknown(cmd) => write!(f, "{}", cmd),
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = parse_command(&input);
        execute_command(&command);
    }
}

fn parse_command(input: &str) -> Command {
    let parts = input.trim().split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        return Command::Unknown(String::new());
    }
    match parts[0] {
        "echo" => Command::Echo(parts[1..].join(" ")),
        "type" => Command::Type(parts[1..].join(" ")),
        "exit" => Command::Exit,
        other => Command::Unknown(other.to_string()),
    }
}

fn execute_command(command: &Command) {
    match command {
        Command::Echo(text) => println!("{}", text),
        Command::Type(text) => match parse_command(text) {
            Command::Unknown(cmd) => println!("{}: not found", cmd),
            other => println!("{} is a shell builtin", other),
        },
        Command::Exit => std::process::exit(0),
        Command::Unknown(cmd) => println!("Unknown command: {}", cmd),
    }
}
