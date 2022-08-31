// temporary parking for these types

use crate::{traits::FileIO, utils::join_paths};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum Item {
    Vl,
    Nt,
    Dr,
}

impl Item {
    pub fn to_vault_item(&self) -> VaultItem {
        match self {
            Item::Vl => VaultItem::Dr,
            Item::Nt => VaultItem::Nt,
            Item::Dr => VaultItem::Dr,
        }
    }

    pub fn full(&self) -> &str {
        match self {
            Item::Vl => "vault",
            Item::Nt => "note",
            Item::Dr => "folder",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VaultItem {
    Nt,
    Dr,
}

impl VaultItem {
    pub fn to_item(&self) -> Item {
        match self {
            VaultItem::Nt => Item::Nt,
            VaultItem::Dr => Item::Dr,
        }
    }

    pub fn full(&self) -> &str {
        match self {
            VaultItem::Nt => "note",
            VaultItem::Dr => "folder",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    name: Option<String>,
    location: Option<PathBuf>,
    folder: PathBuf,
    history: Vec<(String, PathBuf)>,
}

impl Default for Vault {
    fn default() -> Self {
        Vault {
            name: None,
            location: None,
            folder: PathBuf::new(),
            history: vec![],
        }
    }
}

impl FileIO for Vault {
    fn path(&self) -> PathBuf {
        join_paths(vec![
            self.get_location().to_str().unwrap(),
            self.get_name(),
            ".jot/data",
        ])
    }
}

impl Vault {
    pub fn get_name(&self) -> &String {
        self.name.as_ref().unwrap()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
        if self.location.is_some() {
            self.store()
        }
    }

    pub fn get_location(&self) -> &PathBuf {
        self.location.as_ref().unwrap()
    }

    pub fn set_location(&mut self, location: PathBuf) {
        self.location = Some(location);
        if self.name.is_some() {
            self.store()
        }
    }

    pub fn get_folder(&self) -> &PathBuf {
        &self.folder
    }

    pub fn set_folder(&mut self, folder: PathBuf) {
        self.folder = folder;
        self.store()
    }

    pub fn get_path_data(&self) -> (&String, &PathBuf, &PathBuf) {
        (self.get_name(), self.get_location(), self.get_folder())
    }
}
