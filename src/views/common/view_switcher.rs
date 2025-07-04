use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    widgets::Widget,
};

use crate::views::common::{
    focusable::Focusable,
    view::{FocusableView, View},
};

use anyhow::Result;

pub struct ViewSwitcher<'a> {
    curr_view_idx: usize,
    switch_char: char,

    views: Vec<&'a mut dyn View>,

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

    pub fn with_views(mut self, views: Vec<&'a mut dyn View>) -> Self {
        self.views = views;
        self
    }

    fn curr_view(&self) -> &dyn View {
        return self.views[self.curr_view_idx];
    }

    fn mut_curr_view(&mut self) -> &mut dyn View {
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
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => match key_ev.code {
                KeyCode::Char(v) if v == self.switch_char => {
                    self.next_view();
                }
                _ => {}
            },
            _ => {}
        }

        self.mut_curr_view().handle_event(e)
    }

    fn update(&mut self) {
        self.mut_curr_view().update();
    }
}

impl<'a> Focusable for ViewSwitcher<'a> {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused;
    }

    fn focused(mut self) -> Self {
        self.focused = true;
        self
    }
}

impl<'a> FocusableView for ViewSwitcher<'a> {
    fn handle_event_if_focused(&mut self, e: &Event) -> Result<()> {
        if self.focused {
            self.handle_event(e)
        } else {
            Ok(())
        }
    }
}
