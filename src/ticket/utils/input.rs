use std::{io, num::ParseIntError};

use crate::ticket::{
    ticket::{TicketUpdate, string_to_status},
    user_tickets::UserTickets,
};

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
    var.trim().parse().unwrap()
}

pub fn read_choice() -> Result<u8, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse()
}

pub fn prompt_apply_update(tickets: &mut UserTickets) {
    let id_num: u32 = take_user_num("Enter ticket id to edit");
    let ticket_to_update = tickets.find_mut(id_num);

    if let Some(ticket) = ticket_to_update {
        let title = take_user_string("enter ticket title to edit");
        let description = take_user_string("enter ticket description to edit");
        let status_str = take_user_string("enter ticket status to edit");
        let s_status = string_to_status(&status_str);

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
