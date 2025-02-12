use crate::Iroh;
use crate::Message;
use crate::Ticket;

pub struct Topic {
    iroh: Iroh,
    messages: Vec<Message>,
    ticket: Ticket,
}
