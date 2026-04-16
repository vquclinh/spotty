use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, BorderType, Clear, Table, Row}, 
};

pub fn draw(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(35, 70, area);
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
        Row::new(vec![" Enter", "Select Item"]),
        Row::new(vec![" Backspace, b", "Back to Menu"]),
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