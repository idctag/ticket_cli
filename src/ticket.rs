#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TicketStatus {
    Done,
    Pending,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    id: u32,
    title: String,
    description: String,
    status: TicketStatus,
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

    pub fn status(&self) -> &TicketStatus {
        &self.status
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into()
    }
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into()
    }
    pub fn set_status(&mut self, status: TicketStatus) {
        self.status = status.into()
    }

    pub fn mark_done(&mut self) {
        self.status = TicketStatus::Done
    }

    pub fn archive(&mut self) {
        self.status = TicketStatus::Archived
    }
}
