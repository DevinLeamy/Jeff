use std::path::PathBuf;

use anyhow::anyhow;
use dialoguer::{theme::ColorfulTheme, Confirm};
use skim::prelude::*;

use crate::{enums::ConfigType, prelude::*};

pub struct App {
    config: Config,
    vaults: Vaults,
    editor: Editor,
}

impl App {
    pub fn vault(
        &mut self,
        show_loc: bool,
        name: &Option<String>,
        location: &Option<PathBuf>,
    ) -> JotResult<Message> {
        if let (Some(name), Some(location)) = (name, location) {
            self.vaults.create_vault(name, location)?;
            Ok(Message::ItemCreated(ItemType::Vl, name.to_owned()))
        } else if name.is_some() && show_loc {
            let name = name.clone().unwrap();
            self.vaults.show_vault_location(name);
            Ok(Message::Empty)
        } else {
            self.vaults.list_vaults(show_loc);
            Ok(Message::Empty)
        }
    }

    pub fn enter_vault(&mut self, name: &String) -> JotResult<Message> {
        self.vaults.enter_vault(name)?;
        return Ok(Message::VaultEntered(name.to_owned()));
    }

    pub fn create_note(&mut self, name: &String) -> JotResult<Message> {
        let vault = self.vaults.ref_current()?;
        let maybe_note = vault.get_note_with_name(name);
        if let Ok(note) = maybe_note {
            return Err(anyhow!(
                "Note with name [{}] already exists",
                note.get_name()
            ));
        }

        let note_path = Note::generate_abs_path(&vault.get_active_location(), name);

        Note::create(note_path)?;

        return Ok(Message::ItemCreated(ItemType::Nt, name.to_owned()));
    }

