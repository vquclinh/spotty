use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::QueueBlock {
        Color::Green
    } else {
        Color::White
    };

    let queue_text = if app.player.queue.is_empty() {
        " Hàng chờ đang trống. Hãy thêm bài hát vào đây!".to_string()
    } else {
        app.player.queue.join("\n ")
    };

    let block = Paragraph::new(queue_text).block(
        Block::default()
            .title(" Danh Sách Phát Tiếp Theo (Queue) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    f.render_widget(block, area);
}
