use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Alignment,
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, Widget},
};

use crate::ui::common::{
    focusable::Focusable,
    view::{FocusableView, View},
};

// Popup open state can be controlled using Focusable trait
pub struct Popup<'a, V: FocusableView> {
    title: &'a str,
    content: V,
    border_style: Style,
    title_style: Style,
    open: bool,
}

impl<'a, V: FocusableView> Popup<'a, V> {
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

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn title_style(&self) -> Style {
        self.title_style
    }

    pub fn border_style(&self) -> Style {
        self.border_style
    }

    pub fn content(&self) -> &V {
        &self.content
    }

    pub fn open(&self) -> bool {
        self.open
    }
}

impl<V: FocusableView> View for Popup<'_, V> {
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self) {
        self.content.update();
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
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

impl<V: FocusableView> Focusable for Popup<'_, V> {
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

impl<V: FocusableView> FocusableView for Popup<'_, V> {
    fn handle_event_if_focused(&mut self, e: &Event) -> anyhow::Result<()> {
        if self.open {
            self.handle_event(e)?;
        }
        self.content.handle_event_if_focused(e)
    }
}
