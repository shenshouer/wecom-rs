use crate::{
    client::{Client, BASE_URL},
    dto::user::*,
    error::Result,
    model::{Response, User},
};
use async_trait::async_trait;
use reqwest::Method;

/// 成员管理
#[async_trait]
pub trait UserManager {
    /// 创建成员
    async fn user_create(&self, params: ParamsCreateUser) -> Result<()>;
    /// 读取成员
    async fn user_get(&self, user_id: &str) -> Result<User>;
    /// 删除成员
    async fn user_delete(&self, user_id: &str) -> Result<()>;
    /// 批量删除成员
    async fn user_batch_delete(&self, userids: &[&str]) -> Result<()>;
}

#[async_trait]
impl UserManager for Client {
    /// API参考：https://developer.work.weixin.qq.com/document/path/90195
    async fn user_create(&self, params: ParamsCreateUser) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/user/create?access_token={token}"),
            Some(serde_json::to_value(&params)?),
        )
        .await?;

        Ok(())
    }

    /// API参考：https://developer.work.weixin.qq.com/document/path/90196
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

    async fn user_delete(&self, user_id: &str) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::GET,
            &format!("{BASE_URL}/user/delete?access_token={token}&userid={user_id}"),
            None,
        )
        .await?;

        Ok(())
    }

    async fn user_batch_delete(&self, userids: &[&str]) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/user/batchdelete?access_token={token}"),
            Some(serde_json::json!({ "useridlist": userids })),
        )
        .await?;

        Ok(())
    }
}
