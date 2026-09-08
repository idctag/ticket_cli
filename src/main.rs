use todo_cli::core::UserTickets;

fn main() {
    let mut t = UserTickets::new();
    t.add_ticket(
        "grocery".to_string(),
        "1. carrot, 2. cabbage, 3. beef".to_string(),
    );

    t.add_ticket(
        "ticket_cli".to_string(),
        "finish the engine logics".to_string(),
    );

    t.add_ticket(
        "ticket_cli".to_string(),
        "add cli navigation feature".to_string(),
    );

    t.add_ticket(
        "ticket_cli".to_string(),
        "add cloud storage system".to_string(),
    );

    t.add_ticket(
        "ticket_cli".to_string(),
        "user authentication system".to_string(),
    );

    for ticket in t.tickets() {
        println!("{ticket}")
    }
}
