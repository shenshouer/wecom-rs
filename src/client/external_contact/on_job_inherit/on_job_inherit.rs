use super::*;
use crate::{client::BASE_URL, model::Response, Client};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl OnJobInheritManager for Client {
    async fn transfer_customer(
        &self,
        params: ParamsTransferCustomer,
    ) -> Result<Vec<ResponeTransferCustomer>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<Vec<ResponeTransferCustomer>>>(
                Method::GET,
                &format!(
                    "{BASE_URL}/cgi-bin/externalcontact/transfer_customer?access_token={token}"
                ),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.data.unwrap())
    }
}
