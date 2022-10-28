use anyhow::anyhow;

use crate::items::{Folder, Item, Note};
use crate::output::error::JotResult;
use crate::prelude::JotDisplay;

pub trait Collection: Item {
    fn get_note_with_name(&self, name: &String) -> JotResult<Note> {
        for note in self.get_notes() {
            if &note.get_name() == name {
                return Ok(note);
            }
        }

        Err(anyhow!(
            "Note [{}] does not exist in [{}]",
            name,
            self.get_name()
        ))
    }

    fn get_folder_with_name(&self, name: &String) -> JotResult<Folder> {
        for folder in self.get_folders() {
            if &folder.get_name() == name {
                return Ok(folder);
            }
        }

        Err(anyhow!(
            "Folder [{}] does not exist in [{}]",
            name,
            self.get_name()
        ))
    }

    fn get_notes(&self) -> Vec<Note>;

    fn get_notes_sorted(&self) -> Vec<Note> {
        let mut notes = self.get_notes();
        notes.sort_by_key(|note| note.get_name());

        notes
    }

    fn get_folders(&self) -> Vec<Folder>;

    fn get_folders_sorted(&self) -> Vec<Folder> {
        let mut folders = self.get_folders();
        folders.sort_by_key(|folder| folder.get_name());

        folders
    }

    /**
     * TODO: Move into [JotDisplay] trait
     */
    fn list(&self) {
        for folder in self.get_folders_sorted() {
            println!("└── {}", folder.to_display_string());
            folder.list_with_buffer("".to_string());
        }

        let notes = self.get_notes_sorted();
        for (i, note) in notes.iter().enumerate() {
            if i == notes.len() - 1 {
                println!("└── {}", note.to_display_string());
            } else {
                println!("├── {}", note.to_display_string());
            }
        }
    }

    fn list_with_buffer(&self, buffer: String) {
        for folder in self.get_folders_sorted() {
            println!("{} └── {}", buffer, folder.to_display_string());
            folder.list_with_buffer(format!("{}    ", buffer).to_string());
        }

        let notes = self.get_notes_sorted();
        for (i, note) in notes.iter().enumerate() {
            if i == notes.len() - 1 {
                println!("{}    └── {}", buffer, note.to_display_string());
            } else {
                println!("{}    ├── {}", buffer, note.to_display_string());
            }
        }
    }
}
