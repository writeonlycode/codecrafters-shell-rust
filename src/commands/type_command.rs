pub fn type_command(arguments: Vec<&str>) {
    match arguments.into_iter().next() {
        Some("type") => println!("type is a shell builtin"),
        Some("echo") => println!("echo is a shell builtin"),
        Some("exit") => println!("exit is a shell builtin"),
        Some(name) => type_executable(name),
        None => println!("take must have 1 argument"),
    }
}

fn type_executable(name: &str) {
    if let Ok(path) = which::which(name) {
        println!("{name} is {}", path.to_str().unwrap());
    } else {
        println!("{name}: not found")
    }
}
