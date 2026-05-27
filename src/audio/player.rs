use anyhow::Context;
use librespot_connect::{
    ConnectConfig,
    LoadRequest,
    LoadRequestOptions,
    Spirc
};
use librespot_core::config::DeviceType;
use librespot_playback::audio_backend;
use librespot_playback::config::{AudioFormat, Bitrate, PlayerConfig};
use librespot_playback::mixer::{softmixer::SoftMixer, Mixer, MixerConfig};
use librespot_playback::player::Player;
use librespot_connect::PlayingTrack;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::app::state::SharedState;
use crate::network::client::SpotifyClient;
use crate::network::models::PlaybackContext;
use super::events::AudioEvent;
use crate::network::request::ClientRequest;

// audio commands
pub enum AudioCommand {
    Play(String), // play a single track or episode by uri
    PlayContext(String, LoadRequestOptions),
    Pause,
    Resume,
    NextTrack,
    PreviousTrack,

    Seek(u32),

    SetVolume(u16),

    Shutdown(PlaybackContext, oneshot::Sender<()>),
}

// convert volume to librespot volume
pub fn percent_to_librespot_volume(percent: u8) -> u16 {
    (f64::from(percent.min(100)) / 100.0 * 65535.0).round() as u16
}

// start_audio_worker
pub async fn start_audio_worker(
    client: Arc<SpotifyClient>,
    mut cmd_rx: mpsc::UnboundedReceiver<AudioCommand>, // receive from UI
    event_tx: mpsc::UnboundedSender<AudioEvent>, // send event signal to UI
    net_tx: mpsc::UnboundedSender<ClientRequest>, // send request fetch API to network
    shared_state: SharedState,
    initial_volume_percent: u8,
) -> anyhow::Result<()> {
    // mixer
    let mixer = Arc::new(
        SoftMixer::open(MixerConfig::default()).context("Failed to open SoftMixer")?,
    );
    let initial_volume = percent_to_librespot_volume(initial_volume_percent);
    mixer.set_volume(initial_volume);

    // backend + player
    let backend = audio_backend::find(None).expect("No audio backend found");
    let player_config = PlayerConfig {
        bitrate: Bitrate::Bitrate320,
        ..Default::default()
    };

    let player = Player::new(
        player_config,
        client.session.clone(),
        mixer.get_soft_volume(),
        move || backend(None, AudioFormat::default()),
    );

    // update shared_state - event loop
    // get player event by librespot
    let mut player_event_rx = player.get_player_event_channel();
    let event_tx_clone = event_tx.clone();
    let net_tx_clone = net_tx.clone();
    let state_for_events = shared_state.clone();

    // a loop for converting to AudioEvent and send to ui
    tokio::spawn(async move {
        while let Some(raw_event) = player_event_rx.recv().await {
            if let Some(app_event) = AudioEvent::from_librespot(raw_event) {
                match &app_event {
                    AudioEvent::Changed { .. } => {
                        let net_tx_clone = net_tx.clone();
                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_millis(500)).await;
                            let _ = net_tx_clone.send(ClientRequest::GetCurrentPlayback);
                        });
                    }
                    AudioEvent::Playing { position_ms, .. } => {
                        if let Ok(mut state) = state_for_events.lock()
                        && let Some(pb) = state.playback.as_mut() {
                            pb.is_playing = true;
                            pb.progress = Duration::from_millis(*position_ms as u64);
                        }
                    }
                    AudioEvent::Paused { position_ms, .. } => {
                        if let Ok(mut state) = state_for_events.lock()
                        && let Some(pb) = state.playback.as_mut() {
                            pb.is_playing = false;
                            pb.progress = Duration::from_millis(*position_ms as u64);
                        }
                    }
                    AudioEvent::EndOfTrack { .. } => {
                        let net_tx_delayed = net_tx.clone();
                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_millis(500)).await;
                            let _ = net_tx_delayed.send(ClientRequest::GetCurrentPlayback);
                        });
                    }
                    _ => {}
                }

                // send signal to ui immediately (sth like change track for lyrics page)
                let _ = event_tx_clone.send(app_event);
            }
        }
    });

    // spotify connect device (spirc setup)
    let connect_config = ConnectConfig {
        name: "Spotty TUI".to_string(),
        device_type: DeviceType::Computer,
        initial_volume,
        is_group: false,
        disable_volume: false,
        volume_steps: 64,
    };

    let (spirc, spirc_task) = Spirc::new(connect_config, client.session.clone(), client.creds.clone(), player, mixer.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize Spirc: {e:#}"))?;

    // Spawn the task here so the device is registered as online
    tokio::spawn(spirc_task);
    let _ = net_tx_clone.send(ClientRequest::GetDevices);

    // Sync remote playback
    let (reply_tx, reply_rx) = oneshot::channel();
    let _ = net_tx_clone.send(ClientRequest::GetCurrentPlaybackReply(reply_tx));
    
    // Timeout so init doesn't hang forever
    let playback = tokio::time::timeout(Duration::from_secs(2), reply_rx)
        .await
        .ok()
        .and_then(|res| res.ok())
        .flatten();
    
    if playback.is_none() {
        let _ = spirc.activate();
        load_spirc_from_cache(&spirc, ".spotty_cache/playback.json");
    }

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(300)).await;
        let _ = net_tx_clone.send(ClientRequest::GetDevices);
    });

    // command loop
    tokio::spawn(async move {
        while let Some(cmd) = cmd_rx.recv().await {
            match cmd {
                AudioCommand::Play(uri) => {
                    // use from_tracks for playing single playable item (track/episode)
                    let req = LoadRequest::from_tracks(
                        vec![uri],
                        LoadRequestOptions {
                            start_playing: true,
                            ..Default::default()
                        },
                    );
                    let _ = spirc.load(req);
                }

                AudioCommand::PlayContext(context_uri, options) => {
                    // use from_context_uri for playing context (album, playlist, artist)
                    let req = LoadRequest::from_context_uri(
                        context_uri,
                        options
                    );
                    let _ = spirc.load(req);
                }

                AudioCommand::Pause => {
                    let _ = spirc.pause();
                }

                AudioCommand::Resume => {
                    let _ = spirc.play();
                }

                AudioCommand::NextTrack => {
                    let _ = spirc.next();
                }

                AudioCommand::PreviousTrack => {
                    let _ = spirc.prev();
                }

                AudioCommand::Seek(position_ms) => {
                    let _ = spirc.set_position_ms(position_ms);
                }

                AudioCommand::SetVolume(vol) => {
                    mixer.set_volume(vol); // set volume local
                    let _ = spirc.set_volume(vol); // sync data with server
                }

                AudioCommand::Shutdown(pb, reply_rx) => {
                    // Extract state and save to cache
                    if let Err(e) = std::fs::create_dir_all(".spotty_cache") {
                        let _ = std::fs::write("cache_debug.log", format!("Failed to create dir: {e}"));
                    }
                    match serde_json::to_string(&pb) {
                        Ok(cache_str) => {
                            if let Err(e) = std::fs::write(".spotty_cache/playback.json", cache_str) {
                                let _ = std::fs::write("cache_debug.log", format!("Failed to write cache: {e}"));
                            }
                        }
                        Err(e) => {
                            let _ = std::fs::write("cache_debug.log", format!("Failed to serialize cache: {e}"));
                        }
                    }

                    let _ = spirc.shutdown();
                    let _ = reply_rx.send(());
                    break;
                }
            }
        }
    });

    Ok(())
}

