use super::{
    model::{
        ResponseTagCreate, ResponseTagList, ResponseTagUser, ResponseTagUserGet, Tag, TagUser,
    },
    TagManager,
};
use crate::{client::BASE_URL, common::model::Response, error::Error, Client, Result};
use async_trait::async_trait;
use reqwest::Method;
use serde_json::{json, Map, Value};

#[async_trait]
impl TagManager for Client {
    async fn tag_create(&self, name: &str, id: Option<i64>) -> Result<i64> {
        let mut params: Map<String, Value> = Map::new();
        params.insert("tagname".to_string(), name.into());
        if let Some(id) = id {
            params.insert("tagid".to_string(), id.into());
        }

        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseTagCreate>>(
                Method::POST,
                &format!("{BASE_URL}/tag/update?access_token={token}"),
                Some(params.into()),
            )
            .await?;

        Ok(resp.data.unwrap().tag_id)
    }

    async fn tag_update(&self, id: i64, name: &str) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<Vec<TagUser>>>(
            Method::POST,
            &format!("{BASE_URL}/tag/update?access_token={token}"),
            Some(json!({"tagid": id,"tagname": name})),
        )
        .await?;

        Ok(())
    }

    async fn tag_delete(&self, id: i64) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::GET,
            &format!("{BASE_URL}/user/delete?access_token={token}&tagid={id}"),
            None,
        )
        .await?;
        Ok(())
    }

    async fn tag_user_get(&self, id: i64) -> Result<Vec<TagUser>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseTagUserGet>>(
                Method::GET,
                &format!(
                    "{BASE_URL}/user/getuserid?access_token={token}&tagid={}",
                    id
                ),
                None,
            )
            .await?;

        Ok(resp.data.unwrap().user_list)
    }

    async fn tag_user_add(
        &self,
        id: i64,
        user_list: Option<&[&str]>,
        party_list: Option<&[i64]>,
    ) -> Result<ResponseTagUser> {
        let params = if let (None, None) = (user_list, party_list) {
            return Err(Error::General(
                "user_list、party_list不能同时为空".to_string(),
            ));
        } else {
            let mut params: Map<String, Value> = Map::new();
            params.insert("tagid".to_string(), id.into());
            if let Some(ul) = user_list {
                params.insert("userlist".to_string(), ul.into());
            }
            if let Some(pl) = party_list {
                params.insert("partylist".to_string(), pl.into());
            }
            params
        };

        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseTagUser>>(
                Method::POST,
                &format!("{BASE_URL}/tag/addtagusers?access_token={token}"),
                Some(params.into()),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn tag_user_delete(
        &self,
        id: i64,
        user_list: Option<&[&str]>,
        party_list: Option<&[i64]>,
    ) -> Result<ResponseTagUser> {
        let params = if let (None, None) = (user_list, party_list) {
            return Err(Error::General(
                "user_list、party_list不能同时为空".to_string(),
            ));
        } else {
            let mut params: Map<String, Value> = Map::new();
            params.insert("tagid".to_string(), id.into());
            if let Some(ul) = user_list {
                params.insert("userlist".to_string(), ul.into());
            }
            if let Some(pl) = party_list {
                params.insert("partylist".to_string(), pl.into());
            }
            params
        };

        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseTagUser>>(
                Method::POST,
                &format!("{BASE_URL}/tag/deltagusers?access_token={token}"),
                Some(params.into()),
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    async fn tag_list(&self) -> Result<Vec<Tag>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseTagList>>(
                Method::POST,
                &format!("{BASE_URL}/tag/list?access_token={token}"),
                None,
            )
            .await?;

        Ok(resp.data.unwrap().tag_list)
    }
}
