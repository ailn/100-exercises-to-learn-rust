use std::sync::mpsc::{Receiver, Sender};
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { response_sender: Sender<TicketId>, draft: TicketDraft },
    Get { response_sender: Sender<Option<Ticket>>, id: TicketId }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert { response_sender, draft}) => {
                let id = store.add_ticket(draft);
                response_sender.send(id).unwrap()
            }
            Ok(Command::Get { response_sender, id}) => {
                let ticket = store.get(id);
                response_sender.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
