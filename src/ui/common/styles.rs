// Some styles used in many (2+) places in app

use ratatui::style::{Color, Modifier, Style, Stylize};

pub fn title_active_style() -> Style {
    Style::default().fg(Color::Green).bold()
}

pub fn title_not_active_style() -> Style {
    Style::default().add_modifier(Modifier::DIM)
}

pub fn title_style(focused: bool) -> Style {
    if focused {
        title_active_style()
    } else {
        title_not_active_style()
    }
}
