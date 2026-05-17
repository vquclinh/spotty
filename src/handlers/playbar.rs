use crate::app::{ActiveBlock, App, route::Route};
use crate::app::queue_state::QueueState;
use crate::app::playbar_state::PlaybarItem;
use crate::network::request::{ClientRequest, PlayerRequest};
use crate::network::models::RepeatState;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_playbar_events(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Left | KeyCode::Char('h') => {
            app.playbar.hovered_item = app.playbar.hovered_item.prev();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.playbar.hovered_item = app.playbar.hovered_item.next();
        }
        KeyCode::Down | KeyCode::Char('j') if app.playbar.hovered_item == PlaybarItem::Volume => {
            if let Some(pb) = &mut app.playback {
                let vol = &mut pb.device.volume;
                *vol = vol.saturating_sub(5);
                let _ = app.network_tx.send(ClientRequest::Player {
                    request: PlayerRequest::SetVolume(*vol),
                    is_active_device: app.device_state.is_active_device()
                });
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(pb) = &mut app.playback {
                let vol = &mut pb.device.volume;
                *vol = vol.saturating_add(5).min(100);
                let _ = app.network_tx.send(ClientRequest::Player {
                    request: PlayerRequest::SetVolume(*vol),
                    is_active_device: app.device_state.is_active_device()
                });
            }
        }
        KeyCode::Enter => {
            let Some(playback) = &mut app.playback else { return };

            match app.playbar.hovered_item {
                PlaybarItem::Volume => {
                    // TODO
                }
                PlaybarItem::Lyrics => {
                    // TODO
                }
                PlaybarItem::Queue => {
                    if !matches!(app.route, Route::Queue(_)) {
                        app.set_current_route(Route::Queue(QueueState::default()));
                    }
                    app.active_block = ActiveBlock::QueueBlock;
                }
                PlaybarItem::Shuffle => {
                    let shuffling = playback.shuffle_state;
                    playback.shuffle_state = !shuffling;
                    let _ = app.network_tx.send(ClientRequest::Player {
                        request: PlayerRequest::ToggleShuffle(shuffling),
                        is_active_device: app.device_state.is_active_device()
                    });
                }
                PlaybarItem::Repeat => {
                    let state = match playback.repeat_state {
                        RepeatState::Off => RepeatState::Context,
                        RepeatState::Context => RepeatState::Track,
                        RepeatState::Track => RepeatState::Off,
                    };
                    playback.repeat_state = state;
                    let _ = app.network_tx.send(ClientRequest::Player {
                        request: PlayerRequest::SetRepeatMode(state),
                        is_active_device: app.device_state.is_active_device()
                    });
                }
            }
        }
        _ => {}
    }
}
