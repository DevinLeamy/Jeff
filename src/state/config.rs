use std::fmt::Display;
use std::path::PathBuf;

use colored::Color;
use serde::{Deserialize, Serialize};

use crate::utils::application_config_path;
use crate::{enums::ConfigType, fileio::FileIO};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorData {
    pub editor: String,
    pub conflict: bool,
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData {
            editor: "nvim".to_string(),
            conflict: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    vault_color: String,
    folder_color: String,
    note_color: String,
    editor_data: EditorData,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor_data: EditorData::default(),
            vault_color: "red".to_string(),
            folder_color: "blue".to_string(),
            note_color: "yellow".to_string(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", toml::to_string_pretty(self).unwrap())
    }
}

impl FileIO for Config {
    fn path(&self) -> PathBuf {
        let mut path = application_config_path();
        path.push("config");
        path
    }
}

impl Config {
    pub fn get_editor_data(&self) -> EditorData {
        self.editor_data.clone()
    }

    pub fn set_config_value(&mut self, config_type: &ConfigType, value: String) {
        match config_type {
            ConfigType::Editor => self.editor_data.editor = value,
            ConfigType::Conflict => self.editor_data.conflict = value == "true".to_string(),
            ConfigType::VaultColor => self.vault_color = value,
            ConfigType::FolderColor => self.folder_color = value,
            ConfigType::NoteColor => self.note_color = value,
        }

        self.store()
    }

    #[allow(unused)]
    pub fn get_config_value(&mut self, config_type: &ConfigType) -> String {
        match config_type {
            ConfigType::Editor => self.editor_data.editor.to_string(),
            ConfigType::Conflict => self.editor_data.conflict.to_string(),
            ConfigType::VaultColor => self.vault_color.to_string(),
            ConfigType::FolderColor => self.folder_color.to_string(),
            ConfigType::NoteColor => self.note_color.to_string(),
        }
    }

    pub fn get_vault_color(&self) -> Color {
        Color::try_from(self.vault_color.to_owned()).unwrap_or(Color::Red)
    }

    pub fn get_note_color(&self) -> Color {
        Color::try_from(self.note_color.to_owned()).unwrap_or(Color::Yellow)
    }

    pub fn get_folder_color(&self) -> Color {
        Color::try_from(self.folder_color.to_owned()).unwrap_or(Color::Blue)
    }
}
