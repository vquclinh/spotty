#[tokio::main]
async fn main() -> anyhow::Result<()> {
    console_spotify::run().await
}
