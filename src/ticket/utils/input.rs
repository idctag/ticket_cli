use std::{io, num::ParseIntError};

pub fn take_user_string(title: &str) -> String {
    let mut var = String::new();
    println!("{title}");
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    var
}
pub fn take_user_num(title: &str) -> u32 {
    let mut var = String::new();
    println!("{title}");
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    var.parse().unwrap()
}

pub fn read_choice() -> Result<u8, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse()
}
