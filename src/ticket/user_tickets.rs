use crate::ticket::ticket::Ticket;

pub struct UserTickets {
    tickets: Vec<Ticket>,
    next_id: usize,
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

    pub fn add_ticket(&mut self, title: String, description: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let new_ticket = Ticket::new(id, title, description);
        self.tickets.push(new_ticket);
        id
    }

    pub fn tickets(&self) -> &[Ticket] {
        &self.tickets
    }

    pub fn find(&self, id: usize) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.id() == id)
    }

    pub fn find_mut(&mut self, id: usize) -> Option<&mut Ticket> {
        let ticket = self.tickets.iter_mut().find(|t| t.id() == id);

        ticket
    }

    pub fn select_next(&mut self) {
        if self.tickets.is_empty() {
            return self.selected = None;
        }

        if self.selected.is_none() {
            return self.selected = Some(0);
        }

        if let Some(n) = self.selected {
            if n == self.tickets.len() - 1 {
                return;
            } else {
                self.selected = Some(n + 1)
            }
        }
    }

    // pub fn select_previous(&mut self) {
    //     if let Some(n) = self.selected {
    //         // find the previous ticket
    //     }
    //     if !self.tickets().is_empty() && self.selected.is_none() {
    //         // select the last ticket
    //     }
    // }
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

    // #[test]
    // fn return_none_when_empty() {
    //     let mut tickets = UserTickets::new();
    // }
}
