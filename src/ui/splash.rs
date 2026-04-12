use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Clear, canvas::Canvas},
    Frame,
};
use crate::app::splash_state::SplashState;

pub fn draw(f: &mut Frame, _state: &SplashState, area: Rect) {
    let canvas = Canvas::default()
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            let symbols = ["♫", "♪", "♩", "♬", ".", "°"];
            
            for i in 0..40 {
                let x = ((i * 137) % 100) as f64;
                let y = ((i * 331) % 100) as f64;
                
                let symbol = symbols[i % symbols.len()];
                
                // Trộn 2 màu xám để nốt nhạc ẩn nhẹ vào nền
                let color = if i % 2 == 0 { Color::DarkGray } else { Color::Gray };
                
                ctx.print(x, y, ratatui::text::Span::styled(symbol, Style::default().fg(color)));
            }
        });
    f.render_widget(canvas, area);

    let logo = [
        "  ███████  ███████   ███████  ████████ ████████ ██    ██  ",
        " ██     ██ ██    ██ ██     ██    ██       ██     ██  ██   ",
        " ██        ██    ██ ██     ██    ██       ██      ████    ",
        "  ███████  ███████  ██     ██    ██       ██       ██     ",
        "        ██ ██       ██     ██    ██       ██       ██     ",
        " ██     ██ ██       ██     ██    ██       ██       ██     ",
        "  ███████  ██        ███████     ██       ██       ██     ",
    ].join("\n");

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35), 
            Constraint::Length(7),      
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(65),
            Constraint::Min(0),
        ])
        .split(vertical_chunks[1]);

    let splash_text = Paragraph::new(logo)
        .style(Style::default().fg(Color::LightBlue));

    f.render_widget(Clear, horizontal_chunks[1]);
    f.render_widget(splash_text, horizontal_chunks[1]);
}