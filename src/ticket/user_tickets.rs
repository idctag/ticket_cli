use crate::ticket::{
    ticket::{Ticket, TicketUpdate, string_to_status},
    utils::input::{take_user_num, take_user_string},
};

pub struct UserTickets {
    tickets: Vec<Ticket>,
    next_id: u32,
    selected: Option<usize>,
}

impl UserTickets {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            next_id: 1,
            selected: None,
        }
    }

    pub fn add_ticket(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let new_ticket = Ticket::new(id, title, description);
        self.tickets.push(new_ticket);
        id
    }

    pub fn tickets(&self) -> &[Ticket] {
        &self.tickets
    }

    pub fn find(&self, id: u32) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.id() == id)
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Ticket> {
        self.tickets.iter_mut().find(|ticket| ticket.id() == id)
    }

    pub fn edit_ticket(&mut self) {
        let id_num: u32 = take_user_num("Enter ticket id to edit");
        let ticket_to_update = self.find_mut(id_num);

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

    pub fn insert_ticket(&mut self) {
        let title = take_user_string("Enter Title");
        let description = take_user_string("Enter Description");
        self.add_ticket(title, description);
    }

    pub fn select_next(&mut self) {
        if let Some(n) = self.selected {
            // find the next ticket
            // if no next ticket do nothing
        }
        if !self.tickets().is_empty() && self.selected.is_none() {
            // select the first ticket
        }
    }
    pub fn select_previous(&mut self) {
        if let Some(n) = self.selected {
            // find the previous ticket
        }
        if !self.tickets().is_empty() && self.selected.is_none() {
            // select the last ticket
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::ticket::ticket::TicketStatus;

    use super::*;

    #[test]
    fn new_ticket_starts_as_pending() {
        let ticket = Ticket::new(1, "Learn Rust", "Study Ownership");
        assert_eq!(ticket.status(), TicketStatus::Pending)
    }

    #[test]
    fn adding_ticket_returns_id() {
        let mut tickets = UserTickets::new();
        let first_id = tickets.add_ticket("First".to_string(), "Description".to_string());
        let second_id = tickets.add_ticket("Second".to_string(), "Description".to_string());
        assert_eq!(first_id, 1);
        assert_eq!(second_id, 2)
    }

    #[test]
    fn can_find_ticket_by_id() {
        let mut tickets = UserTickets::new();
        let id = tickets.add_ticket("Ticket".to_string(), "Description".to_string());

        let ticket = tickets.find(id).expect("should exist");

        assert_eq!(ticket.title(), "Ticket".to_string())
    }
}
