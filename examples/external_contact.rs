use anyhow::Result;
use dotenv::dotenv;
use tracing::info;
use wecom_rs::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;

    // use wecom_rs::EnterpriseServiceManager;
    // let user_ids = c.get_follow_user_list().await?;
    // info!("user_id: {}", serde_json::to_string(&user_ids)?);

    // use wecom_rs::OnJobInheritManager;
    // let params = wecom_rs::ParamsTransferResult::default(); // <- 修改参数
    // let transfer_result = c.transfer_result(params).await?;
    // info!(
    //     "transfer_result: {}",
    //     serde_json::to_string(&transfer_result)?
    // );

    use wecom_rs::EnterpriseServiceManager;
    let contact_way_list = c.contact_way_list(None).await?;
    info!(
        "contact_way_list: {}",
        serde_json::to_string(&contact_way_list)?
    );

    Ok(())
}
