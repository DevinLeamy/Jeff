use std::path::{Path, PathBuf};

use chrono;
use dialoguer::{theme::ColorfulTheme, Confirm};
use directories::ProjectDirs;

use crate::prelude::*;

#[allow(unused)]
fn valid_name(name: &str) -> bool {
    name.chars().all(|char| !r#"\/?%*:|"<>"#.contains(char))
}

pub fn join_paths<T: AsRef<Path>>(paths: Vec<T>) -> PathBuf {
    let mut full_path = PathBuf::new();
    for path in paths {
        full_path.push(path);
    }

    full_path
}

pub fn get_absolute_path(parent_dir: &PathBuf, file_name: &String) -> PathBuf {
    join_paths(vec![parent_dir.to_str().unwrap(), file_name])
}

// returns new pathbuf -> with slashes formatted according to os & '..'s collapsed
// use this when storing or displaying paths
// not using canonicalize because it returns \\?\C:\*path* on windows
pub fn process_path(path: &Path) -> PathBuf {
    let mut processed_path = PathBuf::new();

    for element in path.iter() {
        if element == ".." {
            processed_path.pop();
        } else if element != "." {
            processed_path.push(element);
        }
    }

    processed_path
}

fn generate_date_string() -> String {
    let local_timestamp = chrono::offset::Local::now();
    let local_date = local_timestamp.date_naive();

    local_date.to_string()
}

pub fn daily_note_name() -> String {
    generate_date_string()
}

pub fn path_to_string(path: PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

pub fn application_config_path() -> PathBuf {
    if cfg!(not(test)) {
        let project_dirs = ProjectDirs::from("com", "", "jot").unwrap();
        project_dirs.config_dir().to_path_buf()
    } else {
        (*TEST_CONFIG).clone()
    }
}

pub fn application_data_path() -> PathBuf {
    if cfg!(not(test)) {
        let project_dirs = ProjectDirs::from("com", "", "jot").unwrap();
        project_dirs.data_dir().to_path_buf()
    } else {
        (*TEST_CONFIG).clone()
    }
}

pub fn application_templates_path() -> PathBuf {
    if cfg!(not(test)) {
        let template_dir = ProjectDirs::from("com", "", "jot").unwrap();
        let mut template_dir_path = template_dir.data_dir().to_path_buf();
        template_dir_path.push("templates");
        template_dir_path
    } else {
        (*TEST_TEMPLATES).clone()
    }
}

pub fn create_file<P: AsRef<Path>>(path: P) -> JotResult<()> {
    std::fs::File::options()
        .create_new(true)
        .write(true)
        .open(path)?;

    Ok(())
}

pub fn item_with_name<'a, I: Item>(items: &'a Vec<I>, name: &String) -> Option<&'a I> {
    items.iter().find(|item| &item.get_name() == name)
}

/// Displays a confirmation prompt with the given text.
/// Returns the outcome, "true" if the action was confirmed
/// and "false" otherwise.
pub fn confirmation_prompt(prompt: String) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}
