use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, remove_dir_all, rename};
use std::path::PathBuf;

use crate::enums::VaultItem;
use crate::items::{Collection, Error, Folder, Item, Note};
use crate::output::error::{Error::*, JotResult};
use crate::traits::FileIO;
use crate::utils::{get_absolute_path, join_paths};

#[derive(Debug, Clone)]
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

impl Collection for Vault {
    fn get_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    fn get_folders(&self) -> Vec<Folder> {
        self.folders.clone()
    }
}

impl Item for Vault {
    fn get_location(&self) -> &PathBuf {
        &self.absolute_path
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn relocate(&mut self, new_absolute_path: PathBuf) -> JotResult<()> {
        assert!(Vault::is_valid_path(&new_absolute_path));
        rename(&self.absolute_path, &new_absolute_path)?;
        self.absolute_path = new_absolute_path.clone();
        self.vault_store.set_absolute_path(new_absolute_path);

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let vault_parent_dir = self.absolute_path.parent().unwrap();
        let new_absolute_path = get_absolute_path(&vault_parent_dir.to_path_buf(), &new_name);

        assert!(Vault::is_valid_path(&new_absolute_path));
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
        join_paths(vec![parent_dir.to_str().unwrap(), vault_name])
    }

    /**
     * Creates a new new at the given location.
     */
    fn create(absolute_path: PathBuf) -> JotResult<Self> {
        if absolute_path.exists() {
            return Err(anyhow!(
                "{}",
                VaultAlreadyExists("Vault already exists".to_string())
            ));
        }

        create_dir_all(&absolute_path)?;
        let new_vault = Vault {
            absolute_path: absolute_path.clone(),
            name: absolute_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
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
    fn load(absolute_path: PathBuf) -> JotResult<Self> {
        assert!(absolute_path.is_dir());
        let mut new_vault = Vault {
            absolute_path: absolute_path.clone(),
            name: absolute_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            folders: vec![],
            notes: vec![],
            vault_store: VaultStore::load_path(join_paths(vec![
                absolute_path.to_str().unwrap(),
                ".jot/data",
            ])),
        };

        new_vault.load_contents()?;

        Ok(new_vault)
    }

    /**
     * Check if a given absolute path is a valid `jot` [[Vault]]
     */
    fn is_valid_path(absolute_path: &PathBuf) -> bool {
        // TOOD: add check to ensure that this vault
        // is not inside of another vault
        absolute_path.is_dir() && absolute_path.file_name().unwrap() != ".jot"
    }
}

impl Vault {
    /**
     * Loads the contents of a folder into notes and folders vectors.
     * Note: Folders inside of `self` are also loaded.
     */
    pub fn load_contents(&mut self) -> JotResult<()> {
        for item in self.absolute_path.read_dir().unwrap() {
            let item_location = item.unwrap().path();

            if Folder::is_valid_path(&item_location) {
                let folder = Folder::load(item_location)?;
                self.folders.push(folder);
            } else if Note::is_valid_path(&item_location) {
                let note = Note::load(item_location)?;
                self.notes.push(note);
            }
        }

        Ok(())
    }

    /**
     * Retrieve the path to the vault's persisted data store.
     */
    pub fn get_data_path(&self) -> PathBuf {
        join_paths(vec![self.absolute_path.to_str().unwrap(), ".jot/data"])
    }
}

impl Vault {
    pub fn create_vault_item(item: VaultItem, name: &String) {}

    pub fn create_and_open_note() {}

    pub fn remove_alias_from_note() {}

    pub fn set_alias() {}

    pub fn change_folder() {}

    pub fn rename_vault_item() {}

    pub fn remove_vault_item() {}

    pub fn move_vault_item() {}

    pub fn list(&self) {
        println!("{}", self.get_name());

        for folder in self.get_folders_sorted() {
            folder.list_with_buffer("".to_string());
        }

        for note in self.get_notes_sorted() {
            println!("├── {}", note.get_name());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultStore {
    /// path to the current active folder inside of the vault
    current_folder: Option<String>,
    /// absolute path to the vault store (in `.jot`, relative to [[Vault]])
    /// Option<T> type because [[FileIO]] has [[Default]] trait bound
    location: Option<PathBuf>,
    /// aliases for notes inside of the vault
    aliases: HashMap<String, String>,
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
        self.location.clone().unwrap()
    }
}

impl VaultStore {
    /**
     * Creates a [[VaultStore]] from the absolute path
     * of the folder it will stored inside.
     */
    pub fn create_from_path(parent_directory: PathBuf) -> Self {
        let location = join_paths(vec![parent_directory.to_str().unwrap(), ".jot/data"]);

        let mut vault_store: VaultStore = FileIO::load_path(location.clone());
        vault_store.location = Some(location);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn create_vault_test() {
        run_test(|| {
            let new_vault_path = test_path("new_vault");
            Vault::create(new_vault_path.clone()).unwrap();

            sleep();

            assert!(new_vault_path.exists() && new_vault_path.is_dir());
        });
    }

    #[test]
    fn cannot_create_duplicate_vaults() {
        run_test(|| {
            let vault_1 = test_path("vault_1");
            let vault_2 = test_path("vault_1");

            Vault::create(vault_1.clone()).unwrap();

            sleep();

            assert!(vault_1.exists() && vault_1.is_dir());
            match Vault::create(vault_2) {
                Ok(_) => assert!(false), // should never happen
                Err(_) => (),
            }
        });
    }
}
