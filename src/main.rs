use std::{io, num::ParseIntError};

use todo_cli::core::UserTickets;

fn main() {
    let mut tickets = UserTickets::new();
    loop {
        print_menu();
        match read_choice() {
            Ok(1) => list_tickets(&tickets),
            Ok(2) => add_ticket(&mut tickets),
            Ok(3) => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn print_menu() {
    println!("Choose from options");
    println!("[1] list");
    println!("[2] add");
    println!("[3] quit");
}

fn list_tickets(tickets: &UserTickets) {
    for t in tickets.tickets() {
        println!("{t}")
    }
}

fn add_ticket(tickets: &mut UserTickets) {
    let mut title = String::new();
    let mut description = String::new();
    println!("enter Title");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    println!("Description");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    tickets.add_ticket(title, description);
}

fn read_choice() -> Result<u8, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse()
}
