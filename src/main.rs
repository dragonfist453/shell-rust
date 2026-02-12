mod command;

use command::Command;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command: Command = input.parse().unwrap();
        command.run();
    }
}
