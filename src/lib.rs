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

pub fn run() -> anyhow::Result<()> {
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

    let mut app = App::new();
    let tick_rate = Duration::from_millis(50);

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let evt = event::read(tick_rate)?;

        handlers::handle(evt, &mut app);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
