use crate::*;

mod menu;
mod settings;

pub use menu::*;
pub use settings::*;
use tui::layout::{Alignment, Corner};
pub use tui::{
    layout::{Layout, Direction, Constraint, Rect},
    text::Span,
    style::Modifier
};


pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState){
    
    let input_map = app_state.input_map.to_owned();
    
    fn new_block(title: &str) -> Block {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);
        block
    }
    fn new_block_no_outline(title: &str) -> Block {
        let block = Block::default()
            .title(title)
            .borders(Borders::NONE);
        block
    }
    
    
    fn create_spans(text:Vec<[&str;2]>) -> Spans {
        let mut spans = vec![];
        for letters in text.iter() {
            spans.append(&mut vec![
                Span::styled(letters[0], Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(letters[1], Style::default()),
            ]);
            if text.iter().last().unwrap() == letters {
                continue;
            }
            spans.append(&mut vec![
                Span::styled(" | ", Style::default()),
            ])
        }
        
        Spans::from(spans)
    }
    
    
    let root_container:RootContainer = RootContainer::new(f);
    let layouts = if app_state.fullscreen {
        MainContainer::from(f.size())
    } else {
        MainContainer::from(root_container.center)
    };
    
    let title = input_map.get_current().unwrap_or(String::new());
    let menu_block = new_block(&title);
    let header = new_block("");
    let block_2 = new_block("");
    let block_3 = new_block("main");
    let commands_block: Block = new_block("commands");
    
    
    
    let footer = vec![
        ["q", "uit"],
        ["c", "onfig"],
        ["f", "ullscreen"],
    ];
    let prayertime = app_state.timeset_data.today_data();
    let prayer_times = prayertime.format_time(&app_state.config);
    
    let text = create_spans(footer);
    let menu_list = tui::widgets::List::new(vec![
        ListItem::new(format!("Fajr:   {}", prayer_times[0])),
        ListItem::new(format!("Sun:    {}", prayer_times[1])),
        ListItem::new(format!("Dhuhur: {}", prayer_times[2])),
        ListItem::new(format!("Asr:    {}", prayer_times[3])),
        ListItem::new(format!("Magrib: {}", prayer_times[4])),
        ListItem::new(format!("Isha:   {}", prayer_times[5])),
    ])
        .block(menu_block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ")
        .style(Style::default());
    
    ListState::default().select(Some(1));
    let footer = tui::widgets::Paragraph::new(text).block(commands_block);
    // widgets::Paragrapha
    
    // let titles:Vec<Spans> = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(Spans::from).collect();
    
    // let tabthings = Tabs::new(titles)
    // .block(Block::default().title("Tabs").borders(Borders::ALL))
    // .style(Style::default().fg(Color::White))
    // .highlight_style(Style::default().fg(Color::Yellow))
    // .divider(DOT);
    
    // f.render_widget(block_1, layouts.settings[0]);
    // f.render_widget(block_2, layouts.settings[1]);
    // f.render_widget(block_3, layouts.settings[2]);
    // f.render_widget(block_4, layouts.settings[3]);
    // 
    // f.render_widget(tabthings, layouts.menu[0]);

    f.render_widget(block_2, layouts.title);
    f.render_widget(menu_list, layouts.salat);
    if app_state.fullscreen {return;}
    f.render_widget(header, root_container.header);
    f.render_widget(footer, root_container.footer);
    // f.render_widget(block_3, root_container.center);
}

pub struct SettingsContainer {
    
}
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
    fn new<B:Backend>(f: &mut Frame<B>) -> Self {
        let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ]).split(f.size());
        
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
            Constraint::Length(3),
            Constraint::Min(16),
        ]).split(area);
        
        let salat = layouts[1];
        let title = layouts[0];
        
        MainContainer { title , salat }
    }
}






