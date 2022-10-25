#![allow(dead_code)]
#![allow(unused_variables)]

use crate::App;
use crate::{enums::Item as ItemType, state::Command};
use core::time;
use std::sync::Mutex;
use std::{panic::UnwindSafe, path::PathBuf, thread};

/*
 * Because file system delete operations are slow/unperdictable at times, we do
 * not delete vaults, notes, or notes during tests.
 *
 * Solutions to this problem are welcome.
 */

static VAULT_COUNTER: Mutex<i32> = Mutex::new(0);
pub const TEST_HOME: &'static str = "/Users/Devin/Desktop/Github/OpenSource/jot/tests";

fn setup() {}

pub fn run_test<T>(test: T)
where
    T: FnOnce() -> () + UnwindSafe,
{
    setup();
    let result = std::panic::catch_unwind(test);
    teardown();

    assert!(result.is_ok())
}

fn teardown() -> () {}

pub fn test_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/vaults/{}", TEST_HOME, name))
}
pub fn test_config_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/config/{}", TEST_HOME, name))
}

pub fn next_vault() -> String {
    let vault_number = VAULT_COUNTER.lock().unwrap().clone();
    *VAULT_COUNTER.lock().unwrap() += 1;

    format!("test_vault_{}", vault_number)
}

/// returns the new vault name
pub fn create_app_and_vault() -> (App, String) {
    let mut app = App::new().unwrap();
    let vault_name = next_vault();
    execute_commands(
        &mut app,
        vec![
            Command::Vault {
                show_loc: false,
                name: Some(vault_name.to_owned()),
                location: Some(PathBuf::from(format!("{}/vaults", TEST_HOME))),
            },
            Command::Enter {
                name: vault_name.to_owned(),
            },
        ],
    );

    (app, vault_name)
}

pub fn execute_commands(app: &mut App, commands: Vec<Command>) {
    for command in commands {
        println!("{:?}", command);
        app.handle_command(command).unwrap();
        *app = App::new().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_framework() {
        run_test(|| {
            let sum = 2 + 2;
            assert!(sum == 4);
        });
    }

    #[test]
    fn create_app_and_vault_test() {
        let (_app, _vault_name) = create_app_and_vault();
        let (_app, _vault_name) = create_app_and_vault();
        let (_app, _vault_name) = create_app_and_vault();
    }
}
