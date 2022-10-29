use std::borrow::Cow;

use colored::*;
use skim::prelude::*;

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
        Color::Cyan
    }
}

impl Colored for Folder {
    fn get_color() -> Color {
        Color::Red
    }
}

impl Colored for Vault {
    fn get_color() -> Color {
        Color::Magenta
    }
}

impl SkimItem for Note {
    fn text(&self) -> Cow<str> {
        Cow::from(self.get_name())
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(format!("{}\n<PREVIEW>", self.to_display_string()))
    }
}
