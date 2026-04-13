use crate::app::state::SharedState;
use crate::network::client::{self, WebApiClient};
use crate::network::request::{self, ClientRequest};
use tokio::sync::mpsc;

pub async fn start_network_worker(
    mut client: WebApiClient,
    mut rx: mpsc::UnboundedReceiver<ClientRequest>,
    shared_state: SharedState,
) {
    while let Some(request) = rx.recv().await {
        match request {
            ClientRequest::GetRecentlyPlayed { limit } => {
                if let Ok(tracks) = client.get_recently_played(limit).await {
                    let mut state = shared_state.lock().unwrap();
                    state.recent_tracks = tracks;
                }
            }

            _ => {}
        }
    }
}