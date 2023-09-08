use super::*;

pub fn get_settings_layout(rect: Rect, container_size: [f64;4]) -> Vec<Rect> {

    
    let total = container_size[0]/100.0 + container_size[1]/100.0 + container_size[2]/100.0 + container_size[3]/100.0;
    
    let containers : Vec<u16> = container_size.iter().map(|x| (x/total) as u16 ).collect::<Vec<u16>>();
    
    
    // dbg!(total);
    // dbg!(&containers);
    
    Layout::default()
        .direction(Direction::Horizontal) .constraints([
            Constraint::Percentage(containers[0]),
            Constraint::Percentage(containers[1]),
            Constraint::Percentage(containers[2]),
            Constraint::Percentage(containers[3]),
        ].as_ref()
    ).split(rect).to_vec()
}



pub fn draw_settings<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState, ui_state: &mut UIState) {
    use input::*;
    // match app_state.input_map.get_key().unwrap_or_default() {
    //     (Key::Escape, _) => app_state.screen = Screen::Menu,
    //     _ => {},
    // }
    
    ui_state.set_header("Settings");
    ui_state.set_footer(
        vec![
        ["esc","ape"],
        ["q","uit"],

    ]);
    
    let layouts = get_settings_layout(ui_state.main.rect, [1.0,1.0,1.0,1.0]);
    
    let a = new_color_block("aaaaa", Color::Red);
    let b = new_color_block("bbbbb", Color::Red);
    let c = new_color_block("ccccc", Color::Red);
    let d = new_color_block("ddddd", Color::Red);
    
    
    f.render_widget(a, layouts[0]);
    f.render_widget(b, layouts[1]);
    f.render_widget(c, layouts[2]);
    f.render_widget(d, layouts[3]);
    
}
