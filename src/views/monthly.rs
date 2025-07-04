use chrono::{Datelike, Local};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Widget},
};

use anyhow::Result;

use crate::views::{
    common::{
        focusable::Focusable,
        styles::title_style,
        utils::month_info,
        view::{FocusableView, View},
    },
    monthly::cursor::Cursor,
};

mod cursor;
mod render;

pub struct MonthlyView {
    curr_year: i32,
    curr_month: u32,

    first_day: i32,
    month_len: i32,

    focused: bool,

    c: Cursor,
}

impl MonthlyView {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            curr_month: now.month(),
            curr_year: now.year(),
            c: Cursor::new().with_w(7).with_h(6),
            first_day: 0,
            month_len: 0,
            focused: false,
        }
    }

    fn handle_key_press_ev(&mut self, key_ev: &KeyEvent) -> Result<()> {
        // TODO: move keys to cfg
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
            KeyCode::Char('s') => {
                self.c.change_shown();
            }

            KeyCode::Char('h')
            | KeyCode::Char('j')
            | KeyCode::Char('k')
            | KeyCode::Char('l')
            | KeyCode::Down
            | KeyCode::Left
            | KeyCode::Up
            | KeyCode::Right => self.handle_movement_keys(key_ev.code)?,

            _ => {}
        };

        Ok(())
    }

    fn handle_movement_keys(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Char('h') | KeyCode::Left => self.c.move_left(),
            KeyCode::Char('j') | KeyCode::Down => self.c.move_bottom(),
            KeyCode::Char('k') | KeyCode::Up => self.c.move_top(),
            KeyCode::Char('l') | KeyCode::Right => self.c.move_right(),

            _ => unreachable!(),
        }

        Ok(())
    }
}

impl View for MonthlyView {
    fn update(&mut self) {
        let (first_day_idx, days_in_month) = month_info(self.curr_year, self.curr_month);
        self.first_day = first_day_idx;
        self.month_len = days_in_month;

        self.c.set_max_day(self.month_len);
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
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
                .fg(Color::Green)
                .style(title_style(self.focused)),
        );

        let block = Block::new().title(title.alignment(Alignment::Center));
        let mut inner_area = block.inner(area);

        // little offset
        inner_area.y += 1;
        inner_area.height -= 1;

        block.render(area, buf);

        self.render_main_grid(inner_area, buf);
    }

    fn handle_event(&mut self, e: &Event) -> Result<()> {
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => {
                self.handle_key_press_ev(key_ev)
            }
            _ => Ok(()),
        }
    }
}

impl Focusable for MonthlyView {
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

impl FocusableView for MonthlyView {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()> {
        if self.focused {
            self.handle_event(e)
        } else {
            Ok(())
        }
    }
}
