use crate::ticket::user_tickets::UserTickets;

impl UserTickets {
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
