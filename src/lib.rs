pub mod app;
pub mod config;
pub mod event;
pub mod handlers;
pub mod network;
pub mod redirect_uri;
pub mod ui;
pub mod util;

use app::App;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
};
use std::{io, panic, time::Duration};
use anyhow::{Result, Context, anyhow, bail};
use crate::network::client::*;

pub async fn run() -> Result<()> {
    panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        eprintln!("App crashed: {}", info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new().await;
    let tick_rate = Duration::from_millis(50);

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;

        let evt = event::read(tick_rate)?;

        handlers::handle(evt, &mut app);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

pub async fn test_auth() -> Result<()> {
    let app = App::new().await;
    let Some(client) = app.client else {
        bail!("Client not initialized");
    };

    match client.get_user_profile().await {
        Ok(profile) => {
            println!("User profile:");
            println!("Id: {}", profile.id);
            println!("Name: {}\n", profile.display_name);
        }
        Err(e) => {
            eprintln!("Cannot fetch user profile: {}", e);
        }
    }

    match client.get_playback_state().await {
        Ok(Some(playback)) => {
            println!("Current playback:");
            println!("Device: {}", playback.device_name);

            // Handling the Playable enum (Track or Episode)
            if let Some(item) = playback.item {
                match item {
                    Playable::Track(t) => println!("Track: {} by {}", t.name, t.artists[0].name),
                    Playable::Episode(e) => println!("Episode: {}", e.name),
                }
            } else {
                println!("No playable item recognized");
            }
            println!("Status: {}", if playback.is_playing { "Playing" } else { "Paused" });
        }
        Ok(None) => {
            println!("Nothing is currently playing.");
        }
        Err(e) => {
            eprintln!("Error fetching playback: {}", e);
        }
    }

    Ok(())
}
