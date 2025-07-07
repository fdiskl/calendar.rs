use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Position, Rect},
    prelude::Backend,
    widgets::Widget,
};

use crate::ui::{
    common::{
        focusable::Focusable,
        layout::render_layout,
        view::{FocusableView, View, ViewWithCursorControl},
    },
    components::{input_popup::InputPopup, view_switcher::ViewSwitcher},
    journal::Journal,
};

enum AppState {
    Running,
    Exiting,
}

pub struct App<'a> {
    state: AppState,

    main_view: ViewSwitcher<'a>,

    journal: Journal,

    ics_import_modal: InputPopup<'a>,
}

impl<'a> App<'a> {
    pub fn new(
        daily_view: &'a mut dyn FocusableView,
        monthly_view: &'a mut dyn FocusableView,
    ) -> Self {
        let mut s = Self {
            state: AppState::Running,
            journal: Journal::new(),
            main_view: ViewSwitcher::new('v').with_views(vec![daily_view, monthly_view]),
            ics_import_modal: InputPopup::new_input_popup("You .ics file", None, None),
        };

        s.main_view.focus();

        s
    }

    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<()> {
        loop {
            match self.state {
                AppState::Running => self.update(term)?,
                AppState::Exiting => return Ok(()),
            }
        }
    }

    fn update<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<()> {
        self.journal.update();
        self.main_view.update();

        term.draw(|frame| self.draw(frame))?;

        self.handle_events()?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let e = event::read()?;
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => match key_ev.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char(' ') => self.change_focus(),
                KeyCode::Char('i') => self.ics_import_modal.focus(),
                _ => {}
            },

            _ => {}
        }

        self.ics_import_modal.handle_event_if_focused(&e)?;
        self.journal.handle_event_if_focused(&e)?;
        self.main_view.handle_event_if_focused(&e)
    }

    fn exit(&mut self) {
        self.state = AppState::Exiting;
    }

    fn change_focus(&mut self) {
        self.main_view.toggle_focus();
        self.journal.toggle_focus();
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let modal_width = 50;
        let modal_height = 8;

        let x = area.x + (area.width.saturating_sub(modal_width)) / 2;
        let y = area.y + (area.height.saturating_sub(modal_height)) / 2;

        let modal_area = Rect::new(x, y, modal_width, modal_height);

        frame.render_widget(self, frame.area());

        let mut cursor_pos = None;

        let buffer = frame.buffer_mut();

        self.ics_import_modal
            .render_with_cursor(modal_area, buffer, &mut |x, y| {
                cursor_pos = Some(Position::new(x, y));
            });

        if let Some(pos) = cursor_pos {
            frame.set_cursor_position(pos);
        }
    }
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        render_layout(area, buf, &self.main_view, &self.journal);
    }
}
