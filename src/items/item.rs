use std::path::PathBuf;

use crate::items::Error;
use crate::output::error::JotResult;

pub trait Item : Clone {
    /// Get the absolute path of an item
    fn get_location(&self) -> &PathBuf;
    /// Get the name of an item. Names do not include file extensions
    fn get_name(&self) -> String;
    /// Move the given item to a new location.
    /// 
    /// `new_location` - absolute path to the new location
    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()>;
    fn rename(&mut self, new_name: String) -> JotResult<()>;
    fn delete(&self) -> JotResult<()>;
    fn generate_abs_path(parent_dir: &PathBuf, item_name: &String) -> PathBuf;
    fn load(path: PathBuf) -> JotResult<Self>;
    fn create(path: PathBuf) -> JotResult<Self>;
}


