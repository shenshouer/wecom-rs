use super::*;
use crate::{
    client::{common::model::Response, Client, BASE_URL},
    error::Result,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl DepartmentManager for Client {
    /// https://developer.work.weixin.qq.com/document/path/90205
    async fn department_create(&self, params: ParamsCreateDepartment) -> Result<()> {
        let token = self.access_token().await?;

        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/department/create?access_token={token}"),
            Some(serde_json::to_value(&params)?),
        )
        .await?;

        Ok(())
    }

    /// https://developer.work.weixin.qq.com/document/path/90206
    async fn department_update(&self, params: ParamsUpdateDepartment) -> Result<()> {
        if params.is_empty() {
            return Err(crate::error::Error::EmptyFiledsUpdate);
        }

        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/department/update?access_token={token}"),
            Some(serde_json::to_value(&params)?),
        )
        .await?;

        Ok(())
    }

    /// https://developer.work.weixin.qq.com/document/path/90207
    async fn department_delete(&self, id: u64) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::GET,
            &format!("{BASE_URL}/department/delete?access_token={token}&id={id}"),
            None,
        )
        .await?;

        Ok(())
    }

    /// https://developer.work.weixin.qq.com/document/path/90208
    async fn department_list(&self, id: Option<u64>) -> Result<DepartmentList<Vec<Department>>> {
        let token = self.access_token().await?;
        let url = if let Some(id) = id {
            format!("{BASE_URL}/department/list?access_token={token}&id={id}")
        } else {
            format!("{BASE_URL}/department/list?access_token={token}")
        };

        let resp = self
            .request::<Response<DepartmentList<Vec<Department>>>>(Method::GET, &url, None)
            .await?;

        Ok(resp.data.unwrap())
    }

    /// https://developer.work.weixin.qq.com/document/path/95350
    async fn department_sample_list(&self, id: Option<u64>) -> Result<DepartmentSimpleList> {
        let token = self.access_token().await?;
        let url = if let Some(id) = id {
            format!("{BASE_URL}/department/simplelist?access_token={token}&id={id}")
        } else {
            format!("{BASE_URL}/department/simplelist?access_token={token}")
        };

        let resp = self
            .request::<Response<DepartmentSimpleList>>(Method::GET, &url, None)
            .await?;

        Ok(resp.data.unwrap())
    }

    /// https://developer.work.weixin.qq.com/document/path/95351
    async fn department_get(&self, id: u64) -> Result<DepartmentList<Department>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<DepartmentList<Department>>>(
                Method::GET,
                &format!("{BASE_URL}/department/get?access_token={token}&id={id}"),
                None,
            )
            .await?;

        Ok(resp.data.unwrap())
    }
}
