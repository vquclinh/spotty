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

use std::io::Write;

pub async fn test_auth() -> Result<()> {
    let mut app = App::new().await;
    let Some(mut client) = app.client else {
        bail!("Client not initialized");
    };

    loop {
        println!("\n--- Spotify TUI Test Menu ---");
        println!("1. Get Profile");
        println!("2. Get Current Playback");
        println!("3. Get Playlists");
        println!("4. Get Queue");
        println!("5. Toggle Play/Pause");
        println!("6. Next Track");
        println!("7. Previous Track");
        println!("0. Exit");
        print!("Select an option: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => {
                let profile = client.get_user_profile().await?;
                println!("User: {} (ID: {})", profile.display_name, profile.id);
            }
            "2" => {
                match client.get_playback_state().await? {
                    Some(pb) => {
                        println!("Device: {} | Playing: {}", pb.device_name, pb.is_playing);
                        if let Some(item) = pb.item {
                            match item {
                                Playable::Track(t) => println!("Track: {} - {}", t.name, t.artists[0].name),
                                Playable::Episode(e) => println!("Episode: {} ({})", e.name, e.show_name),
                            }
                        }
                    }
                    None => println!("No active playback session found."),
                }
            }
            "3" => {
                let lists = client.get_user_playlists().await?;
                for (i, p) in lists.iter().enumerate() {
                    println!("{}. {}", i + 1, p.name);
                }
            }
            "4" => {
                let tracks = client.get_queue().await?;
                println!("Upcoming tracks: {}", tracks.len());
                for t in tracks.iter().take(5) {
                    println!("  - {}", t.name);
                }
            }
            "5" => {
                // Fetch state first to determine toggle action
                if let Some(pb) = client.get_playback_state().await? {
                    client.toggle_playback(pb.is_playing).await?;
                    println!("Playback toggled.");
                } else {
                    println!("Cannot toggle: No active session.");
                }
            }
            "6" => {
                client.next_track().await?;
                println!("Skipped to next.");
            }
            "7" => {
                client.prev_track().await?;
                println!("Went to previous.");
            }
            "0" => break,
            _ => println!("Invalid option."),
        }
    }

    Ok(())
}