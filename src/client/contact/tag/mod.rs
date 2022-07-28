use crate::Result;
use async_trait::async_trait;
use model::{ResponseTagUser, Tag, TagUser};

#[async_trait]
pub trait TagManager {
    /// 创建标签
    /// https://developer.work.weixin.qq.com/document/path/90210
    async fn tag_create(&self, name: &str, id: Option<i64>) -> Result<i64>;
    /// 更新标签名称
    /// https://developer.work.weixin.qq.com/document/path/90211
    async fn tag_update(&self, id: i64, name: &str) -> Result<()>;
    /// 删除标签
    /// https://developer.work.weixin.qq.com/document/path/90212
    async fn tag_delete(&self, id: i64) -> Result<()>;
    /// 获取标签成员
    /// https://developer.work.weixin.qq.com/document/path/90213
    async fn tag_user_get(&self, id: i64) -> Result<Vec<TagUser>>;
    /// 增加标签成员
    /// https://developer.work.weixin.qq.com/document/path/90214
    /// @params
    ///     user_list: 企业成员ID列表，注意：user_list、party)list不能同时为空，单次请求个数不超过1000
    ///     party_list: 企业部门ID列表，注意：user_list、party_list不能同时为空，单次请求个数不超过100
    async fn tag_user_add(
        &self,
        id: i64,
        user_list: Option<&[&str]>,
        party_list: Option<&[i64]>,
    ) -> Result<ResponseTagUser>;
    /// 删除标签成员
    /// https://developer.work.weixin.qq.com/document/path/90215
    /// @params
    ///     user_list: 企业成员ID列表，注意：user_list、party)list不能同时为空，单次请求个数不超过1000
    ///     party_list: 企业部门ID列表，注意：user_list、party_list不能同时为空，单次请求个数不超过100
    async fn tag_user_delete(
        &self,
        id: i64,
        user_list: Option<&[&str]>,
        party_list: Option<&[i64]>,
    ) -> Result<ResponseTagUser>;
    /// 获取标签列表
    /// https://developer.work.weixin.qq.com/document/path/90216
    async fn tag_list(&self) -> Result<Vec<Tag>>;
}

pub mod model;
mod tag;
