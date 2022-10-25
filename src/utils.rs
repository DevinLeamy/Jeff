use crate::prelude::*;
use chrono;
use colored::*;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

use anyhow::anyhow;

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

pub fn color_text(text: &String, color: Color) -> String {
    text.color(color).to_string()
}

pub fn application_config_path() -> PathBuf {
    if cfg!(not(test)) {
        let project_dirs = ProjectDirs::from("com", "", "jot").unwrap();
        project_dirs.config_dir().to_path_buf()
    } else {
        PathBuf::from(format!("{}/config", TEST_HOME))
    }
}

pub fn application_data_path() -> PathBuf {
    if cfg!(not(test)) {
        let project_dirs = ProjectDirs::from("com", "", "jot").unwrap();
        project_dirs.data_dir().to_path_buf()
    } else {
        PathBuf::from(format!("{}/config", TEST_HOME))
    }
}
