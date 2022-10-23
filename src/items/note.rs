use anyhow::anyhow;
use std::fs::{remove_file, rename, File};
use std::path::PathBuf;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Note {
    location: PathBuf,
}

impl Item for Note {
    fn get_location(&self) -> &PathBuf {
        &self.location
    }

    fn get_name(&self) -> String {
        self.location
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()> {
        assert!(Note::is_valid_path(&new_location));
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

    /**
     * Initializes an existing note from its path
     */
    fn load(note_location: PathBuf) -> JotResult<Self> {
        if !Note::is_valid_path(&note_location) {
            return Err(anyhow!("Invalid note path [{:?}]", note_location));
        }

        Ok(Note {
            location: note_location,
        })
    }

    fn create(note_location: PathBuf) -> JotResult<Self> {
        if !Note::is_valid_path(&note_location) {
            return Err(anyhow!("Invalid note path [{:?}]", note_location));
        }

        let _result = File::options()
            .create_new(true)
            .write(true)
            .open(&note_location)?;
        let note = Note::load(note_location)?;

        Ok(note)
    }

    /**
     * Checks if a path points to a valid jot [Note].
     */
    fn is_valid_path(absolute_path: &PathBuf) -> bool {
        if absolute_path.extension().is_none() {
            return false;
        }

        !absolute_path.is_dir() && absolute_path.extension().unwrap() == "md"
    }
}

impl Note {
    /*
     * Creates a note inside of the given folder.
     */
    // pub fn create_from_folder(name: String, folder: &Folder) -> Result<Self, Error> {
    //     let location = join_paths(vec![folder.get_location().to_str().unwrap(), &name]);
    //     File::create(&location)?;
    //
    //     Ok(Note { location })
    // }
}
