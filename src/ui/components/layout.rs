use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

use crate::ui::common::{
    focusable::{FocusStatus, Focusable},
    view::{FocusableView, View},
};

pub struct Layout<V, J>
where
    V: FocusableView + FocusStatus,
    J: FocusableView + FocusStatus,
{
    v: V,
    j: J,

    focused: bool,

    was_v_focused: bool,
}

impl<V, J> Layout<V, J>
where
    V: FocusableView + FocusStatus,
    J: FocusableView + FocusStatus,
{
    pub fn new(v: V, j: J) -> Self {
        Self {
            v,
            j,
            focused: false,
            was_v_focused: true,
        }
    }

    fn change_focus(&mut self) {
        self.v.toggle_focus();
        self.j.toggle_focus();
    }
}

impl<V, J> View for Layout<V, J>
where
    V: FocusableView + FocusStatus,
    J: FocusableView + FocusStatus,
{
    fn handle_event(&mut self, e: &ratatui::crossterm::event::Event) -> anyhow::Result<()> {
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => match key_ev.code {
                KeyCode::Char(' ') => self.change_focus(),
                _ => {}
            },

            _ => {}
        }
        Ok(())
    }

    fn update(&mut self) {
        self.v.update();
        self.j.update();
    }

    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.v.render(
            Rect::new(area.x, area.y, area.width * 3 / 4, area.height),
            buf,
        );

        render_vertical_line(
            Rect::new(area.x + area.width * 3 / 4, area.y, 1, area.height),
            buf,
        );

        self.j.render(
            Rect::new(
                area.x + area.width * 3 / 4 + 1,
                area.y,
                area.width / 4,
                area.height,
            ),
            buf,
        );
    }
}

impl<V, J> Focusable for Layout<V, J>
where
    V: FocusableView + FocusStatus,
    J: FocusableView + FocusStatus,
{
    fn focus(&mut self) {
        self.focused = true;
        if self.was_v_focused {
            self.j.unfocus();
            self.v.focus();
        } else {
            self.v.unfocus();
            self.j.focus();
        }
    }

    fn unfocus(&mut self) {
        self.was_v_focused = self.v.is_focused();

        self.focused = false;
        self.v.unfocus();
        self.j.unfocus();
    }

    fn toggle_focus(&mut self) {
        if self.focused {
            self.unfocus();
        } else {
            self.focus();
        }
    }
}

impl<V, J> FocusableView for Layout<V, J>
where
    V: FocusableView + FocusStatus,
    J: FocusableView + FocusStatus,
{
    fn handle_event_if_focused(
        &mut self,
        e: &ratatui::crossterm::event::Event,
    ) -> anyhow::Result<()> {
        if self.focused {
            self.handle_event(e)?;
            self.v.handle_event_if_focused(e)?;
            self.j.handle_event_if_focused(e)?;
        }

        Ok(())
    }
}

fn render_vertical_line(area: Rect, buf: &mut ratatui::prelude::Buffer) {
    let height = area.height as usize;
    let vertical_line: String = std::iter::repeat("│\n").take(height).collect();

    let line_paragraph = Paragraph::new(vertical_line).block(Block::default());

    line_paragraph.render(area, buf);
}
