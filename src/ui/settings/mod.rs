use super::*;
use crate::structs::input::Key;
pub use state::*;

mod state;

const LABEL_WIDTH: usize = 16;
const CENSORED: &str = "••••••";

pub fn draw_settings(f: &mut Frame, app_state: &mut AppState, ui_state: &mut UIState) {
    ui_state.set_header("Settings");
    ui_state.set_footer(match app_state.settings.mode {
        SettingsMode::Normal => vec![
            ["tab", " section"],
            ["↑↓", " select"],
            ["←→", " change"],
            ["enter", " edit"],
            ["c", " lose"],
            ["esc", " back"],
            ["q", "uit"],
        ],
        SettingsMode::TextInput { .. } => vec![
            ["type", " value"],
            ["enter", " commit"],
            ["esc", " cancel"],
        ],
        SettingsMode::Popup { .. } => vec![
            ["↑↓", " select"],
            ["type", " filter"],
            ["enter", " apply"],
            ["esc", " cancel"],
        ],
    });

    let area = ui_state.get_screen_rect();
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(22), Constraint::Min(20)])
        .split(area);

    let cat_items: Vec<ListItem> = CATEGORIES
        .iter()
        .map(|c| ListItem::new(*c))
        .collect();
    let cat_list = List::new(cat_items)
        .block(new_color_block("sections", Color::DarkGray))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut list_state = ListState::default();
    list_state.select(Some(app_state.settings.category));
    f.render_stateful_widget(cat_list, panes[0], &mut list_state);

    let msg_height = u16::from(app_state.message.is_some());
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(msg_height),
            Constraint::Min(5),
        ])
        .split(panes[1]);

    let title = Paragraph::new(CATEGORIES[app_state.settings.category])
        .block(new_color_block("", Color::Green).title_alignment(Alignment::Center))
        .alignment(Alignment::Center);
    f.render_widget(title, right[0]);

    if let Some(message) = &app_state.message {
        let message = Paragraph::new(message.clone()).style(Style::default().fg(Color::Red));
        f.render_widget(message, right[1]);
    }

    let fields = fields_for(app_state.settings.category, &app_state.config);
    if app_state.settings.cursor >= fields.len() {
        app_state.settings.cursor = fields.len().saturating_sub(1);
    }
    let visible_height = right[2].height.saturating_sub(2) as usize;
    adjust_scroll(&mut app_state.settings, fields.len(), visible_height.max(1));

    let editing = match &app_state.settings.mode {
        SettingsMode::TextInput { field, buffer } => Some((*field, buffer.clone())),
        SettingsMode::Normal | SettingsMode::Popup { .. } => None,
    };

    let start = app_state.settings.offset.min(fields.len());
    let items: Vec<ListItem> = fields[start..]
        .iter()
        .enumerate()
        .map(|(vis, row)| {
            let index = start + vis;
            let selected = index == app_state.settings.cursor;
            let value = match &editing {
                Some((field, buffer)) if *field == row.id => format!("{buffer}█"),
                _ if row.id.censored() && !selected => CENSORED.to_string(),
                _ => row.id.value_str(&app_state.config),
            };
            let body = match row.kind {
                FieldKind::Toggle => format!(
                    "{:<LABEL_WIDTH$}[{}]",
                    row.label,
                    if value == "true" { "x" } else { " " }
                ),
                FieldKind::Cycle => format!("{:<LABEL_WIDTH$}‹ {} ›", row.label, value),
                FieldKind::Text { .. } | FieldKind::Pick(_) => format!("{:<LABEL_WIDTH$}{}", row.label, value),
            };
            let style = if selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(body).style(style)
        })
        .collect();

    let field_list = List::new(items).block(new_color_block("", Color::Green));
    f.render_widget(field_list, right[2]);

    if let SettingsMode::Popup { kind, cursor, offset, filter } = &app_state.settings.mode {
        draw_popup(f, area, *kind, *cursor, *offset, filter);
    }
}

fn draw_popup(f: &mut Frame, area: Rect, kind: PopupKind, cursor: usize, mut offset: usize, filter: &str) {
    let entries = popup_entries(kind, filter);

    let popup = centered_rect(area, 60, 16);
    f.render_widget(Clear, popup);

    let title = match kind {
        PopupKind::Location => format!("select location [{}/{}]", entries.len(), CITIES.len()),
        PopupKind::Island => format!("select island [{}/{}]", entries.len(), ISLAND_DATA.len()),
    };

    let inner = shrink(popup);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);
    let filter_line = Paragraph::new(if filter.is_empty() { " ".to_string() } else { format!("filter: {filter}") })
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(filter_line, rows[0]);

    let visible = rows[1].height as usize;
    if cursor < offset {
        offset = cursor;
    }
    if cursor >= offset + visible.max(1) {
        offset = cursor + 1 - visible.max(1);
    }

    let start = offset.min(entries.len());
    let items: Vec<ListItem> = entries[start..]
        .iter()
        .enumerate()
        .map(|(vis, entry)| {
            let selected = start + vis == cursor;
            let style = if selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(popup_label(entry)).style(style)
        })
        .collect();

    let list = List::new(items).block(new_color_block(&title, Color::Cyan));
    f.render_widget(list, rows[1]);
}

