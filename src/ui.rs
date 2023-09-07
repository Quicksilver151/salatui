use crate::*;

mod menu;
mod settings;

pub use menu::*;
pub use settings::*;
use tui::{layout::Alignment, widgets::Widget};
pub use tui::{
    layout::{Layout, Direction, Constraint, Rect},
    text::Span,
    style::Modifier
};



pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState){
    
    
    let root_container:RootContainer = RootContainer::new(f.size());
    draw_main(f, app_state);
    
    app_state.ui_state.set_header_rect(root_container.header);
    app_state.ui_state.set_screen_rect(root_container.center);
    app_state.ui_state.set_footer_rect(root_container.footer);
    
    
    let header_rect = root_container.header;
    let footer_rect = root_container.footer;
    let commands_block: Block = new_color_block("commands",Color::DarkGray);
    
    let footer = tui::widgets::Paragraph::new("awdw").block(commands_block);
    
    let header = new_color_block("header", Color::DarkGray);
    
    
    // let titles:Vec<Line> = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(Line::from).collect();
    
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
    
    if app_state.fullscreen {return;}
    f.render_widget(header, root_container.header);
    f.render_widget(footer, root_container.footer);
    // f.render_widget(block_3, root_container.center);
}

fn new_color_block(title: &str, color: Color) -> Block {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain).border_style(Style::default().fg(color));
    block
}

pub fn draw_main<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState){
    
    // let input_map = app_state.input_map.to_owned();
    
    let ui_state = &mut app_state.ui_state;

    let layouts = if app_state.fullscreen {
        MainContainer::from(f.size())
    } else {
        MainContainer::from(ui_state.get_screen_rect())
    };
    
    app_state.ui_state.set_footer(vec![
        ["q", "uit"],
        ["c", "onfig"],
        ["f", "ullscreen"],
        ["esc", "ape"],
    ]);
    // let title = input_map.get_current().unwrap_or(String::new());
    
    let current_time = chrono::offset::Local::now().time();
    let current_date = chrono::offset::Local::now().date_naive() + chrono::Duration::days(app_state.day_offset);
    
    let title_block = if app_state.day_offset == 0 {
        new_color_block("←────salatui────→", Color::Green).title_alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD))
    }else{
        new_color_block("←────salatui────→", Color::Red).title_alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD))
    };

    let menu_block = new_color_block("", Color::Green);
    
    let title_text: Vec<Line> = vec![
        Line::from(vec![
            Span::styled(format!("Time: {}", current_time.format("%I:%M:%S %p")), Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(format!("Date: {}", current_date.format("%d %b %Y")), Style::default().add_modifier(Modifier::BOLD)),
        ])
    ];
    
    let title_widget = Paragraph::new(title_text).block(title_block);
    
    
    let prayer_times = app_state.timeset_data.data_from_day(current_date.ordinal() as usize);
    
    let salat_index = prayer_times.get_current_index() - match app_state.config.display.indicator{
        TimeIndicator::Next => 0,
        TimeIndicator::Current => 1,
        _ => 0,
    };
    let prayer_times = prayer_times.format_time(&app_state.config);
    
    
    // let salat_index = 5;
    let menu_list = vec![
        ListItem::new(format!("Fajr   : {}", prayer_times[0])),
        ListItem::new(format!("Sun    : {}", prayer_times[1])),
        ListItem::new(format!("Dhuhur : {}", prayer_times[2])),
        ListItem::new(format!("Asr    : {}", prayer_times[3])),
        ListItem::new(format!("Magrib : {}", prayer_times[4])),
        ListItem::new(format!("Isha   : {}", prayer_times[5])),
    ]
        .into_iter()
        .enumerate()
        .map(|(i, item)|
            if salat_index == i && app_state.day_offset == 0{ 
                item.style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                 item.style(Style::default())
            }
        )
        .collect::<Vec<ListItem>>();
    
    
    let menu_widget = tui::widgets::List::new(menu_list)
        .block(menu_block)
        // .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        // .highlight_symbol("> ")
        .style(Style::default().add_modifier(Modifier::BOLD));
    
    
    
    f.render_widget(title_widget, layouts.title);
    f.render_widget(menu_widget, layouts.salat);
    
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






