use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::anyhow;

use colored::Colorize;
#[cfg(not(test))]
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

use crate::{enums::ConfigType, prelude::*};

lazy_static! {
    // Mutex is used to allow for mutable access of global state.
    // CONFIG should remain the ONLY mutable global struct.
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::load());
}

pub struct App {
    vaults: Vaults,
    editor: Editor,
    templates: Folder,
}

impl App {
    pub fn vault(
        &mut self,
        show_loc: bool,
        name: &Option<String>,
        location: &Option<PathBuf>,
    ) -> JeffResult<Message> {
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

    pub fn enter_vault(&mut self, name: &String) -> JeffResult<Message> {
        self.vaults.enter_vault(name)?;
        return Ok(Message::VaultEntered(name.to_owned()));
    }

    pub fn create_note(
        &mut self,
        name: &String,
        from_template: bool,
        template_name: &Option<String>,
    ) -> JeffResult<Message> {
        let vault = self.vaults.ref_current()?;
        let maybe_note = vault.get_note_with_name(name);
        let templates = self.templates.notes();

        if from_template && template_name.is_none() {
            return Err(anyhow!("Must specify template name"));
        }

        if let Ok(note) = maybe_note {
            return Err(anyhow!(
                "Note with name [{}] already exists",
                note.get_name()
            ));
        }

        let note_path = Note::generate_abs_path(&vault.get_active_location(), name);

        if from_template {
            let template_name = template_name.to_owned().unwrap();
            let maybe_template = item_with_name::<Note>(&templates, &template_name);

            if maybe_template.is_none() {
                return Err(anyhow!(
                    "Template [{}] does not exist",
                    template_name.blue()
                ));
            }

            let new_note = Note::create(note_path)?;

            Editor::copy_note(maybe_template.unwrap(), &new_note)?;
        } else {
            Note::create(note_path)?;
        }

        return Ok(Message::ItemCreated(ItemType::Nt, name.to_owned()));
    }

    pub fn today(&mut self) -> JeffResult<Message> {
        let daily_note_name = daily_note_name();
        let vault = self.vaults.ref_current()?;
        let mut message = Message::Empty;
        let daily_note = match vault.get_note_with_name(&daily_note_name) {
            Ok(note) => note,
            Err(_) => {
                // daily note does does not exist
                #[cfg(not(test))]
                let create_daily_note = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Create daily note {}?", daily_note_name))
                    .interact()
                    .unwrap();
                #[cfg(test)]
                let create_daily_note = true;

                if create_daily_note {
                    let note_path = Note::generate_abs_path(vault.get_location(), &daily_note_name);
                    message = Message::ItemCreated(ItemType::Nt, daily_note_name);
                    Note::create(note_path)?
                } else {
                    return Err(anyhow!("Daily note does not exist"));
                }
            }
        };

        self.editor.open_note(daily_note)?;

        Ok(message)
    }

    #[cfg(test)]
    pub fn open_note(&mut self, name: &String) -> JeffResult<Message> {
        let note = self
            .vaults
            .ref_current()?
            .get_note_from_active_folder(name)?;
        self.editor.open_note(note)?;
        Ok(Message::Empty)
    }

    pub fn template(&mut self, name: &Option<String>) -> JeffResult<Message> {
        if name.is_none() {
            self.templates.list();
            return Ok(Message::Empty);
        }

        let name = name.to_owned().unwrap();

        let templates = self.templates.notes();
        let maybe_template = item_with_name::<Note>(&templates, &name);

        if let Some(template) = maybe_template {
            self.editor.open_note(template.to_owned())?;

            return Ok(Message::Empty);
        }

        let create_template = confirmation_prompt(format!(
            "Would you like to create a template [{}]",
            name.blue()
        ));

        if create_template {
            let template_path = Note::generate_abs_path(self.templates.get_location(), &name);
            let template = Note::create(template_path)?;
            self.editor.open_note(template.to_owned())?;

            Ok(Message::TemplateCreated(name.to_owned()))
        } else {
            Ok(Message::Empty)
        }
    }

    #[cfg(not(test))]
    pub fn open_note(&mut self, name: &String) -> JeffResult<Message> {
        let vault = self.vaults.ref_current()?;
        let active_collection: Box<dyn Collection> = vault.active_collection()?;
        let notes = active_collection.notes_sorted();
        let maybe_note = notes.iter().find(|note| &note.get_name() == name);

        /*
         * If the given name is a valid note, open it. Otherwise, fuzzysearch
         * for a note.
         */
        if let Some(note) = maybe_note {
            self.editor.open_note(note.clone())?;
            return Ok(Message::Empty);
        }

        let mut selections = vec![];

        for note in &notes {
            selections.push(note.get_name());
        }

        let maybe_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&selections.to_owned())
            .default(0)
            // Note: This can be uncommented once https://github.com/mitsuhiko/dialoguer/pull/226 is part of
            //       crates.io release.
            // ---
            // .with_initial_text(name.to_owned())
            .interact_opt()?;

        if let Some(selection) = maybe_selection {
            let note_name = selections[selection].to_owned();
            let note = notes
                .iter()
                .find(|note| note.get_name() == note_name)
                .unwrap();
            self.editor.open_note(note.to_owned())?;

            Ok(Message::Empty)
        } else {
            Err(anyhow!(Error::ItemNotFound(
                ItemType::Note,
                name.to_owned()
            )))
        }
    }

