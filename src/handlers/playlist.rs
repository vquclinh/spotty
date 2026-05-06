use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::network::models::*;
use crate::network::request::ClientRequest;

pub fn handle_playlist_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::PlaylistDetail(playlist_state) = &mut app.route {
        match key {
            KeyEvent{ code: KeyCode::Down, .. }
            | KeyEvent { code: KeyCode::Char('j'), ..}
            | KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let steps = if key.code == KeyCode::Char('d') { 10 } else { 1 };
                playlist_state.tracks.next(steps, false);
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

            KeyEvent { code: KeyCode::Up, .. }
            | KeyEvent { code: KeyCode::Char('k'), .. }
            | KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let steps = if key.code == KeyCode::Char('u') { 10 } else { 1 };
                playlist_state.tracks.previous(steps, false);
            }
            
            KeyEvent { code: KeyCode::Char('t'), .. } => {
                target_to_open = playlist_state.tracks.state.selected()
                    .and_then(|idx| playlist_state.tracks.items.get(idx))
                    .map(|item| match item {
                        PlayableItem::Track(t) => MenuTarget::Track(t.clone()),
                        PlayableItem::Episode(e) => MenuTarget::Episode(e.clone()),
                    });
            }

            KeyEvent { code: KeyCode::Backspace, .. }
            | KeyEvent{ code: KeyCode::Char('b'), .. } => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}
