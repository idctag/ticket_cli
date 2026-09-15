use std::{io, num::ParseIntError};

use todo_cli::core::UserTickets;

fn main() {
    let mut tickets = UserTickets::new();
    tickets.add_ticket("One".to_string(), "One Description".to_string());
    tickets.add_ticket("Two".to_string(), "Two Description".to_string());
    tickets.add_ticket("Three".to_string(), "Three Description".to_string());
    loop {
        print_menu();
        match read_choice() {
            Ok(1) => list_tickets(&tickets),
            Ok(2) => add_ticket(&mut tickets),
            Ok(3) => edit_ticket(&mut tickets),
            Ok(4) => break,
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

fn edit_ticket(u_tickets: &mut UserTickets) {
    let mut id = String::new();
    println!("enter ticket id to edit");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id_num = id.parse().expect("invalid id");
    let mut t_to_update = u_tickets.find_mut(id_num);
    if let Some(t) = t_to_update {
        //take user input

        t.apply_update(update);
    }
}

fn add_ticket(u_tickets: &mut UserTickets) {
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
    u_tickets.add_ticket(title, description);
}

fn read_choice() -> Result<u8, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse()
}
