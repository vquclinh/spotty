use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    console_spotify::test_auth().await?;
    Ok(())
}
