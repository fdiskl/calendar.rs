use chrono::{Datelike, NaiveDate, Weekday};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct MonthlyView {}

impl MonthlyView {
    pub fn new() -> Self {
        Self {}
    }

    fn render_days_titles(&self, inner_area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let col_constraints = (0..7).map(|_| Constraint::Length(inner_area.width / 7));
        let row_constraints = [Constraint::Length(1)];
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(inner_area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let title = match i {
                0 => "MONDAY",
                1 => "TUESDAY",
                2 => "WEDNESDAY",
                3 => "THURSDAY",
                4 => "FRIDAY",
                5 => "SATURDAY",
                6 => "SUNDAY",
                _ => "",
            };
            Text::from(title).render(cell, buf);
        }
    }

    fn render_main_grid(
        &self,
        mut inner_area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        year: i32,
        month: u32,
    ) {
        self.render_days_titles(inner_area, buf);

        // days title use 1 unit of h
        inner_area.y += 1;
        inner_area.height -= 1;

        let col_width = inner_area.width / 7;
        let row_height = inner_area.height / 6;

        // Constraints for layout
        let col_constraints = (0..7).map(|_| Constraint::Length(col_width));
        let row_constraints = (0..6).map(|_| Constraint::Length(row_height));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(inner_area);
        let cells: Vec<Rect> = rows
            .iter()
            .flat_map(|&row| horizontal.split(row).to_vec())
            .collect();

        let (first_day_idx, days_in_month) = month_info(year, month);

        for (i, cell) in cells.iter().enumerate() {
            let day_num = (i as i32) - first_day_idx;

            let text = if day_num >= 0 && day_num < days_in_month {
                format!("{}", day_num + 1)
            } else {
                "".to_string()
            };

            Paragraph::new(text)
                .block(Block::new())
                .alignment(Alignment::Left)
                .render(*cell, buf);
        }
    }
}

impl Widget for &MonthlyView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("JULY 2025".bold());

        let block = Block::new().title(title.alignment(Alignment::Left));
        let inner_area = block.inner(area);
        block.render(area, buf);

        self.render_main_grid(inner_area, buf, 2025, 7);
    }
}

// returns
// 1. first day number (Mon-Sat) [0;6]
// 2. amount of days in a month
fn month_info(year: i32, month: u32) -> (i32, i32) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");

    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    let first_day_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let last_day = first_day_next_month.pred_opt().unwrap();

    let days_in_month = last_day.day();

    let n = match first_day.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    (n, days_in_month as i32)
}
