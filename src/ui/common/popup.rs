use crate::ui::common::{
    focusable::Focusable,
    view::{FocusableView, FocusableViewWithCursorControl, View},
};

pub enum MultiPopup {
    P(Box<dyn FocusableView>),
    C(Box<dyn FocusableViewWithCursorControl>),
}

impl View for MultiPopup {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        match self {
            MultiPopup::P(p) => p.handle_event(e),
            MultiPopup::C(c) => c.handle_event(e),
        }
    }

    fn update(&mut self) {
        match self {
            MultiPopup::P(p) => p.update(),
            MultiPopup::C(c) => c.update(),
        }
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self {
            MultiPopup::P(p) => p.render(area, buf),
            MultiPopup::C(c) => c.render(area, buf),
        }
    }
}

impl Focusable for MultiPopup {
    fn focus(&mut self) {
        match self {
            MultiPopup::P(p) => p.focus(),
            MultiPopup::C(c) => c.focus(),
        }
    }

    fn unfocus(&mut self) {
        match self {
            MultiPopup::P(p) => p.unfocus(),
            MultiPopup::C(c) => c.unfocus(),
        }
    }

    fn toggle_focus(&mut self) {
        match self {
            MultiPopup::P(p) => p.toggle_focus(),
            MultiPopup::C(c) => c.toggle_focus(),
        }
    }
}

impl FocusableView for MultiPopup {
    fn handle_event_if_focused(
        &mut self,
        e: &ratatui::crossterm::event::Event,
    ) -> anyhow::Result<()> {
        match self {
            MultiPopup::P(p) => p.handle_event_if_focused(e),
            MultiPopup::C(c) => c.handle_event_if_focused(e),
        }
    }
}
