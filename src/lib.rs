pub mod app;
pub mod event;
pub mod handlers;
pub mod network;
pub mod ui;

use app::App;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    event::{poll, read, Event}
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};
use std::{io, panic, time::{Duration, Instant}};

use handlers::handle_key_events;
use anyhow::{Result};

use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use crate::network::client::WebApiClient;
use crate::network::request::ClientRequest;
use crate::network::handler::start_network_worker;
use crate::network::models::PlayableItem;

use crate::app::state::IoSharedState;

pub async fn run() -> Result<()> {
    // when app crash, call disable_raw_mode()
    panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        eprintln!("App crashed: {}", info);
    }));

    let (network_tx, network_rx) = mpsc::unbounded_channel::<ClientRequest>();

    let spotify_client = WebApiClient::new(Some(1800)).await?;
    
    let shared_state = Arc::new(Mutex::new(IoSharedState::default()));
    let network_shared_state = Arc::clone(&shared_state);
    
    // Move the receiver to a dedicated background thread
    tokio::spawn(async move {
        start_network_worker(spotify_client, network_rx, network_shared_state).await;     
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(network_tx, Arc::clone(&shared_state));
    
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if poll(timeout)? && let Event::Key(key) = read()? {
            handle_key_events(key, &mut app);
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick(timeout);

            // Fetch the new song when current song ends (locally)
            #[allow(clippy::collapsible_if)]
            if let Some(playback) = &app.playback && let Some(item) = &playback.item {
                if let PlayableItem::Track(t) = item && playback.progress >= t.duration {
                    let _ = app.network_tx.send(ClientRequest::GetCurrentPlayback);
                }
            }

            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
