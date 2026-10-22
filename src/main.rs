#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let mut commands = command.split_whitespace();

        match commands.next() {
            Some("type") => {
                commands::type_command::type_command(commands.collect());
            }
            Some("echo") => {
                commands::echo::echo(commands.collect());
            }
            Some("exit") => {
                commands::exit::exit();
            }
            Some(command) => {
                println!("{}: command not found", command);
            }
            None => {}
        }
    }
}
