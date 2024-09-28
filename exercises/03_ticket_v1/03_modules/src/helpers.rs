// TODO: Make this code compile, either by adding a `use` statement or by using
//  the appropriate path to refer to the `Ticket` struct.

use crate::Ticket;

fn create_todo_ticket(title: String, description: String) -> Ticket {
    Ticket::new(title, description, "To-Do".into())
}
