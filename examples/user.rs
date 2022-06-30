use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::debug;
use wecom_rs::{Client, UserManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("CORP_ID")?, env::var("CORP_SECRET")?);
    // create user
    // client
    //     .user_create(wecom_rs::client::ParamsCreateUser::new_simple(
    //         "test20220630@ipalfish.com".to_string(),
    //         "测试邮箱".to_string(),
    //         "18612424367".to_string(),
    //         vec![1361979], // 运维研发
    //     ))
    // .await?;

    // get user
    // let user = client.user_get("test20220630@ipalfish.com").await?;
    // debug!("user_get: {}", serde_json::to_string(&user)?);

    // client.user_delete("test20220630@ipalfish.com").await?;

    let uids_delete = vec!["test20220630@ipalfish.com"];
    client.user_batch_delete(&uids_delete).await?;

    Ok(())
}
