use std::path::PathBuf;

use crate::items::Error;
use crate::output::error::JotResult;

pub trait Item : Clone {
    fn get_location(&self) -> &PathBuf;
    fn get_name(&self) -> String;
    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()>;
    fn rename(&mut self, new_name: String) -> JotResult<()>;
    fn delete(&self) -> JotResult<()>;
    fn generate_abs_path(parent_dir: &PathBuf, item_name: &String) -> PathBuf;

}


