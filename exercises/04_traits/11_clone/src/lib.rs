// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let summary = ticket.summary();
    (ticket.clone(), summary)
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(&self) -> Summary {
        Summary {
            title: self.title.clone(),
            status: self.status.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Summary {
    pub title: String,
    pub status: String,
}
