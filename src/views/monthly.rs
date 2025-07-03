use chrono::{Datelike, Local};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Alignment,
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

mod render;

pub struct MonthlyView {
    curr_year: i32,
    curr_month: u32,
}

impl MonthlyView {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            curr_month: now.month(),
            curr_year: now.year(),
        }
    }

    pub fn handle_key_press_ev(&mut self, key_ev: KeyEvent) {
        match key_ev.code {
            KeyCode::Char('n') => {
                self.curr_month += 1;
                if self.curr_month > 12 {
                    self.curr_month = 1;
                    self.curr_year += 1;
                }
            }
            KeyCode::Char('p') => {
                if self.curr_month == 1 {
                    self.curr_month = 12;
                    self.curr_year -= 1;
                } else {
                    self.curr_month -= 1;
                }
            }

            _ => {}
        }
    }
}

impl Widget for &MonthlyView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from(
            (match self.curr_month {
                1 => "JANUARY",
                2 => "FEBRUARY",
                3 => "MARCH",
                4 => "APRIL",
                5 => "MAY",
                6 => "JUNE",
                7 => "JULY",
                8 => "AUGUST",
                9 => "SEPTEMBER",
                10 => "OCTOBER",
                11 => "NOVEMBER",
                12 => "DECEMBER",
                _ => "",
            }
            .to_string()
                + " "
                + &self.curr_year.to_string())
                .bold(),
        );

        let block = Block::new().title(title.alignment(Alignment::Left));
        let inner_area = block.inner(area);
        block.render(area, buf);

        self.render_main_grid(inner_area, buf, self.curr_year, self.curr_month);
    }
}
