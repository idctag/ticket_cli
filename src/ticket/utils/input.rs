use std::{
    io::{self, Write},
    num::ParseIntError,
};

use crate::ticket::{
    ticket::{TicketStatus, TicketUpdate},
    user_tickets::UserTickets,
};

pub fn string_to_status(s_status: &str) -> TicketStatus {
    match s_status {
        "done" => TicketStatus::Done,
        "pending" => TicketStatus::Pending,
        "archived" => TicketStatus::Archived,
        _ => TicketStatus::Undecided,
    }
}

pub fn take_user_string(title: &str) -> io::Result<String> {
    println!("{title}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn take_user_num(title: &str) -> Result<u32, ParseIntError> {
    let mut var = String::new();
    println!("{title}");
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    var.trim().parse()
}

pub fn read_choice() -> Result<u8, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse()
}

pub fn prompt_apply_update(tickets: &mut UserTickets) -> io::Result<()> {
    let id_num = match take_user_num("Enter ticket id to edit") {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter valid ticket id");
            Error
        }
    };

    let ticket_to_update = tickets.find_mut(id_num);

    if let Some(ticket) = ticket_to_update {
        let title = take_user_string("enter ticket title to edit")?;
        let description = take_user_string("enter ticket description to edit")?;
        let status_str = take_user_string("enter ticket status to edit")?;
        let s_status = string_to_status(status_str);

        let mut update = TicketUpdate::new();

        update.title = Some(title);
        update.description = Some(description);
        update.status = Some(s_status);
        ticket.apply_update(update);
    }
}

pub fn prompt_add_ticket(tickets: &mut UserTickets) {
    let title = take_user_string("Enter Title");
    let description = take_user_string("Enter Description");
    tickets.add_ticket(title, description);
}
