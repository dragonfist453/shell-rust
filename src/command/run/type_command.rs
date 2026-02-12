use super::handle_unknown_command;
use crate::command::Command;

pub fn type_command(command: &Command) {
    match command {
        Command::Unknown(cmd) => handle_unknown_command(cmd),
        other => println!("{} is a shell builtin", other),
    }
}
