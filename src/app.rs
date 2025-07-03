use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::Backend,
    widgets::Widget,
};

use crate::views::{
    common::{journal::Journal, layout::render_layout},
    daily::DailyView,
    monthly::MonthlyView,
};

enum AppState {
    Running,
    Exiting,
}

#[derive(Default)]
enum AppView {
    Daily,
    #[default]
    Monthly,
}

pub struct App {
    state: AppState,
    curr_view: AppView,

    daily: DailyView,
    monthly: MonthlyView,

    journal: Journal,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Running,
            curr_view: AppView::default(),
            daily: DailyView::new(),
            monthly: MonthlyView::new(),
            journal: Journal::new(),
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
        term.draw(|frame| self.draw(frame))?;
        self.handle_events()?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => {
                self.handle_key_press_ev(key_ev)?
            }

            _ => {}
        }
        Ok(())
    }

    fn handle_key_press_ev(&mut self, key_ev: KeyEvent) -> Result<()> {
        match key_ev.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('v') => self.change_mode(),
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.state = AppState::Exiting;
    }

    fn change_mode(&mut self) {
        match self.curr_view {
            AppView::Daily => self.curr_view = AppView::Monthly,
            AppView::Monthly => self.curr_view = AppView::Daily,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self.curr_view {
            AppView::Daily => render_layout(area, buf, &self.daily, &self.journal),
            AppView::Monthly => render_layout(area, buf, &self.monthly, &self.journal),
        }
    }
}
