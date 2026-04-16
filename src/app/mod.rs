pub mod app;
pub mod route;

pub mod types;
pub mod state;

pub mod search_state;
pub mod home_state;
pub mod splash_state;
pub mod playlist_state;
pub mod playbar_state;

pub use types::ActiveBlock;
pub use search_state::SearchState;
pub use app::App;
pub use route::Route;