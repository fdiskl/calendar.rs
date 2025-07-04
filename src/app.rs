use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::Backend,
    widgets::Widget,
};

use crate::views::{
    common::{
        focusable::Focusable,
        layout::render_layout,
        view::{FocusableView, View},
        view_switcher::ViewSwitcher,
    },
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
}

impl<'a> App<'a> {
    pub fn new(daily_view: &'a mut dyn View, monthly_view: &'a mut dyn View) -> Self {
        Self {
            state: AppState::Running,
            journal: Journal::new(),
            main_view: ViewSwitcher::new('v')
                .with_views(vec![daily_view, monthly_view])
                .focused(),
        }
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
                _ => {}
            },

            _ => {}
        }

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
        frame.render_widget(self, frame.area());
    }
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        render_layout(area, buf, &self.main_view, &self.journal);
    }
}
