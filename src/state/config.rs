use crate::{enums::ConfigType, fileio::FileIO};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::utils::application_config_path;

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
    editor_data: EditorData,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor_data: EditorData::default(),
        }
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

    pub fn set_config(&mut self, config_type: &ConfigType, value: &String) {
        match config_type {
            ConfigType::Editor => self.set_editor(value.to_owned()),
            ConfigType::Conflict => self.set_conflict(value.to_owned()),
        }
    }

    pub fn get_config(&self, config_type: &ConfigType) -> String {
        match config_type {
            ConfigType::Editor => self.get_editor().to_owned(),
            ConfigType::Conflict => match self.get_conflict() {
                true => "true".to_string(),
                false => "false".to_string(),
            },
        }
    }

    fn get_editor(&self) -> &String {
        &self.editor_data.editor
    }

    fn set_editor(&mut self, editor: String) {
        self.editor_data.editor = editor;
        self.store()
    }

    fn get_conflict(&self) -> &bool {
        &self.editor_data.conflict
    }

    fn set_conflict(&mut self, conflict: String) {
        self.editor_data.conflict = conflict.parse().unwrap();
        self.store()
    }
}
