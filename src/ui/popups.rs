use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, BorderType, Clear, Table, Row, List, ListItem},
    text::{Line, Span}, 
};
use crate::app::App;
use crate::network::models::MenuTarget;

// ---------------------------------- Keybind Popup -------------------------------
pub fn draw_help(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(35, 75, area);
    f.render_widget(Clear, popup_area);

    let bg_color = Color::Rgb(28, 28, 28);

    let block = Block::default()
        .title(" ⌨  SHORTCUTS ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(bg_color));

    let rows = vec![
        Row::new(vec![" Tab", "Switch Focus"]),
        Row::new(vec![" 1-3", "Switch Home Tabs"]),
        Row::new(vec![" Enter", "Select Item / Play"]),
        Row::new(vec![" t", "Open Action Menu"]),
        Row::new(vec![" Backspace, b", "Back to Menu"]),
        Row::new(vec![" H", "Return to Home"]),
        Row::new(vec![""]),

        Row::new(vec![" Space", "Play / Pause"]),
        Row::new(vec![" n / p", "Next / Prev Track"]),
        Row::new(vec![""]),

        Row::new(vec![" j / k", "Up / Down (List)"]),
        Row::new(vec![" h / l", "Left / Right (Tab)"]),
        Row::new(vec![""]),

        Row::new(vec![" ?", "Close Help"]),
        Row::new(vec![" q, Esc", "Quit App"]),
    ];

    let table = Table::new(rows, [Constraint::Percentage(40), Constraint::Percentage(60)])
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
pub fn draw_quick_actions(f: &mut Frame, area: Rect) {
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

    let rows = vec![
        Row::new(vec![
            "h → Go to Home",
            "n → Next track",
        ]),
        Row::new(vec![
            "s → Go to Search",
            "p → Previous track",
        ]),
    ];

    let widths = [
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
    ];

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
