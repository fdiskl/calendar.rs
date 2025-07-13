use ratatui::{
    crossterm::event::Event,
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

use anyhow::Result;

use crate::ui::common::{
    focusable::Focusable,
    view::{FocusableView, View},
};

pub struct DailyView {
    focused: bool,
}

impl DailyView {
    pub fn new() -> Self {
        Self { focused: false }
    }
}

impl View for DailyView {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("DAILY VIEW".bold());

        let block = Block::new().title(title.centered());

        block.render(area, buf);
    }

    fn handle_event(&mut self, e: &Event) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) {}
}

impl Focusable for DailyView {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused
    }
}

impl FocusableView for DailyView {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()> {
        if self.focused {
            self.handle_event(e)
        } else {
            Ok(())
        }
    }
}
