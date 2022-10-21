use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::items::{Item, Error, Note, Folder};
use crate::traits::FileIO;
use crate::utils::join_paths;
use crate::enums::VaultItem;

#[derive(Debug)]
pub struct Vault {
    /// name of the vault
    name: String,
    /// absolute path of the vault 
    location: PathBuf,
    /// active folder inside of the vault,
    folder: Option<Folder>,
    /// folders inside of the vault,
    folders: Vec<Folder>,
    /// notes inside of the vault
    notes: Vec<Note>,
    /// persisted data locally managed by the vault 
    vault_store: VaultStore,
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
            folders: vec![],
            notes: vec![],
            vault_store: VaultStore::create_from_path(location),
        };

        /*
         * TODO: make sure that this vault is added to the application 
         * data's [vaults] list
         */

        Ok(new_vault)
    }
    /**
     * Initializes an existing folder and loads it's contents
     * into notes and folders.
     */
    pub fn load(name: String, location: PathBuf) -> Result<Self, Error> {
        assert!(location.is_dir());
        let mut new_vault = Vault {
            location,
            name,
            folder: None,
            folders: vec![],
            notes: vec![],
            // TODO: consider creating FileIO::load_from_path(&self, path: PathBuf)
            vault_store: VaultStore::load(),
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
                let folder = Folder::load(item_location)?;
                self.folders.push(folder);
            } else if Note::is_jot_note(&item_location) {
                let note = Note::new(item_location)?;
                self.notes.push(note);
            }

        }

        Ok(())
    }
}

impl Vault {
    pub fn create_vault_item(item: VaultItem, name: &String) {}

    pub fn create_and_open_note() {}

    pub fn remove_alias_from_note() {}

    pub fn set_alias() {}

    pub fn open_note() {}

    pub fn change_folder() {}

    pub fn rename_vault_item() {}

    pub fn remove_vault_item() {}

    pub fn move_vault_item() {

    }

    pub fn list(&self) {
        println!("TODO: List items in the current folder")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultStore {
    /// path to the current active folder inside of the vault
    current_folder: Option<String>,
    /// aliases for notes inside of the vault
    aliases: HashMap<String, String>,
    /// absolute path to the vault store (in `.jot`, relative to [[Vault]])
    /// Option<T> type because [[FileIO]] has [[Default]] trait bound
    location: Option<PathBuf>,
}

impl Default for VaultStore {
    fn default() -> Self {
        VaultStore {
            current_folder: None,
            aliases: HashMap::new(),
            location: None,
        }
    }
}

impl FileIO for VaultStore {
    /**
     * Path to the vault's persistent data
     * store.
     */
    fn path(&self) -> PathBuf {
        join_paths(vec![
            self.location.unwrap().to_str().unwrap(),
            ".jot/data",
        ])
    }
}

impl VaultStore {
    /**
     * Creates a [[VaultStore]] from the absolute path
     * of the folder it will stored inside.
     */
    pub fn create_from_path(parent_directory: PathBuf) -> Self {
        let location = join_paths(vec![
            parent_directory.to_str().unwrap(),
            ".jot/data"
        ]);

        let vault_store = VaultStore {
            current_folder: None,
            aliases: HashMap::new(),
            location: Some(location),
        };

        vault_store.store();

        vault_store
    }
}

