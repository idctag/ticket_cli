use std::{io, num::ParseIntError};

use todo_cli::{
    ticket::{
        ticket::{TicketUpdate, string_to_status},
        user_tickets::UserTickets,
    },
    utils::display::{list_tickets, print_menu},
};

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

fn edit_ticket(u_tickets: &mut UserTickets) {
    let mut id = String::new();
    println!("enter ticket id to edit");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id_num: u32 = id.trim().parse().unwrap();
    let ticket_to_update = u_tickets.find_mut(id_num);

    if let Some(ticket) = ticket_to_update {
        let mut title = String::new();
        println!("enter ticket title to edit");
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");
        let mut description = String::new();
        println!("enter ticket description to edit");
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");
        let mut status_str = String::new();
        println!("enter ticket status to edit");
        io::stdin()
            .read_line(&mut status_str)
            .expect("Failed to read line");
        let s_status = string_to_status(&status_str);
        let mut update = TicketUpdate::new();
        update.title = Some(title);
        update.description = Some(description);
        update.status = Some(s_status);
        ticket.apply_update(update);
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
