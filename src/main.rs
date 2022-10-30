// #![feature(stmt_expr_attributes)]
mod app;
mod editor;
mod enums;
mod fileio;
mod items;
mod jot_path;
mod output;
mod prelude;
mod state;
mod tests;
mod utils;

#[macro_use]
extern crate lazy_static;

use crate::{
    app::App,
    output::{Message, Output},
};
use clap::Parser;

fn main() {
    let mut app = App::new().unwrap();
    let command = crate::state::Args::parse().command;

    match app.handle_command(command) {
        Ok(Message::Empty) => (),
        Ok(message) => println!("{}", Output::Message(message)),
        Err(message) => println!("{}", message),
    }
}
