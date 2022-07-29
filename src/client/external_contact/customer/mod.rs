use crate::Result;
use async_trait::async_trait;

/// 客户管理
#[async_trait]
pub trait CustomerManager {
    /// 获取客户列表
    /// https://developer.work.weixin.qq.com/document/path/92113
    /// @return
    ///     external_userid	外部联系人的userid列表
    async fn external_contact_list(&self, userid: &str) -> Result<Vec<String>>;
    /// 获取客户详情
    /// https://developer.work.weixin.qq.com/document/path/92114
    async fn external_contact_get(
        &self,
        external_userid: &str,
        cursor: Option<&str>,
    ) -> Result<ExternalContact>;
    /// 批量获取客户详情
    /// https://developer.work.weixin.qq.com/document/path/92994
    async fn external_contact_batch_get_by_user(
        &self,
        params: ParamsExternalContactBatchGetByUser,
    ) -> Result<Vec<ExternalContact>>;
    /// 修改客户备注信息
    /// https://developer.work.weixin.qq.com/document/path/92115
    async fn external_contact_remark(&self, params: ParamsExternalContactRemark) -> Result<()>;
    /// 客户联系规则组管理
    /// https://developer.work.weixin.qq.com/document/path/94883
    async fn external_contact_customer_strategy(
        &self,
        cursor: Option<String>,
        limit: Option<i64>,
    ) -> Result<Vec<StrategyId>>;
}

mod model;
pub use model::*;

mod dto;
pub use dto::*;
