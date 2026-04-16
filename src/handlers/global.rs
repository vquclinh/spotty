use crate::app::{ActiveBlock, App, route::Route};
use crate::app::home_state::HomeState;
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

pub fn handle_global_events(key: KeyEvent, app: &mut App) -> bool {
    // quit
    if key.code == KeyCode::Char('q') 
        || key.code == KeyCode::Esc 
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)) 
    {
        app.should_quit = true;
        return true;
    }

    // home
    if key.code == KeyCode::Char('H') {
        if matches!(app.route, Route::Home(_)) {
            return true; 
        }

        app.set_current_route(Route::Home(HomeState::default())); 
        app.active_block = ActiveBlock::HomeBlock;

        return true;
    }

    // active block
    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        app.active_block = match app.active_block {
            ActiveBlock::LibraryMenu => ActiveBlock::PlaylistsMenu,
            ActiveBlock::PlaylistsMenu => match app.route {
                Route::PlaylistDetail(_) => ActiveBlock::PlaylistTracks,
                _ => ActiveBlock::HomeBlock,
            },
            ActiveBlock::HomeBlock | ActiveBlock::PlaylistTracks => ActiveBlock::QueueBlock,
            ActiveBlock::QueueBlock => ActiveBlock::LibraryMenu,
            _ => ActiveBlock::LibraryMenu,
        };
        return true; 
    }

    if key.code == KeyCode::Char('?') {
        app.show_help = true;
        return true;
    }


    false
}