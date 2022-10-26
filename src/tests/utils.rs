#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::JotResult;
use crate::App;
use crate::{enums::Item as ItemType, state::Command::*};
use std::sync::Mutex;
use std::{panic::UnwindSafe, path::PathBuf};

static VAULT_COUNTER: Mutex<i32> = Mutex::new(0);
pub const TEST_HOME: &'static str = "/Users/Devin/Desktop/Github/OpenSource/jot/tests";
pub const TEST_VAULTS: &'static str = "/Users/Devin/Desktop/Github/OpenSource/jot/tests/vaults";
pub const TEST_CONFIG: &'static str = "/Users/Devin/Desktop/Github/OpenSource/jot/tests/config";
pub const INITIAL_VAULT: &'static str = "vault_1";

fn setup() {
    std::fs::create_dir_all(TEST_HOME).unwrap();
    std::fs::create_dir_all(format!("{}/vaults", TEST_HOME)).unwrap();
    std::fs::create_dir_all(format!("{}/config", TEST_HOME)).unwrap();
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
    std::fs::remove_dir_all(TEST_HOME).unwrap();
}

pub fn test_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/vaults/{}", TEST_HOME, name))
}
pub fn test_vaults() -> PathBuf {
    PathBuf::from(format!("{}/vaults", TEST_HOME))
}
pub fn test_config_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/config/{}", TEST_HOME, name))
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
pub use Test::*;

pub fn execute_command(test: Test) {
    let mut app = App::new().unwrap();
    match test {
        Pass(command) => {
            if let Err(_) = app.handle_command(command.clone()) {
                panic!(
                    "\n{}\n",
                    format!("Failed on command: [{:?}]", command).red()
                );
            }
        }
        Fail(command) => {
            if let Ok(_) = app.handle_command(command.clone()) {
                panic!(
                    "\n{}\n",
                    format!("Failed on command: [{:?}]", command).red()
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
