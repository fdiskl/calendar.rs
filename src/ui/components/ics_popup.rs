use crate::ui::{
    common::{
        focusable::Focusable,
        view::{
            FocusableView, FocusableViewWithCursorControl, Resettable, View, ViewWithCursorControl,
        },
    },
    components::popup_with_cursor::Popup,
};

pub struct IcsPopupContent {}

impl View for IcsPopupContent {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        todo!()
    }
}
impl Focusable for IcsPopupContent {
    fn focus(&mut self) {
        todo!()
    }

    fn unfocus(&mut self) {
        todo!()
    }

    fn toggle_focus(&mut self) {
        todo!()
    }
}
impl FocusableView for IcsPopupContent {
    fn handle_event_if_focused(
        &mut self,
        e: &ratatui::crossterm::event::Event,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
impl ViewWithCursorControl for IcsPopupContent {
    fn render_with_cursor(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        todo!()
    }
}
impl FocusableViewWithCursorControl for IcsPopupContent {}
impl Resettable for IcsPopupContent {
    fn reset(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}

pub fn new_ics_popup<'a>() -> Popup<'a, IcsPopupContent> {
    Popup::new("Your .ics file", IcsPopupContent {}, None, None)
}
