use super::*;

pub fn new_color_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain).border_style(Style::default().fg(color))
}

