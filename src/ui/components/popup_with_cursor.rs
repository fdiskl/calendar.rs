use ratatui::layout::Alignment;
use ratatui::prelude::Widget;
use ratatui::{
    crossterm::event::Event,
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Block, Borders, Clear},
};

use crate::ui::{
    common::{
        focusable::Focusable,
        view::{FocusableView, View, ViewWithCursorControl},
    },
    components::popup::Popup,
};

pub struct PopupWithCursorControl<'a, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    inner_popup: Popup<'a, V>,
}

impl<'a, V> PopupWithCursorControl<'a, V>
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
            inner_popup: Popup::new(title, content, border_style, title_style),
        }
    }
}

impl<V> View for PopupWithCursorControl<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn handle_event(&mut self, e: &Event) -> anyhow::Result<()> {
        self.inner_popup.handle_event(e)
    }

    fn update(&mut self) {
        self.inner_popup.update();
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.inner_popup.render(area, buf);
    }
}

impl<V> Focusable for PopupWithCursorControl<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn focus(&mut self) {
        self.inner_popup.focus();
    }

    fn unfocus(&mut self) {
        self.inner_popup.unfocus();
    }

    fn toggle_focus(&mut self) {
        self.inner_popup.toggle_focus();
    }
}

impl<V> FocusableView for PopupWithCursorControl<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn handle_event_if_focused(&mut self, e: &Event) -> anyhow::Result<()> {
        self.inner_popup.handle_event_if_focused(e)
    }
}

impl<V> ViewWithCursorControl for PopupWithCursorControl<'_, V>
where
    V: FocusableView + ViewWithCursorControl,
{
    fn render_with_cursor(
        &self,
        area: Rect,
        buf: &mut Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        if self.inner_popup.open() {
            Clear.render(area, buf);
            let block = Block::new()
                .title(self.inner_popup.title())
                .title_alignment(Alignment::Center)
                .title_style(self.inner_popup.title_style())
                .borders(Borders::ALL)
                .border_style(self.inner_popup.border_style());

            let inner = block.inner(area);
            block.render(area, buf);

            self.inner_popup
                .content()
                .render_with_cursor(inner, buf, set_cursor);
        }
    }
}
