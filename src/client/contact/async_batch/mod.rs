use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AsyncBatch {
    /// 增量更新成员
    /// https://developer.work.weixin.qq.com/document/path/90980
    /// @return string job_id
    async fn batch_user_sync(&self, params: ParamsBatchUserSync) -> Result<String>;
    /// 全量覆盖成员
    /// https://developer.work.weixin.qq.com/document/path/90981
    /// @return string job_id
    async fn batch_user_replace(&self, params: ParamsBatchUserReplace) -> Result<String>;
    /// 全量覆盖部门
    /// https://developer.work.weixin.qq.com/document/path/90982
    /// @return string job_id
    async fn batch_party_replace(&self, params: ParamsBatchPartyReplace) -> Result<String>;
    /// 获取异步任务结果
    /// https://developer.work.weixin.qq.com/document/path/90983
    async fn batch_result_get(&self, job_id: &str) -> Result<JobStatus>;
}

mod dto;
pub use dto::*;

mod model;
pub use model::*;
