use crate::app::state::SharedState;
use crate::network::client::WebApiClient;
use crate::network::request::ClientRequest;
use tokio::sync::mpsc;

// Match request type and execute it with WebApiClient
// then store in shared state
pub async fn start_network_worker(
    client: WebApiClient,
    mut rx: mpsc::UnboundedReceiver<ClientRequest>,
    shared_state: SharedState,
) {
    while let Some(request) = rx.recv().await {
        match request {
            ClientRequest::GetUserPlaylists => {
                match client.get_user_playlists().await {
                    Ok(playlists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playlists = playlists;
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetRecentlyPlayed { limit } => {
                match client.get_recently_played(limit).await { 
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.recent_tracks = tracks;
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetTopTracks { limit } => {
                match client.get_user_top_tracks(limit).await {
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.top_tracks = tracks; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetTopArtists { limit } => {
                match client.get_user_top_artists(limit).await {
                    Ok(artists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.top_artists = artists;
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetPlaylistTracks { playlist_id, limit, offset } => {
                match client.get_playlist_tracks(&playlist_id, limit, offset).await {
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playlist_tracks = tracks; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetCurrentPlayback => {
                match client.get_playback_state().await {
                    Ok(playback) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playback = playback; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::SearchAll { query, limit } => {
                match client.search_all(&query, limit).await {
                    Ok(results) => {
                        let debug_info = format!(" SEARCH QUERY: {} \n{:#?}", query, results);
                        let _ = std::fs::write("debug_search.txt", debug_info);

                        if let Ok(mut state) = shared_state.lock() {
                            state.search_results = results; 
                        }
                    }
                    Err(e) => {
                        let error_info = format!(" ERROR QUERY: {} \n{:#?}", query, e);
                        let _ = std::fs::write("debug_search.txt", error_info);
                    }
                }
            }

            _ => {}
        }
    }
}
