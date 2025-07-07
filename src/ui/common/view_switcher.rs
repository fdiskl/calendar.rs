use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    widgets::Widget,
};

use crate::ui::common::{
    focusable::Focusable,
    view::{FocusableView, View},
};

use anyhow::Result;

pub struct ViewSwitcher<'a> {
    curr_view_idx: usize,
    switch_char: char,

    views: Vec<&'a mut dyn FocusableView>,

    focused: bool,
}

impl<'a> ViewSwitcher<'a> {
    pub fn new(switch_char: char) -> Self {
        Self {
            curr_view_idx: 0,
            switch_char,
            views: vec![],
            focused: false,
        }
    }

    pub fn with_views(mut self, views: Vec<&'a mut dyn FocusableView>) -> Self {
        self.views = views;
        self
    }

    fn curr_view(&self) -> &dyn FocusableView {
        return self.views[self.curr_view_idx];
    }

    fn mut_curr_view(&mut self) -> &mut dyn FocusableView {
        return self.views[self.curr_view_idx];
    }

    fn next_view(&mut self) {
        self.curr_view_idx += 1;

        if self.curr_view_idx >= self.views.len() {
            self.curr_view_idx = 0;
        }
    }
}

impl<'a> View for ViewSwitcher<'a> {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.curr_view().render(area, buf);
    }

    fn handle_event(&mut self, e: &Event) -> Result<()> {
        // handle view switching
        if let Event::Key(key_ev) = e {
            if key_ev.kind == KeyEventKind::Press {
                if let KeyCode::Char(v) = key_ev.code {
                    if v == self.switch_char {
                        self.next_view();
                    }
                }
            }
        }
        Ok(())
    }

    fn update(&mut self) {
        self.mut_curr_view().update();
    }
}

impl<'a> Focusable for ViewSwitcher<'a> {
    fn focus(&mut self) {
        self.focused = true;
        self.mut_curr_view().focus();
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.mut_curr_view().unfocus();
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused;
        self.mut_curr_view().toggle_focus();
    }
}

impl<'a> FocusableView for ViewSwitcher<'a> {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()> {
        if self.focused {
            self.handle_event(e)?
        }
        self.mut_curr_view().handle_event_if_focused(e)
    }
}
