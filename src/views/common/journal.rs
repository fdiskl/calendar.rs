use ratatui::widgets::{Block, Widget};

pub struct Journal {}

impl Journal {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for &Journal {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new();
        block.render(area, buf);
    }
}
