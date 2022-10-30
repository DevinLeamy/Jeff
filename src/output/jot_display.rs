use colored::*;

use crate::prelude::*;

pub trait Colored {
    fn get_color() -> Color {
        // Default color, should never be used
        Color::BrightYellow
    }
}

pub trait JotDisplay {
    fn to_display_string(&self) -> String;
}

impl<T> JotDisplay for T
where
    T: Item + Colored,
{
    fn to_display_string(&self) -> String {
        let color = <Self as Colored>::get_color();
        self.get_name().color(color).to_string()
    }
}

impl Colored for Note {
    fn get_color() -> Color {
        CONFIG.lock().unwrap().get_note_color()
    }
}

impl Colored for Folder {
    fn get_color() -> Color {
        CONFIG.lock().unwrap().get_folder_color()
    }
}

impl Colored for Vault {
    fn get_color() -> Color {
        CONFIG.lock().unwrap().get_vault_color()
    }
}
