use crate::*;

mod calender;
mod menu;
mod settings;
mod utils;

use calender::*;
use menu::*;
use settings::*;
use utils::*;



pub use tui::{
    layout::{Alignment, Layout, Direction, Constraint, Rect},
    text::Span,
    style::Modifier
};

#[derive(Default, Debug)]
struct Header {
    rect: Rect,
    text: String,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Screen {
    #[default]
    Menu,
    Settings,
    Calender,
}

#[derive(Default, Debug)]
struct Main {
    rect: Rect,
    screen: Screen,
}

#[derive(Default, Debug)]
struct Footer {
    commands: Vec<[String;2]>,
    rect: Rect,
}

#[derive(Default, Debug)]
pub struct UIState {
    header:Header,
    main: Main,
    footer: Footer,
}
impl UIState {
    pub fn set_header(&mut self, header_text: &str) {
        self.header.text = header_text.to_string();
    }
    pub fn set_screen(&mut self, screen: Screen) {
        self.main.screen = screen;
    }
    pub fn set_footer(&mut self, commands: Vec<[&str;2]>) {
        self.footer.commands = commands.iter().map(|v| [v[0].to_string(), v[1].to_string()]).collect();
    }
    
    
    pub fn set_header_rect(&mut self, rect: Rect) {
        self.header.rect = rect;
    }
    pub fn set_screen_rect(&mut self, rect: Rect) {
        self.main.rect = rect;
    }
    pub fn get_screen_rect(&mut self) -> Rect {
        self.main.rect
    }
    pub fn set_footer_rect(&mut self, rect: Rect) {
        self.footer.rect = rect;
    }
    
    pub fn get_footer_line(&self) -> Line {
        let mut spans = vec![];
        for letters in self.footer.commands.iter() {
            spans.append(&mut vec![
                Span::styled(letters[0].to_owned(), Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(letters[1].to_owned(), Style::default()),
            ]);
            if self.footer.commands.iter().last().unwrap() == letters {
                continue;
            }
            spans.append(&mut vec![
                Span::styled(" | ", Style::default()),
            ])
        }
        
        Line::from(spans)
    }

    pub fn get_header_line(&self) -> Line {
        Line::from(self.header.text.to_owned())
    }
    
    pub fn render_screen<B: Backend>(f: &mut Frame<B> ) {
        
    }
}


pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState){
    // inits
    let mut ui_state = UIState::default();
    ui_state.set_screen(app_state.screen);
    
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






