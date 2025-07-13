use ratatui::{
    crossterm::event::KeyCode,
    widgets::{Block, Widget, block::title},
};

use crate::{
    state::AppState,
    ui::{
        common::{
            focusable::Focusable,
            view::{
                FocusableView, FocusableViewWithCursorControl, Resettable, View,
                ViewWithCursorControl,
            },
        },
        components::{input::UserInput, popup_with_cursor::Popup},
    },
};

pub struct TmpPopupContent {
    state: AppState,
    f: bool,
}

impl View for TmpPopupContent {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self) {}

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let state_ref = self.state.state.borrow();
        let title = state_ref.tmp.as_deref().unwrap_or("none");
        let block = Block::new().title(title);

        block.render(area, buf);
    }
}
impl Focusable for TmpPopupContent {
    fn focus(&mut self) {
        self.f = true;
    }

    fn unfocus(&mut self) {
        self.f = false;
    }

    fn toggle_focus(&mut self) {
        self.f = !self.f;
    }
}
impl FocusableView for TmpPopupContent {
    fn handle_event_if_focused(
        &mut self,
        e: &ratatui::crossterm::event::Event,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
impl ViewWithCursorControl for TmpPopupContent {
    fn render_with_cursor(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        self.render(area, buf);
    }
}
impl FocusableViewWithCursorControl for TmpPopupContent {}
impl Resettable for TmpPopupContent {
    fn reset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub fn new_tmp_popup<'a>(state: AppState) -> Popup<'a, TmpPopupContent> {
    Popup::new(
        "Your .ics file",
        TmpPopupContent { state, f: false },
        None,
        None,
    )
}
