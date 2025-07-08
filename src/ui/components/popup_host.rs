use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::ui::common::view::{
    FocusableView, FocusableViewWithCursorControl, PopupView, View, ViewWithCursorControl,
};

pub struct PopupHost<V>
where
    V: FocusableView,
{
    inner: V,
    popups: Vec<Box<dyn PopupView>>,

    popups_triggers: Vec<KeyCode>,

    active_popup: Option<usize>,
}

impl<V> PopupHost<V>
where
    V: FocusableView,
{
    pub fn new(inner: V) -> Self {
        let mut s = Self {
            active_popup: None,
            inner,
            popups: vec![],
            popups_triggers: vec![],
        };

        s.inner.focus();

        s
    }

    pub fn with_popups(
        mut self,
        popups: Vec<Box<dyn PopupView>>,
        popups_triggers: Vec<KeyCode>,
    ) -> Self {
        self.popups = popups;
        self.popups_triggers = popups_triggers;
        self
    }

    fn show(&mut self, idx: usize) {
        if let Some(active) = self.active_popup_mut() {
            active.unfocus();
        }

        self.active_popup = Some(idx);
        self.inner.unfocus();

        if let Ok(_) = self.popups[idx].reset() {
            self.popups[idx].focus();
        }
    }

    fn hide(&mut self) {
        if let Some(active) = self.active_popup_mut() {
            active.unfocus();
        }

        self.active_popup = None;

        self.inner.focus();
    }

    fn active_popup_mut(&mut self) -> Option<&mut Box<dyn PopupView>> {
        if let Some(active_idx) = self.active_popup {
            Some(&mut self.popups[active_idx])
        } else {
            None
        }
    }

    fn active_popup(&self) -> Option<&Box<dyn PopupView>> {
        if let Some(active_idx) = self.active_popup {
            Some(&self.popups[active_idx])
        } else {
            None
        }
    }
}

impl<V: FocusableView> View for PopupHost<V> {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => match key_ev.code {
                KeyCode::Esc => self.hide(),
                k => {
                    if let None = self.active_popup {
                        if let Some(idx) = self.popups_triggers.iter().position(|&x| x == k) {
                            self.show(idx);
                            return Ok(()); // if we open popup we won't pass events down for anyone
                        }
                    }
                }
            },

            _ => {}
        }

        match self.active_popup {
            Some(v) => self.popups[v].handle_event_if_focused(e)?,
            None => self.inner.handle_event_if_focused(e)?,
        }
        Ok(())
    }

    fn update(&mut self) {
        self.inner.update();
        if let Some(active) = self.active_popup_mut() {
            active.update()
        }
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.inner.render(area, buf);
        if let Some(active) = self.active_popup() {
            active.render(area, buf)
        }
    }
}

impl<V: FocusableView> ViewWithCursorControl for PopupHost<V> {
    fn render_with_cursor(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        set_cursor: &mut dyn FnMut(u16, u16),
    ) {
        self.inner.render(area, buf);
        if let Some(active) = self.active_popup() {
            active.render_with_cursor(area, buf, set_cursor);
        }
    }
}
