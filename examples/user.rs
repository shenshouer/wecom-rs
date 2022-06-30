use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::debug;
use wecom_rs::{Client, ContactManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("CORP_ID")?, env::var("CORP_SECRET")?);
    let user = client.user_get("shenshouer2955@ipalfish.com").await?;

    debug!("user_get: {}", serde_json::to_string(&user)?);

    Ok(())
}
