use crate::app::home_state::HomeState;
use crate::app::queue_state::QueueState;
use crate::app::search_state::SearchState;
use crate::app::splash_state::SplashState;
use crate::app::playlist_state::PlaylistState;

// Route is like state, to know where you are in app
#[derive(Clone)]
pub enum Route {
    Splash(SplashState),
    Home(HomeState),
    Search(SearchState),
    PlaylistDetail(PlaylistState),
    Queue(QueueState),
    Lyrics,
}

impl Route {
    // call in "on_tick" function in app, to manage Route update
    pub fn update(&mut self) -> Option<Route> {
        match self {
            Route::Home(_) => None,
            Route::Search(_) => None,
            Route::Splash(state) => state.update(),
            Route::PlaylistDetail(_) => None,
            Route::Queue(_) => None,
            _ => None,
        }
    }
}