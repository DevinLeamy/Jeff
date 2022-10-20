use std::path::PathBuf;

use crate::items::Error;

pub trait Item {
    fn get_location(&self) -> &PathBuf;
    fn get_name(&self) -> String;
    fn relocate(&mut self, new_location: PathBuf) -> Result<(), Error>;
    fn rename(&mut self, new_name: String) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>;
}


