use std::path::PathBuf;
use std::fs::{remove_dir_all, File, rename};

use crate::items::{Note, Error, Item};
use crate::utils::{join_paths, get_absolute_path};
use crate::output::error::JotResult;

// use crate::utils::join_paths;


#[derive(Debug)]
pub struct Folder {
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

    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()> {
        assert!(Folder::is_jot_folder(&new_location));
        rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let file_parent = self.location.parent().unwrap();
        let new_location = get_absolute_path(&file_parent.to_path_buf(), &new_name);

        assert!(Folder::is_jot_folder(&new_location));
        rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    /**
     * Deletes the folder and all of its contents.
     */
    fn delete(&self) -> JotResult<()> {
        // TODO: make sure the user is prompted before executing 
        // NOTE: this could potentially delete a lot of information! 
        remove_dir_all(&self.location)?;

        Ok(())
    }

    fn generate_abs_path(parent_dir: &PathBuf, folder_name: &String) -> PathBuf {
        join_paths(vec![
            parent_dir.to_str().unwrap(),
            folder_name,
        ])
    }
}

impl Folder {
    /**
     * Creates a new folder at the given location.
     */
    pub fn create(location: PathBuf) -> JotResult<Self> {
        
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
    pub fn load(location: PathBuf) -> JotResult<Self> {
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
    pub fn load_contents(&mut self) -> JotResult<()> {
        for item in self.location.read_dir().unwrap() {
            let item_location = item.unwrap().path();

            if Folder::is_jot_folder(&item_location) {
                let folder = Folder::load(item_location)?;
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
     * `jot` [[Folder]]
     */
    pub fn is_jot_folder(location: &PathBuf) -> bool {
        location.is_dir() && location.file_name().unwrap() != ".jot"
    }
}
