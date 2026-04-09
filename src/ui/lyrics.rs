use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, _app: &App, area: Rect) {
    let text = "\n\n♫ ... Lyrics here ... ♫\n\n";

    let block = Paragraph::new(text)
        .block(
            Block::default()
                .title(" LYRICS ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightCyan)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, area);
}
