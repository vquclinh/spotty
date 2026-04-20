use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, BorderType, Clear, Table, Row, List, ListItem}, 
};
use crate::app::App;
use crate::network::models::MenuTarget;

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

    let menu_width = 32;
    let menu_height = (app.action_menu.actions.len() as u16) + 2;
    let relative_idx = selected_idx.saturating_sub(scroll_offset) as u16;

    let mut x = list_area.x + 2;
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
        Some(MenuTarget::Track(t)) => format!(" Track: {} ", t.name),
        Some(MenuTarget::Artist(a)) => format!(" Artist: {} ", a.name),
        Some(MenuTarget::Album(a)) => format!(" Album: {} ", a.name),
        Some(MenuTarget::Playlist(p)) => format!(" Playlist: {} ", p.name),
        Some(MenuTarget::Episode(e)) => format!(" Episode: {} ", e.name),
        None => " Options ".to_string(),
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(Color::Rgb(28, 28, 28)));

    let items: Vec<ListItem> = app.action_menu.actions
        .iter()
        .map(|action| ListItem::new(format!("  {}", action.as_str())))
        .collect();

    let list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut app.action_menu.state);
}

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