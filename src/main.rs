use crate::jav::real_string;

mod jav;

fn res_cd_ln(){
    jav::print("Shbozz@Term:~$ ");
    let what_to_do = jav::read_line();
    do_stuff(what_to_do);
}

fn main() {
    println!("Welcome to the Shbozz Rust Shell!");
    loop {
        jav::print("Username: ");
        let username = jav::read_line();
        jav::print("Password: ");
        let password = jav::read_line();
        println!("Authenticating...");
        if authenticate(&username, &password) {
            break;
        }
    }
    res_cd_ln();
}

fn do_stuff(string: String) {
    if string == "\n".to_string() {
        res_cd_ln();
    }else if string == jav::real_string("exit") {
        println!("Closing... Bye");
    }else if string == jav::real_string("update") {
        update();
        res_cd_ln();
    }else if string == jav::real_string("clear") {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        res_cd_ln();
    }else if string == jav::real_string("help") {
        println!("Available commands: exit, clear, update, help");
        res_cd_ln();
    }else if string == jav::real_string("rick") {
        println!("https://youtu.be/dQw4w9WgXcQ");
        res_cd_ln();
    }else {
        println!("Command not found.");
        res_cd_ln();
    }
}
fn update(){
    let mut load = String::from("[********************************************************]");
    use std::{thread};
    println!("Updating...");
    let mut tim = 60;


    while tim > 0{
        use std::io::{self, Write};
        load = load.replacen('*', "X", 1);
        print!("{}", load);
        io::stdout().flush().unwrap();
        thread::sleep_ms(500);
        jav::print("\r");
        tim -= 1;
    }
}

fn authenticate(username: &str, password: &str) -> bool {
    return if username == real_string("Shbozz") && password == real_string("none") {
        println!("Authentication successful!");
        true
    } else {
        println!("Authentication failed! Please try again.");
        false
    }
}


