use super::{
    ContactWay, EnterpriseServiceManager, ParamsContactWayCreate, ParamsContactWayList,
    ParamsContactWayUpdate, RespContactWayCreate,
};
use crate::{client::BASE_URL, model::Response, Client, Result};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[async_trait]
impl EnterpriseServiceManager for Client {
    async fn get_follow_user_list(&self) -> Result<Vec<String>> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<FollowUserList>>(
                Method::GET,
                &format!("{BASE_URL}/externalcontact/get_follow_user_list?access_token={token}"),
                None,
            )
            .await?;

        Ok(resp.data.unwrap().follow_user)
    }

    async fn contact_way_create(
        &self,
        params: ParamsContactWayCreate,
    ) -> Result<RespContactWayCreate> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<RespContactWayCreate>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/add_contact_way?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn contact_way_get(&self, config_id: &str) -> Result<ContactWay> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<ContactWay>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/get_contact_way?access_token={token}"),
                Some(serde_json::json!({ "config_id": config_id })),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn contact_way_list(
        &self,
        params: Option<ParamsContactWayList>,
    ) -> Result<Vec<ContactWay>> {
        let token = self.custom_contact_access_token().await?;

        let params = if let Some(params) = params {
            Some(serde_json::to_value(&params)?)
        } else {
            None
        };
        let resp = self
            .request::<Response<Vec<ContactWay>>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/list_contact_way?access_token={token}"),
                params,
            )
            .await?;

        Ok(resp.data.unwrap_or(vec![]))
    }

    async fn contact_way_update(&self, params: ParamsContactWayUpdate) -> Result<()> {
        let token = self.custom_contact_access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/externalcontact/update_contact_way?access_token={token}"),
            Some(serde_json::to_value(&params)?),
        )
        .await?;

        Ok(())
    }

    async fn contact_way_delete(&self, config_id: &str) -> Result<()> {
        let token = self.custom_contact_access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}//externalcontact/del_contact_way?access_token={token}"),
            Some(serde_json::json!({ "config_id": config_id })),
        )
        .await?;

        Ok(())
    }

    async fn temp_chat_close(&self, userid: &str, external_userid: &str) -> Result<()> {
        let token = self.custom_contact_access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/externalcontact/close_temp_chat?access_token={token}"),
            Some(serde_json::json!({"userid": userid, "external_userid": external_userid})),
        )
        .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct FollowUserList {
    /// 配置了客户联系功能的成员userid列表
    follow_user: Vec<String>,
}
