use crate::ticket::Ticket;

pub struct UserTickets {
    tickets: Vec<Ticket>,
    next_id: u32,
}

impl UserTickets {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            next_id: 1,
        }
    }

    pub fn list(&self) {
        for ticket in &self.tickets {
            println!("{ticket}")
        }
    }

    pub fn add_ticket(&mut self, title: String, description: String) {
        self.next_id += 1;
        let new_ticket = Ticket::new(self.next_id, title, description);
        self.tickets.push(new_ticket);
    }
}
