use anyhow::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::prelude::Widget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::common::focusable::{FocusStatus, Focusable};
use crate::ui::common::view::{
    FocusableView, FocusableViewWithCursorControl, Resettable, View, ViewWithCursorControl,
};

pub struct UserInput {
    title: String,
    input: String,
    cursor_position: usize,
    focused: bool,
}

impl UserInput {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            input: String::new(),
            cursor_position: 0,
            focused: true,
        }
    }

    pub fn content(&self) -> &str {
        &self.input
    }
}

impl View for UserInput {
    fn handle_event(&mut self, e: &Event) -> Result<()> {
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => {
                match key_ev.code {
                    KeyCode::Char(c) => {
                        if key_ev.modifiers.contains(KeyModifiers::CONTROL) {
                            // ignore control + char input
                        } else {
                            self.input.insert(self.cursor_position, c);
                            self.cursor_position += 1;
                        }
                    }
                    KeyCode::Backspace => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                            self.input.remove(self.cursor_position);
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor_position < self.input.len() {
                            self.cursor_position += 1;
                        }
                    }
                    KeyCode::Home => {
                        self.cursor_position = 0;
                    }
                    KeyCode::End => {
                        self.cursor_position = self.input.len();
                    }
                    _ => {}
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self) {}

    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_with_cursor(area, buf, &mut |_x, _y| {});
    }
}

impl ViewWithCursorControl for UserInput {
    fn render_with_cursor(
        &self,
        area: Rect,
        buf: &mut Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        let text = Span::raw(&self.input);
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_style(Style::default());

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default())
            .scroll((0, 0));

        paragraph.render(area, buf);

        if self.focused {
            let x = area.x + self.cursor_position as u16 + 1;
            let y = area.y + 1;
            set_cursor(x, y)
        }
    }
}

impl Focusable for UserInput {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused;
    }
}

impl FocusableView for UserInput {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()> {
        if self.focused {
            self.handle_event(e)
        } else {
            Ok(())
        }
    }
}

impl FocusableViewWithCursorControl for UserInput {}

impl Resettable for UserInput {
    fn reset(&mut self) -> Result<()> {
        self.input = String::new();
        self.cursor_position = 0;

        Ok(())
    }
}

impl FocusStatus for UserInput {
    fn is_focused(&self) -> bool {
        self.focused
    }
}
