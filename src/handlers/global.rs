use crate::app::{ActiveBlock, App, route::Route};
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

    false
}