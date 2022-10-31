#[cfg(not(test))]
use std::process::Command;

use crate::prelude::*;

#[allow(unused)]
pub struct Editor {
    /// CLI name of the editor (ex: "nvim" or "vim")
    name: String,
    /// whether the editor should conflict with the running terminal process
    conflict: bool,
}

impl Editor {
    pub fn from_config(config: EditorData) -> Self {
        Editor {
            name: config.editor,
            conflict: config.conflict,
        }
    }

    #[cfg(not(test))]
    /// Opens the given note using the editor.
    pub fn open_note(&self, note: Note) -> JotResult<()> {
        let note_path = note.get_location();
        let mut open_editor_command = Command::new(self.name.to_owned())
            .arg(note_path.to_str().unwrap())
            .spawn()?;

        if self.conflict {
            open_editor_command.wait()?;
        }

        Ok(())
    }

    #[cfg(test)]
    /// "Pretends" to open the given note.
    pub fn open_note(&self, note: Note) -> JotResult<()> {
        let note_path = note.get_location();
        assert!(Note::is_valid_path(&note_path.to_path_buf()) && note_path.to_path_buf().is_file());

        Ok(())
    }

    /// Copy the contents of one note into another note
    pub fn copy_note(from_note: &Note, to_note: &Note) -> JotResult<()> {
        let _ = std::fs::copy(
            from_note.get_location().to_path_buf(),
            to_note.get_location().to_path_buf(),
        )?;

        Ok(())
    }
}
