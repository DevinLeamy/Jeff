pub use crate::types::Vault as CurrentVault;
use crate::{
    types::VaultItem,
    utils::{create_item, join_paths, move_item, process_path, remove_item, rename_item},
};
use std::{path::PathBuf, process::Command};
use walkdir::WalkDir;

impl CurrentVault {
    pub fn list(&self) {
        let location = self.generate_location();

        for entry in WalkDir::new(location).into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
        }
    }

    pub fn create_vault_item(&self, item_type: VaultItem, name: &String) {
        let location = self.generate_location();

        create_item(item_type.to_item(), name, &location);
        print!("{} {} created", item_type.full(), name)
    }

    pub fn remove_vault_item(&self, item_type: VaultItem, name: &String) {
        let location = self.generate_location();

        remove_item(item_type.to_item(), name, &location);
        print!("{} {} removed", item_type.full(), name)
    }

    pub fn rename_vault_item(&self, item_type: VaultItem, name: &String, new_name: &String) {
        let location = self.generate_location();

        rename_item(item_type.to_item(), name, new_name, &location);
        print!("{} {} renamed to {}", item_type.full(), name, new_name)
    }

    pub fn move_vault_item(&self, item_type: VaultItem, name: &String, new_location: &PathBuf) {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let original_location = join_paths(vec![&vault_path, self.get_folder()]);

        let new_location = join_paths(vec![&original_location, new_location]);
        let new_location = process_path(&new_location);

        let vault_path = vault_path.to_str().unwrap();

        if !new_location.to_str().unwrap().contains(vault_path) {
            panic!("path crosses the bounds of vault")
        }

        move_item(item_type.to_item(), name, &original_location, &new_location);

        print!("{} {} moved", item_type.full(), name)
    }

    pub fn vmove_vault_item(
        &self,
        item_type: &VaultItem,
        name: &String,
        vault_name: &String,
        vault_location: &PathBuf,
    ) {
        let original_location = self.generate_location();

        if vault_name == self.get_name() {
            panic!(
                "{} {} already exists in vault {}",
                item_type.full(),
                name,
                vault_name
            )
        }

        let new_location = join_paths(vec![vault_location.to_str().unwrap(), vault_name]);
        move_item(item_type.to_item(), name, &original_location, &new_location);

        print!(
            "{} {} moved to vault {}",
            item_type.full(),
            name,
            vault_name
        )
    }

    pub fn open_note(&self, name: &String, editor_data: (&String, bool)) {
        let location = self.generate_location();
        let mut path = join_paths(vec![location.to_str().unwrap(), name]);
        path.set_extension("md");

        if !path.exists() {
            panic!("note {} doesn't exist", name)
        }

        let (editor, conflict) = editor_data;

        let mut cmd = Command::new(editor)
            .arg(path.to_str().unwrap())
            .spawn()
            .unwrap();

        if conflict {
            cmd.wait().unwrap();
        }
    }

    pub fn change_folder(&mut self, path: &PathBuf) {
        let vault_path = join_paths(vec![self.get_location().to_str().unwrap(), self.get_name()]);
        let new_location = join_paths(vec![&vault_path, self.get_folder(), path]);

        if !new_location.exists() {
            panic!("path doesn't exist")
        }

        let new_location = process_path(&new_location);
        let new_location = new_location.to_str().unwrap();
        let vault_path = vault_path.to_str().unwrap();

        if !new_location.contains(vault_path) {
            panic!("path crosses the bounds of vault")
        }

        let mut destination_folder = new_location.replace(vault_path, "");
        if destination_folder.starts_with(r"\") || destination_folder.starts_with("/") {
            destination_folder = destination_folder[1..].to_string();
        }
        let destination_folder = PathBuf::from(destination_folder);

        self.set_folder(destination_folder);
        print!("changed folder");
    }

    fn generate_location(&self) -> PathBuf {
        let (current_vault_name, current_vault_location, folder) = self.get_path_data();
        join_paths(vec![
            current_vault_location,
            &PathBuf::from(current_vault_name),
            folder,
        ])
    }
}