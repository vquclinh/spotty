use crate::app::state::SharedState;
use crate::network::client::WebApiClient;
use crate::network::request::{ClientRequest, PlayerRequest};
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
            ClientRequest::GetCurrentUser => {
                match client.get_current_user().await {
                    Ok(user) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.user = user;
                        }
                    }

                    Err(_e) => {}
                }   
            }

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

            ClientRequest::GetRecentlyPlayed { limit, offset } => {
                match client.get_recently_played_tracks(limit, offset).await { 
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.recent_tracks = tracks;
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetUserTopTracks { time_range, limit, offset } => {
                match client.get_user_top_tracks(time_range, limit, offset).await {
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.top_tracks = tracks; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetUserTopArtists { time_range, limit, offset } => {
                match client.get_user_top_artists(time_range, limit, offset).await {
                    Ok(artists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.top_artists = artists;
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetPlaylistItems { playlist_id, limit, offset } => {
                match client.get_playlist_items(&playlist_id, limit, offset).await {
                    Ok(items) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playlist_items = items; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetCurrentPlayback => {
                match client.get_current_playback().await {
                    Ok(playback) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playback = playback; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::SearchItems { query, search_types, limit, offset } => {
                match client.search_items(&query, search_types, limit, offset).await {
                    Ok(results) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.search_results = results; 
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetQueue => {
                match client.get_queue().await {
                    Ok(queue_res) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.queue_data = Some((queue_res.currently_playing, queue_res.queue));
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::AddItemsToPlaylist { playlist_id, uris } => {
                let uris_ref: Vec<&str> = uris.iter().map(|s| s.as_str()).collect();

                match client.add_items_to_playlist(&playlist_id, uris_ref).await {
                    Ok(_) => {
                        #[allow(clippy::collapsible_if)]
                        if let Ok(playlists) = client.get_user_playlists().await {
                            if let Ok(mut state) = shared_state.lock() {
                                state.playlists = playlists;
                            }
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetAlbum { id } => {
                match client.get_album(&id).await {
                    Ok(album) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.album_detail = Some(album);
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetUserLikedSongs { limit, offset } => {
                match client.get_user_liked_songs(limit, offset).await {
                    Ok(liked_songs) => {
                        if let Ok(mut state) = shared_state.lock() {
                            if offset == 0 {
                                state.liked_songs = liked_songs.items;
                            } else {
                                state.liked_songs.extend(liked_songs.items);
                            };
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetUserSavedAlbums { limit, offset } => {
                match client.get_user_saved_albums(limit, offset).await {
                    Ok(saved_albums) => {
                        if let Ok(mut state) = shared_state.lock() {
                            if offset == 0 {
                                state.saved_albums = saved_albums.items;
                            } else {
                                state.saved_albums.extend(saved_albums.items);
                            };
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetUserSavedArtists { limit, after } => {
                match client.get_user_saved_artists(limit, after.as_deref()).await {
                    Ok(saved_artists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            if after.is_none() {
                                state.saved_artists = saved_artists.items;
                            } else {
                                state.saved_artists.extend(saved_artists.items);
                            };
                        }
                    }
                    Err(_e) => {}
                }
            }

            ClientRequest::GetUserSavedPodcasts { limit, offset } => {
                match client.get_user_saved_podcasts(limit, offset).await {
                    Ok(saved_podcasts) => {
                        if let Ok(mut state) = shared_state.lock() {
                            if offset == 0 {
                                state.saved_podcasts = saved_podcasts.items;
                            } else {
                                state.saved_podcasts.extend(saved_podcasts.items);
                            };
                        }
                    }
                    Err(_e) => {}
                }
            }

            #[allow(clippy::collapsible_if)]
            ClientRequest::Player(player_req) => {
                let update_playback = || async {
                    // Optionally sleep here to wait for the server before we update
                    // This will result in a 100ms delay in the UI for operations
                    // that do not have client data like next_track
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    if let Ok(playback) = client.get_current_playback().await {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playback = playback; 
                        }
                    }
                };

                match player_req {
                    PlayerRequest::AddItemToQueue(uri) => {
                        let _ = client.add_item_to_queue(&uri).await;

                        if let Ok(queue_res) = client.get_queue().await {
                            if let Ok(mut state) = shared_state.lock() {
                                state.queue_data = Some((queue_res.currently_playing, queue_res.queue));
                            }
                        }
                    }

                    PlayerRequest::TogglePlayback(playing) => {
                        let _ = client.toggle_playback(playing).await;
                        // Mainly for debugging right now
                        update_playback().await;
                    }

                    PlayerRequest::NextTrack => {
                        let _ = client.next_track().await;
                        update_playback().await;
                    }

                    PlayerRequest::PreviousTrack => {
                        let _ = client.prev_track().await;
                        update_playback().await;
                    }

                    PlayerRequest::SetRepeatMode(state) => {
                        let _ = client.set_repeat_mode(state).await;
                        // update_playback().await;
                    }

                    PlayerRequest::ToggleShuffle(shuffling) => {
                        let _ = client.toggle_shuffle(shuffling).await;
                        // update_playback().await;
                    }

                    PlayerRequest::SetVolume(vol) => {
                        let _ = client.set_volume(vol).await;
                        // update_playback().await;
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }
}
