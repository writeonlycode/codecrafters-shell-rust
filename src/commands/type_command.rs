use std::fs::read_dir;

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
    let paths = std::env::var("PATH");

    if let Ok(paths) = paths {
        for path in std::env::split_paths(&paths) {
            if path.is_dir() {
                if let Ok(mut dir) = read_dir(path) {
                    let result = dir.find(|entry| {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            let file_name = file_name.to_str().unwrap();
                            file_name == name
                        } else {
                            false
                        }
                    });

                    if let Some(Ok(result)) = result {
                        println!("{name} is {}", result.path().to_str().unwrap());
                        return;
                    }
                }
            }
        }
    }
}
