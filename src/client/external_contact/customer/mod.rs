use async_trait::async_trait;
use crate::Result;

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
    async fn external_contact_get(&self, external_userid: &str, cursor: Option<&str>) -> Result<Vec<String>>;
}

