use super::*;

pub fn new_color_block(title: &str, color: Color) -> Block {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain).border_style(Style::default().fg(color));
    block
}

