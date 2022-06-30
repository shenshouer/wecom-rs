use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::debug;
use wecom_rs::{Client, DepartmentManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("CORP_ID")?, env::var("CORP_SECRET")?);

    let departments = client.department_list(None).await?;

    debug!("department_list: {}", serde_json::to_string(&departments)?);

    Ok(())
}
