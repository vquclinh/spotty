pub mod app;
pub mod event;
pub mod handlers;
pub mod network;
pub mod ui;
pub mod audio;

use app::App;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    event::{poll, read, Event, KeyEventKind}
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};
use std::{io, panic, time::{Duration, Instant}};

use handlers::handle_key_events;
use anyhow::Result;

use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio::sync::oneshot;
use crate::network::{client::SpotifyClient, models::PlaybackContext};
use crate::network::request::{ClientRequest, PlayerRequest};
use crate::network::handler::start_network_worker;

use crate::audio::events::*;
use crate::audio::player::*;

use crate::app::state::IoSharedState;

#[macro_export]
macro_rules! log_to_file {
    ($file:expr, $($arg:tt)*) => {{
        use ::std::io::Write;
        
        let mut file = ::std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open($file)
            .expect("Failed to open the log file");
            
        ::std::writeln!(file, $($arg)*).expect("Failed to write to the log file");
    }};
}

pub async fn run() -> Result<()> {
    // when app crash, call disable_raw_mode()
    panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        eprintln!("App crashed: {}", info);
    }));

    let (network_tx, network_rx) = mpsc::unbounded_channel::<ClientRequest>();
    let (audio_cmd_tx, audio_cmd_rx) = mpsc::unbounded_channel::<AudioCommand>();
    let (audio_event_tx, audio_event_rx) = mpsc::unbounded_channel::<AudioEvent>();

    let spotify_client = Arc::new(SpotifyClient::new().await?);

    // shared_state
    let shared_state = Arc::new(Mutex::new(IoSharedState::default()));

    // network
    let network_client = Arc::clone(&spotify_client);
    let audio_cmd_tx_for_net = audio_cmd_tx.clone();
    let network_shared_state = Arc::clone(&shared_state);

    tokio::spawn(async move {
        start_network_worker(network_client, network_rx, audio_cmd_tx_for_net, network_shared_state).await;
    });

    // audio
    let network_client = Arc::clone(&spotify_client);
    let net_tx_for_audio = network_tx.clone();
    let audio_shared_state = Arc::clone(&shared_state);
    tokio::spawn(async move {
        if let Err(e) = start_audio_worker(network_client, audio_cmd_rx, audio_event_tx, net_tx_for_audio, audio_shared_state).await {
            let _ = std::fs::write("audio_crash.log", format!("Audio Worker crash:\n{:#?}", e));
        }
    });

    // app
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(network_tx, audio_event_rx, Arc::clone(&shared_state));

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        if app.should_quit {
            if let Some(pb) = &app.playback {
                // Send shutdown request to cache current playback
                let (reply_tx, reply_rx) = oneshot::channel();
                let _ = app.network_tx.send(ClientRequest::Player {
                    request: PlayerRequest::Shutdown(PlaybackContext::from_playback(pb), reply_tx),
                    is_active_device: app.device_state.is_active_device()
                });
                let _ = reply_rx.await;
            }

            break;
        }

        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if poll(timeout)? && let Event::Key(key) = read()? {
            if key.kind == KeyEventKind::Press {
                handle_key_events(key, &mut app);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick(last_tick.elapsed());

            last_tick = Instant::now();
        }
    }

    let _ = std::fs::create_dir_all(".spotty_cache");
    let _ = app.app_cache.save(App::APP_CACHE_PATH);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
