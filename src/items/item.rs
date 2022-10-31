use std::path::PathBuf;

use crate::jot_path::JotPath;
use crate::output::error::JotResult;

pub trait Item {
    /// Get the absolute path of an item
    fn get_location(&self) -> &JotPath;
    /// Get the name of an item. Names do not include file extensions.
    fn get_name(&self) -> String {
        self.get_location().file_name()
    }
    /// Get the name of an item, including any extension.
    fn get_full_name(&self) -> String {
        self.get_location().file_with_extension()
    }
    /// Move the given item to a new location.
    ///
    /// `new_location` - absolute path to the new location
    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()>;
    fn rename(&mut self, new_name: String) -> JotResult<()>;
    fn delete(&self) -> JotResult<()>;
}
