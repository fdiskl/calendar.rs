use anyhow::Result;

use ratatui::{buffer::Buffer, crossterm::event::Event, layout::Rect};

// extended widget trait
pub trait View {
    fn handle_event(&mut self, e: Event) -> Result<()>;
    fn update(&mut self);
    fn render(&self, area: Rect, buf: &mut Buffer);
}
