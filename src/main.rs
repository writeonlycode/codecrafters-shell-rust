#[allow(unused_imports)]
use std::{
    io::{self, Write},
    process,
};

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
                external_program(command, commands.collect());
            }
            None => {}
        }
    }
}

fn external_program(name: &str, arguments: Vec<&str>) {
    if let Ok(path) = which::which(name) {
        let file_name = path.file_name().unwrap();
        let _ = process::Command::new(file_name).args(arguments).status();
    } else {
        println!("{name}: not found")
    }
}
