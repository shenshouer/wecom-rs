use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CustomerTagManager {
    /// 获取企业标签库
    /// https://developer.work.weixin.qq.com/document/path/92117
    /// @params
    ///     tag_ids: 要查询的标签id
    ///     group_ids: 要查询的标签组id，返回该标签组以及其下的所有标签信息
    /// 若tag_id和group_id均为空，则返回所有标签。
    /// 同时传递tag_id和group_id时，忽略tag_id，仅以group_id作为过滤条件
    async fn external_contact_corp_tag_list(
        &self,
        tag_ids: Option<&[&str]>,
        group_ids: Option<&[&str]>,
    ) -> Result<Vec<CorpTag>>;
    /// 获取指定郭泽下的企业客户标签
    /// https://developer.work.weixin.qq.com/document/path/94882
    /// @params
    ///     tag_ids: 要查询的标签id
    ///     group_ids: 要查询的标签组id，返回该标签组以及其下的所有标签信息
    /// 若tag_id和group_id均为空，则返回所有标签。
    /// 同时传递tag_id和group_id时，忽略tag_id，仅以group_id作为过滤条件
    async fn external_contact_strategy_tag_list(
        &self,
        tag_ids: Option<&[&str]>,
        group_ids: Option<&[&str]>,
    ) -> Result<Vec<CorpTag>>;
    /// 编辑客户企业标签
    /// https://developer.work.weixin.qq.com/document/path/92118
    async fn external_contact_mark_tag(&self, params: ParamsExternalContactMarkTag) -> Result<()>;
}

mod model;
pub use model::*;
mod dto;
pub use dto::*;
