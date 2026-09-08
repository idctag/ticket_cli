use todo_cli::core::UserTickets;

fn main() {
    let mut tickets = UserTickets::new();
    tickets.add_ticket(
        "grocery".to_string(),
        "1. carrot, 2. cabbage, 3. beef".to_string(),
    );

    tickets.add_ticket(
        "ticket_cli".to_string(),
        "finish the engine logics".to_string(),
    );

    tickets.add_ticket(
        "ticket_cli".to_string(),
        "add cli navigation feature".to_string(),
    );

    tickets.add_ticket(
        "ticket_cli".to_string(),
        "add cloud storage system".to_string(),
    );

    tickets.add_ticket(
        "ticket_cli".to_string(),
        "user authentication system".to_string(),
    );

    tickets.list();
}
