use ratatui::crossterm::event::{Event, KeyCode};

use crate::{
    state::AppState,
    ui::{
        common::{
            focusable::{FocusStatus, Focusable},
            view::{
                FocusableView, FocusableViewWithCursorControl, Resettable, View,
                ViewWithCursorControl,
            },
        },
        components::{input::UserInput, popup_with_cursor::Popup},
    },
};

pub struct IcsPopupContent {
    state: AppState,
    input: UserInput,
}

impl View for IcsPopupContent {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        match e {
            Event::Key(key_ev) => match key_ev.code {
                KeyCode::Enter => {
                    self.state.state.try_borrow_mut()?.tmp =
                        Some(String::from(self.input.content()))
                }

                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn update(&mut self) {
        self.input.update();
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.input.render(area, buf);
    }
}
impl Focusable for IcsPopupContent {
    fn focus(&mut self) {
        self.input.focus();
    }

    fn unfocus(&mut self) {
        self.input.unfocus();
    }

    fn toggle_focus(&mut self) {
        self.input.toggle_focus();
    }
}
impl FocusableView for IcsPopupContent {
    fn handle_event_if_focused(
        &mut self,
        e: &ratatui::crossterm::event::Event,
    ) -> anyhow::Result<()> {
        if self.input.is_focused() {
            self.handle_event(e)?
        }
        self.input.handle_event_if_focused(e)
    }
}
impl ViewWithCursorControl for IcsPopupContent {
    fn render_with_cursor(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        self.input.render_with_cursor(area, buf, set_cursor);
    }
}
impl FocusableViewWithCursorControl for IcsPopupContent {}
impl Resettable for IcsPopupContent {
    fn reset(&mut self) -> anyhow::Result<()> {
        self.input.reset()
    }
}

pub fn new_ics_popup<'a>(state: AppState) -> Popup<'a, IcsPopupContent> {
    Popup::new(
        "Your .ics file",
        IcsPopupContent {
            state,
            input: UserInput::new(""),
        },
        None,
        None,
    )
}