    pub fn today(&mut self, create_if_dne: bool) -> JotResult<Message> {
        let daily_note_name = daily_note_name();
        let vault = self.vaults.ref_current()?;
        let maybe_note = vault.get_note_with_name(&daily_note_name);

        if maybe_note.is_err() && !create_if_dne {
            return Err(anyhow!(
                "Daily note does not exist, consider supplying the --create flag"
            ));
        }

        /*
         * Edit the daily note. If --create is supplied, create and edit the
         * daily note.
         */
        let message;
        let note = if create_if_dne {
            /*
             * We use vault.get_location() rather than vault.get_active_location() here because daily notes
             * are created per-vault, not per-folder. Currently, they are always top-level.
             */
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

    pub fn open_note(&mut self, name: &String) -> JotResult<Message> {
        let vault = self.vaults.ref_current()?;
        let maybe_note = vault.get_note_from_active_folder(name);

        /*
         * If the given name is a valid note, open it. Otherwise, fuzzysearch
         * for a note.
         *
         * TODO: make the fuzzysearch start with input text "name"
         * TODO: make fuzzysearch have colored text
         */
        if let Ok(note) = maybe_note {
            self.editor.open_note(note)?;
            return Ok(Message::Empty);
        }

        let notes = vault.get_notes_sorted();

        // let options = SkimOptionsBuilder::default()
        //     .height(Some("20%"))
        //     .interactive(false)
        //     .expect(Some(name.to_owned()))
        //     .build()
        //     .unwrap();

        // let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
        // for note in notes {
        //     tx_item.send(Arc::new(note.clone()))?;
        // }

        // // stop waiting for more items
        // drop(tx_item);

        // let selected_items = Skim::run_with(&options, Some(rx_item))
        //     .map(|out| out.selected_items)
        //     .unwrap_or_else(Vec::new);

        // let selected_notes = selected_items
        //     .iter()
        //     .map(|selected_item| {
        //         (**selected_item)
        //             .as_any()
        //             .downcast_ref::<Note>()
        //             .unwrap()
        //             .to_owned()
        //     })
        //     .collect::<Vec<Note>>();

        // if selected_notes.len() == 1 {
        //     self.editor.open_note(selected_notes[0].clone())?;
        // }

        return Ok(Message::Empty);
    }

    pub fn create_folder(&mut self, name: &String) -> JotResult<Message> {
        let vault = self.vaults.ref_current()?;

        let maybe_folder = vault.get_folder_with_name(name);
        if let Ok(folder) = maybe_folder {
            return Err(anyhow!(
                "Folder with name [{}] already exists",
                folder.get_name()
            ));
        }

        let folder_path = Folder::generate_abs_path(&vault.get_active_location(), name);

        Folder::create(folder_path)?;

        return Ok(Message::ItemCreated(ItemType::Fd, name.to_owned()));
    }

    pub fn change_directory(&mut self, path: &PathBuf) -> JotResult<Message> {
        let vault = self.vaults.mut_current()?;
        vault.change_folder(path)?;

        Ok(Message::FolderChanged)
    }

    pub fn remove_item(&mut self, item_type: ItemType, name: &String) -> JotResult<Message> {
        // display a dialog to confirm the action
        let remove_item = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Are you sure you want to remove {}?", name))
            .interact()
            .unwrap();

        if !remove_item {
            return Ok(Message::Empty);
        }

        match item_type {
            ItemType::Fd | ItemType::Folder => {
                let vault = self.vaults.ref_current()?;
                let folder = vault.get_folder_with_name(name)?;
                folder.delete()?;
            }
            ItemType::Nt | ItemType::Note => {
                let vault = self.vaults.ref_current()?;
                let note = vault.get_note_from_active_folder(name)?;
                note.delete()?;
            }
            ItemType::Vl | ItemType::Vault => {
                self.vaults.remove_vault(name)?;
            }
        };

        Ok(Message::ItemRemoved(item_type.to_owned(), name.to_owned()))
    }

    pub fn rename_item(
        &mut self,
        item_type: ItemType,
        name: &String,
        new_name: &String,
    ) -> JotResult<Message> {
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

    pub fn list(&self) -> JotResult<Message> {
        let vault = self.vaults.ref_current()?;

        if let Ok(Some(active_folder)) = vault.get_active_folder() {
            println!(
                "{} > {}",
                vault.to_display_string(),
                active_folder.to_display_string()
            );
            active_folder.list();
        } else {
            println!("{}", vault.to_display_string());
            vault.list();
        }

        return Ok(Message::Empty);
    }

    pub fn set_config(
        &mut self,
        config_type: &ConfigType,
        value: &Option<String>,
    ) -> JotResult<Message> {
        if let Some(value) = value {
            self.config.set_config(config_type, value);
            return Ok(Message::ConfigSet(config_type.to_owned(), value.to_owned()));
        } else {
            let value = self.config.get_config(config_type);
            return Ok(Message::Config(config_type.to_owned(), value));
        }
    }

    pub fn move_item(
        &mut self,
        item_type: ItemType,
        name: &String,
        new_location: &PathBuf,
    ) -> JotResult<Message> {
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
                /*
                 * New location is relative to current location
                 * TODO: currently note needs to be the name of the note. Ideally,
                 * we will make it the relative path to the note.
                 */
                let vault = self.vaults.ref_current()?;
                let mut note = vault.get_note_from_active_folder(name)?;
                let new_absolute_path = process_path(&join_paths(vec![
                    vault.get_active_location().as_path(),
                    new_location,
                    &PathBuf::from(note.get_full_name()),
                ]));

                note.relocate(new_absolute_path.to_owned())?;
            }
            ItemType::Vl | ItemType::Vault => {
                self.vaults.move_vault(name, new_location)?;
            }
        }

        return Ok(Message::ItemMoved(item_type.to_owned(), name.to_owned()));
    }

    pub fn move_item_to_new_vault(
        &mut self,
        item_type: VaultItemType,
        name: &String,
        vault_name: &String,
    ) -> JotResult<Message> {
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
                    &PathBuf::from(note.get_full_name()),
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

    pub fn handle_command(&mut self, command: Command) -> JotResult<Message> {
        #[rustfmt::skip]
        match &command {
            Command::Vault { show_loc, name, location, } => self.vault(*show_loc, name, location),
            Command::Enter { name } => self.enter_vault(name),
            Command::Note { name } => self.create_note(name),
            Command::Today { create_if_dne } => self.today(*create_if_dne),
            // Command::Alias { name, maybe_alias, remove_alias, } => { todo!() }
            Command::Open { name } => self.open_note(name),
            Command::Folder { name } => self.create_folder(name),
            Command::Chdir { path } => self.change_directory(path),
            Command::Remove { item_type, name } => self.remove_item(*item_type, name),
            Command::Rename { item_type, name, new_name, } => self.rename_item(*item_type, name, new_name),
            Command::Move { item_type, name, new_location, } => self.move_item(*item_type, name, new_location),
            Command::Vmove { item_type, name, vault_name, } => self.move_item_to_new_vault(*item_type, name, vault_name),
            Command::List => self.list(),
            Command::Config { config_type, value } => self.set_config(config_type, value),
            _ => Ok(Message::Empty),
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use super::*;

    macro_rules! run {
        ( $( $x:expr ),* ) => {
            let mut tests = Vec::new();
            tests.push(Pass(Command::Vault { show_loc: false, name: Some("vault_1".to_string()), location: Some(test_vaults())}));
            tests.push(Pass(Command::Enter { name: "vault_1".to_string() }));

            $(
                tests.push($x);
            )*
            run_test(|| {
                execute_commands(tests);
            })
        }
    }

    #[test]
    fn note_test() {
        run![
            Pass(Command::Note { name: "test_note".to_string() }),
            Pass(Command::Open { name: "test_note".to_string() }),
            Pass(Command::Remove { item_type: ItemType::Nt, name: "test_note".to_string() }),
            Fail(Command::Open { name: "test_note".to_string() }),
            Fail(Command::Open { name: "fake_note".to_string() })
        ];
    }

    #[test]
    fn cannot_create_duplicate_vaults() {
        run![
            Pass(Command::Vault { show_loc: false, name: Some("vault_2".to_string()), location: Some(test_vaults()) }),
            Pass(Command::Enter { name: "vault_2".to_string() }),
            Fail(Command::Vault { show_loc: false, name: Some("vault_2".to_string()), location: Some(test_vaults()) }) // Err: duplicate
        ];
    }

    #[test]
    fn move_note_between_vaults() {
        run![
            Pass(Command::Vault { show_loc: false, name: Some("vault_2".to_string()), location: Some(test_vaults()) }),
            Pass(Command::Note { name: "test_note".to_string() }),
            Pass(Command::Open { name: "test_note".to_string() }),
            Pass(Command::Vmove { item_type: VaultItemType::Nt, name: "test_note".to_string(), vault_name: "vault_2".to_string() }),
            Fail(Command::Open { name: "test_note".to_string() }), // Err: open test_note from vault_1
            Pass(Command::Enter { name: "vault_2".to_string() }),
            Pass(Command::Open { name: "test_note".to_string() })
        ];
    }

    #[test]
    fn move_folder_between_vaults() {
        run![
            Pass(Command::Vault { show_loc: false, name: Some("vault_2".to_string()), location: Some(test_vaults()) }),
            Pass(Command::Folder { name: "folder_1".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Vmove { item_type: VaultItemType::Fd, name: "folder_1".to_string(), vault_name: "vault_2".to_string() }),
            Fail(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Enter { name: "vault_2".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("folder_1") })
        ];
    }

    #[test]
    fn move_note_between_folders() {
        run![
            Pass(Command::Folder { name: "folder_1".to_string() }),
            Pass(Command::Folder { name: "folder_2".to_string() }),
            Pass(Command::Note { name: "test_note".to_string() }),
            Pass(Command::Move { item_type: ItemType::Nt, name: "test_note".to_string(), new_location: PathBuf::from("folder_1") }),
            Fail(Command::Open { name: "test_note".to_string() }), // Err: test_note was moved to folder_1 
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Open { name: "test_note".to_string() }),
            Pass(Command::Move { item_type: ItemType::Nt, name: "test_note".to_string(), new_location: PathBuf::from("../folder_2") }),
            Pass(Command::Chdir { path: PathBuf::from("../folder_2") }),
            Pass(Command::Open { name: "test_note".to_string() })
        ];
    }

    #[test]
    fn create_and_remove_folder_note_vault() {
        run! [
            Fail(Command::Remove { item_type: ItemType::Fd, name: "folder_1".to_string() }),
            Pass(Command::Folder { name: "folder_1".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Remove { item_type: ItemType::Fd, name: "folder_1".to_string() }),
            Fail(Command::Chdir { path: PathBuf::from("folder_1") })
        ];
        run! [
            Fail(Command::Remove { item_type: ItemType::Vault, name: "vault_2".to_string() }),
            Pass(Command::Vault { show_loc: false, name: Some("vault_2".to_string()), location: Some(test_vaults()) }),
            Pass(Command::Enter { name: "vault_2".to_string() }),
            Pass(Command::Remove { item_type: ItemType::Vault, name: "vault_2".to_string() }),
            Fail(Command::Enter { name: "vault_2".to_string() })
        ];
        run! [
            Fail(Command::Remove { item_type: ItemType::Note, name: "note_1".to_string() }),
            Pass(Command::Note { name: "note_1".to_string() }),
            Pass(Command::Open { name: "note_1".to_string() }),
            Pass(Command::Remove { item_type: ItemType::Note, name: "note_1".to_string() }),
            Fail(Command::Open { name: "note_1".to_string() })
        ];
    }

    #[test]
    fn create_and_edit_daily_note() {
        run! [
            Fail(Command::Today { create_if_dne: false }), // does not exist
            Pass(Command::Today { create_if_dne: true }),  // create
            Fail(Command::Today { create_if_dne: true }),  // already exists
            Pass(Command::Today { create_if_dne: false }), // open
            Pass(Command::Today { create_if_dne: false })  // open again
        ];
    }

    #[test]
    fn create_note_inside_folder() {
        run! [
            Pass(Command::Folder { name: "folder_1".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Note { name: "note_1".to_string() }),
            Pass(Command::Open { name: "note_1".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("..") }),
            Fail(Command::Open { name: "note_1".to_string() }), // cannot open note in ./folder_1 from ./
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Remove { item_type: ItemType::Nt, name: "note_1".to_string() })
        ];
    }
}
