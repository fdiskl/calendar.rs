use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Position, Rect},
    prelude::Backend,
    widgets::Widget,
};

use crate::{
    state::AppState,
    ui::{
        common::view::{FocusableView, View, ViewWithCursorControl},
        components::{
            ics_popup::new_ics_popup, input_popup::InputPopup, layout::Layout,
            popup_host::PopupHost, tmp_popup::new_tmp_popup, view_switcher::ViewSwitcher,
        },
        journal::Journal,
    },
};

enum AppStatus {
    Running,
    Exiting,
}

pub struct App<'a> {
    status: AppStatus,
    state: AppState,

    main: PopupHost<Layout<ViewSwitcher<'a>, Journal>>,
}

impl<'a> App<'a> {
    pub fn new(
        daily_view: &'a mut dyn FocusableView,
        monthly_view: &'a mut dyn FocusableView,
        state: AppState,
    ) -> Self {
        let s = Self {
            status: AppStatus::Running,
            state: state.clone(),
            main: PopupHost::new(Layout::new(
                ViewSwitcher::new('v').with_views(vec![daily_view, monthly_view]),
                Journal::new(),
            ))
            .with_popups(
                vec![
                    Box::new(new_ics_popup(state.clone())),
                    Box::new(new_tmp_popup(state.clone())),
                ],
                vec![KeyCode::Char('i'), KeyCode::Char('j')],
            ),
        };

        s
    }

    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<()> {
        loop {
            match self.status {
                AppStatus::Running => self.update(term)?,
                AppStatus::Exiting => return Ok(()),
            }
        }
    }

    fn update<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<()> {
        self.main.update();

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

        self.main.handle_event(&e)
    }

    fn exit(&mut self) {
        self.status = AppStatus::Exiting;
    }

    fn draw(&self, frame: &mut Frame) {
        let mut cursor_pos = None;

        self.main
            .render_with_cursor(frame.area(), frame.buffer_mut(), &mut |x, y| {
                cursor_pos = Some(Position::new(x, y))
            });

        if let Some(pos) = cursor_pos {
            frame.set_cursor_position(pos);
        }
    }
}
