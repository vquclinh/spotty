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
        #[allow(clippy::single_match)]   // alow for single match to delete warning
        match request {
            ClientRequest::GetRecentlyPlayed { limit } => {
                if let Ok(tracks) = client.get_recently_played(limit).await {
                    let mut state = shared_state.lock().unwrap();
                    state.recent_tracks = tracks;
                }
            }

            // TODO
            ClientRequest::Player(player_req) => {
                match player_req {
                    PlayerRequest::TogglePlayback(p) => { 
                        if let Err(e) = client.toggle_playback(p).await {
                            eprintln!("Network Error: {}", e); 
                        }
                    }
                    PlayerRequest::NextTrack => { 
                        let _ = client.next_track().await;
                    }
                    PlayerRequest::PreviousTrack => { 
                        let _ = client.next_track().await;
                    }
                }
            }
            
            _ => {}
        }
    }
}