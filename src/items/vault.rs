use std::path::PathBuf;
use std::collections::HashMap;

use crate::items::{Item, Error, Note, Folder};

pub struct Vault {
    /// name of the vault
    name: String,
    /// absolute path of the vault 
    location: PathBuf,
    /// active folder inside of the vault,
    folder: Option<Folder>,
    /// aliases for notes inside of the vault
    aliases: HashMap<String, String>,
    /// folders inside of the vault,
    folders: Vec<Folder>,
    /// notes inside of the vault
    notes: Vec<Note>,
}

impl Item for Vault {
    fn get_location(&self) -> &PathBuf {
        &self.location 
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn relocate(&mut self, new_location: PathBuf) -> Result<(), Error> {
        todo!()
    }

    fn rename(&mut self, new_name: String) -> Result<(), Error> {
        todo!()
    }

    fn delete(&self) -> Result<(), Error> {
        todo!()
    }
} 

impl Vault {
    /**
     * Creates a new new at the given location.
     */
    pub fn create(name: String, location: PathBuf) -> Result<Self, Error> {
        let mut new_vault = Vault {
            location,
            name,
            folder: None,
            aliases: HashMap::new(), 
            folders: vec![],
            notes: vec![],
        };

        /**
         * TODO: Create vault
         */
        Ok(new_vault)
    }
    /**
     * Initializes an existing folder and loads it's contents
     * into notes and folders.
     */
    pub fn new(name: String, location: PathBuf) -> Result<Self, Error> {
        assert!(location.is_dir());
        let mut new_vault = Vault {
            location,
            name,
            folder: None,
            aliases: HashMap::new(),
            folders: vec![],
            notes: vec![],
        };

        new_vault.load_contents()?;

        Ok(new_vault)
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
                self.folders.push(folder);
            } else if Note::is_jot_note(&item_location) {
                let note = Note::new(item_location)?;
                self.notes.push(note);
            }

        }

        Ok(())
    }

}
