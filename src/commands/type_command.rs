pub fn type_command(arguments: Vec<&str>) {
    match arguments.into_iter().next() {
        Some("type") => println!("type is a shell builtin"),
        Some("echo") => println!("echo is a shell builtin"),
        Some("exit") => println!("exit is a shell builtin"),
        Some(name) => println!("{name}: not found"),
        None => println!("take must have 1 argument"),
    }
}
