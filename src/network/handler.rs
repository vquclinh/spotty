use crate::app::state::SharedState;
use crate::network::client::WebApiClient;
use crate::network::request::ClientRequest;
use tokio::sync::mpsc;

// Match request type and execute it with WebApiClient
// then store in shared state
pub async fn start_network_worker(
    mut client: WebApiClient,
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
            _ => {}
        }
    }
}
