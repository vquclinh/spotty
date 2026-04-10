use super::route::Route;
use super::home_state::HomeState;
#[derive(Clone)]
pub struct SplashState {
    pub ticks: u32,
}

impl SplashState {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }

    pub fn update(&mut self) -> Option<Route> {
        self.ticks += 1;

        if self.ticks > 40 {
            return Some(Route::Home(HomeState::new())); 
        }
        
        None
    }
}