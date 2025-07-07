use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, Widget},
};

use crate::ui::common::{
    focusable::Focusable,
    view::{FocusableView, View},
};

// Popup open state can be controlled using Focusable trait
pub struct Popup<'a, V: View> {
    title: &'a str,
    content: V,
    border_style: Style,
    title_style: Style,
    open: bool,
}

impl<'a, V: View> Popup<'a, V> {
    pub fn new(
        title: &'a str,
        content: V,
        border_style: Option<Style>,
        title_style: Option<Style>,
    ) -> Self {
        Self {
            open: false,
            title,
            content,
            border_style: match border_style {
                Some(v) => v,
                None => Style::default(),
            },
            title_style: match title_style {
                Some(v) => v,
                None => Style::default().bold(),
            },
        }
    }
}

impl<V: View> View for Popup<'_, V> {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => match key_ev.code {
                KeyCode::Char('q') | KeyCode::Esc => self.unfocus(),
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn update(&mut self) {
        self.content.update();
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);

        let inner = block.inner(area);
        self.content.render(inner, buf);
    }
}

impl<V: View> Focusable for Popup<'_, V> {
    fn focus(&mut self) {
        self.open = true;
    }

    fn unfocus(&mut self) {
        self.open = false;
    }

    fn toggle_focus(&mut self) {
        self.open = !self.open;
    }
}

impl<V: View> FocusableView for Popup<'_, V> {
    fn handle_event_if_focused(&mut self, e: &Event) -> anyhow::Result<()> {
        if self.open {
            self.handle_event(e)?;
            self.content.handle_event(e)
        } else {
            Ok(())
        }
    }
}
