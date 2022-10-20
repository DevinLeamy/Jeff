use std::path::PathBuf;
use std::fs::{remove_file, File};

use crate::utils::join_paths;
use crate::items::{Item, Error, Folder};


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

    fn relocate(&mut self, new_location: PathBuf) -> Result<(), Error> {
        assert!(new_location.is_file());
        std::fs::rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> Result<(), Error> {
        let file_parent = self.location.parent().unwrap();
        let new_location = join_paths(vec![file_parent.to_str().unwrap(), &new_name]);

        std::fs::rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    fn delete(&self) -> Result<(), Error> {
        remove_file(&self.location)
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
        location.is_file() && location.extension().unwrap() == "md"
    }
}


