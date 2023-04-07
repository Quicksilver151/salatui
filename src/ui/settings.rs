use crate::*;

pub fn get_settings_layout<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal) .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
Constraint::Percentage(25),
        ].as_ref()
    ).split(f.size())
}

