/**
 * Thin wrapper on Path/PathBuf to make managing paths within Jot
 * easier
 */
use crate::utils::join_paths;

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct JotPath {
    path: PathBuf,
}

impl Display for JotPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", self.path)
    }
}

impl From<String> for JotPath {
    fn from(path: String) -> Self {
        JotPath {
            path: PathBuf::from(path),
        }
    }
}

impl From<PathBuf> for JotPath {
    fn from(path: PathBuf) -> Self {
        JotPath { path }
    }
}

impl Deref for JotPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for JotPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl JotPath {
    pub fn from_directory(parent_dir: &PathBuf, file_name: String) -> Self {
        let path = join_paths(vec![parent_dir, &PathBuf::from(file_name)]);

        path.into()
    }

    pub fn set_path(&mut self, new_path: &PathBuf) {
        *self = JotPath::from(new_path.to_owned())
    }

    pub fn file_name(&self) -> String {
        self.path.file_stem().unwrap().to_str().unwrap().to_string()
    }

    pub fn file_with_extension(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn parent(&self) -> PathBuf {
        self.path.parent().unwrap().to_owned()
    }
}

#[test]
fn basic_tests() {
    let path_one: JotPath = "parent/child.txt".to_string().into();
    assert_eq!("parent", path_one.parent().to_str().unwrap());
    assert_eq!("child", path_one.file_name());
    assert_eq!("child.txt", path_one.file_with_extension());

    let path_two: JotPath = "/parent_1/parent_2/child".to_string().into();
    assert_eq!("/parent_1/parent_2", path_two.parent().to_str().unwrap());
    assert_eq!("child", path_two.file_name());
    assert_eq!("child", path_two.file_with_extension());
}
