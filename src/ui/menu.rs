use super::*;

pub fn draw_menu<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState, ui_state: &mut UIState){
    // let input_map = app_state.input_map.to_owned();
    
    let layouts = MainContainer::from(ui_state.get_screen_rect());
    ui_state.set_header("lol, lmao");
    ui_state.set_footer(vec![
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

