use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait OnJobInheritManager {
    /// 分配在职成员的客户
    /// https://developer.work.weixin.qq.com/document/path/92125
    async fn transfer_customer(
        &self,
        params: ParamsTransferCustomer,
    ) -> Result<Vec<ResponeTransferCustomer>>;
}

mod dto;
mod on_job_inherit;
pub use dto::*;
mod model;
pub use model::*;
