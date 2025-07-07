use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

use crate::ui::monthly::MonthlyView;

impl MonthlyView {
    fn render_days_titles(&self, inner_area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let col_constraints = (0..7).map(|_| Constraint::Length(inner_area.width / 7));
        let row_constraints = [Constraint::Length(1)];
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(inner_area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        let long_titles = inner_area.width as usize / 7 > ("SATURDAY".len() + 1);

        for (i, cell) in cells.enumerate() {
            let title = if long_titles {
                match i {
                    0 => "MONDAY",
                    1 => "TUESDAY",
                    2 => "WEDNESDAY",
                    3 => "THURSDAY",
                    4 => "FRIDAY",
                    5 => "SATURDAY",
                    6 => "SUNDAY",
                    _ => "",
                }
            } else {
                match i {
                    0 => "MON",
                    1 => "TUE",
                    2 => "WED",
                    3 => "THU",
                    4 => "FRI",
                    5 => "SAT",
                    6 => "SUN",
                    _ => "",
                }
            };

            let style = if i == 5 || i == 6 {
                Style::new().fg(Color::Red)
            } else {
                Style::new().fg(Color::Blue)
            };

            Text::from(title).style(style).render(cell, buf)
        }
    }

    pub(super) fn render_main_grid(
        &self,
        mut inner_area: Rect,
        buf: &mut ratatui::prelude::Buffer,
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

        for (i, cell) in cells.iter().enumerate() {
            let day_num = (i as i32) - self.first_day;

            // hide days not in curr month
            let text = if day_num >= 0 && day_num < self.month_len {
                format!("{}", day_num + 1)
            } else {
                "".to_string()
            };

            // make weekdays red
            let weekday = i % 7;

            let style = if self.c.current_day() == Some(i as i32) {
                Style::new().bg(Color::Blue).fg(Color::White)
            } else if weekday == 5 || weekday == 6 {
                Style::new().fg(Color::Red)
            } else {
                Style::default()
            };

            Paragraph::new(text)
                .style(style)
                .block(Block::new())
                .alignment(Alignment::Left)
                .render(*cell, buf);
        }
    }
}
