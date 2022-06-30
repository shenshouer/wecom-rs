use crate::{
    client::{Client, ContactManager, BASE_URL},
    error::Result,
    model::{Response, User},
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl ContactManager for Client {
    async fn user_create(&self) -> Result<()> {
        todo!()
    }

    /// API参考https://developer.work.weixin.qq.com/document/path/90196
    async fn user_get(&self, user_id: &str) -> Result<User> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<User>>(
                Method::GET,
                &format!("{BASE_URL}/user/get?access_token={token}&userid={user_id}"),
                None,
            )
            .await?;

        Ok(resp.data)
    }
}