fn centered_rect(area: Rect, percent_x: u16, max_h: u16) -> Rect {
    let height = max_h.min(area.height.saturating_sub(2)).max(5);
    let width = (area.width * percent_x / 100).clamp(24, area.width);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}

fn shrink(rect: Rect) -> Rect {
    Rect {
        x: rect.x.saturating_add(1),
        y: rect.y.saturating_add(1),
        width: rect.width.saturating_sub(2),
        height: rect.height.saturating_sub(2),
    }
}

fn popup_label(entry: &PopupEntry) -> String {
    match entry {
        PopupEntry::City(c) => format!(
            "{} ({}) [{:.4}, {:.4}]",
            c[0],
            c[1],
            c[2].parse::<f64>().unwrap_or(0.0),
            c[3].parse::<f64>().unwrap_or(0.0)
        ),
        PopupEntry::Island(name) => name.clone(),
    }
}

fn adjust_scroll(state: &mut SettingsState, len: usize, visible: usize) {
    if state.cursor < state.offset {
        state.offset = state.cursor;
    }
    if state.cursor >= state.offset + visible {
        state.offset = state.cursor + 1 - visible;
    }
    let max_offset = len.saturating_sub(visible);
    state.offset = state.offset.min(max_offset);
}

pub fn handle_settings_key(app_state: &mut AppState) -> bool {
    app_state.message = None;
    let key = app_state
        .input_map
        .get_key()
        .unwrap_or((Key::None, crate::structs::input::Modifier::None));
    if matches!(app_state.settings.mode, SettingsMode::TextInput { .. }) {
        handle_text_key(app_state, key.0);
    } else if matches!(app_state.settings.mode, SettingsMode::Popup { .. }) {
        handle_popup_key(app_state, key);
    } else {
        handle_normal_key(app_state, key);
    }
    true
}

fn handle_normal_key(app_state: &mut AppState, (key, modifier): (Key, crate::structs::input::Modifier)) {
    let step = match modifier {
        crate::structs::input::Modifier::Shift => 10,
        _ => 1,
    };
    let row_count = fields_for(app_state.settings.category, &app_state.config).len();
    if row_count == 0 {
        return;
    }
    match key {
        Key::Prev => {
            app_state.settings.category = (app_state.settings.category + CATEGORIES.len() - 1) % CATEGORIES.len();
            app_state.settings.cursor = 0;
        }
        Key::Next => {
            app_state.settings.category = (app_state.settings.category + 1) % CATEGORIES.len();
            app_state.settings.cursor = 0;
        }
        Key::Up => app_state.settings.cursor = app_state.settings.cursor.saturating_sub(1),
        Key::Down => app_state.settings.cursor = (app_state.settings.cursor + 1).min(row_count - 1),
        Key::Left => edit_current(app_state, -step),
        Key::Right => edit_current(app_state, step),
        Key::Enter => activate_current(app_state),
        Key::Escape | Key::Config => app_state.screen = Screen::Menu,
        _ => {}
    }
}

fn current_row(app_state: &AppState) -> Option<FieldRow> {
    let rows = fields_for(app_state.settings.category, &app_state.config);
    rows.into_iter().nth(app_state.settings.cursor)
}

fn edit_current(app_state: &mut AppState, delta: isize) {
    let Some(row) = current_row(app_state) else {
        return;
    };
    match row.kind {
        FieldKind::Cycle => {
            if row.id == FieldId::ProviderType {
                switch_provider_type(app_state);
            } else {
                row.id.cycle(&mut app_state.config, delta);
                after_change(app_state, row.id.is_provider());
            }
        }
        FieldKind::Text { numeric: true } if row.id.steppable() => {
            row.id.step(&mut app_state.config, delta);
            after_change(app_state, false);
        }
        FieldKind::Pick(_) => {}
        _ => {}
    }
}

fn activate_current(app_state: &mut AppState) {
    let Some(row) = current_row(app_state) else {
        return;
    };
    match row.kind {
        FieldKind::Toggle => {
            row.id.toggle(&mut app_state.config);
            after_change(app_state, false);
        }
        FieldKind::Cycle => edit_current(app_state, 1),
        FieldKind::Pick(kind) => open_picker(app_state, kind),
        FieldKind::Text { .. } => {
            let buffer = row.id.value_str(&app_state.config);
            app_state.settings.mode = SettingsMode::TextInput {
                field: row.id,
                buffer,
            };
        }
    }
}

