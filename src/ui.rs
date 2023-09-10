use crate::*;

mod calender;
mod menu;
mod settings;
mod utils;
mod structs;

use calender::*;
use menu::*;
use settings::*;
use utils::*;
pub use structs::*;


pub use tui::{
    layout::{Alignment, Layout, Direction, Constraint, Rect},
    text::Span,
    style::Modifier
};


pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState){
    // inits
    let mut ui_state = UIState::default();
    ui_state.set_screen(app_state.screen);
    ui::structs::Screen::Menu;
    let root_container:RootContainer = RootContainer::new(f.size());
    
    // fullscreen area toggle
    if app_state.fullscreen {
        ui_state.set_screen_rect(f.size());
    } else {
        ui_state.set_header_rect(root_container.header);
        ui_state.set_screen_rect(root_container.center);
        ui_state.set_footer_rect(root_container.footer);
    }
    
    // center display
    match ui_state.main.screen {
        Screen::Menu => draw_menu(f, app_state, &mut ui_state),
        Screen::Settings => draw_settings(f, app_state, &mut ui_state),
        _ => todo!("make other screens"),
    }
    
    // header and footer
    let header_line: Line = ui_state.get_header_line();
    let header_block: Block = new_color_block("header", Color::DarkGray);
    let header_widget: Paragraph = Paragraph::new(header_line).block(header_block);
    
    let footer_line: Line = ui_state.get_footer_line();
    let footer_block: Block = new_color_block("commands",Color::DarkGray);
    let footer_widget: Paragraph = Paragraph::new(footer_line).block(footer_block);
    
    
    f.render_widget(header_widget, ui_state.header.rect);
    f.render_widget(footer_widget, ui_state.footer.rect);
}



#[derive(Debug, Default)]
pub struct CalenderContainer {

}

#[derive(Debug, Default)]
pub struct SettingsContainer {
    
}
#[derive(Debug, Default)]
pub struct MainContainer {
    title: Rect,
    salat: Rect,
}

#[derive(Debug, Default)]
pub struct RootContainer {
    pub header: Rect,
    pub center: Rect,
    pub footer: Rect,
}
impl RootContainer {
    fn new(rect: Rect) -> Self {
        let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ]).split(rect);
        
        RootContainer { header: layouts[0], center: layouts[1], footer: layouts[2] }
    }
}

// changable setting ui + struct needed here Option{SettingType}; settingtype::choice(vec), string(or int)
impl MainContainer {
    /// the main layout
    pub fn from(area: Rect) -> MainContainer {
        
        let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
        ]).split(area);
        
        let salat = layouts[1];
        let title = layouts[0];
        
        MainContainer { title , salat }
    }
}






