use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

pub struct DailyView {}

impl DailyView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for &DailyView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("DAILY VIEW".bold());

        let block = Block::new().title(title.centered());

        block.render(area, buf);
    }
}
