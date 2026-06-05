use crate::app::{App, Route};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_lyrics_events(key: KeyEvent, app: &mut App) {
    if let Route::Lyrics(state) = &mut app.state.current_mut().route {
        // check if synced data or not
        let is_unsynced = state.data.as_ref().map_or(false, |lyrics_data| {
            format!("{:?}", lyrics_data.lyrics.sync_type).to_uppercase().contains("UNSYNCED")
                || lyrics_data.lyrics.lines.last().map_or(false, |l| {
                    l.start_time_ms == "0" || l.start_time_ms.is_empty()
                })
        });

        if is_unsynced {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    state.scroll_offset = state.scroll_offset.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let max_scroll = state.data.as_ref().map_or(0, |lyrics_data| {
                        let total_lines = lyrics_data.lyrics.lines.len() as u16;
                        total_lines.saturating_sub(25)
                    });

                    if state.scroll_offset < max_scroll {
                        state.scroll_offset = state.scroll_offset.saturating_add(1);
                    }
                }
                _ => {}
            }
        }
    }
}