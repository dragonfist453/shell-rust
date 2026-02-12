#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parts = command.trim().split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "exit" => break,
            "echo" => {
                println!("{}", parts[1..].join(" "));
            }
            other => println!("{}: command not found", other),
        }
    }
}
