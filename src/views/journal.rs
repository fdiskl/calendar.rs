use ratatui::widgets::{Block, Widget};

use crate::views::common::view::View;

pub struct Journal {}

impl Journal {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for Journal {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new();
        block.render(area, buf);
    }

    fn handle_event(&mut self, e: ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}
