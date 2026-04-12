use crate::network::client::WebApiClient;

pub struct App {
    pub should_quit: bool,
    pub client: Option<WebApiClient>
}

impl App {
    pub async fn new() -> Self {
        let client = WebApiClient::new(None).await.ok();
        Self { should_quit: false, client }
    }
}
