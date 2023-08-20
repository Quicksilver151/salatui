use crate::*;

mod menu;
mod settings;

pub use menu::*;
pub use settings::*;
pub use tui::layout::{Layout, Direction, Constraint, Rect};


pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState){
    
    let input_map = app_state.input_map.to_owned();
    
    fn new_block(title: &str) -> Block{
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        block
    }
    
    let layouts = MainLayout::from(f, &app_state);
    let title = input_map.get_current().unwrap_or(String::new());
    let menu_block = new_block(&title);
    
    let block_1 = new_block("1");
    let block_2 = new_block("2");
    let block_3 = new_block("3");
    let block_4 = new_block("4");
    
    let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(Spans::from).collect();
    
    let tabthings = Tabs::new(titles)
    .block(Block::default().title("Tabs").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Yellow))
    .divider(DOT);
    
    // f.render_widget(block_1, layouts.settings[0]);
    // f.render_widget(block_2, layouts.settings[1]);
    // f.render_widget(block_3, layouts.settings[2]);
    // f.render_widget(block_4, layouts.settings[3]);
    // 
    // f.render_widget(tabthings, layouts.menu[0]);
    // 
    f.render_widget(block_1, layouts.title);
    f.render_widget(menu_block, layouts.salat);
}

pub struct SettingsLayout {
    
}
pub struct MainLayout {
    title: Rect,
    salat: Rect,
}
pub struct HeaderLayout {

}
pub struct FooterLayout {

}
pub enum CenterLayout{
    Main(MainLayout),
    Settings(SettingsLayout),
}
pub struct RootLayout {
    pub header: HeaderLayout,
    pub center: CenterLayout,
    pub footer: FooterLayout,
}

// changable setting ui + struct needed here Option{SettingType}; settingtype::choice(vec), string(or int)
impl MainLayout {
    /// the main layout
    pub fn from<B: Backend>(f: &mut Frame<B>, app_state: &AppState) -> MainLayout {
        
        let menu: Vec<Rect> = get_menu_layout(f);     
        
        let layouts = Layout::default()
        .direction(Direction::Vertical) .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ].as_ref()
        ).split(f.size());
        
        let salat = layouts[1];
        let title = layouts[0];
        
        MainLayout { title , salat }
    }
}






