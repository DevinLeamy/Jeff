use crate::prelude::*;
use clap::Parser;
use anyhow::anyhow;

pub struct App {
    args: Args,
    config: Config,
    vaults: Vaults,
    editor: Editor
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        let editor_data = config.get_editor_data();
        App {
            args: Args::parse(),
            config: config,
            vaults: Vaults::load(),
            editor: Editor::from_config(editor_data),
        }
    }

    pub fn handle_args(&mut self) -> JotResult<Message> {
        match &self.args.command {
            Command::Vault { show_loc, name, location, } => {
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
                    return Err(anyhow!("Note with name [{}] already exists", note.get_name()));
                }

                let note_path = Note::generate_abs_path(vault.get_location(), name);

                Note::create(note_path)?;

                return Ok(Message::ItemCreated(ItemType::Nt, name.to_owned()));
            }
            Command::Today { create_if_dne } => {
                todo!()
                // let daily_note_name = daily_note_name(); 
                // let vault = self.vaults.mut_current()?;

                // /*
                //  * Edit the daily note. If -c is supplied, create the 
                //  * daily note if it doesn't exist. 
                //  */
                // if *create_if_dne {
                //     vault.create_and_open_note(&daily_note_name, self.config.get_editor_data())?;
                // } else {
                //     vault.open_note(&daily_note_name, self.config.get_editor_data())?;
                // }

                // return Ok(Message::Empty);
            }
            Command::Alias { name, maybe_alias, remove_alias } => {
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
                todo!()
                // self.vaults
                //     .ref_current()?
                //     .create_vault_item(VaultItem::Fd, name)?;
                // return Ok(Message::ItemCreated(Item::Fd, name.to_owned()));
            }
            Command::Chdir { path } => {
                todo!()
                // self.vaults.mut_current()?.change_folder(path)?;
                // return Ok(Message::FolderChanged);
            }
            Command::Remove { item_type, name } => {
                todo!()
                // match item_type {
                //     Item::Vl | Item::Vault => self.vaults.remove_vault(name)?,
                //     _ => self
                //         .vaults
                //         .ref_current()?
                //         .remove_vault_item(item_type.to_vault_item(), name)?,
                // };
                // return Ok(Message::ItemRemoved(item_type.to_owned(), name.to_owned()));
            }
            Command::Rename {
                item_type,
                name,
                new_name,
            } => {
                todo!()
                // match item_type {
                //     Item::Vl | Item::Vault => self.vaults.rename_vault(name, new_name)?,
                //     _ => self.vaults.ref_current()?.rename_vault_item(
                //         item_type.to_vault_item(),
                //         name,
                //         new_name,
                //     )?,
                // };
                // return Ok(Message::ItemRenamed(
                //     item_type.to_owned(),
                //     name.to_owned(),
                //     new_name.to_owned(),
                // ));
            }
            Command::Move {
                item_type,
                name,
                new_location,
            } => {
                todo!()
                // match item_type {
                //     Item::Vl | Item::Vault => self.vaults.move_vault(name, new_location)?,
                //     _ => self.vaults.ref_current()?.move_vault_item(
                //         item_type.to_vault_item(),
                //         name,
                //         new_location,
                //     )?,
                // };
                // return Ok(Message::ItemMoved(item_type.to_owned(), name.to_owned()));
            }
            Command::Vmove {
                item_type,
                name,
                vault_name,
            } => {
                self.vaults.move_to_vault(item_type, name, vault_name)?;
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

#[test]
fn open_note_test() {
    run_test(|| {

    });
}

#[test]
fn create_note_test() {
    run_test(|| {

    })
}
