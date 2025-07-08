use ratatui::{
    crossterm::event::Event,
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, Widget},
};

use crate::ui::common::{
    focusable::Focusable,
    view::{FocusableView, FocusableViewWithCursorControl, View, ViewWithCursorControl},
};

pub struct Popup<'a, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    title: &'a str,
    content: V,
    border_style: Style,
    title_style: Style,
    open: bool,
}

impl<'a, V> Popup<'a, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    pub fn new(
        title: &'a str,
        content: V,
        border_style: Option<Style>,
        title_style: Option<Style>,
    ) -> Self {
        Self {
            title,
            content,
            border_style: border_style.unwrap_or_default(),
            title_style: title_style.unwrap_or_else(|| Style::default().bold()),
            open: false,
        }
    }

    pub fn open(&self) -> bool {
        self.open
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn title_style(&self) -> Style {
        self.title_style
    }

    pub fn border_style(&self) -> Style {
        self.border_style
    }
}

impl<V> View for Popup<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn handle_event(&mut self, e: &Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self) {
        self.content.update();
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.open {
            Clear.render(area, buf);
            let block = Block::new()
                .title(self.title)
                .title_alignment(Alignment::Center)
                .title_style(self.title_style)
                .borders(Borders::ALL)
                .border_style(self.border_style);

            let inner = block.inner(area);
            block.render(area, buf);
            self.content.render(inner, buf);
        }
    }
}

impl<V> ViewWithCursorControl for Popup<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn render_with_cursor(
        &self,
        area: Rect,
        buf: &mut Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        if self.open {
            Clear.render(area, buf);
            let block = Block::new()
                .title(self.title)
                .title_alignment(Alignment::Center)
                .title_style(self.title_style)
                .borders(Borders::ALL)
                .border_style(self.border_style);

            let inner = block.inner(area);
            block.render(area, buf);
            self.content.render_with_cursor(inner, buf, set_cursor);
        }
    }
}

impl<V> Focusable for Popup<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn focus(&mut self) {
        self.open = true;
        self.content.focus();
    }

    fn unfocus(&mut self) {
        self.open = false;
        self.content.unfocus();
    }

    fn toggle_focus(&mut self) {
        self.open = !self.open;
        self.content.toggle_focus();
    }
}

impl<V> FocusableView for Popup<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn handle_event_if_focused(&mut self, e: &Event) -> anyhow::Result<()> {
        if self.open {
            self.handle_event(e)?;
        }
        self.content.handle_event_if_focused(e)
    }
}

impl<V> FocusableViewWithCursorControl for Popup<'_, V> where
    V: FocusableView + ViewWithCursorControl
{
}
