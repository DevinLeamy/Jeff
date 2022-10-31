use crate::enums::{ConfigType, Item, VaultItem};
use colored::Colorize;
use std::fmt::Display;

pub enum Message {
    VaultEntered(String),
    #[allow(unused)]
    NoteAliasCreated(String, String),
    #[allow(unused)]
    NoteAliasRemoved(String, String),
    TemplateCreated(String),
    ItemCreated(Item, String),
    ItemRemoved(Item, String),
    ItemRenamed(Item, String, String),
    ItemMoved(Item, String),
    ItemVMoved(VaultItem, String, String),
    FolderChanged,
    Config(ConfigType, String),
    Custom(String),
    Empty,
}

impl Message {
    fn create_message(content: String) -> String {
        format!("ϟ {} ϟ {}", "Jot".yellow(), content)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Message::create_message(match self {
                Message::VaultEntered(name) => format!("entered \x1b[0;34m{}\x1b[0m", name),
                Message::ItemCreated(item_type, name) =>
                    format!("{} \x1b[0;34m{}\x1b[0m created", item_type.full(), name),
                Message::ItemRemoved(item_type, name) =>
                    format!("{} \x1b[0;34m{}\x1b[0m removed", item_type.full(), name),
                Message::ItemRenamed(item_type, name, new_name) => format!(
                    "{} \x1b[0;34m{}\x1b[0m renamed to \x1b[0;34m{}\x1b[0m",
                    item_type.full(),
                    name,
                    new_name
                ),
                Message::ItemMoved(item_type, name) =>
                    format!("{} \x1b[0;34m{}\x1b[0m moved", item_type.full(), name),
                Message::ItemVMoved(item_type, name, vault_name) => format!(
                    "{} \x1b[0;34m{}\x1b[0m moved to vault \x1b[0;34m{}\x1b[0m",
                    item_type.full(),
                    name,
                    vault_name
                ),
                Message::FolderChanged => "changed folder".to_string(),
                Message::Config(config_type, value) => format!(
                    "Configuration option [\x1b[0;34m{}\x1b[0m] is set to {}",
                    config_type.to_str(),
                    value
                ),
                Message::NoteAliasCreated(note_name, alias_name) => {
                    format!(
                        "created alias \x1b[0;34m{}\x1b[0m -> \x1b[0;34m{}\x1b[0m",
                        note_name, alias_name
                    )
                }
                Message::NoteAliasRemoved(note_name, alias_name) => {
                    format!(
                        "removed alias \x1b[0;34m{}\x1b[0m -> \x1b[0;34m{}\x1b[0m",
                        note_name, alias_name
                    )
                }
                Message::TemplateCreated(template_name) => {
                    format!("Created template [{}]", template_name.blue())
                }
                Message::Custom(content) => content.to_string(),
                Message::Empty => "".to_string(),
            })
        )
    }
}
