use std::{cell::RefCell, rc::Rc};

use icalendar::Calendar;

pub struct State {
    pub calendar: Option<Calendar>,
    pub tmp: Option<String>,
}

#[derive(Clone)]
/// global app state
pub struct AppState {
    pub state: Rc<RefCell<State>>,
}
