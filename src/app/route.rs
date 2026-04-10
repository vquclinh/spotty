use crate::app::home::HomeState;
use crate::app::search_state::SearchState;

#[derive(Clone)]
pub enum Route {
    Home(HomeState),
    Search(SearchState),
    PlaylistDetail,
    Queue,
    Lyrics,
}

impl Route {
    pub fn update(&mut self) -> Option<Route> {
        match self {
            Route::Home(_) => None,
            Route::Search(_) => None, 
            _ => None,
        }
    }
}