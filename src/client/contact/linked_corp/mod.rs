use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LinkedCorpManager {
    /// 获取应用的可见范围
    /// https://developer.work.weixin.qq.com/document/path/93172
    async fn linked_corp_agent_get_perm_list(&self) -> Result<ResponseAgentPermList>;
    /// 获取互联企业成员详情信息
    /// https://developer.work.weixin.qq.com/document/path/93171
    async fn linked_corp_user_get(&self, user_id: &str) -> Result<UserInfo>;
    /// 获取互联企业部门成员
    /// https://developer.work.weixin.qq.com/document/path/93168
    async fn linked_corp_user_simple_list(
        &self,
        department_id: &str,
    ) -> Result<Vec<UserSimpleInfo>>;
    /// 获取互联企业部门成员详情
    /// https://developer.work.weixin.qq.com/document/path/93169
    async fn linked_corp_user_list(&self, department_id: &str) -> Result<Vec<UserInfo>>;
    /// 获取互联企业部门列表
    /// https://developer.work.weixin.qq.com/document/path/93170
    async fn linked_corp_department_list(&self, department_id: &str)
        -> Result<Vec<DepartmentInfo>>;
}

mod model;
pub use model::*;
