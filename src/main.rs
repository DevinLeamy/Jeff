mod app;
mod editor;
mod enums;
mod items;
mod output;
mod prelude;
mod state;
mod tests;
mod traits;
mod utils;

use crate::{
    app::App,
    output::{Message, Output},
};
fn main() {
    let mut app = App::new().unwrap();

    match app.handle_args() {
        Ok(msg) => match msg {
            Message::Empty => (),
            _ => println!("{}", Output::Message(msg)),
        },
        Err(message) => println!("{}", message),
    }
}
