use todo_cli::ticket::{
    user_tickets::UserTickets,
    utils::{
        display::print_menu,
        input::{prompt_add_ticket, prompt_apply_update, read_choice},
    },
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
            Ok(2) => prompt_add_ticket(&mut tickets),
            Ok(3) => prompt_apply_update(&mut tickets),
            Ok(4) => break,
            _ => println!("Invalid choice"),
        }
    }
}
