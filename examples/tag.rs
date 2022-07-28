use anyhow::Result;
use dotenv::dotenv;
use tracing::info;
use wecom_rs::{Client, TagManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;

    let tags = c.tag_list().await?;
    info!("tags: {}", serde_json::to_string(&tags)?);

    Ok(())
}
