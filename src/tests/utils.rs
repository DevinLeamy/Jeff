use core::time;
use std::{path::PathBuf, thread, fs::{create_dir_all, remove_dir_all}, panic::UnwindSafe};

const TEST_HOME: &'static str = "/Users/Devin/Desktop/Github/OpenSource/jot/tests";

pub fn sleep() {
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
}

fn setup() {
    let _res = create_dir_all(PathBuf::from(TEST_HOME));
    sleep();
}

pub fn run_test<T>(test: T)
where 
    T: FnOnce() -> () + UnwindSafe 
{
    setup();
    let result = std::panic::catch_unwind(test); 
    teardown();

    assert!(result.is_ok())
}

fn teardown() -> () {
    let _res = remove_dir_all(PathBuf::from(TEST_HOME));
    sleep();
}

pub fn test_path(name: &str) -> PathBuf {
    format!("{}/{}", TEST_HOME, name).into()
}
