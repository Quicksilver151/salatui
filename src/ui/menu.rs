use crate::*;

pub fn get_menu_layout<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {

    Layout::default()
        .direction(Direction::Horizontal) .constraints([
            Constraint::Percentage(100),
        ].as_ref()
    ).split(f.size())
}
