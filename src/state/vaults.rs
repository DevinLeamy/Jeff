use crate::prelude::*;
use anyhow::anyhow;
use data::Data;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Vaults {
    current: Option<Vault>,
    data: Data,
}

impl Vaults {
    pub fn load() -> JotResult<Self> {
        let mut vaults = Vaults {
            current: None,
            data: Data::load(),
        };
        vaults.load_current_vault()?;
        Ok(vaults)
    }

    pub fn get_vault_path(&self, name: &String) -> JotResult<PathBuf> {
        if let Some(vault_parent_dir) = self.data.get_vault_location(name) {
            Ok(get_absolute_path(vault_parent_dir, name))
        } else {
            Err(anyhow!("Invalid vault path"))
        }
    }

    pub fn get_vault(&self, name: &String) -> JotResult<Vault> {
        let vault_abs_path = self.get_vault_path(name)?;
        let vault = Vault::load(vault_abs_path)?;
        Ok(vault)
    }

    fn load_current_vault(&mut self) -> JotResult<()> {
        self.current = if let Some(current_vault_name) = self.data.get_current_vault() {
            let vault_absolute_path = self.get_vault_path(current_vault_name)?;
            let current_vault = Vault::load(vault_absolute_path)?;
            Some(current_vault)
        } else {
            None
        };

        Ok(())
    }

    pub fn list_vaults(&self, show_loc: bool) {
        for vault_name in self.data.get_vaults().keys() {
            if self.is_current_vault(vault_name) {
                print!("👉 \x1b[0;34m{}\x1b[0m", vault_name)
            } else {
                print!("   {}", vault_name)
            }

            if show_loc {
                println!(
                    " \t {}",
                    self.data.get_vault_location(vault_name).unwrap().display()
                )
            } else {
                println!();
            }
        }
    }

    pub fn show_vault_location(&self, vault_name: String) {
        if let Some(vault_location) = self.data.get_vault_location(vault_name.as_str()) {
            if self.is_current_vault(&vault_name) {
                println!(
                    "👉 \x1b[0;34m{}\x1b[0m \t {}",
                    vault_name,
                    vault_location.display()
                );
            } else {
                println!("{} \t {}", vault_name, vault_location.display());
            }
        }
    }

    fn is_current_vault(&self, vault_name: &String) -> bool {
        let current_vault_name = self.data.get_current_vault();

        current_vault_name.is_some() && vault_name == current_vault_name.unwrap()
    }

    pub fn ref_current(&self) -> JotResult<&Vault> {
        if self.current.is_none() {
            return Err(anyhow!("{}", Error::NotInsideVault));
        }

        Ok(self.current.as_ref().unwrap())
    }

    pub fn mut_current(&mut self) -> JotResult<&mut Vault> {
        if self.current.is_none() {
            return Err(anyhow!("{}", Error::NotInsideVault));
        }

        Ok(self.current.as_mut().unwrap())
    }

    pub fn create_vault(&mut self, name: &str, location: &Path) -> JotResult<()> {
        println!("{}", name);
        if self.data.vault_exists(name) {
            return Err(anyhow!(Error::VaultAlreadyExists(name.to_owned())));
        }

        let location = process_path(location);
        let absolute_path = Vault::generate_abs_path(&location.to_path_buf(), &name.to_string());

        Vault::create(absolute_path)?;

        self.data.add_vault(name.to_owned(), location);

        Ok(())
    }

    pub fn remove_vault(&mut self, name: &str) -> JotResult<()> {
        let maybe_vault_location = self.data.get_vault_location(name);
        if maybe_vault_location.is_none() {
            return Err(anyhow!(Error::VaultNotFound(name.to_owned())));
        }

        let vault_location = maybe_vault_location.unwrap();
        let vault_to_remove = Vault::load(vault_location.to_path_buf())?;

        self.data.remove_vault(name);
        vault_to_remove.delete()?;

        if self.data.get_current_vault() == Some(&vault_to_remove.get_name()) {
            self.data.set_current_vault(None);
        }

        Ok(())
    }

    pub fn rename_vault(&mut self, name: &str, new_name: &str) -> JotResult<()> {
        if self.data.vault_exists(new_name) {
            return Err(anyhow!(Error::VaultAlreadyExists(new_name.to_owned())));
        } else if !self.data.vault_exists(name) {
            return Err(anyhow!(Error::VaultNotFound(name.to_owned())));
        }

        let vault_parent_dir = self.data.get_vault_location(name).unwrap();
        let vault_absolute_path = Vault::generate_abs_path(&vault_parent_dir, &name.to_string());
        let mut vault = Vault::load(vault_absolute_path)?;

        vault.rename(new_name.to_owned())?;
        self.data.rename_vault(name, new_name.to_owned());

        if self.data.get_current_vault() == Some(&name.to_string()) {
            self.data.set_current_vault(Some(new_name.to_string()));
        }

        Ok(())
    }

    pub fn move_vault(&mut self, name: &str, new_location: &Path) -> JotResult<()> {
        if !self.data.vault_exists(name) {
            return Err(anyhow!(Error::VaultNotFound(name.to_owned())));
        }

        let vault_parent_dir = self.data.get_vault_location(name).unwrap();
        let vault_absolute_path = Vault::generate_abs_path(&vault_parent_dir, &name.to_string());
        let mut vault = Vault::load(vault_absolute_path)?;

        let new_absolute_path = process_path(&join_paths(vec![
            new_location.to_str().unwrap().to_string(),
            vault.get_name(),
        ]));

        vault.relocate(new_absolute_path.to_owned())?;

        self.data
            .set_vault_location(name, new_location.to_path_buf());

        Ok(())
    }

    pub fn enter_vault(&mut self, name: &str) -> JotResult<()> {
        if !self.data.vault_exists(name) {
            return Err(anyhow!(Error::VaultNotFound(name.to_owned())));
        }

        if let Some(current_vault_name) = self.data.get_current_vault() {
            if name == current_vault_name {
                return Err(anyhow!(Error::AlreadyInVault(name.to_owned())));
            }
        }

        self.current = Some(self.get_vault(&name.to_owned())?);
        self.data.set_current_vault(Some(name.to_owned()));

        Ok(())
    }
}
