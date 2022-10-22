use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::{remove_dir_all, rename};

use crate::items::{Item, Error, Note, Folder};
use crate::traits::FileIO;
use crate::utils::{join_paths, get_absolute_path};
use crate::enums::VaultItem;
use crate::output::error::JotResult;

#[derive(Debug)]
pub struct Vault {
    /// name of the vault
    name: String,
    /// absolute path of the vault 
    absolute_path: PathBuf,
    /// folders inside of the vault,
    folders: Vec<Folder>,
    /// notes inside of the vault
    notes: Vec<Note>,
    /// persisted data locally managed by the vault 
    vault_store: VaultStore,
}

impl Item for Vault {
    fn get_location(&self) -> &PathBuf {
        &self.absolute_path 
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn relocate(&mut self, new_absolute_path: PathBuf) -> JotResult<()> {
        assert!(Vault::is_jot_vault(&new_absolute_path));
        rename(&self.absolute_path, &new_absolute_path)?;
        self.absolute_path = new_absolute_path.clone();
        self.vault_store.set_absolute_path(new_absolute_path);


        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let vault_parent_dir = self.absolute_path.parent().unwrap();
        let new_absolute_path = get_absolute_path(&vault_parent_dir.to_path_buf(), &new_name);
        
        assert!(Vault::is_jot_vault(&new_absolute_path));
        rename(&self.absolute_path, &new_absolute_path)?;
        self.absolute_path = new_absolute_path.clone();
        self.vault_store.set_absolute_path(new_absolute_path);

        Ok(())
    }

    fn delete(&self) -> JotResult<()> {
        // TODO: make sure the user is prompted before executing 
        // NOTE: this could potentially delete a lot of information! 
        remove_dir_all(&self.absolute_path)?;

        Ok(())
    }

    fn generate_abs_path(parent_dir: &PathBuf, vault_name: &String) -> PathBuf {
        join_paths(vec![
            parent_dir.to_str().unwrap(),
            vault_name,
        ])
    }
} 

impl Vault {
    /**
     * Creates a new new at the given location.
     */
    pub fn create(absolute_path: PathBuf) -> JotResult<Self> {
        let new_vault = Vault {
            absolute_path: absolute_path.clone(),
            name: absolute_path.file_name().unwrap().to_str().unwrap().to_string(),
            folders: vec![],
            notes: vec![],
            vault_store: VaultStore::create_from_path(absolute_path),
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
    pub fn load(absolute_path: PathBuf) -> JotResult<Self> {
        assert!(absolute_path.is_dir());
        let mut new_vault = Vault {
            absolute_path: absolute_path.clone(),
            name: absolute_path.file_name().unwrap().to_str().unwrap().to_string(),
            folders: vec![],
            notes: vec![],
            vault_store: VaultStore::load_path(
                join_paths(vec![
                    absolute_path.to_str().unwrap(),
                    ".jot/data",
                ])
            ),
        };

        new_vault.load_contents()?;

        Ok(new_vault)
    }
    /**
     * Loads the contents of a folder into notes and folders vectors.
     * Note: Folders inside of `self` are also loaded.
     */
    pub fn load_contents(&mut self) -> JotResult<()> {
        for item in self.absolute_path.read_dir().unwrap() {
            let item_location = item.unwrap().path();

            if Folder::is_jot_folder(&item_location) {
                println!("Loading: {:?}", item_location);
                let folder = Folder::load(item_location)?;
                self.folders.push(folder);
            } else if Note::is_jot_note(&item_location) {
                println!("Found note: {:?}", item_location);
                let note = Note::new(item_location)?;
                self.notes.push(note);
            }

        }

        Ok(())
    }

    /**
     * Check if a given absolute path is a valid `jot` [[Vault]]
     */
    pub fn is_jot_vault(absolute_path: &PathBuf) -> bool {
        // TOOD: add check to ensure that this vault 
        // is not inside of another vault
        absolute_path.is_dir() && absolute_path.file_name().unwrap() != ".jot"
    }

    /**
     * Retrieve the path to the vault's persisted data store.
     */
    pub fn get_data_path(&self) -> PathBuf {
        join_paths(vec![
            self.absolute_path.to_str().unwrap(),
            ".jot/data",
        ])
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
        let location_str = self.location.clone().unwrap(); 
        join_paths(vec![
            location_str.to_str().unwrap(),
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

    /**
     * Updates the absolute path to the vault.
     */
    pub fn set_absolute_path(&mut self, vault_path: PathBuf) {
        self.location = Some(vault_path);
        self.store();
    }
}

