use super::*;
use crate::event;
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
    app_state.ui_mouse.settings_panes[0] = panes[0];

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
    app_state.ui_mouse.settings_panes[1] = right[2];

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

    if let Some((kind, cursor, offset, filter)) = popup_mode_snapshot(app_state) {
        draw_popup(f, app_state, area, kind, cursor, offset, &filter);
    }
}

fn draw_popup(f: &mut Frame, app_state: &mut AppState, area: Rect, kind: PopupKind, cursor: usize, mut offset: usize, filter: &str) {
    let entries = popup_entries(kind, filter);

    let popup = centered_rect(area, 60, 16);
    app_state.ui_mouse.popup_rect = popup;
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
    app_state.ui_mouse.popup_list = rows[1];
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

// mouse ============================================

/// global mouse dispatch, called from the main event loop
pub fn handle_mouse_event(app_state: &mut AppState, mouse: event::MouseEvent) {
    let (column, row) = (mouse.column, mouse.row);
    match mouse.kind {
        event::MouseEventKind::ScrollDown if app_state.screen == Screen::Menu => {
            app_state.day_offset += 1;
        }
        event::MouseEventKind::ScrollUp if app_state.screen == Screen::Menu => {
            app_state.day_offset -= 1;
        }
        event::MouseEventKind::ScrollDown if app_state.screen == Screen::Settings => {
            settings_scroll(app_state, 1);
        }
        event::MouseEventKind::ScrollUp if app_state.screen == Screen::Settings => {
            settings_scroll(app_state, -1);
        }
        event::MouseEventKind::Down(event::MouseButton::Right) if app_state.screen == Screen::Menu => {
            app_state.screen = Screen::Settings;
        }
        event::MouseEventKind::Down(event::MouseButton::Left) if app_state.screen == Screen::Settings => {
            handle_settings_click(app_state, column, row);
        }
        event::MouseEventKind::Down(event::MouseButton::Right) if app_state.screen == Screen::Settings => {
            handle_settings_right_click(app_state, column, row);
        }
        _ => {}
    }
}

/// wheel moves the picker cursor (scroll), or the focused field in normal mode;
/// offset follows automatically at draw time
fn settings_scroll(app_state: &mut AppState, delta: isize) {
    match &app_state.settings.mode {
        SettingsMode::Popup { kind, cursor, offset, filter } => {
            let (kind, cursor, offset, filter) = (*kind, *cursor, *offset, filter.clone());
            let len = popup_entries(kind, &filter).len();
            let max = len.saturating_sub(1);
            let next = if delta > 0 { (cursor + 1).min(max) } else { cursor.saturating_sub(1).min(max) };
            store_popup(app_state, kind, next, offset, filter);
        }
        SettingsMode::Normal => {
            let row_count = fields_for(app_state.settings.category, &app_state.config).len();
            if row_count == 0 {
                return;
            }
            if delta > 0 {
                app_state.settings.cursor = (app_state.settings.cursor + 1).min(row_count - 1);
            } else {
                app_state.settings.cursor = app_state.settings.cursor.saturating_sub(1);
            }
        }
        _ => {}
    }
}

/// right-click: on a selected cycling field steps backwards; on empty space
/// returns to the main menu; unselected fields only get selected, other
/// buttons are a no-op
fn handle_settings_right_click(app_state: &mut AppState, column: u16, row: u16) {
    if !matches!(app_state.settings.mode, SettingsMode::Normal) {
        return;
    }
    let panes = app_state.ui_mouse.settings_panes;
    // on a real field row: select first, act on the selected field
    if let Some(vis) = row_in_rect(column, row, panes[1]) {
        let rows = fields_for(app_state.settings.category, &app_state.config);
        let index = app_state.settings.offset + vis;
        if index < rows.len() {
            if app_state.settings.cursor == index && matches!(rows[index].kind, FieldKind::Cycle) {
                edit_current(app_state, -1);
            } else {
                app_state.settings.cursor = index;
            }
            return;
        }
        // empty space below the fields is not a button -> fall through
    }
    // section rows are buttons too
    if row_in_rect(column, row, panes[0]).is_some() {
        return;
    }
    // nothing interactive under the pointer -> back to the main screen
    app_state.screen = Screen::Menu;
}

fn handle_settings_click(app_state: &mut AppState, column: u16, row: u16) {
    if matches!(app_state.settings.mode, SettingsMode::Popup { .. }) {
        handle_popup_click(app_state, column, row);
        return;
    }
    if matches!(app_state.settings.mode, SettingsMode::TextInput { .. }) {
        handle_text_click(app_state, column, row);
        return;
    }

    let panes = app_state.ui_mouse.settings_panes;
    // left pane: sections
    if let Some(cat) = row_in_rect(column, row, panes[0]) {
        if cat < CATEGORIES.len() {
            app_state.settings.category = cat;
            app_state.settings.cursor = 0;
        }
        return;
    }
    // right pane: fields -> a click selects the field; clicking the already
    // selected (highlighted) field activates it
    if let Some(vis) = row_in_rect(column, row, panes[1]) {
        let len = fields_for(app_state.settings.category, &app_state.config).len();
        let index = app_state.settings.offset + vis;
        if index < len {
            if app_state.settings.cursor == index {
                activate_current(app_state);
            } else {
                app_state.settings.cursor = index;
            }
        }
    }
}

/// clicks while editing a text field: anything outside the edited field
/// exits edit mode
fn handle_text_click(app_state: &mut AppState, column: u16, row: u16) {
    let editing = match &app_state.settings.mode {
        SettingsMode::TextInput { field, .. } => *field,
        _ => return,
    };
    let fields = app_state.ui_mouse.settings_panes[1];
    let clicked = row_in_rect(column, row, fields).map(|vis| {
        let len = fields_for(app_state.settings.category, &app_state.config).len();
        let index = app_state.settings.offset + vis;
        if index < len {
            Some(index)
        } else {
            None
        }
    });
    let clicked = clicked.flatten().and_then(|index| {
        let rows = fields_for(app_state.settings.category, &app_state.config);
        rows.get(index).map(|r| (index, r.id))
    });
    // clicking the currently-edited field keeps editing
    if clicked.is_some_and(|(_, id)| id == editing) {
        return;
    }
    app_state.settings.mode = SettingsMode::Normal;
    if let Some((index, _)) = clicked {
        app_state.settings.cursor = index;
    }
}

fn handle_popup_click(app_state: &mut AppState, column: u16, row: u16) {
    let Some((kind, cursor, offset, filter)) = popup_mode_snapshot(app_state) else {
        return;
    };
    // click outside the popup window closes it
    if !in_rect(column, row, app_state.ui_mouse.popup_rect) {
        app_state.settings.mode = SettingsMode::Normal;
        return;
    }
    let Some(vis) = row_in_rect(column, row, app_state.ui_mouse.popup_list) else {
        return;
    };
    let len = popup_entries(kind, &filter).len();
    let index = offset + vis;
    if index >= len {
        return;
    }
    if index == cursor {
        // already-selected entry applies
        apply_popup(app_state, kind, index, offset, filter);
    } else {
        store_popup(app_state, kind, index, offset, filter);
    }
}

fn popup_mode_snapshot(app_state: &AppState) -> Option<(PopupKind, usize, usize, String)> {
    match &app_state.settings.mode {
        SettingsMode::Popup { kind, cursor, offset, filter } => {
            Some((*kind, *cursor, *offset, filter.clone()))
        }
        _ => None,
    }
}

/// click index within a widget's inner area (border excluded), or None
fn row_in_rect(x: u16, y: u16, area: Rect) -> Option<usize> {
    if !in_rect(x, y, area) {
        return None;
    }
    let top = area.y + 1;
    let bottom = area.y + area.height.saturating_sub(1);
    if y >= top && y < bottom {
        Some((y - top) as usize)
    } else {
        None
    }
}

fn in_rect(x: u16, y: u16, r: Rect) -> bool {
    x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_in_rect_boundaries() {
        let area = Rect { x: 10, y: 5, width: 20, height: 8 };
        // horizontally outside
        assert_eq!(row_in_rect(9, 6, area), None);
        assert_eq!(row_in_rect(30, 6, area), None);
        // borders excluded
        assert_eq!(row_in_rect(11, 5, area), None);
        assert_eq!(row_in_rect(11, 12, area), None); // bottom border
        assert_eq!(row_in_rect(11, 13, area), None); // beyond
        // content rows
        assert_eq!(row_in_rect(11, 6, area), Some(0));
        assert_eq!(row_in_rect(11, 9, area), Some(3));
    }

    #[test]
    fn click_below_fields_selects_nothing() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        let len = fields_for(0, &app.config).len();
        // map a click far below the last visible row
        let y = app.ui_mouse.settings_panes[1].y + 1 + len as u16 + 20;
        handle_settings_click(&mut app, 30, y);
        assert_eq!(app.settings.cursor, 0);
        assert_eq!(app.settings.mode, SettingsMode::Normal);
    }

    #[test]
    fn fields_rect_offset_maps_rows() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 6, width: 50, height: 20 },
        ];
        // simulate a 3-row title + message shifting the fields list down
        app.settings.category = 0;
        // click the first actual field row (index 0 = "type")
        handle_settings_click(&mut app, 40, 7);
        assert_eq!(app.settings.cursor, 0);
    }

    #[test]
    fn edit_mode_exits_on_outside_click() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.mode = SettingsMode::TextInput {
            field: FieldId::RawCustomString,
            buffer: "x".into(),
        };
        // click on the sections pane (outside the field list)
        handle_settings_click(&mut app, 5, 2);
        assert_eq!(app.settings.mode, SettingsMode::Normal);
    }

    #[test]
    fn edit_mode_kept_on_same_field_click() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        let index = app.settings.cursor;
        let id = fields_for(0, &app.config)[index].id;
        app.settings.mode = SettingsMode::TextInput {
            field: id,
            buffer: "35.5".into(),
        };
        // click back on the same field row
        let y = app.ui_mouse.settings_panes[1].y + 1 + index as u16;
        handle_settings_click(&mut app, 40, y);
        assert!(matches!(app.settings.mode, SettingsMode::TextInput { .. }));
    }

    #[test]
    fn picker_wheel_scrolls() {
        let mut app = AppState::default();
        open_picker(&mut app, PopupKind::Location);
        settings_scroll(&mut app, 1);
        let SettingsMode::Popup { cursor, offset, .. } = &app.settings.mode else {
            panic!("expected popup");
        };
        assert_eq!((*cursor, *offset), (1, 0));
        settings_scroll(&mut app, -1);
        settings_scroll(&mut app, -1);
        let SettingsMode::Popup { cursor, .. } = &app.settings.mode else {
            panic!("expected popup");
        };
        assert_eq!(*cursor, 0);
    }

    #[test]
    fn right_click_selects_then_reverses() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        // madhab is field index 3
        let y = app.ui_mouse.settings_panes[1].y + 1 + 3;
        let before = app.config.calculation.madhab;
        // right-click on an unselected field only selects it
        handle_settings_right_click(&mut app, 40, y);
        assert_eq!(app.settings.cursor, 3);
        assert_eq!(app.config.calculation.madhab, before);
        // right-click on the now-selected field reverses the cycle
        handle_settings_right_click(&mut app, 40, y);
        // a backward step from before cycles to the other variant, and
        // stepping forward again restores it (2-variant madhab)
        let after = app.config.calculation.madhab;
        assert_ne!(after, before);
        FieldId::ProviderMadhab.cycle(&mut app.config, 1);
        assert_eq!(app.config.calculation.madhab, before);
    }

    #[test]
    fn right_click_selects_unhighlighted_non_cycle() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        // latitude is a text field (index 4)
        let y = app.ui_mouse.settings_panes[1].y + 1 + 4;
        let lat_before = app.config.coordinates.latitude;
        handle_settings_right_click(&mut app, 40, y);
        // select-only: cursor moves, nothing edits
        assert_eq!(app.settings.cursor, 4);
        assert_eq!(app.config.coordinates.latitude, lat_before);
        assert_eq!(app.settings.mode, SettingsMode::Normal);
        // right-click on the already-selected (non-cycle) field is a no-op
        handle_settings_right_click(&mut app, 40, y);
        assert_eq!(app.settings.cursor, 4);
        assert_eq!(app.config.coordinates.latitude, lat_before);
        assert_eq!(app.settings.mode, SettingsMode::Normal);
    }

    #[test]
    fn click_twice_activates_cycle() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        // method is a cycle field (index 2)
        let y = app.ui_mouse.settings_panes[1].y + 1 + 2;
        let before = app.config.calculation.method;
        // first click selects only
        handle_settings_click(&mut app, 40, y);
        assert_eq!(app.settings.cursor, 2);
        assert_eq!(app.config.calculation.method, before);
        // clicking the already-selected field activates (cycles forward)
        handle_settings_click(&mut app, 40, y);
        assert_ne!(app.config.calculation.method, before);
    }

    #[test]
    fn click_twice_edits_text() {
        let mut app = AppState::default();
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        // latitude is a text field (index 4)
        let y = app.ui_mouse.settings_panes[1].y + 1 + 4;
        handle_settings_click(&mut app, 40, y);
        assert!(matches!(app.settings.mode, SettingsMode::Normal));
        handle_settings_click(&mut app, 40, y);
        assert!(matches!(app.settings.mode, SettingsMode::TextInput { .. }));
    }

    #[test]
    fn menu_right_click_opens_config() {
        let mut app = AppState::default();
        app.screen = Screen::Menu;
        handle_mouse_event(
            &mut app,
            event::MouseEvent {
                kind: event::MouseEventKind::Down(event::MouseButton::Right),
                column: 10,
                row: 10,
                modifiers: event::KeyModifiers::empty(),
            },
        );
        assert_eq!(app.screen, Screen::Settings);
    }

    #[test]
    fn settings_right_click_empty_goes_back() {
        let mut app = AppState::default();
        app.screen = Screen::Settings;
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        // click the title/border area (above the field list)
        handle_mouse_event(
            &mut app,
            event::MouseEvent {
                kind: event::MouseEventKind::Down(event::MouseButton::Right),
                column: 30,
                row: 1,
                modifiers: event::KeyModifiers::empty(),
            },
        );
        assert_eq!(app.screen, Screen::Menu);
    }

    #[test]
    fn settings_right_click_empty_space_below_fields_goes_back() {
        let mut app = AppState::default();
        app.screen = Screen::Settings;
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 30 },
            Rect { x: 21, y: 1, width: 50, height: 30 },
        ];
        app.settings.category = 0;
        let len = fields_for(0, &app.config).len();
        // right-click well below the last field, still inside the fields pane
        let y = app.ui_mouse.settings_panes[1].y + 1 + len as u16 + 15;
        handle_mouse_event(
            &mut app,
            event::MouseEvent {
                kind: event::MouseEventKind::Down(event::MouseButton::Right),
                column: 50,
                row: y,
                modifiers: event::KeyModifiers::empty(),
            },
        );
        assert_eq!(app.screen, Screen::Menu);
    }

    #[test]
    fn settings_right_click_on_button_is_noop() {
        let mut app = AppState::default();
        app.screen = Screen::Settings;
        app.ui_mouse.settings_panes = [
            Rect { x: 0, y: 1, width: 20, height: 20 },
            Rect { x: 21, y: 1, width: 50, height: 20 },
        ];
        app.settings.category = 0;
        // right-click a section row -> no-op (stays in settings)
        handle_mouse_event(
            &mut app,
            event::MouseEvent {
                kind: event::MouseEventKind::Down(event::MouseButton::Right),
                column: 10,
                row: 4,
                modifiers: event::KeyModifiers::empty(),
            },
        );
        assert_eq!(app.screen, Screen::Settings);
    }
}
