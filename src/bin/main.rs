use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use taskrs::{
    app::App,
    state::{AppState, State},
    ui::{daily::DailyView, monthly::MonthlyView},
};

fn main() -> Result<()> {
    let mut term = ratatui::init();

    let state = AppState {
        state: Rc::new(RefCell::new(State {
            calendar: None,
            tmp: None,
        })),
    };

    let mut daily_view = DailyView::new();
    let mut monthly_view = MonthlyView::new();
    let mut app = App::new(&mut monthly_view, &mut daily_view, state);

    let res = app.run(&mut term);

    ratatui::restore();

    res
}
