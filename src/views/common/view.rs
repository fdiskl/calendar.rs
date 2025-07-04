use anyhow::Result;

use ratatui::{buffer::Buffer, crossterm::event::Event, layout::Rect};

use crate::views::common::focusable::Focusable;

// extended widget trait
pub trait View {
    fn handle_event(&mut self, e: &Event) -> Result<()>;
    fn update(&mut self);
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait FocusableView: View + Focusable {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()>;
}
