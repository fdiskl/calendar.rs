use ratatui::{
    crossterm::event::Event,
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

use anyhow::Result;

use crate::views::common::view::View;

pub struct DailyView {}

impl DailyView {
    pub fn new() -> Self {
        Self {}
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
