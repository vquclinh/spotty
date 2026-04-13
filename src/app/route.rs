use crate::app::home_state::HomeState;
use crate::app::search_state::SearchState;
use crate::app::splash_state::SplashState;

// Route is like state, to know where you are in app
#[derive(Clone)]
pub enum Route {
    Splash(SplashState),
    Home(HomeState),
    Search(SearchState),
    PlaylistDetail,
    Queue,
    Lyrics,
}

impl Route {
    // call in "on_tick" function in app, to manage Route update
    pub fn update(&mut self) -> Option<Route> {
        match self {
            Route::Home(_) => None,
            Route::Search(_) => None,
            Route::Splash(state) => state.update(),
            _ => None,
        }
    }
}