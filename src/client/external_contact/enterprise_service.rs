use super::EnterpriseServiceManager;
use crate::{client::BASE_URL, model::Response, Client, Result};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[async_trait]
impl EnterpriseServiceManager for Client {
    /// https://developer.work.weixin.qq.com/document/path/92571
    async fn get_follow_user_list(&self) -> Result<Vec<String>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<FollowUserList>>(
                Method::GET,
                &format!("{BASE_URL}/externalcontact/get_follow_user_list?access_token={token}"),
                None,
            )
            .await?;

        Ok(resp.data.follow_user)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct FollowUserList {
    /// 配置了客户联系功能的成员userid列表
    follow_user: Vec<String>,
}
