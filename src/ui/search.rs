use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let input_color = if app.active_block == ActiveBlock::SearchInput {
        Color::Green
    } else {
        Color::White
    };
    let input = Paragraph::new(app.search.input.as_str()).block(
        Block::default()
            .title(" Searching ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(input_color)),
    );
    f.render_widget(input, chunks[0]);

    let result_color = if app.active_block == ActiveBlock::SearchResults {
        Color::Green
    } else {
        Color::White
    };
    let results = Paragraph::new("Output here...").block(
        Block::default()
            .title(" Songs & Artists ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(result_color)),
    );
    f.render_widget(results, chunks[1]);
}
