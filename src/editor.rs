use std::process::Command;

use crate::state::config::EditorData;
use crate::items::{Note, Item};
use crate::output::error::JotResult;

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

    pub fn open_note(&self, note: Note) -> JotResult<()> {
        let note_path = note.get_location();
        let mut open_editor_command = Command::new(self.name.to_owned()).arg(note_path.to_str().unwrap()).spawn()?;

        if self.conflict {
            open_editor_command.wait()?;
        }

        Ok(())
    }
}
