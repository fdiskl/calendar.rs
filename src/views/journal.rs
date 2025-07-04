use ratatui::{
    crossterm::event::Event,
    widgets::{Block, Widget},
};

use crate::views::common::{
    focusable::Focusable,
    view::{FocusableView, View},
};

pub struct Journal {
    focused: bool,
}

impl Journal {
    pub fn new() -> Self {
        Self { focused: false }
    }
}

impl View for Journal {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new().title(if self.focused { "FOCUSED" } else { "UNFOCUSED" });
        block.render(area, buf);
    }

    fn handle_event(&mut self, e: &Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self) {}
}

impl Focusable for Journal {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused;
    }

    fn focused(mut self) -> Self {
        self.focused = true;
        self
    }
}

impl FocusableView for Journal {
    fn handle_event_if_focused(&mut self, e: &Event) -> anyhow::Result<()> {
        if self.focused {
            self.handle_event(e)
        } else {
            Ok(())
        }
    }
}
