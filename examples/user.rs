use anyhow::Result;
use dotenv::dotenv;
use tracing::debug;
use wecom_rs::{Client, UserManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new_from_env()?;

    // client
    //     .user_create(wecom_rs::client::ParamsCreateUser::new_simple(
    //         "test20220630@ipalfish.com".to_string(),
    //         "测试邮箱".to_string(),
    //         "18612424367".to_string(),
    //         vec![1361979], // 运维研发
    //     ))
    //     .await?;

    // client
    //     .user_update(wecom_rs::client::ParamsUpdateUser {
    //         user_id: "test20220630@ipalfish.com".to_string(),
    //         name: Some("测试邮箱111".to_string()),
    //         ..Default::default()
    //     })
    //     .await?;

    // let user = client.user_get("test20220630@ipalfish.com").await?;
    // debug!("user_get: {}", serde_json::to_string(&user)?);

    // client.user_delete("test20220630@ipalfish.com").await?;

    // let uids_delete = vec!["test20220630@ipalfish.com"];
    // client.user_batch_delete(&uids_delete).await?;

    // let users = client.user_list(1361979).await?;
    // debug!("user_list: {}", serde_json::to_string(&users)?);

    let user_id = client.userid_get("18612424366").await?;
    debug!("user_id: {user_id}");

    Ok(())
}