    pub fn create_folder(&mut self, name: &String) -> JeffResult<Message> {
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

    pub fn change_directory(&mut self, path: &PathBuf) -> JeffResult<Message> {
        let vault = self.vaults.mut_current()?;
        vault.change_folder(path)?;

        Ok(Message::FolderChanged)
    }

    pub fn remove_item(&mut self, item_type: ItemType, name: &String) -> JeffResult<Message> {
        // display a dialog to confirm the action
        #[cfg(not(test))]
        let remove_item = confirmation_prompt(format!("Are you sure you want to remove {}?", name));
        #[cfg(test)]
        let remove_item = true;

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
    ) -> JeffResult<Message> {
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

    pub fn list(&self) -> JeffResult<Message> {
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
        config_type: Option<ConfigType>,
        maybe_value: Option<String>,
    ) -> JeffResult<Message> {
        if config_type.is_none() {
            return Ok(Message::Custom(format!(
                "\nConfiguration\n---\n{}",
                CONFIG.lock().unwrap()
            )));
        }

        let config_type = config_type.unwrap();
        let value = match config_type {
            ConfigType::Editor => maybe_value.unwrap(),
            ConfigType::Conflict => maybe_value.unwrap(),
            ConfigType::VaultColor => {
                maybe_value.unwrap_or_else(|| display_item_color_select::<Vault>())
            }
            ConfigType::FolderColor => {
                maybe_value.unwrap_or_else(|| display_item_color_select::<Folder>())
            }
            ConfigType::NoteColor => {
                maybe_value.unwrap_or_else(|| display_item_color_select::<Note>())
            }
        };

        CONFIG
            .lock()
            .unwrap()
            .set_config_value(&config_type, value.to_owned());
        return Ok(Message::Config(config_type.to_owned(), value.to_owned()));
    }

    pub fn move_item(
        &mut self,
        item_type: ItemType,
        name: &String,
        new_location: &PathBuf,
    ) -> JeffResult<Message> {
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
    ) -> JeffResult<Message> {
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
    pub fn new() -> JeffResult<Self> {
        let editor_data = CONFIG.lock().unwrap().get_editor_data();
        let templates_path = application_templates_path();

        if !templates_path.is_dir() {
            std::fs::create_dir(&templates_path)?
        }

        Ok(App {
            vaults: Vaults::load()?,
            editor: Editor::from_config(editor_data),
            templates: Folder::load(templates_path)?,
        })
    }

    #[rustfmt::skip]
    pub fn handle_command(&mut self, command: Command) -> JeffResult<Message> {
        match &command {
            Command::Vault { show_loc, name, location, } => self.vault(*show_loc, name, location),
            Command::Enter { name } => self.enter_vault(name),
            Command::Note { name, from_template, template_name} => self.create_note(name, *from_template, template_name),
            Command::Today => self.today(),
            // Command::Alias { name, maybe_alias, remove_alias, } => { todo!() }
            Command::Open { name } => self.open_note(name),
            Command::Folder { name } => self.create_folder(name),
            Command::Chdir { path } => self.change_directory(path),
            Command::Remove { item_type, name } => self.remove_item(*item_type, name),
            Command::Rename { item_type, name, new_name, } => self.rename_item(*item_type, name, new_name),
            Command::Move { item_type, name, new_location, } => self.move_item(*item_type, name, new_location),
            Command::Vmove { item_type, name, vault_name, } => self.move_item_to_new_vault(*item_type, name, vault_name),
            Command::List => self.list(),
            Command::Config { config_type, value } => self.set_config(config_type.clone(), value.to_owned()),
            Command::Template { name } => self.template(name),
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
            Pass(Command::Note { name: "test_note".to_string(), from_template: false, template_name: None }),
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
            Pass(Command::Note { name: "test_note".to_string(), from_template: false, template_name: None }),
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
            Pass(Command::Note { name: "test_note".to_string(), from_template: false, template_name: None }),
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
            Pass(Command::Note { name: "note_1".to_string(), from_template: false, template_name: None }),
            Pass(Command::Open { name: "note_1".to_string() }),
            Pass(Command::Remove { item_type: ItemType::Note, name: "note_1".to_string() }),
            Fail(Command::Open { name: "note_1".to_string() })
        ];
    }

    #[test]
    fn create_and_edit_daily_note() {
        run! [
            Pass(Command::Today)  // create
        ];
    }

    #[test]
    fn create_note_inside_folder() {
        run! [
            Pass(Command::Folder { name: "folder_1".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Note { name: "note_1".to_string(), from_template: false, template_name: None }),
            Pass(Command::Open { name: "note_1".to_string() }),
            Pass(Command::Chdir { path: PathBuf::from("..") }),
            Fail(Command::Open { name: "note_1".to_string() }), // cannot open note in ./folder_1 from ./
            Pass(Command::Chdir { path: PathBuf::from("folder_1") }),
            Pass(Command::Remove { item_type: ItemType::Nt, name: "note_1".to_string() })
        ];
    }

    #[test]
    fn create_and_edit_and_list_templates() {
        run! [
            Pass(Command::Template { name: Some("template".to_string()) }), // create
            Pass(Command::Template { name: Some("template".to_string()) }), // edit
            Pass(Command::Template { name: None } ) // list 
        ];
    }

    #[test]
    fn create_note_from_template() {
        run! [
            Fail(Command::Note { name: "note_1".to_string(), from_template: true, template_name: None }), // no template name
            Fail(Command::Note { name: "note_1".to_string(), from_template: true, template_name: Some("template".to_string())}), // template does not exist
            Pass(Command::Template { name: Some("template".to_string()) }), // create template 
            Pass(Command::Note { name: "note_1".to_string(), from_template: true, template_name: Some("template".to_string())}), // template does not exist
            Pass(Command::Open { name: "note_1".to_string() })
        ];
    }
}
