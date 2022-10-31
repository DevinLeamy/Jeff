#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;
use crate::App;

use std::sync::Mutex;
use std::{panic::UnwindSafe, path::PathBuf};

static VAULT_COUNTER: Mutex<i32> = Mutex::new(0);

#[rustfmt::skip]
lazy_static! {
    pub static ref TEST_HOME: PathBuf = PathBuf::from(format!("{}/tests", env!("CARGO_MANIFEST_DIR")));
    pub static ref TEST_VAULTS: PathBuf = PathBuf::from(format!("{}/tests/vaults", env!("CARGO_MANIFEST_DIR")));
    pub static ref TEST_CONFIG: PathBuf = PathBuf::from(format!("{}/tests/config", env!("CARGO_MANIFEST_DIR")));
}
pub const INITIAL_VAULT: &'static str = "vault_1";

pub fn test_vaults() -> PathBuf {
    (*TEST_VAULTS).clone()
}

fn setup() {
    std::fs::create_dir_all(&*TEST_HOME).unwrap();
    std::fs::create_dir_all(&*TEST_VAULTS).unwrap();
    std::fs::create_dir_all(&*TEST_CONFIG).unwrap();
    *VAULT_COUNTER.lock().unwrap() = 0;
}

pub fn run_test<T>(test: T)
where
    T: FnOnce() -> () + UnwindSafe,
{
    setup();
    let result = std::panic::catch_unwind(test);
    teardown();

    assert!(result.is_ok())
}

fn teardown() {
    std::fs::remove_dir_all(&*TEST_HOME).unwrap();
}

pub fn next_vault() -> String {
    let vault_number = VAULT_COUNTER.lock().unwrap().clone();
    *VAULT_COUNTER.lock().unwrap() += 1;

    format!("test_vault_{}", vault_number)
}

pub enum Test {
    Pass(crate::state::Command),
    Fail(crate::state::Command),
}

use colored::Colorize;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
pub use Test::*;

pub fn execute_command(test: Test) {
    let mut app = App::new().unwrap();
    match test {
        Pass(command) => {
            if let Err(_) = app.handle_command(command.clone()) {
                panic!(
                    "\n{}\n",
                    format!("Expected to pass on command: [{:?}]", command).red()
                );
            }
        }
        Fail(command) => {
            if let Ok(_) = app.handle_command(command.clone()) {
                panic!(
                    "\n{}\n",
                    format!("Expected to fail on command: [{:?}]", command).red()
                );
            }
        }
    };
}

pub fn execute_commands(commands: Vec<Test>) {
    for test in commands {
        execute_command(test);
    }
}

pub fn display_item_color_select<T: Item + Colored>() -> String {
    // let prompt = format!("Select a {} color.", T::type_name().color(T::get_color()));
    let prompt = format!("Select a color.");
    display_color_select(prompt)
}

pub fn display_color_select(prompt: String) -> String {
    let colors = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "purple",
        "cyan",
        "white",
        "bright black",
        "bright red",
        "bright green",
        "bright yellow",
        "bright blue",
        "bright magenta",
        "bright cyan",
        "bright white",
    ];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&colors)
        .default(0)
        .interact()
        .unwrap();

    colors[selection].to_string()
}
