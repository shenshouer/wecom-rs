use anyhow::Result;
use dotenv::dotenv;
use tracing::debug;
use wecom_rs::{Client, DepartmentManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new_from_env()?;

    // let departments = client.department_list(None).await?;
    // let departments = client.department_sample_list(None).await?;
    let departments = client.department_get(1000000).await?;
    debug!("{}", serde_json::to_string(&departments)?);

    Ok(())
}
