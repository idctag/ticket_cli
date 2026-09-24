use std::io::Result;

use todo_cli::ticket::{
    user_tickets::UserTickets,
    utils::{
        display::print_menu,
        input::{prompt_add_ticket, prompt_apply_update},
    },
};

fn main() -> Result<()> {
    let mut tickets = UserTickets::new();
    tickets.add_ticket("One".to_string(), "One Description".to_string());
    tickets.add_ticket("Two".to_string(), "Two Description".to_string());
    tickets.add_ticket("Three".to_string(), "Three Description".to_string());

    loop {
        print_menu();
        match choice {
            Ok(1) => {
                for t in tickets.tickets() {
                    println!("{t}")
                }
            }
            Ok(2) => {
                if let Err(error) = prompt_add_ticket(&mut tickets) {
                    eprintln!("Could not read input {error}")
                }
            }
            Ok(3) => {
                if let Err(error) = prompt_apply_update(&mut tickets) {
                    eprintln!("Could not apply update: {error}")
                }
            }
            Ok(4) => break,
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}
