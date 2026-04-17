use crate::app::{ActiveBlock, App, route::Route, playlist_state::PlaylistState};
use crossterm::event::{KeyCode, KeyEvent};
use crate::ClientRequest;

pub fn handle_sidebar_events(key: KeyEvent, app: &mut App) {
    let App { playlists, route, active_block, network_tx, .. } = app;

    match key.code {
        KeyCode::Down | KeyCode::Char('j') => playlists.next(),
        KeyCode::Up | KeyCode::Char('k') => playlists.previous(),
        KeyCode::Enter => {
            if let Some(selected_idx) = playlists.state.selected() {
                if let Some(playlist) = playlists.items.get(selected_idx).cloned() {
                    
                    *route = Route::PlaylistDetail(PlaylistState::new(playlist.clone()));
                    *active_block = ActiveBlock::PlaylistTracks; 
                    
                    let _ = network_tx.send(ClientRequest::GetPlaylistTracks { 
                        playlist_id: playlist.id,
                        limit: Some(50), 
                        offset: None 
                    });
                }
            }
        }
        _ => {}
    }
}