// Nhớ import Event của bạn vào nhé
use crate::event::core::Event; 
use crate::app::{App, route::Route};
use crossterm::event::KeyCode;

pub fn handle(evt: Event, app: &mut App) {
    match evt {
        // Nếu là Tick (hết 50ms), ta không làm gì cả vì app.on_tick() ở lib.rs đã lo rồi
        Event::Tick => {}

        // Nếu người dùng bấm phím
        Event::Key(key) => {
            // Phím tắt toàn cục: Bấm Ctrl+C hoặc Q để thoát ở mọi nơi
            if key.code == KeyCode::Char('q') {
                app.should_quit = true;
                return;
            }

            // Uỷ quyền xử lý phím bấm cho từng màn hình
            match &mut app.route {
                Route::Search(search_state) => {
                    // Truyền phím bấm cho màn hình Search tự lo
                    // search_state.handle_key(key); 
                }
                Route::Home(home_state) => {
                    // home_state.handle_key(key);
                }
                // Các Route khác...
                _ => {}
            }
        }
    }
}