use std::path::PathBuf;
use std::fs::{remove_dir_all, rename, create_dir_all};
use anyhow::anyhow;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Folder {
    folders: Vec<Box<Folder>>,
    notes: Vec<Note>,
    location: PathBuf,
}

impl Collection for Folder {
    fn get_notes(&self) -> Vec<Note> {
        self.notes.clone() 
    }

    fn get_folders(&self) -> Vec<Folder> {
        self.folders.iter().map(|folder_box| *folder_box.clone()).collect::<Vec<Folder>>()
    }
}

impl Item for Folder {
    fn get_location(&self) -> &PathBuf {
        &self.location
    }

    fn get_name(&self) -> String {
        self.location.display().to_string()
    }

    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()> {
        assert!(Folder::is_valid_path(&new_location));
        rename(&self.location, &new_location)?;
        self.location = new_location;

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let file_parent = self.location.parent().unwrap();
        let new_location = get_absolute_path(&file_parent.to_path_buf(), &new_name);

        assert!(Folder::is_valid_path(&new_location));
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
    /**
     * Creates a new folder at the given location.
     */
    fn create(absolute_path: PathBuf) -> JotResult<Self> {
        println!("{:?}", absolute_path);
        if !Folder::is_valid_path(&absolute_path) {
            return Err(anyhow!("Invalid folder path"));
        }

        let folder = Folder {
            location: absolute_path.clone(),
            folders: vec![],
            notes: vec![],
        };

        // TODO: enforce that the folder is only one nesting level deeper
        // than the current note.
        create_dir_all(absolute_path)?;

        Ok(folder)
    }
    /**
     * Initializes an existing folder and loads it's contents
     * into notes and folders.
     */
    fn load(absolute_path: PathBuf) -> JotResult<Self> {
        if !Folder::is_valid_path(&absolute_path) {
            return Err(anyhow!("Invalid folder path"));
        }

        let mut folder = Folder {
            location: absolute_path,
            folders: vec![],
            notes: vec![],
        };

        folder.load_contents()?;

        Ok(folder)
    }

    /**
     * Check if a given location points to a valid 
     * `jot` [Folder]
     */
    fn is_valid_path(location: &PathBuf) -> bool {
        location.file_name().unwrap() != ".jot" && !location.is_file()
    }
}

impl Folder {
    
    /**
     * Loads the contents of a folder into notes and folders vectors.
     * Note: Folders inside of `self` are also loaded.
     */
    pub fn load_contents(&mut self) -> JotResult<()> {
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

    

    pub fn list_with_buffer(&self, buffer: String) {
        println!("├── {}{}", buffer, self.get_name());

        for folder in self.get_folders_sorted() {
            folder.list_with_buffer(format!("{}  ", buffer).to_string());
        }

        for note in self.get_notes_sorted() {
            println!("├── {}{}", buffer, note.get_name());
        }
    }
}
