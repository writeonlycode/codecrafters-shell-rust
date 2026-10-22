pub fn echo(argument: Vec<&str>) {
    let s = argument.join(" ");
    println!("{s}");
}