fn open_picker(app_state: &mut AppState, kind: PopupKind) {
    app_state.settings.mode = SettingsMode::Popup {
        kind,
        cursor: 0,
        offset: 0,
        filter: String::new(),
    };
}

fn store_popup(app_state: &mut AppState, kind: PopupKind, cursor: usize, offset: usize, filter: String) {
    app_state.settings.mode = SettingsMode::Popup {
        kind,
        cursor,
        offset,
        filter,
    };
}

fn handle_popup_key(app_state: &mut AppState, (key, modifier): (Key, crate::structs::input::Modifier)) {
    let step = match modifier {
        crate::structs::input::Modifier::Shift => 10,
        _ => 1,
    };
    let taken = std::mem::take(&mut app_state.settings.mode);
    let SettingsMode::Popup { kind, cursor, offset, filter } = taken else {
        return;
    };
    let len = popup_entries(kind, &filter).len();
    match key {
        Key::Up => store_popup(app_state, kind, cursor.saturating_sub(step).min(len.saturating_sub(1)), offset, filter),
        Key::Down if len > 0 => store_popup(app_state, kind, (cursor + step).min(len - 1), offset, filter),
        Key::Enter => apply_popup(app_state, kind, cursor, offset, filter),
        Key::Escape => app_state.settings.mode = SettingsMode::Normal,
        Key::Backspace => {
            let mut next = filter;
            next.pop();
            store_popup(app_state, kind, 0, 0, next);
        }
        _ => {
            if let Some(c) = app_state.input_map.get_raw_char() {
                let mut next = filter;
                next.push(c);
                store_popup(app_state, kind, 0, 0, next);
            } else {
                store_popup(app_state, kind, cursor, offset, filter);
            }
        }
    }
}

fn apply_popup(app_state: &mut AppState, kind: PopupKind, cursor: usize, offset: usize, filter: String) {
    let selected = popup_entries(kind, &filter)
        .get(cursor)
        .map(|entry| match entry {
            PopupEntry::City(city) => (Some(*city), None),
            PopupEntry::Island(key) => (None, Some(key.clone())),
        });
    let Some((city, island)) = selected else {
        store_popup(app_state, kind, cursor, offset, filter);
        return;
    };

    if let Some(city) = city {
        apply_city(&mut app_state.config, city);
        app_state.settings.mode = SettingsMode::Normal;
        after_change(app_state, true);
        return;
    }

    let Some(key) = island else {
        return;
    };
    if timeset_for_island(&key).is_none() {
        app_state.message = Some("failed to parse island data".to_string());
        store_popup(app_state, kind, cursor, offset, filter);
        return;
    }
    app_state.config.salatmv.island = key;
    let len = fields_for(0, &app_state.config).len();
    app_state.settings.cursor = app_state.settings.cursor.min(len.saturating_sub(1));
    app_state.settings.mode = SettingsMode::Normal;
    after_change(app_state, true);
}

fn handle_text_key(app_state: &mut AppState, key: Key) {
    let taken = std::mem::take(&mut app_state.settings.mode);
    let SettingsMode::TextInput { field, mut buffer } = taken else {
        return;
    };
    match key {
        Key::Enter => {
            let is_provider = field.is_provider();
            match field.commit_text(&mut app_state.config, &buffer) {
                Ok(()) => after_change(app_state, is_provider),
                Err(err) => {
                    app_state.message = Some(err);
                    app_state.settings.mode = SettingsMode::TextInput { field, buffer };
                }
            }
        }
        Key::Escape => {}
        Key::Backspace => {
            buffer.pop();
            app_state.settings.mode = SettingsMode::TextInput { field, buffer };
        }
        _ => {
            if let Some(c) = app_state.input_map.get_raw_char() {
                buffer.push(c);
            }
            app_state.settings.mode = SettingsMode::TextInput { field, buffer };
        }
    }
}

fn switch_provider_type(app_state: &mut AppState) {
    // both provider sections stay stored in the config; only the selector flips
    app_state.config.provider = match app_state.config.provider {
        ProviderKind::Calculation => ProviderKind::SalatMv,
        ProviderKind::SalatMv => ProviderKind::Calculation,
    };
    let len = fields_for(0, &app_state.config).len();
    app_state.settings.cursor = app_state.settings.cursor.min(len.saturating_sub(1));
    after_change(app_state, true);
}

fn after_change(app_state: &mut AppState, provider_changed: bool) {
    if provider_changed {
        app_state.init_provider();
    }
    if let Err(err) = app_state.config.save() {
        app_state.message = Some(format!("save failed: {err}"));
    }
}
