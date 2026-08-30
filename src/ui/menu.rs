use super::*;

pub fn draw_menu(f: &mut Frame, app_state: &mut AppState, ui_state: &mut UIState){
    // let input_map = app_state.input_map.to_owned();

    let location_text = location_line(app_state);
    let layouts = MainContainer::from(ui_state.get_screen_rect(), u16::from(location_text.is_some()));
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
    
    let time_fmt = if app_state.config.display.seconds { "%I:%M:%S %p" } else { "%I:%M %p" };
    let time_str = format!("Time: {}", current_time.format(time_fmt));
    let date_str = format!("Date: {}", current_date.format("%d %b %Y"));

    // HACK: slat calc testing ==============================================
    // let conf: SalahCalcConfig = salah_calc::SalahCalcConfig::tmpnew();
    // let prayer_times = app_state.timeset_data.data_from_day(current_date.ordinal() as usize);
    // ENDHACK
    
    let prayer_times: PrayerTimes = app_state.get_prayer_times();

    // indicator only applies to today; other days render no markers/highlight
    let in_between = app_state.config.display.indicator == TimeIndicator::Inbetween;
    let indicator_data = if app_state.day_offset == 0 {
        let window = app_state.config.notifications.offset.unsigned_abs();
        indicator(app_state.config.display.indicator, &prayer_times, now_minutes(), window)
    } else {
        IndicatorData::default()
    };
    let prayer_times: Vec<String> = prayer_times.format_time(&app_state.config);
    
    // let salat_index = 5;
    let prayer_names = ["Fajr", "Sun", "Dhuhur", "Asr", "Magrib", "Isha"];
    let sep = if in_between { AXIS } else { ":" };
    let rows: Vec<String> = prayer_names
        .into_iter()
        .zip(prayer_times)
        .map(|(name, time)| format!("{name:<7}{sep} {time}"))
        .collect();

    let inner_width = layouts.salat.width.saturating_sub(2) as usize;
    let longest = rows
        .iter()
        .map(|line| line.chars().count())
        .chain([time_str.chars().count(), date_str.chars().count()])
        .chain(location_text.iter().map(|line| line.chars().count()))
        .max()
        .unwrap_or(0);
    let pad = " ".repeat(inner_width.saturating_sub(longest) / 2);

    let mut title_lines = vec![
        Line::from(Span::styled(format!("{pad}{time_str}"), Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("{pad}{date_str}"), Style::default().add_modifier(Modifier::BOLD))),
    ];
    if let Some(location) = &location_text {
        title_lines.push(Line::from(Span::styled(
            format!("{pad}{location}"),
            Style::default(),
        )));
    }
    let title_widget = Paragraph::new(title_lines).block(title_block);

    let menu_list = rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let padded = format!("{pad}{row}");
            let marker = match &indicator_data.marker {
                Some(IndicatorMarker::Pair(Pair { current, .. })) if i == *current => Some(MARKER_CURRENT),
                Some(IndicatorMarker::Pair(Pair { next, .. })) if i == *next => Some(MARKER_NEXT),
                Some(IndicatorMarker::Single(row)) if i == *row => Some(MARKER_SINGLE),
                _ => None,
            };
            let line = match marker {
                Some(glyph) => marker_line(&padded, glyph, pad.len() + 7),
                None => Line::from(Span::styled(padded, Style::default())),
            };
            let style = if indicator_data.highlight == Some(i) {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(line).style(style)
        })
        .collect::<Vec<ListItem>>();
    
    
    let menu_widget = tui::widgets::List::new(menu_list)
        .block(menu_block)
        // .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        // .highlight_symbol("> ")
        .style(Style::default().add_modifier(Modifier::BOLD));
    
    
    
    f.render_widget(title_widget, layouts.title);
    f.render_widget(menu_widget, layouts.salat);

}

/// (city, country) derived from the active provider.
/// Calculation providers store "city, country" as one string;
/// Salat MV islands are atoll-prefixed keys like "AA. Male'".
fn location_parts(app_state: &AppState) -> (String, String) {
    match app_state.config.provider {
        ProviderKind::Calculation => match app_state.config.calculation.location.split_once(", ") {
            Some((city, country)) => (city.to_string(), country.to_string()),
            None => (app_state.config.calculation.location.clone(), String::new()),
        },
        ProviderKind::SalatMv => (app_state.config.salatmv.island.clone(), "Maldives".to_string()),
    }
}

/// the rendered location line for the menu title, if any
fn location_line(app_state: &AppState) -> Option<String> {
    let mode = app_state.config.display.location;
    if mode == LocationDisplay::Hide {
        return None;
    }
    let (city, country) = location_parts(app_state);
    let text = match mode {
        LocationDisplay::Hide => return None,
        LocationDisplay::Country => country,
        LocationDisplay::City => city,
        LocationDisplay::CityCountry => format!("{city}, {country}"),
    };
    let text = text.trim().trim_matches(',').trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// minutes since local midnight
fn now_minutes() -> u32 {
    use chrono::Timelike;
    let now = chrono::offset::Local::now();
    now.hour() * 60 + now.minute()
}

/// the marker glyph is drawn bold, splitting the row line into styled spans
fn marker_line(padded: &str, glyph: &'static str, split: usize) -> Line<'static> {
    let marker_style = Style::default().add_modifier(Modifier::BOLD);
    let mut spans = vec![Span::styled(padded.chars().take(split).collect::<String>(), Style::default())];
    spans.push(Span::styled(glyph, marker_style));
    spans.push(Span::styled(
        padded.chars().skip(split + glyph.chars().count()).collect::<String>(),
        Style::default(),
    ));
    Line::from(spans)
}

