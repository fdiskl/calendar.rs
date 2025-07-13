pub trait Focusable {
    fn focus(&mut self);
    fn unfocus(&mut self);

    fn toggle_focus(&mut self);
}

pub trait FocusStatus {
    fn is_focused(&self) -> bool;
}
