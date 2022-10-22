use std::path::PathBuf;
use std::fs::{remove_file, File, rename};

use crate::utils::join_paths;
use crate::items::{Item, Error, Folder};
use crate::output::error::JotResult;

#[derive(Debug)]
pub struct Note {
    location: PathBuf,
}

impl Item for Note {
    fn get_location(&self) -> &PathBuf {
        &self.location
    }

    fn get_name(&self) -> String {
        self.location.file_name().unwrap().to_str().unwrap().to_string()
    }

    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()> {
        assert!(Note::is_jot_note(&new_location));
        rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let file_parent = self.location.parent().unwrap();
        let new_location = join_paths(vec![file_parent.to_str().unwrap(), &new_name]);

        rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    fn delete(&self) -> JotResult<()> {
        // TODO: make sure the user is prompted before executing 
        // NOTE: this could potentially delete a lot of information! 
        remove_file(&self.location)?;

        Ok(())
    }

    fn generate_abs_path(parent_dir: &PathBuf, note_name: &String) -> PathBuf {
        join_paths(vec![
            parent_dir.to_str().unwrap(),
            format!("{}.md", note_name).as_str(),
        ])
    }
}

impl Note {
    /**
     * Creates a note inside of the given folder.
     */
    // pub fn create_from_folder(name: String, folder: &Folder) -> Result<Self, Error> {
    //     let location = join_paths(vec![folder.get_location().to_str().unwrap(), &name]);
    //     File::create(&location)?;
    //
    //     Ok(Note { location })
    // }

    /**
     * Initializes an existing note from its path
     */
    pub fn new(note_location: PathBuf) -> Result<Self, Error> {
        Ok(Note { location: note_location })
    }

    /**
     * Checks if a path points to a valid jot note.
     */
    pub fn is_jot_note(location: &PathBuf) -> bool {
        if location.extension().is_none() {
            return false;
        }

        location.is_file() && location.extension().unwrap() == "md"
    }
}


