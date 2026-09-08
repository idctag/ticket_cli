use crate::ticket::Ticket;

pub struct UserTickets {
    tickets: Vec<Ticket>,
    next_id: u32,
}

impl UserTickets {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_ticket(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let new_ticket = Ticket::new(self.next_id, title, description);
        self.tickets.push(new_ticket);
        id
    }

    pub fn tickets(&self) -> &[Ticket] {
        &self.tickets
    }

    pub fn find(&self, id: u32) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.id() == id)
    }

    pub fn find_mud(&mut self, id: u32) -> Option<&mut Ticket> {
        self.tickets.iter_mut().find(|ticket| ticket.id() == id)
    }
}
