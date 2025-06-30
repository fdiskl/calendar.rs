use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

pub struct MonthlyView {}

impl MonthlyView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for &MonthlyView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("MONTHLY VIEW".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        block.render(area, buf);
    }
}
