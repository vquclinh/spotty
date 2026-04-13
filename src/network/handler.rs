use crate::app::state::SharedState;
use crate::network::client::WebApiClient;
use crate::network::request::ClientRequest;
use crate::network::request::PlayerRequest;
use tokio::sync::mpsc;

// to know what type of Request and do it with WebApiClient
// and store it into SharedState
pub async fn start_network_worker(
    mut client: WebApiClient,
    mut rx: mpsc::UnboundedReceiver<ClientRequest>,
    shared_state: SharedState,
) {
    while let Some(request) = rx.recv().await {

        while let Some(request) = rx.recv().await {
            match request {
                ClientRequest::GetUserPlaylists => {
                    if let Ok(playlists) = client.get_user_playlists().await {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playlists = playlists;
                        }
                    }
                }
                ClientRequest::GetRecentlyPlayed { limit } => {
                    if let Ok(tracks) = client.get_recently_played(limit).await {
                        if let Ok(mut state) = shared_state.lock() {
                            state.recent_tracks = tracks;
                        }
                    }
                }
                _ => {}
            }
        }
    
    }
}