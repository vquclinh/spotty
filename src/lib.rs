pub mod app;
pub mod event;
pub mod handlers;
pub mod network;
pub mod ui;
pub mod util;

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
    let mut last_tick = Instant::now();

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if poll(timeout)? && let Event::Key(key) = read()? {
            handle_key_events(key, &mut app);
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}