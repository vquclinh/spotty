use crate::app::{ActiveBlock, App, route::Route};
use crate::app::home_state::HomeState;
use crate::app::search_state::{SearchState, SearchHoveredPane};
use crate::app::queue_state::QueueState;
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use crate::handlers::playbar;

pub fn handle_global_events(key: KeyEvent, app: &mut App) -> bool {
    // quit
    if key.code == KeyCode::Char('q') 
        || key.code == KeyCode::Esc 
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)) 
    {
        app.should_quit = true;
        return true;
    }

    // Playbar events
    if playbar::handle_playbar_events(key, app) {
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

    // search
    if key.code == KeyCode::Char('s') {
        if !matches!(app.route, Route::Search(_)) {
            app.set_current_route(Route::Search(SearchState::default()));
        }

        app.active_block = ActiveBlock::SearchInput;
        
        return true;
    }

    // queue
    if key.code == KeyCode::Char('Q') {
        if matches!(app.route, Route::Queue(_)) {
            app.active_block = ActiveBlock::QueueBlock;
            return true;
        }

        app.set_current_route(Route::Queue(QueueState::default()));
        app.active_block = ActiveBlock::QueueBlock;
        
        return true;
    }

    // active block
    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        if app.active_block == ActiveBlock::SearchResults {
            return false;
        }
        
        app.active_block = match app.active_block {
            ActiveBlock::LibraryMenu => ActiveBlock::PlaylistsMenu,
            ActiveBlock::PlaylistsMenu => match app.route {
                Route::PlaylistDetail(_) => ActiveBlock::PlaylistTracks,
                Route::Search(_) => ActiveBlock::SearchInput,
                Route::Queue(_) => ActiveBlock::QueueBlock,
                Route::AlbumDetail(_) => ActiveBlock::AlbumBlock,
                _ => ActiveBlock::HomeBlock,
            },

            ActiveBlock::SearchInput => {
                if let Route::Search(ref mut search_state) = app.route {
                    search_state.hovered_pane = SearchHoveredPane::Tracks;
                }
                ActiveBlock::SearchResults
            },

            ActiveBlock::HomeBlock 
            | ActiveBlock::PlaylistTracks 
            | ActiveBlock::QueueBlock 
            | ActiveBlock::LyricsText
            | ActiveBlock::AlbumBlock => {
                ActiveBlock::Playbar
            },

            ActiveBlock::Playbar => {
                if matches!(app.route, Route::Search(_)) {
                    ActiveBlock::SearchInput 
                } else {
                    ActiveBlock::LibraryMenu
                }
            },

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
