use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent};
use crate::network::models::*;
use crate::network::request::ClientRequest;

pub fn handle_playlist_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::PlaylistDetail(playlist_state) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                playlist_state.tracks.next(false);
                // Fetch more if the cursor is near the end at this threshold
                let threshold = 20;

                if let Some(selected) = playlist_state.tracks.state.selected()
                    && playlist_state.tracks.items.len() - selected <= threshold
                    && !playlist_state.is_loading
                    && !playlist_state.is_end {
                    let _ = app.network_tx.send(ClientRequest::GetPlaylistItems {
                        playlist_id: playlist_state.playlist.id.clone(),
                        limit: app.page_limit,
                        offset: playlist_state.tracks.items.len() as u32
                    });

                    playlist_state.is_loading = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => playlist_state.tracks.previous(false),
            
            KeyCode::Char('t') => {
                target_to_open = playlist_state.tracks.state.selected()
                    .and_then(|idx| playlist_state.tracks.items.get(idx))
                    .map(|item| match item {
                        PlayableItem::Track(t) => MenuTarget::Track(t.clone()),
                        PlayableItem::Episode(e) => MenuTarget::Episode(e.clone()),
                    });
            }

            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}
