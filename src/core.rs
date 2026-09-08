use crate::ticket::Ticket;

pub struct UserTickets {
    tickets: Vec<Ticket>,
    counter: u32,
}

impl UserTickets {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            counter: 0,
        }
    }

    pub fn list(&self) {
        for ticket in &self.tickets {
            ticket.display();
        }
    }

    pub fn add_ticket(&mut self, title: String, description: String) {
        self.counter += 1;
        let new_ticket = Ticket::new(self.counter, title, description);
        self.tickets.push(new_ticket);
    }
}
