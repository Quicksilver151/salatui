use super::*;

pub fn draw_menu(f: &mut Frame, app_state: &mut AppState, ui_state: &mut UIState){
    // let input_map = app_state.input_map.to_owned();
    
    let layouts = MainContainer::from(ui_state.get_screen_rect());
    ui_state.set_header("SalaTUI");
    ui_state.set_footer(vec![
        ["q", "uit"],
        ["c", "onfig"],
        ["f", "ullscreen"],
        ["Esc", "ape"],
    ]);
    // let title = input_map.get_current().unwrap_or(String::new());
    
    let current_time = chrono::offset::Local::now().time();
    let current_date = chrono::offset::Local::now().date_naive() + chrono::Duration::days(app_state.day_offset);
    
    let border_title = if app_state.fullscreen { "SalaTUI" } else { "" };
    let title_block = if app_state.day_offset == 0 {
        new_color_block(border_title, Color::Green).title_alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD))
    }else{
        new_color_block(border_title, Color::Red).title_alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD))
    };

    let menu_block = new_color_block("", Color::Green);
    
    let time_str = format!("Time: {}", current_time.format("%I:%M:%S %p"));
    let date_str = format!("Date: {}", current_date.format("%d %b %Y"));

    // HACK: slat calc testing ==============================================
    // let conf: SalahCalcConfig = salah_calc::SalahCalcConfig::tmpnew();
    // let prayer_times = app_state.timeset_data.data_from_day(current_date.ordinal() as usize);
    // ENDHACK
    
    let prayer_times: PrayerTimes = app_state.get_prayer_times();

    
    let salat_index = (prayer_times.get_current_index() + match app_state.config.display.indicator{
        TimeIndicator::Next => 1,
        TimeIndicator::Current => 0,
        _ => 0,
    } + 5 ) % 6;
    let prayer_times: Vec<String> = prayer_times.format_time(&app_state.config);
    
    
    // let salat_index = 5;
    let prayer_names = ["Fajr", "Sun", "Dhuhur", "Asr", "Magrib", "Isha"];
    let rows: Vec<String> = prayer_names
        .into_iter()
        .zip(prayer_times)
        .map(|(name, time)| format!("{name:<7}: {time}"))
        .collect();

    let inner_width = layouts.salat.width.saturating_sub(2) as usize;
    let longest = rows
        .iter()
        .chain([&time_str, &date_str])
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let pad = " ".repeat(inner_width.saturating_sub(longest) / 2);

    let title_widget = Paragraph::new(vec![
        Line::from(Span::styled(format!("{pad}{time_str}"), Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("{pad}{date_str}"), Style::default().add_modifier(Modifier::BOLD))),
    ]).block(title_block);

    let menu_list = rows
        .into_iter()
        .map(|row| format!("{pad}{row}"))
        .enumerate()
        .map(|(i, line)|
            if salat_index == i && app_state.day_offset == 0{
                ListItem::new(line).style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                 ListItem::new(line).style(Style::default())
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

