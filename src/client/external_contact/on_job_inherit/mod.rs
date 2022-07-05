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

    /// 查询客户接替状态
    /// https://developer.work.weixin.qq.com/document/path/94088
    async fn transfer_result(
        &self,
        params: ParamsTransferResult,
    ) -> Result<ResponeTransferResultList>;
    /// 分配在职成员的客户群
    /// 企业可通过此接口，将在职成员为群主的群，分配给另一个客服成员
    /// https://developer.work.weixin.qq.com/document/path/95703
    /// chat_id_list 需要转群主的客户群ID列表。取值范围： 1 ~ 100
    /// new_owner 新群主ID
    async fn transfer_onjob_groupchat(
        &self,
        chat_id_list: &[&str],
        new_owner: &str,
    ) -> Result<Vec<FailedChat>>;
}

mod dto;
mod on_job_inherit;
pub use dto::*;
mod model;
pub use model::*;
