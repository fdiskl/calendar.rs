use anyhow::Result;

use ratatui::{buffer::Buffer, crossterm::event::Event, layout::Rect};

use crate::ui::common::focusable::Focusable;

// extended widget trait
pub trait View {
    fn handle_event(&mut self, e: &Event) -> Result<()>;
    fn update(&mut self);
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait FocusableView: View + Focusable {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()>;
}

pub trait ViewWithCursorControl: View {
    fn render_with_cursor(
        &self,
        area: Rect,
        buf: &mut Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    );
}
