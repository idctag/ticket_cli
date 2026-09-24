use todo_cli::ticket::{
    user_tickets::UserTickets,
    utils::{display::print_menu, input::read_choice},
};

fn main() {
    let mut tickets = UserTickets::new();
    tickets.add_ticket("One".to_string(), "One Description".to_string());
    tickets.add_ticket("Two".to_string(), "Two Description".to_string());
    tickets.add_ticket("Three".to_string(), "Three Description".to_string());

    loop {
        print_menu();
        match read_choice() {
            Ok(1) => {
                for t in tickets.tickets() {
                    println!("{t}")
                }
            }
            Ok(2) => tickets.insert_ticket(),
            Ok(3) => tickets.edit_ticket(),
            Ok(4) => break,
            _ => println!("Invalid choice"),
        }
    }
}
