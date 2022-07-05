use anyhow::Result;
use dotenv::dotenv;
use tracing::info;
use wecom_rs::{Client, EnterpriseServiceManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new_from_env()?;

    let user_ids = client.get_follow_user_list().await?;
    info!("user_id: {}", serde_json::to_string(&user_ids)?);

    Ok(())
}
