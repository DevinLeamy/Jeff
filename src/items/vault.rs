use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{remove_dir_all, rename};
use std::path::PathBuf;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Vault {
    /// absolute path of the vault
    path: JotPath,
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
    fn get_location(&self) -> &JotPath {
        &self.path
    }

    fn relocate(&mut self, new_absolute_path: PathBuf) -> JotResult<()> {
        assert!(Vault::is_valid_path(&new_absolute_path));
        rename(&self.path.as_path(), &new_absolute_path)?;
        self.path = new_absolute_path.to_owned().into();
        self.vault_store.set_absolute_path(new_absolute_path);

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let vault_parent_dir = self.path.parent();
        let new_absolute_path = get_absolute_path(&vault_parent_dir.to_path_buf(), &new_name);

        assert!(Vault::is_valid_path(&new_absolute_path));
        rename(&self.path.as_path(), &new_absolute_path)?;
        self.path = new_absolute_path.to_owned().into();
        self.vault_store.set_absolute_path(new_absolute_path);

        Ok(())
    }

    fn delete(&self) -> JotResult<()> {
        // TODO: make sure the user is prompted before executing
        // NOTE: this could potentially delete a lot of information!
        remove_dir_all(&self.path.as_path())?;

        Ok(())
    }

    fn generate_abs_path(parent_dir: &PathBuf, vault_name: &String) -> PathBuf {
        join_paths(vec![parent_dir.to_str().unwrap(), vault_name])
    }

    /**
     * Creates a new new at the given location.
     */
    fn create(absolute_path: PathBuf) -> JotResult<Self> {
        let path: JotPath = absolute_path.to_owned().into();
        if path.exists() {
            return Err(anyhow!(VaultAlreadyExists(
                "Vault already exists".to_string()
            )));
        }

        std::fs::create_dir(&absolute_path.as_path())?;

        let store_path = JotPath::from_parent(&path, ".jot/data".to_string()).to_path_buf();
        let mut new_store = VaultStore::load_path(store_path.clone());
        new_store.set_absolute_path(store_path);

        let new_vault = Vault {
            path: path.to_owned(),
            folders: vec![],
            notes: vec![],
            vault_store: new_store,
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
        let path: JotPath = absolute_path.into();

        let mut new_vault = Vault {
            path: path.to_owned(),
            folders: vec![],
            notes: vec![],
            vault_store: VaultStore::load_path(
                JotPath::from_parent(&path, ".jot/data".to_string()).to_path_buf(),
            ),
        };

        new_vault.load_contents()?;

        Ok(new_vault)
    }

    /**
     * Check if a given absolute path is a valid `jot` [Vault]
     */
    fn is_valid_path(absolute_path: &PathBuf) -> bool {
        // TOOD: add check to ensure that this vault
        // is not inside of another vault
        !absolute_path.is_file() && absolute_path.file_name().unwrap() != ".jot"
    }
}

impl Vault {
    pub fn get_note_from_active_folder(&self, name: &String) -> JotResult<Note> {
        if let Ok(Some(active_folder)) = self.get_active_folder() {
            Ok(active_folder.get_note_with_name(name)?)
        } else {
            // If there is no active folder, search the vault itself.
            Ok(self.get_note_with_name(name)?)
        }
    }

    pub fn get_active_folder(&self) -> JotResult<Option<Folder>> {
        if let Some(active_folder_path) = self.get_active_folder_path() {
            /*
             * We use active_folder_path (the relative path to the active folder) as
             * the name of the folder here in get_folder_with_name because the relative
             * path is just the name of the folder.
             *
             * This can, and should, be improved.
             */
            let active_folder = self.get_folder_with_name(&active_folder_path)?;
            Ok(Some(active_folder))
        } else {
            Ok(None)
        }
    }

    /// Returns the path absolute path to the folder inside of the vault that
    /// is currently "active". If no folder has been "cd"ed into, than the
    /// absolute path to the vault is returned.
    pub fn get_active_location(&self) -> JotPath {
        let active_folder = self.vault_store.get_folder_path();

        if active_folder.is_none() {
            return self.get_location().to_owned();
        }

        JotPath::from_parent(&self.get_location(), active_folder.unwrap())
    }
    /**
     * Loads the contents of a folder into notes and folders vectors.
     * Note: Folders inside of `self` are also loaded.
     */
    pub fn load_contents(&mut self) -> JotResult<()> {
        for item in self.path.read_dir().unwrap() {
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

    pub fn change_folder(&mut self, path: &PathBuf) -> JotResult<()> {
        let vault_path = self.get_location();
        let maybe_folder_path = self.vault_store.get_folder_path();
        let new_location = if let Some(folder_path) = maybe_folder_path {
            process_path(&join_paths(vec![
                vault_path.as_path(),
                &PathBuf::from(folder_path),
                path.as_path(),
            ]))
        } else {
            process_path(&join_paths(vec![vault_path.as_path(), path]))
        };

        if !new_location.exists() {
            return Err(anyhow!(Error::PathNotFound));
        }

        if !new_location.starts_with(&vault_path.as_path()) {
            return Err(anyhow!(Error::OutOfBounds));
        }

        let mut destination_folder = new_location.strip_prefix(vault_path.as_path()).unwrap();
        if destination_folder.has_root() {
            destination_folder = destination_folder.strip_prefix("/").unwrap();
        }
        let destination_folder = destination_folder.to_path_buf();

        self.vault_store
            .set_folder_path(Some(path_to_string(destination_folder)));

        Ok(())
    }

    fn get_active_folder_path(&self) -> Option<String> {
        self.vault_store.get_folder_path()
    }
}

impl Vault {
    // pub fn remove_alias_from_note() {}
    // pub fn set_alias() {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultStore {
    /// relative path from this vault to the active folder
    current_folder: Option<String>,
    /// absolute path to the vault store data (some `<vault-path>/.jot/data`)
    /// Option<T> type because [FileIO] has [Default] trait bound
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
     * Updates the absolute path to the vault.
     */
    pub fn set_absolute_path(&mut self, vault_path: PathBuf) {
        self.location = Some(vault_path);
        self.store();
    }

    pub fn set_folder_path(&mut self, folder_path: Option<String>) {
        self.current_folder = folder_path;
        self.store();
    }

    pub fn get_folder_path(&self) -> Option<String> {
        self.current_folder.clone()
    }
}
