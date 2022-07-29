use super::{
    Customer, FailedChat, LeaveJobInheritManager, ParamsTransferCustomer, ParamsUnassignedList,
    ResponseUnassignedList, TransferCustomer, TransferResult,
};
use crate::{
    client::{common::model::Response, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl LeaveJobInheritManager for Client {
    async fn unassigned_list(
        &self,
        params: ParamsUnassignedList,
    ) -> Result<ResponseUnassignedList> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<ResponseUnassignedList>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/get_unassigned_list?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn leave_job_transfer_customer(
        &self,
        params: ParamsTransferCustomer,
    ) -> Result<Customer<Vec<TransferCustomer>>> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<Customer<Vec<TransferCustomer>>>>(
                Method::POST,
                &format!(
                    "{BASE_URL}/externalcontact/resigned/transfer_customer?access_token={token}"
                ),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn transfer_result_get(
        &self,
        handover_userid: &str,
        takeover_userid: &str,
        cursor: Option<&str>,
    ) -> Result<Customer<Vec<TransferResult>>> {
        let params = if let Some(cursor) = cursor {
            serde_json::json!({"handover_userid": handover_userid, "takeover_userid": takeover_userid, "cursor": cursor})
        } else {
            serde_json::json!({"handover_userid": handover_userid, "takeover_userid": takeover_userid})
        };
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<Customer<Vec<TransferResult>>>>(
                Method::POST,
                &format!(
                    "{BASE_URL}/externalcontact/resigned/transfer_result?access_token={token}"
                ),
                Some(params),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn groupchat_transfer(
        &self,
        chat_id_list: &[&str],
        new_owner: &str,
    ) -> Result<Vec<FailedChat>> {
        let token = self.custom_contact_access_token().await?;
        let resp = self
            .request::<Response<Vec<FailedChat>>>(
                Method::POST,
                &format!("{BASE_URL}/externalcontact/groupchat/transfer?access_token={token}"),
                Some(serde_json::json!({"chat_id_list": chat_id_list, "new_owner": new_owner})),
            )
            .await?;

        Ok(resp.data.unwrap())
    }
}
