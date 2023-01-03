/**
 * Thin wrapper on Path/PathBuf to make managing paths easier
 */
use crate::utils::join_paths;

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct JeffPath {
    path: PathBuf,
}

impl Display for JeffPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", self.path)
    }
}

impl From<String> for JeffPath {
    fn from(path: String) -> Self {
        JeffPath {
            path: PathBuf::from(path),
        }
    }
}

impl From<PathBuf> for JeffPath {
    fn from(path: PathBuf) -> Self {
        JeffPath { path }
    }
}

impl Deref for JeffPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for JeffPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl JeffPath {
    pub fn from_parent(parent_dir: &PathBuf, file_name: String) -> Self {
        let path = join_paths(vec![parent_dir, &PathBuf::from(file_name)]);

        path.into()
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
    let path_one: JeffPath = "parent/child.txt".to_string().into();
    assert_eq!("parent", path_one.parent().to_str().unwrap());
    assert_eq!("child", path_one.file_name());
    assert_eq!("child.txt", path_one.file_with_extension());

    let path_two: JeffPath = "/parent_1/parent_2/child".to_string().into();
    assert_eq!("/parent_1/parent_2", path_two.parent().to_str().unwrap());
    assert_eq!("child", path_two.file_name());
    assert_eq!("child", path_two.file_with_extension());
}
