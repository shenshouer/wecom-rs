use super::*;
use crate::{
    client::{Client, BASE_URL},
    error::Result,
    model::{Response, User, UserList},
};
use async_trait::async_trait;
use reqwest::Method;

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
    async fn user_update(&self, params: ParamsUpdateUser) -> Result<()> {
        if params.is_empty() {
            return Err(crate::error::Error::EmptyFiledsUpdate);
        }
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/user/update?access_token={token}"),
            Some(serde_json::to_value(params)?),
        )
        .await?;

        Ok(())
    }

    /// https://developer.work.weixin.qq.com/document/path/90198
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

    /// https://developer.work.weixin.qq.com/document/path/90199
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

    /// https://developer.work.weixin.qq.com/document/path/90201
    async fn user_list(&self, department_id: u64) -> Result<UserList> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<UserList>>(
                Method::GET,
                &format!("{BASE_URL}/user/list?access_token={token}&department_id={department_id}"),
                None,
            )
            .await?;

        Ok(resp.data)
    }

    /// https://developer.work.weixin.qq.com/document/path/95402
    async fn userid_get(&self, mobile: &str) -> Result<String> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<GetUserIdResponse>>(
                Method::POST,
                &format!("{BASE_URL}/user/getuserid?access_token={token}"),
                Some(serde_json::json!({ "mobile": mobile })),
            )
            .await?;

        Ok(resp.data.userid)
    }
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
struct GetUserIdResponse {
    userid: String,
}
