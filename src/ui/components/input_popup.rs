use ratatui::style::Style;

use crate::ui::components::{input::UserInput, popup_with_cursor::Popup};

pub type InputPopup<'a> = Popup<'a, UserInput>;

impl<'a> InputPopup<'a> {
    pub fn new_input_popup(
        title: &'a str,
        border_style: Option<Style>,
        title_style: Option<Style>,
    ) -> Self {
        Self::new(title, UserInput::new(""), border_style, title_style)
    }
}
