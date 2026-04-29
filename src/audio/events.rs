use librespot_core::{spotify_uri::SpotifyUri, SpotifyUri as SpotifyUriEnum};
use librespot_playback::player::PlayerEvent as LibrespotEvent;

// helper (delete all SpotifyUriEnum except Track and Episode)
fn spotify_uri_to_string(uri: &SpotifyUri) -> Option<String> {
    match uri {
        SpotifyUriEnum::Track { .. } | SpotifyUriEnum::Episode { .. } => {
            uri.to_uri().ok()
        }
        _ => None,
    }
}

// audio events
#[derive(Debug, Clone)]
pub enum AudioEvent {
    Changed { uri: String },
    Playing { uri: String, position_ms: u32 },
    Paused { uri: String, position_ms: u32 },
    EndOfTrack { uri: String },
}

impl AudioEvent {
    // convert from librespot event
    pub fn from_librespot(event: LibrespotEvent) -> Option<Self> {
        match event {
            LibrespotEvent::TrackChanged { audio_item } => {
                let uri = spotify_uri_to_string(&audio_item.track_id)?;
                Some(Self::Changed { uri })
            }

            LibrespotEvent::Playing {
                track_id,
                position_ms,
                ..
            } => {
                let uri = spotify_uri_to_string(&track_id)?;
                Some(Self::Playing { uri, position_ms })
            }

            LibrespotEvent::Paused {
                track_id,
                position_ms,
                ..
            } => {
                let uri = spotify_uri_to_string(&track_id)?;
                Some(Self::Paused { uri, position_ms })
            }

            LibrespotEvent::EndOfTrack { track_id, .. } => {
                let uri = spotify_uri_to_string(&track_id)?;
                Some(Self::EndOfTrack { uri })
            }

            LibrespotEvent::Seeked {
                track_id,
                position_ms,
                ..
            } => {
                let uri = spotify_uri_to_string(&track_id)?;
                Some(Self::Playing { uri, position_ms })
            }

            _ => None,
        }
    }
}