use crate::{
    client::{Client, BASE_URL},
    error::Result,
    model::{DepartmentList, Response},
};
use async_trait::async_trait;
use reqwest::Method;

/// 部门管理
#[async_trait]
pub trait DepartmentManager {
    async fn department_list(&self, id: Option<u64>) -> Result<DepartmentList>;
}

#[async_trait]
impl DepartmentManager for Client {
    async fn department_list(&self, id: Option<u64>) -> Result<DepartmentList> {
        let token = self.access_token().await?;
        let url = if let Some(id) = id {
            format!("{BASE_URL}/department/list?access_token={token}&id={id}")
        } else {
            format!("{BASE_URL}/department/list?access_token={token}")
        };

        let resp = self
            .request::<Response<DepartmentList>>(Method::GET, &url, None)
            .await?;

        Ok(resp.data)
    }
}
