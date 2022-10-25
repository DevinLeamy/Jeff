use std::path::PathBuf;

use crate::prelude::*;
use anyhow::anyhow;

pub struct App {
    config: Config,
    vaults: Vaults,
    editor: Editor,
}

impl App {
    pub fn new() -> JotResult<Self> {
        let config = Config::load();
        let editor_data = config.get_editor_data();
        Ok(App {
            config,
            vaults: Vaults::load()?,
            editor: Editor::from_config(editor_data),
        })
    }

    pub fn get_vaults(&self) -> &Vaults {
        &self.vaults
    }

    pub fn handle_command(&mut self, command: Command) -> JotResult<Message> {
        match &command {
            Command::Vault {
                show_loc,
                name,
                location,
            } => {
                if let (Some(name), Some(location)) = (name, location) {
                    self.vaults.create_vault(name, location)?;
                    return Ok(Message::ItemCreated(ItemType::Vl, name.to_owned()));
                } else if name.is_some() && *show_loc {
                    let name = name.clone().unwrap();
                    self.vaults.show_vault_location(name);
                    return Ok(Message::Empty);
                } else {
                    self.vaults.list_vaults(show_loc);
                    return Ok(Message::Empty);
                }
            }
            Command::Enter { name } => {
                self.vaults.enter_vault(name)?;
                return Ok(Message::VaultEntered(name.to_owned()));
            }
            Command::Note { name } => {
                let vault = self.vaults.ref_current()?;
                let maybe_note = vault.get_note_with_name(name);
                if let Ok(note) = maybe_note {
                    return Err(anyhow!(
                        "Note with name [{}] already exists",
                        note.get_name()
                    ));
                }

                let note_path = Note::generate_abs_path(vault.get_location(), name);

                Note::create(note_path)?;

                return Ok(Message::ItemCreated(ItemType::Nt, name.to_owned()));
            }
            Command::Today { create_if_dne } => {
                let daily_note_name = daily_note_name();
                let vault = self.vaults.ref_current()?;
                let maybe_note =
                    vault.get_note_with_name(&format!("{}.md", daily_note_name).to_string());

                if maybe_note.is_err() && !*create_if_dne {
                    return Err(anyhow!(
                        "Daily note does not exist, consider supplying the --create flag"
                    ));
                }

                /*
                 * Edit the daily note. If --create is supplied, create and edit the
                 * daily note.
                 */
                let message;
                let note = if *create_if_dne {
                    let note_path = Note::generate_abs_path(vault.get_location(), &daily_note_name);
                    message = Message::ItemCreated(ItemType::Nt, daily_note_name);
                    Note::create(note_path)?
                } else {
                    message = Message::Empty;
                    maybe_note.unwrap()
                };

                self.editor.open_note(note)?;

                Ok(message)
            }
            Command::Alias {
                name,
                maybe_alias,
                remove_alias,
            } => {
                todo!()
                // if *remove_alias {
                //     let alias_removed = self.vaults
                //         .mut_current()?
                //         .remove_alias_from_note(name.to_string())?;

                //     return Ok(Message::NoteAliasRemoved(name.to_string(), alias_removed))
                // } else if let Some(alias) = maybe_alias {
                //     self.vaults
                //         .mut_current()?
                //         .set_alias(name.to_string(), alias.to_string())?;
                //     return Ok(Message::NoteAliasCreated(name.to_string(), alias.to_string()))
                // }

                // return Ok(Message::Empty);
            }
            Command::Open { name } => {
                let note = self.vaults.ref_current()?.get_note_with_name(name)?;
                self.editor.open_note(note)?;

                return Ok(Message::Empty);
            }
            Command::Folder { name } => {
                let vault = self.vaults.ref_current()?;

                let maybe_folder = vault.get_folder_with_name(name);
                if let Ok(folder) = maybe_folder {
                    return Err(anyhow!(
                        "Folder with name [{}] already exists",
                        folder.get_name()
                    ));
                }

                let folder_path = Folder::generate_abs_path(vault.get_location(), name);

                Folder::create(folder_path)?;

                return Ok(Message::ItemCreated(ItemType::Fd, name.to_owned()));
            }
            Command::Chdir { path } => {
                let vault = self.vaults.mut_current()?;
                vault.change_folder(path)?;

                Ok(Message::FolderChanged)
            }
            Command::Remove { item_type, name } => {
                match item_type {
                    ItemType::Fd | ItemType::Folder => {
                        let vault = self.vaults.ref_current()?;
                        let folder = vault.get_folder_with_name(name)?;
                        folder.delete()?;
                    }
                    ItemType::Nt | ItemType::Note => {
                        let vault = self.vaults.ref_current()?;
                        let note = vault.get_note_with_name(name)?;
                        note.delete()?;
                    }
                    ItemType::Vl | ItemType::Vault => {
                        self.vaults.remove_vault(name)?;
                        return Ok(Message::Empty);
                    }
                };

                return Ok(Message::ItemRemoved(item_type.to_owned(), name.to_owned()));
            }
            Command::Rename {
                item_type,
                name,
                new_name,
            } => {
                match item_type {
                    ItemType::Fd | ItemType::Folder => {
                        let vault = self.vaults.ref_current()?;
                        let mut folder = vault.get_folder_with_name(name)?;

                        folder.rename(new_name.to_owned())?;
                    }
                    ItemType::Nt | ItemType::Note => {
                        let vault = self.vaults.ref_current()?;
                        let mut note = vault.get_note_with_name(name)?;

                        note.rename(new_name.to_owned())?;
                    }
                    ItemType::Vl | ItemType::Vault => {
                        self.vaults.rename_vault(name, new_name)?;
                    }
                }

                return Ok(Message::ItemRenamed(
                    item_type.to_owned(),
                    name.to_owned(),
                    new_name.to_owned(),
                ));
            }
            Command::Move {
                item_type,
                name,
                new_location,
            } => {
                match item_type {
                    ItemType::Fd | ItemType::Folder => {
                        // new location is relative to the root of the vault
                        let vault = self.vaults.ref_current()?;
                        let mut folder = vault.get_folder_with_name(name)?;
                        let new_absolute_path = process_path(&join_paths(vec![
                            vault.get_location().as_path(),
                            new_location,
                            &PathBuf::from(folder.get_name()),
                        ]));

                        folder.relocate(new_absolute_path.to_owned())?;
                    }
                    ItemType::Nt | ItemType::Note => {
                        // new location is relative to the root of the vault
                        let vault = self.vaults.ref_current()?;
                        let mut note = vault.get_note_with_name(name)?;
                        let new_absolute_path = process_path(&join_paths(vec![
                            vault.get_location().as_path(),
                            new_location,
                            &PathBuf::from(note.get_name()),
                        ]));

                        note.relocate(new_absolute_path.to_owned())?;
                    }
                    ItemType::Vl | ItemType::Vault => {
                        self.vaults.move_vault(name, new_location)?;
                    }
                }

                return Ok(Message::ItemMoved(item_type.to_owned(), name.to_owned()));
            }
            Command::Vmove {
                item_type,
                name,
                vault_name,
            } => {
                let vault = self.vaults.ref_current()?;
                let new_vault = self.vaults.get_vault(vault_name)?;

                match item_type {
                    VaultItemType::Fd | VaultItemType::Folder => {
                        // new location is relative to the root of the vault
                        let mut folder = vault.get_folder_with_name(name)?;
                        let new_absolute_path = process_path(&join_paths(vec![
                            new_vault.get_location().as_path(),
                            &PathBuf::from(folder.get_name()),
                        ]));

                        folder.relocate(new_absolute_path.to_owned())?;
                    }
                    VaultItemType::Nt | VaultItemType::Note => {
                        // new location is relative to the root of the vault
                        let vault = self.vaults.ref_current()?;
                        let mut note = vault.get_note_with_name(name)?;
                        let new_absolute_path = process_path(&join_paths(vec![
                            new_vault.get_location().as_path(),
                            &PathBuf::from(note.get_name()),
                        ]));

                        note.relocate(new_absolute_path.to_owned())?;
                    }
                }

                return Ok(Message::ItemVMoved(
                    item_type.to_owned(),
                    name.to_owned(),
                    vault_name.to_owned(),
                ));
            }
            Command::List => {
                self.vaults.ref_current()?.list();
                return Ok(Message::Empty);
            }
            Command::Config { config_type, value } => {
                if let Some(value) = value {
                    self.config.set_config(config_type, value);
                    return Ok(Message::ConfigSet(config_type.to_owned(), value.to_owned()));
                } else {
                    let value = self.config.get_config(config_type);
                    return Ok(Message::Config(config_type.to_owned(), value));
                }
            }
            _ => Ok(Message::Empty),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_note_test() {
        run_test(|| {
            let (mut app, vault_name) = create_app_and_vault();
            execute_commands(
                &mut app,
                vec![
                    Command::Note {
                        name: "test_note".to_string(),
                    },
                    Command::Open {
                        name: "test_note".to_string(),
                    },
                ],
            );
        })
    }

    #[test]
    fn change_vaults() {
        run_test(|| {
            let (mut app, vault_name) = create_app_and_vault();
            execute_commands(
                &mut app,
                vec![
                    Command::Note {
                        name: "test_note".to_string(),
                    },
                    Command::Open {
                        name: "test_note".to_string(),
                    },
                ],
            );
        })
    }
}
