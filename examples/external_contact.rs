use anyhow::Result;
use dotenv::dotenv;
use tracing::info;
use wecom_rs::{Client, EnterpriseServiceManager, OnJobInheritManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;

    let user_ids = c.get_follow_user_list().await?;
    info!("user_id: {}", serde_json::to_string(&user_ids)?);

    // let params = wecom_rs::ParamsTransferResult::default(); // <- 修改参数
    // let transfer_result = c.transfer_result(params).await?;
    // info!(
    //     "transfer_result: {}",
    //     serde_json::to_string(&transfer_result)?
    // );
    Ok(())
}
