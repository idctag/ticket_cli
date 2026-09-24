use crate::ticket::user_tickets::UserTickets;

pub fn print_menu() {
    println!("Choose from options");
    println!("[1] list");
    println!("[2] add");
    println!("[3] edit");
    println!("[4] quit");
}
pub fn list_tickets(tickets: &UserTickets) {
    for t in &tickets.tickets {
        println!("{t}")
    }
}
