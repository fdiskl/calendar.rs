use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::Backend,
    widgets::Widget,
};

use crate::views::{
    common::{layout::render_layout, view::View, view_switcher::ViewSwitcher},
    daily::DailyView,
    journal::Journal,
    monthly::MonthlyView,
};

enum AppState {
    Running,
    Exiting,
}

#[derive(Default)]
enum Focused {
    Journal,
    #[default]
    MainView,
}

pub struct App<'a> {
    state: AppState,

    focused: Focused,
    main_view: ViewSwitcher<'a>,

    journal: Journal,
}

impl<'a> App<'a> {
    pub fn new(daily_view: &'a mut dyn View, monthly_view: &'a mut dyn View) -> Self {
        Self {
            state: AppState::Running,
            journal: Journal::new(),
            focused: Focused::default(),
            main_view: ViewSwitcher::new('v').with_views(vec![daily_view, monthly_view]),
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
        match self.focused {
            Focused::Journal => todo!(),
            Focused::MainView => self.main_view.update(),
        }

        term.draw(|frame| self.draw(frame))?;

        self.handle_events()?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let e = event::read()?;
        match e {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => match key_ev.code {
                KeyCode::Char('q') => self.exit(),
                _ => {}
            },

            _ => {}
        }

        match self.focused {
            Focused::Journal => todo!(),
            Focused::MainView => self.main_view.handle_event(e),
        }
    }

    fn exit(&mut self) {
        self.state = AppState::Exiting;
    }

    fn change_focus(&mut self) {
        match self.focused {
            Focused::Journal => self.focused = Focused::MainView,
            Focused::MainView => self.focused = Focused::Journal,
        }
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
