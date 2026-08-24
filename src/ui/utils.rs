use super::*;

pub fn get_settings_layout(rect: Rect, container_size: [f64; 4]) -> Vec<Rect> {
    let total = container_size[0]/100.0 + container_size[1]/100.0 + container_size[2]/100.0 + container_size[3]/100.0;

    let containers : Vec<u16> = container_size.iter().map(|x| (x/total) as u16 ).collect::<Vec<u16>>();

    Layout::default()
        .direction(Direction::Horizontal) .constraints([
            Constraint::Percentage(containers[0]),
            Constraint::Percentage(containers[1]),
            Constraint::Percentage(containers[2]),
            Constraint::Percentage(containers[3]),
        ].as_ref()
    ).split(rect).to_vec()
}

pub fn new_color_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain).border_style(Style::default().fg(color))
}

