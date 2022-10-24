mod app;
mod editor;
mod enums;
mod items;
mod jot_path;
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
use clap::Parser;

fn main() {
    let mut app = App::new().unwrap();
    let command = crate::state::Args::parse().command;

    match app.handle_command(command) {
        Ok(msg) => match msg {
            Message::Empty => (),
            _ => println!("{}", Output::Message(msg)),
        },
        Err(message) => println!("{}", message),
    }
}
