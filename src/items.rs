use std::path::PathBuf;
use std::fs::{remove_file, File};

use crate::utils::join_paths;

type Error = std::io::Error;

trait Item {
    fn get_location(&self) -> &PathBuf;
    fn get_name(&self) -> String;
    fn relocate(&mut self, new_location: PathBuf) -> Result<(), Error>;
    fn rename(&mut self, new_name: String) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>;
}

struct Note {
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
    pub fn create(name: String, folder: Folder) -> Result<Self, Error> {
        let location = join_paths(vec![folder.get_location().to_str().unwrap(), &name]);
        File::create(&location)?;

        Ok(Note { location })
    }

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

struct Folder {
    folders: Vec<Box<Folder>>,
    notes: Vec<Note>,
    location: PathBuf,
}

impl Item for Folder {
    fn get_location(&self) -> &PathBuf {
        &self.location
    }

    fn get_name(&self) -> String {
        self.location.display().to_string()
    }

    fn relocate(&mut self, new_location: PathBuf) -> Result<(), Error> {
        todo!()
    }

    fn rename(&mut self, new_name: String) -> Result<(), Error> {
        todo!()
    }

    /**
     * Deletes the folder and all of its contents.
     */
    fn delete(&self) -> Result<(), Error> {
        todo!()
    }
}

impl Folder {
    /**
     * Creates a new folder at the current location.
     */
    pub fn create(location: PathBuf) -> Result<Self, Error> {
        todo!();
        Ok(Folder {
            location,
            folders: vec![],
            notes: vec![],
        })
    }
    /**
     * Initializes an existing folder and loads it's contents
     * into notes and folders.
     */
    pub fn new(location: PathBuf) -> Result<Self, Error> {
        assert!(location.is_dir());
        let mut folder = Folder {
            location,
            folders: vec![],
            notes: vec![],
        };

        folder.load_contents()?;

        Ok(folder)
    }
    /**
     * Loads the contents of a folder into notes and folders vectors.
     * Note: Folders inside of `self` are also loaded.
     */
    pub fn load_contents(&mut self) -> Result<(), Error> {
        for item in self.location.read_dir().unwrap() {
            let item_location = item.unwrap().path();

            if Folder::is_jot_folder(&item_location) {
                let folder = Folder::new(item_location)?;
                self.folders.push(Box::new(folder));
            } else if Note::is_jot_note(&item_location) {
                let note = Note::new(item_location)?;
                self.notes.push(note);
            }

        }

        Ok(())
    }

    /**
     * Check if a given location points to a valid 
     * `jot` folder.
     */
    pub fn is_jot_folder(location: &PathBuf) -> bool {
        location.is_dir() && location.file_name().unwrap() != ".jot"
    }
}
