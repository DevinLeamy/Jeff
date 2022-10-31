use anyhow::anyhow;
use std::path::PathBuf;

use std::fs::{remove_file, rename};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Note {
    location: JotPath,
}

impl Note {
    pub fn type_name() -> String {
        "note".to_string()
    }

    pub fn generate_abs_path(parent_dir: &PathBuf, note_name: &String) -> PathBuf {
        join_paths(vec![
            parent_dir.to_str().unwrap(),
            format!("{}.md", note_name).as_str(),
        ])
    }

    /**
     * Initializes an existing note from its path
     */
    pub fn load(note_location: PathBuf) -> JotResult<Self> {
        if !Note::is_valid_path(&note_location) {
            return Err(anyhow!("Invalid note path [{:?}]", note_location));
        }

        Ok(Note {
            location: note_location.into(),
        })
    }

    pub fn create(note_location: PathBuf) -> JotResult<Self> {
        if !Note::is_valid_path(&note_location) {
            return Err(anyhow!("Invalid note path [{:?}]", note_location));
        }

        create_file(note_location.clone())?;
        let note = Note::load(note_location)?;

        Ok(note)
    }

    /**
     * Checks if a path points to a valid jot [Note].
     */
    pub fn is_valid_path(absolute_path: &PathBuf) -> bool {
        if absolute_path.extension().is_none() {
            return false;
        }

        !absolute_path.is_dir() && absolute_path.extension().unwrap() == "md"
    }
}

impl Item for Note {
    fn get_location(&self) -> &JotPath {
        &self.location
    }

    fn relocate(&mut self, new_location: PathBuf) -> JotResult<()> {
        assert!(Note::is_valid_path(&new_location));
        rename(&self.location.as_path(), &new_location)?;
        self.location = new_location.into();

        Ok(())
    }

    fn rename(&mut self, new_name: String) -> JotResult<()> {
        let new_location = JotPath::from_parent(&self.location.parent(), new_name);

        rename(&self.location.as_path(), &new_location.as_path())?;
        self.location = new_location.into();

        Ok(())
    }

    fn delete(&self) -> JotResult<()> {
        // TODO: make sure the user is prompted before executing
        // NOTE: this could potentially delete a lot of information!
        remove_file(&self.location.as_path())?;

        Ok(())
    }
}
