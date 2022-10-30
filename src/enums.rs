use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, PartialEq, Copy)]
pub enum Item {
    Vault,
    Vl,
    Note,
    Nt,
    Folder,
    Fd,
}

impl Item {
    pub fn to_vault_item(&self) -> VaultItem {
        match self {
            Item::Vault | Item::Vl => VaultItem::Fd,
            Item::Note | Item::Nt => VaultItem::Nt,
            Item::Folder | Item::Fd => VaultItem::Fd,
        }
    }

    pub fn fs_name(&self) -> String {
        self.to_vault_item().full()
    }

    pub fn full(&self) -> String {
        match self {
            Item::Vault | Item::Vl => "vault".to_string(),
            Item::Note | Item::Nt => "note".to_string(),
            Item::Folder | Item::Fd => "folder".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum VaultItem {
    Note,
    Nt,
    Folder,
    Fd,
}

impl VaultItem {
    pub fn to_item(&self) -> Item {
        match self {
            VaultItem::Note | VaultItem::Nt => Item::Nt,
            VaultItem::Folder | VaultItem::Fd => Item::Fd,
        }
    }

    pub fn full(&self) -> String {
        match self {
            VaultItem::Note | VaultItem::Nt => "note".to_string(),
            VaultItem::Folder | VaultItem::Fd => "folder".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ConfigType {
    Editor,
    Conflict,
    VaultColor,
    FolderColor,
    NoteColor,
}

impl ConfigType {
    pub fn to_str(&self) -> &str {
        match self {
            ConfigType::Editor => "editor",
            ConfigType::Conflict => "conflict",
            ConfigType::VaultColor => "vault_color",
            ConfigType::FolderColor => "folder_color",
            ConfigType::NoteColor => "note_color",
        }
    }
}