fn load_spirc_from_cache(spirc: &Spirc, cache_path: &str) {
    match std::fs::read_to_string(cache_path) {
        Ok(cache_data) => {
            match serde_json::from_str::<PlaybackContext>(&cache_data) {
                Ok(cache) => {
                    if let Some(context_uri) = &cache.context_uri {
                        let context_options = Some(cache.to_librespot_options());
                        let playing_track = cache.playing_track_uri.map(PlayingTrack::Uri);
                        let req = LoadRequest::from_context_uri(
                            context_uri.clone(),
                            LoadRequestOptions {
                                start_playing: false,
                                seek_to: cache.progress.as_millis() as u32,
                                playing_track,
                                context_options,
                            },
                        );

                        let _ = spirc.load(req);
                        
                    } else if let Some(track_uri) = &cache.playing_track_uri {
                        // Fallback for single tracks without context
                        let context_options = Some(cache.to_librespot_options());
                        let req = LoadRequest::from_tracks(
                            vec![track_uri.clone()],
                            LoadRequestOptions {
                                start_playing: false,
                                seek_to: cache.progress.as_millis() as u32,
                                context_options,
                                ..Default::default()
                            },
                        );
                        let _ = spirc.load(req);

                    } else {
                        let _ = std::fs::write("cache_debug.log", "Cache read ok, but both context_uri and playing_track_uri were empty");
                    }
                }
                Err(e) => { let _ = std::fs::write("cache_debug.log", format!("Failed to parse JSON: {e}")); }
            }
        }
        Err(e) => { let _ = std::fs::write("cache_debug.log", format!("Failed to read cache file: {e}")); }
    }
}
