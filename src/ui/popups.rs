use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    widgets::{
        Block, Borders, BorderType, Clear,
        Table, Row, List, ListItem, HighlightSpacing
    },
};
use crate::app::App;
use crate::network::models::MenuTarget;

// ---------------------------------- Keybind Popup -------------------------------
pub fn draw_help(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(37, 90, area);
    f.render_widget(Clear, popup_area);

    let bg_color = Color::Rgb(28, 28, 28);
    let section_style = Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD);

    let block = Block::default()
        .title(" ⌨  SHORTCUTS ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(bg_color));

    let rows = vec![
        Row::new(vec!["  General", ""]).style(section_style),
        Row::new(vec!["  Tab", "Cycle Panel Focus"]),
        Row::new(vec!["  1 / 2 / 3 / 4", "Focus Panel 1 / 2 / 3 / 4"]),
        Row::new(vec!["  t", "Open Action Menu"]),
        Row::new(vec!["  g", "Quick Actions"]),
        Row::new(vec!["  ?", "Toggle Help"]),
        Row::new(vec!["  q", "Quit App"]),

        Row::new(vec!["  Views", ""]).style(section_style),
        Row::new(vec!["  H", "Go to Home"]),
        Row::new(vec!["  S", "Go to Search"]),
        Row::new(vec!["  Q", "Go to Queue"]),
        Row::new(vec!["  L", "Go to Lyrics"]),

        Row::new(vec!["  Playback", ""]).style(section_style),
        Row::new(vec!["  Space", "Play / Pause"]),
        Row::new(vec!["  n / p", "Next / Prev Track"]),
        Row::new(vec!["  + / -", "Volume Up / Down"]),
        Row::new(vec!["  s", "Toggle Shuffle"]),
        Row::new(vec!["  r", "Cycle Repeat Mode"]),

        Row::new(vec!["  Navigation", ""]).style(section_style),
        Row::new(vec!["  j / k", "Move Down / Up"]),
        Row::new(vec!["  h / l", "Move Left / Right"]),
        Row::new(vec!["  Ctrl+d / u", "Jump Down / Up 10"]),
        Row::new(vec!["  Enter", "Select / Confirm"]),
        Row::new(vec!["  b", "Go Back"]),
        Row::new(vec!["  Esc", "Close"]),
    ];

    let table = Table::new(rows, [Constraint::Percentage(42), Constraint::Percentage(58)])
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(table, popup_area);
}

// ----------------------------------- Action Menu -------------------------------
pub fn draw_action_menu(
    f: &mut Frame, 
    app: &mut App, 
    list_area: Rect, 
    selected_idx: usize, 
    scroll_offset: usize
) {
    if !app.action_menu.is_open {
        return;
    }

    let menu_width = 34;
    let menu_height = (app.action_menu.actions.len() as u16) + 2;
    let relative_idx = selected_idx.saturating_sub(scroll_offset) as u16;

    let mut x = list_area.x + 4;
    let mut y = list_area.y + relative_idx + 2;

    if x + menu_width > f.area().right() {
        x = f.area().right().saturating_sub(menu_width);
    }

    if y + menu_height > f.area().bottom() {
        y = (list_area.y + relative_idx + 1).saturating_sub(menu_height);
    }

    let area = Rect::new(x, y, menu_width, menu_height);
    f.render_widget(Clear, area);

    let title = match &app.action_menu.target {
        Some(MenuTarget::Track(_)) => " TRACK ",
        Some(MenuTarget::Artist(_)) => " ARTIST ",
        Some(MenuTarget::Album(_)) => " ALBUM ",
        Some(MenuTarget::Playlist(_)) => " PLAYLIST ",
        Some(MenuTarget::Episode(_)) => " EPISODE ",
        None => " OPTIONS ",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::LightCyan))
        .style(Style::default().bg(Color::Rgb(28, 28, 28)));

    let items: Vec<ListItem> = app.action_menu.actions
        .iter()
        .map(|action| {
            // Directly create the ListItem with the action string
            ListItem::new(format!(" {}", action.as_str()))
        })
    .collect();

   let list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut app.action_menu.state);
    
    if app.playlist_selector.is_open {
        draw_playlist_selector(f, app, area);
    }
}

