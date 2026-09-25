use std::io::{self, Write};

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

pub fn take_user_string(title: &str) -> io::Result<Option<String>> {
    println!("{title}");
    io::stdout().flush()?;

    loop {
        let mut input = String::new();
        let byte_count = io::stdin().read_line(&mut input)?;

        if byte_count == 0 {
            return Ok(None);
        }
        let text = input.trim();
        if text.is_empty() {
            continue;
        }
        return Ok(Some(text.to_string()));
    }
}

pub fn prompt_apply_update(tickets: &mut UserTickets) -> io::Result<()> {
    let id = match take_user_string("Enter ticket id to edit")? {
        Some(t) => t,
        None => return Ok(()),
    };
    let id_num = match id.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Enter valid number");
            return Ok(());
        }
    };

    let ticket_to_update = tickets.find_mut(id_num);

    if ticket_to_update.is_none() {
        println!("Ticket not found")
    }

    if let Some(ticket) = ticket_to_update {
        let title = match take_user_string("enter ticket title to edit")? {
            Some(t) => t,
            None => return Ok(()),
        };
        let description = match take_user_string("enter ticket description to edit")? {
            Some(t) => t,
            None => return Ok(()),
        };
        let status_str = match take_user_string("enter ticket status to edit")? {
            Some(t) => t,
            None => return Ok(()),
        };
        let s_status = string_to_status(&status_str);

        let mut update = TicketUpdate::new();

        update.title = Some(title);
        update.description = Some(description);
        update.status = Some(s_status);
        ticket.apply_update(update);
    }

    Ok(())
}

pub fn prompt_add_ticket(tickets: &mut UserTickets) -> io::Result<()> {
    let title = match take_user_string("Enter Title")? {
        Some(t) => t,
        None => return Ok(()),
    };
    let description = match take_user_string("Enter Description")? {
        Some(t) => t,
        None => return Ok(()),
    };
    tickets.add_ticket(title, description);
    Ok(())
}
