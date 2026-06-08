use crate::app::state::SharedState;
use crate::app::device_state::DeviceState;
use crate::network::client::SpotifyClient;
use crate::network::request::{ClientRequest, PlayerRequest};
use crate::network::models::SearchType;
use crate::audio::player::*;
use librespot_connect::LoadContextOptions;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

// Match request type and execute it with WebApiClient
// then store in shared state
pub async fn start_network_worker(
    client: Arc<SpotifyClient>,
    mut rx: mpsc::UnboundedReceiver<ClientRequest>,
    audio_tx: mpsc::UnboundedSender<AudioCommand>,
    shared_state: SharedState,
) {
    crate::spotty_info!("network", "Network worker started successfully");
    while let Some(request) = rx.recv().await {
        match request {
            ClientRequest::GetCurrentUser => {
                match client.get_current_user().await {
                    Ok(user) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.user = user;
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }   
            }

            ClientRequest::GetUserPlaylists { limit, offset } => {
                match client.get_user_playlists(limit, offset).await {
                    Ok(playlists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playlists = playlists.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetRecentlyPlayed { limit, after } => {
                match client.get_recently_played_tracks(limit, after).await { 
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.recent_tracks = tracks.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetUserTopTracks { time_range, limit, offset } => {
                match client.get_user_top_tracks(time_range, limit, offset).await {
                    Ok(tracks) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.top_tracks = tracks.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetUserTopArtists { time_range, limit, offset } => {
                match client.get_user_top_artists(time_range, limit, offset).await {
                    Ok(artists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.top_artists = artists.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetPlaylistItems { playlist_id, limit, offset } => {
                match client.get_playlist_items(&playlist_id, limit, offset).await {
                    Ok(items) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playlist_items = items.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetCurrentPlayback => {
                match client.get_current_playback().await {
                    Ok(playback) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.playback = playback; 
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetCurrentPlaybackReply(reply_rx) => {
                match client.get_current_playback().await {
                    Ok(playback) => {
                        if let Ok(mut state) = shared_state.lock() {
                            let _ = reply_rx.send(playback.clone());
                            state.playback = playback; 
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::SearchItems { query, search_types, limit, offset } => {
                match client.search_items(&query, search_types, limit, offset).await {
                    Ok(results) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.search_results = results;
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }
            
            ClientRequest::SearchItemsUpTo { query, search_types, total_limit, start_offset } => {
                let page_size = 50;
                let mut curr_offset = start_offset;
                let mut search_types = search_types.clone();

                while curr_offset < total_limit && !search_types.is_empty() {
                    let mut batch_max_len = 0;
                    
                    #[allow(clippy::single_match)]
                    match client
                        .search_items(&query, search_types.clone(), page_size, curr_offset)
                        .await
                    {
                        Ok(results) => {
                            if let Ok(mut state) = shared_state.lock() {
                                if let Some(page) = results.tracks && !page.items.is_empty() {
                                    batch_max_len = std::cmp::max(batch_max_len, page.items.len());
                                    if page.next.is_none() {
                                        search_types.retain(|t| *t != SearchType::Track);
                                    }
                                    state.search_results.tracks = Some(page);
                                }
                                if let Some(page) = results.artists && !page.items.is_empty() {
                                    batch_max_len = std::cmp::max(batch_max_len, page.items.len());
                                    if page.next.is_none() {
                                        search_types.retain(|t| *t != SearchType::Artist);
                                    }
                                    state.search_results.artists = Some(page);
                                }
                                if let Some(page) = results.albums && !page.items.is_empty() {
                                    batch_max_len = std::cmp::max(batch_max_len, page.items.len());
                                    if page.next.is_none() {
                                        search_types.retain(|t| *t != SearchType::Album);
                                    }
                                    state.search_results.albums = Some(page);
                                }
                                if let Some(page) = results.playlists && !page.items.is_empty() {
                                    batch_max_len = std::cmp::max(batch_max_len, page.items.len());
                                    if page.next.is_none() {
                                        search_types.retain(|t| *t != SearchType::Playlist);
                                    }
                                    state.search_results.playlists = Some(page);
                                }
                            }
                        }
                        

                        Err(e) => {
                            crate::spotty_error!("network", "{}", e);
                        }
                    }

                    curr_offset += batch_max_len as u32;
                    if batch_max_len == 0 {
                        // Avoid stalling the worker when the page is empty.
                        break;
                    }
                }
            }

            ClientRequest::GetQueue => {
                match client.get_queue().await {
                    Ok(mut res) => {
                        // If the first item in queue_items is a duplicate
                        // of currently_playing then we remove it
                        if let Some(current) = &res.currently_playing {
                            let is_dup = res.queue.first()
                                .is_some_and(|first| current.uri() == first.uri());
        
                            if is_dup {
                                res.queue.remove(0);
                            }
                        }
        
                        if let Ok(mut state) = shared_state.lock() {
                            state.queue_data = Some((res.currently_playing, res.queue));
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::AddItemsToPlaylist { playlist_id, uris } => {
                let uris_ref: Vec<&str> = uris.iter().map(|s| s.as_str()).collect();

                match client.add_items_to_playlist(&playlist_id, uris_ref).await {
                    Ok(_) => {
                        // FIXME
                        if let Ok(page) = client.get_user_playlists(50, 0).await
                            && let Ok(mut state) = shared_state.lock() {
                            state.playlists = page.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::RemoveItemsFromPlaylist { playlist_id, uris } => {
                let uris_ref: Vec<&str> = uris.iter().map(|s| s.as_str()).collect();

                let _ = client.remove_items_from_playlist(&playlist_id, uris_ref).await;
            }

            ClientRequest::GetAlbum { id } => {
                match client.get_album(&id).await {
                    Ok(album) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.album_detail = Some(album);
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetUserLikedSongs { limit, offset } => {
                match client.get_user_liked_songs(limit, offset).await {
                    Ok(liked_songs) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.liked_songs = liked_songs.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetUserSavedAlbums { limit, offset } => {
                match client.get_user_saved_albums(limit, offset).await {
                    Ok(saved_albums) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.saved_albums = saved_albums.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetUserSavedArtists { limit, after } => {
                match client.get_user_saved_artists(limit, after.as_deref()).await {
                    Ok(saved_artists) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.saved_artists = saved_artists.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::GetUserSavedPodcasts { limit, offset } => {
                match client.get_user_saved_podcasts(limit, offset).await {
                    Ok(saved_podcasts) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.saved_podcasts = saved_podcasts.into();
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            #[allow(clippy::collapsible_if)]
            ClientRequest::SaveItemsToLibrary(uris) => {
                if uris.is_empty() { continue; };
                // FIXME
                let uris: Vec<&str> = uris.iter().map(|u| u.as_str()).collect();
                // Assuming all the items are of the same type
                let first_uri = uris[0];

                let _ = client.save_items_to_library(uris).await;

                if first_uri.contains(":track:") {
                    if let Ok(page) = client.get_user_liked_songs(50, 0).await
                        && let Ok(mut state) = shared_state.lock() {
                            state.liked_songs = page.into();
                    }
                } else if first_uri.contains(":album:") {
                    if let Ok(page) = client.get_user_saved_albums(50, 0).await
                        && let Ok(mut state) = shared_state.lock() {
                            state.saved_albums = page.into();
                    }
                } else if first_uri.contains(":artist:") {
                    if let Ok(page) = client.get_user_saved_artists(50, None).await
                        && let Ok(mut state) = shared_state.lock() {
                            state.saved_artists = page.into();
                    }
                } else if first_uri.contains(":episode:") {
                    if let Ok(page) = client.get_user_saved_podcasts(50, 0).await
                        && let Ok(mut state) = shared_state.lock() {
                            state.saved_podcasts = page.into();
                    }
                }
            }

            #[allow(clippy::collapsible_if)]
            ClientRequest::RemoveItemsFromLibrary( uris ) => {
                let uris: Vec<&str> = uris.iter().map(|u| u.as_str()).collect();

                let _ = client.remove_items_from_library(uris).await;
            }

            #[allow(clippy::collapsible_if)]
            ClientRequest::Player { request, is_active_device } => {
                match request {
                    PlayerRequest::AddItemToQueue(uri) => {
                        let _ = client.add_item_to_queue(&uri).await;

                        if let Ok(queue_res) = client.get_queue().await {
                            if let Ok(mut state) = shared_state.lock() {
                                state.queue_data = Some((queue_res.currently_playing, queue_res.queue));
                            }
                        }
                    }

                    PlayerRequest::Play(uri) => {
                        if is_active_device {
                            let _ = audio_tx.send(AudioCommand::Play(uri));
                        } else {
                            let _ = client.start_uris_playback(
                                None,
                                [uri.as_str()],
                                None,
                                None
                            ).await;
                        }
                    }

                    PlayerRequest::PlayContext(context_uri, options) => {
                        if is_active_device {
                            let _ = audio_tx.send(AudioCommand::PlayContext(context_uri, options));
                        } else {
                            let shuffle = options.context_options.is_some_and(|opts| {
                                matches!(opts, LoadContextOptions::Options(opt) if opt.shuffle)
                            });
                            let offset = options.playing_track.map(|t| t.into());
                            let _ = client.start_context_playback(
                                None,
                                &context_uri,
                                offset,
                                None
                            ).await;
                            // Send a separate shuffle request because Spotify endpoint does not
                            // take shuffle state 
                            if shuffle {
                                let client_clone = Arc::clone(&client);
                                tokio::spawn(async move {
                                    tokio::time::sleep(Duration::from_millis(200)).await;
                                    let _ = client_clone.toggle_shuffle(false).await;
                                });
                            }
                        }
                    }
                    
                    PlayerRequest::TogglePlayback(playing) => {
                        if is_active_device {
                            let cmd = if playing { AudioCommand::Pause } else { AudioCommand::Resume };
                            let _ = audio_tx.send(cmd);
                        } else {
                            let _ = client.toggle_playback(playing).await;
                        }
                    }

                    PlayerRequest::NextTrack => {
                        if is_active_device {
                            let _ = audio_tx.send(AudioCommand::NextTrack);
                        } else {
                            let _ = client.next_track().await;
                        }
                    }

                    PlayerRequest::PreviousTrack => {
                        if is_active_device {
                            let _ = audio_tx.send(AudioCommand::PreviousTrack);
                        } else {
                            let _ = client.prev_track().await;
                        }
                    }

                    PlayerRequest::SeekToPosition(ms) => {
                        if is_active_device {
                            let _ = audio_tx.send(AudioCommand::Seek(ms));
                        } else {
                            let _ = client.seek_to_position(ms).await;
                        }
                    }
                    
                    PlayerRequest::SetVolume(vol) => {
                        if is_active_device {
                            let _ = audio_tx.send(AudioCommand::SetVolume(percent_to_librespot_volume(vol)));
                        } else {
                            let _ = client.set_volume(vol).await;
                        }
                    }

                    PlayerRequest::SetRepeatMode(state) => {
                        let _ = client.set_repeat_mode(state).await;
                    }

                    PlayerRequest::ToggleShuffle(shuffling) => {
                        let _ = client.toggle_shuffle(shuffling).await;
                    }

                    PlayerRequest::Shutdown(playback_opt, reply_rx) => {
                        let _ = audio_tx.send(AudioCommand::Shutdown(playback_opt, reply_rx));
                    }
                }
            }

            ClientRequest::GetLyrics { track_id } => {
                match client.get_lyrics(&track_id).await {
                    Ok(lyrics_opt) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.lyrics_data = lyrics_opt;
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }

            ClientRequest::TransferPlayback { device_id, should_play } => {
                let device_id = device_id.unwrap_or(client.session.device_id().to_string());
                let _ = client.transfer_playback(&device_id, should_play).await;
            }
            
            ClientRequest::GetDevices => {
                match client.get_devices().await {
                    Ok(devices) => {
                        if let Ok(mut state) = shared_state.lock() {
                            let mut device_state = DeviceState::default();
                            device_state.online_devices.items = devices;

                            let local_id = client.session.device_id().to_string();
                            device_state.local_device_idx = device_state
                                .online_devices
                                .items
                                .iter()
                                .position(|d| d.id.as_deref() == Some(local_id.as_str()));
                            
                            // Select the active device by default
                            device_state.online_devices.state.select(device_state.active_device_idx());
                            
                            state.devices = Some(device_state);
                        }
                    }

                    Err(e) => {
                        crate::spotty_error!("network", "{}", e);
                    }
                }
            }
        }
    }
}
