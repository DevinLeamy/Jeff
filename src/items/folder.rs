use anyhow::anyhow;
use std::path::PathBuf;

use crate::prelude::*;
use std::fs::{create_dir, remove_dir_all, rename};

#[derive(Debug, Clone)]
pub struct Folder {
    folders: Vec<Box<Folder>>,
    notes: Vec<Note>,
    location: JeffPath,
}

impl Collection for Folder {
    fn notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    fn folders(&self) -> Vec<Folder> {
        self.folders
            .iter()
            .map(|folder_box| *folder_box.clone())
            .collect::<Vec<Folder>>()
    }
}

impl Folder {
    pub fn generate_abs_path(parent_dir: &PathBuf, folder_name: &String) -> PathBuf {
        join_paths(vec![parent_dir.to_str().unwrap(), folder_name])
    }
    /**
     * Creates a new folder at the given location.
     */
    pub fn create(absolute_path: PathBuf) -> JeffResult<Self> {
        println!("{:?}", absolute_path);
        if !Folder::is_valid_path(&absolute_path) {
            return Err(anyhow!("Invalid folder path"));
        }

        let folder = Folder {
            location: absolute_path.to_owned().into(),
            folders: vec![],
            notes: vec![],
        };

        // TODO: enforce that the folder is only one nesting level deeper
        // than the current note.
        create_dir(absolute_path)?;

        Ok(folder)
    }
    /**
     * Initializes an existing folder and loads it's contents
     * into notes and folders.
     */
    pub fn load(absolute_path: PathBuf) -> JeffResult<Self> {
        if !Folder::is_valid_path(&absolute_path) {
            return Err(anyhow!("Invalid folder path"));
        }

        let mut folder = Folder {
            location: absolute_path.into(),
            folders: vec![],
            notes: vec![],
        };

        folder.load_contents()?;

        Ok(folder)
    }

    /**
     * Check if a given location points to a valid
     * `jeff` [Folder]
     */
    pub fn is_valid_path(location: &PathBuf) -> bool {
        location.file_name().unwrap() != ".jeff" && !location.is_file()
    }

    pub fn as_collection(&self) -> Box<dyn Collection> {
        Box::new(self.clone())
    }
}

impl Item for Folder {
    fn get_location(&self) -> &JeffPath {
        &self.location
    }

    fn relocate(&mut self, new_location: PathBuf) -> JeffResult<()> {
        assert!(Folder::is_valid_path(&new_location));
        rename(&self.location.as_path(), &new_location)?;
        self.location = new_location.into();

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JeffResult<()> {
        let file_parent = self.location.parent();
        let new_location = get_absolute_path(&file_parent.to_path_buf(), &new_name);

        assert!(Folder::is_valid_path(&new_location));
        rename(&self.location.as_path(), &new_location)?;
        self.location = new_location.into();

        Ok(())
    }

    /**
     * Deletes the folder and all of its contents.
     */
    fn delete(&self) -> JeffResult<()> {
        // TODO: make sure the user is prompted before executing
        // NOTE: this could potentially delete a lot of information!
        remove_dir_all(&self.location.as_path())?;

        Ok(())
    }
}

impl Folder {
    /**
     * Loads the contents of a folder into notes and folders vectors.
     * Note: Folders inside of `self` are also loaded.
     */
    pub fn load_contents(&mut self) -> JeffResult<()> {
        for item in self.location.read_dir().unwrap() {
            let item_location = item.unwrap().path();

            if Folder::is_valid_path(&item_location) {
                let folder = Folder::load(item_location)?;
                self.folders.push(Box::new(folder));
            } else if Note::is_valid_path(&item_location) {
                let note = Note::load(item_location)?;
                self.notes.push(note);
            }
        }

        Ok(())
    }
}
