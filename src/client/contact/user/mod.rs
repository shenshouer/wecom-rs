use crate::error::Result;
use async_trait::async_trait;

/// 成员管理
#[async_trait]
pub trait UserManager {
    /// 创建成员
    async fn user_create(&self, params: ParamsCreateUser) -> Result<()>;
    /// 读取成员
    async fn user_get(&self, user_id: &str) -> Result<User>;
    /// 更新成员
    async fn user_update(&self, params: ParamsUpdateUser) -> Result<()>;
    /// 删除成员
    async fn user_delete(&self, user_id: &str) -> Result<()>;
    /// 批量删除成员
    async fn user_batch_delete(&self, userids: &[&str]) -> Result<()>;
    /// 获取部门成员
    async fn user_list(&self, department_id: u64) -> Result<UserList>;
    /// 通过手机号获取uid
    async fn userid_get(&self, mobile: &str) -> Result<String>;
}

mod dto;
pub use dto::*;

mod model;
pub use model::*;

mod user;
