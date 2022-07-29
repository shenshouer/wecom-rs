use super::*;
use crate::{
    client::{common::model::Response, BASE_URL},
    Client,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl OnJobInheritManager for Client {
    async fn transfer_customer(
        &self,
        params: ParamsTransferCustomer,
    ) -> Result<Vec<ResponeTransferCustomer>> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<Vec<ResponeTransferCustomer>>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/transfer_customer?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn transfer_result(
        &self,
        params: ParamsTransferResult,
    ) -> Result<ResponeTransferResultList> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<ResponeTransferResultList>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/transfer_result?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn transfer_onjob_groupchat(
        &self,
        chat_id_list: &[&str],
        new_owner: &str,
    ) -> Result<Vec<FailedChat>> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<ResponeTransferOnjobGroupChatList>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/transfer_result?access_token={token}"),
                Some(serde_json::json!({
                    "chat_id_list": chat_id_list,
                    "new_owner": new_owner,
                })),
            )
            .await?;

        Ok(resp.data.unwrap().failed_chat_list)
    }
}
