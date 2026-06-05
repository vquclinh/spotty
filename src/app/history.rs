use crate::app::{Route, ActiveBlock};

pub struct AppState {
    pub route: Route,
    pub active_block: ActiveBlock,
}

impl AppState {
    pub fn new(route: Route, active_block: ActiveBlock) -> Self {
        Self { route, active_block }
    }
}

pub struct StateHistory {
    current: AppState,
    previous: Option<AppState>,
}

impl StateHistory {
    pub fn new(state: AppState) -> Self {
        Self {
            current: state,
            previous: None,
        }
    }
    
    pub fn set_state(&mut self, route: Route, active_block: Option<ActiveBlock>) {
        let next_active_block = active_block.unwrap_or(self.current.active_block);
        
        let next_state = AppState {
            route,
            active_block: next_active_block,
        };
        
        if Self::should_replace(&next_state.route) {
            self.current = next_state;
            self.previous = None;
        } else {
            if Self::should_replace(&self.current.route) {
                self.previous = Some(std::mem::replace(&mut self.current, next_state));
            } else {
                self.current = next_state;
            }
        }
    }

    pub fn pop_state(&mut self) {
        if let Some(state) = self.previous.take() {
            self.current = state;
        }
    }

    pub fn current(&self) -> &AppState {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut AppState {
        &mut self.current
    }

    pub fn previous(&self) -> Option<&AppState> {
        self.previous.as_ref()
    }

    fn should_replace(route: &Route) -> bool {
        !matches!(route, Route::Queue(_) | Route::Lyrics(_) | Route::Search(_))
    }
}