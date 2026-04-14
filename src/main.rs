#[tokio::main]
async fn main() -> anyhow::Result<()> {
    spotty::run().await
}
