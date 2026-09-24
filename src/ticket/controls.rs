use crate::ticket::user_tickets::UserTickets;

impl UserTickets {
    pub fn j_entry(&mut self) {
        if let Some(n) = self.selected {
            // find the next ticket
            // if no next ticket do nothing
        }
        if self.tickets().len() > 0 && self.selected == None {
            // select the first ticket
        }
    }
}
