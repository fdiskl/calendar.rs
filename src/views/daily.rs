use ratatui::{
    crossterm::event::KeyEvent,
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

    pub fn handle_key_press_ev(&mut self, key_ev: KeyEvent) {}
}

impl Widget for &DailyView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("DAILY VIEW".bold());

        let block = Block::new().title(title.centered());

        block.render(area, buf);
    }
}
