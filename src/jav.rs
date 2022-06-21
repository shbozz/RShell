pub fn print(string: &str) {
    use std::io::{self, Write};
    print!("{}", string);
    io::stdout().flush().unwrap();
}

pub fn read_line() -> String {
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

pub fn real_string(string: &str) -> String {
    return format!("{}{}", string, "\n");
}