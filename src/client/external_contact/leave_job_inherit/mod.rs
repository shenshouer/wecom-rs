use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LeaveJobInheritManager {
    /// 获取待分配的离职成员列表
    /// https://developer.work.weixin.qq.com/document/path/92273
    async fn unassigned_list(&self, params: ParamsUnassignedList)
        -> Result<ResponseUnassignedList>;
    /// 分配离职成员的客户
    /// https://developer.work.weixin.qq.com/document/path/94100
    async fn leave_job_transfer_customer(
        &self,
        params: ParamsTransferCustomer,
    ) -> Result<Customer<Vec<TransferCustomer>>>;
    /// 查询客户接替状态
    /// https://developer.work.weixin.qq.com/document/path/94101
    /// @param:
    ///     handover_userid	原添加成员的userid
    ///     takeover_userid	接替成员的userid
    ///     cursor	分页查询的cursor，每个分页返回的数据不会超过1000条；不填或为空表示获取第一个分页
    async fn transfer_result_get(
        &self,
        handover_userid: &str,
        takeover_userid: &str,
        cursor: Option<&str>,
    ) -> Result<Customer<Vec<TransferResult>>>;
    /// 分配离职成员的客户群
    /// https://developer.work.weixin.qq.com/document/path/93242
    /// 注意：
    ///     群主离职了的客户群，才可继承
    ///     继承给的新群主，必须是配置了客户联系功能的成员
    ///     继承给的新群主，必须有设置实名
    ///     继承给的新群主，必须有激活企业微信
    ///     同一个人的群，限制每天最多分配300个给新群主
    /// @params
    ///     chat_id_list: 需要转群主的客户群ID列表。取值范围： 1 ~ 100
    ///     new_owner: 新群主ID
    async fn groupchat_transfer(
        &self,
        chat_id_list: &[&str],
        new_owner: &str,
    ) -> Result<Vec<FailedChat>>;
}

mod model;
pub use self::model::*;

mod dto;
pub use dto::*;

mod leave_job_inherit;
