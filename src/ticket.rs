use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TicketStatus {
    Done,
    Pending,
    Archived,
    Undecided,
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketStatus::Done => write!(f, "Done"),
            TicketStatus::Pending => write!(f, "Pending"),
            TicketStatus::Archived => write!(f, "Archived"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    id: u32,
    title: String,
    description: String,
    status: TicketStatus,
}

pub struct TicketUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
}

impl TicketUpdate {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            status: None,
        }
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} | {} | {}",
            self.id, self.title, self.description, self.status
        )
    }
}

impl Ticket {
    pub fn new(id: u32, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            status: TicketStatus::Pending,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> TicketStatus {
        self.status
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into()
    }
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into()
    }
    pub fn set_status(&mut self, status: TicketStatus) {
        self.status = status
    }

    pub fn mark_done(&mut self) {
        self.status = TicketStatus::Done
    }

    pub fn archive(&mut self) {
        self.status = TicketStatus::Archived
    }

    pub fn apply_update(&mut self, update: TicketUpdate) {
        if let Some(title) = update.title {
            self.set_title(title);
        }
        if let Some(description) = update.description {
            self.set_description(description);
        }
        if let Some(status) = update.status {
            self.set_status(status);
        }
    }
}