// -------------------------------------- Playlist Selector -------------------------------
pub fn draw_playlist_selector(f: &mut Frame, app: &mut App, action_menu_area: Rect) {
    let selector_width = 30;
    let selector_height = 10;

    let mut x = action_menu_area.x + action_menu_area.width - 1;
    if x + selector_width > f.area().right() {
        x = action_menu_area.x.saturating_sub(selector_width).saturating_add(1);
    }

    let selector_area = Rect {
        x,
        y: action_menu_area.y + 2,
        width: selector_width,
        height: selector_height,
    };

    let items: Vec<ListItem> = app.playlist_selector.playlists
        .iter()
        .map(|p| ListItem::new(format!("  {}", p.name)))
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .title(" Add to... ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightCyan))
            .style(Style::default().bg(Color::Rgb(28, 28, 28)))) 
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_widget(Clear, selector_area);
    f.render_stateful_widget(list, selector_area, &mut app.playlist_selector.state);
}

// ------------------------------------------- Quick Actions ------------------------------------
pub fn draw_quick_actions(f: &mut Frame, app: &App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      // Main application area
            Constraint::Length(5),   // Popup area
            Constraint::Length(4)   // Do not cover the playbar
        ])
        .split(area);
    // Leave some space on the two ends
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3)
        ])
        .split(layout[1]);
        
    let popup_area = layout[1];

    let actions: Vec<String> = if app.quick_actions.is_empty() {
        vec!["No quick actions available".to_string()]
    } else {
        app.quick_actions
            .iter()
            .map(|action| format!(
                "{} → {}",
                action.keycode().to_string(),
                action.as_str()
            ))
            .collect()
    };

    let column_count = if actions.len() <= 2 {
        1
    } else if actions.len() <= 4 {
        2
    } else {
        3
    };

    let rows: Vec<Row> = actions
        .chunks(column_count)
        .map(|chunk| {
            let mut cells: Vec<String> = chunk.iter().cloned().collect();
            while cells.len() < column_count {
                cells.push(String::new());
            }
            Row::new(cells)
        })
        .collect();

    let widths: Vec<Constraint> = (0..column_count)
        .map(|_| Constraint::Ratio(1, column_count as u32))
        .collect();

    let block = Block::default()
        .title(" Quick Actions ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default()
        .bg(Color::Rgb(30, 30, 30)));

    let table = Table::new(rows, widths)
        .block(block)
        .style(Style::default().fg(Color::Gray))
        .column_spacing(2);

    f.render_widget(Clear, popup_area);
    f.render_widget(table, popup_area);
}

// ------------------------------------------- Device Selector ------------------------------------
pub fn draw_device_selector(f: &mut Frame, app: &mut App, area: Rect) {
    let device_count = app.device_state.online_devices.items.len().max(1) as u16;
    let popup_height = (device_count + 2).min(area.height);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(popup_height),
            Constraint::Min(0),
        ])
        .split(area);

    let percent_x = 45;
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical_layout[1]);

    let popup_area = horizontal_layout[1];
    app.device_state.last_area = popup_area;

    // Explicitly expand the clear area so the text does not eat up the border
    let clear_x = popup_area.x.saturating_sub(2);
    let clear_width = popup_area.width.saturating_add(4);
    let clear_area = Rect {
        x: clear_x,
        y: popup_area.y,
        width: if clear_x + clear_width > area.right() { area.right() - clear_x } else { clear_width },
        height: popup_area.height,
    };

    f.render_widget(Clear, clear_area);

    let block = Block::default()
        .title(" Devices ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::LightCyan))
        .style(Style::default().bg(Color::Rgb(28, 28, 28)).fg(Color::White));

    let active_idx = app.device_state.active_device_idx();
    let local_idx = app.device_state.local_device_idx;

    let items: Vec<ListItem> = if app.device_state.online_devices.items.is_empty() {
        vec![ListItem::new(" No devices found")]
    } else {
        app.device_state
            .online_devices
            .items
            .iter()
            .enumerate()
            .map(|(i, d)| {
                let name = if d.name.is_empty() { "Unknown device" } else { d.name.as_str() };
                
                let icon = match d.r#type.to_lowercase().as_str() {
                    "computer" | "desktop" | "laptop" | "pc" => "💻",
                    "phone" | "smartphone" | "mobile" => "📱",
                    "speaker" | "audio" => "🔊",
                    _ => "❓",
                };

                let is_active = active_idx == Some(i);
                let is_local = local_idx == Some(i);

                let mut suffix = String::new();
                if is_local {
                    suffix.push_str(" [Local]");
                }
                if is_active {
                    suffix.push_str(" [Current]");
                }

                let content = format!(" {} {}{}", icon, name, suffix);
                let mut item = ListItem::new(content);

                item = item.style(Style::default().fg(Color::White));
                
                item
            })
            .collect()
    };

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(
        list,
        popup_area,
        &mut app.device_state.online_devices.state,
    );
}

// ------------------------------------------- Helper ------------------------------------
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}