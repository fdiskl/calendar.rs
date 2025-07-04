use ratatui::{
    crossterm::event::Event,
    layout::Alignment,
    style::{Modifier, Stylize},
    widgets::{Block, Widget},
};

use crate::views::common::{
    focusable::Focusable,
    styles::title_style,
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
        let block = Block::new()
            .title("JOURNAL")
            .title_alignment(Alignment::Center)
            .style(title_style(self.focused));
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
