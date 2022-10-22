mod app;
mod enums;
mod output;
mod state;
mod traits;
mod utils;
mod items;
mod editor;
mod prelude;
mod tests;

use crate::{
    app::App,
    output::{Message, Output},
};
fn main() {
    let mut app = App::new();

    match app.handle_args() {
        Ok(msg) => match msg {
            Message::Empty => (),
            _ => println!("{}", Output::Message(msg)),
        },
        Err(message) => println!("{}", message),
    }
}
