use std::io::{self, Result};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use todo_cli::ticket::user_tickets::UserTickets;

fn main() -> Result<()> {
    let mut app = App::new();

    Ok(())
}

struct App {
    u_tickets: UserTickets,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            u_tickets: UserTickets::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {}
    }
}
