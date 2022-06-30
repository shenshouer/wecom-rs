use crate::{
    dto::*,
    error::Result,
    model::{Department, DepartmentList, DepartmentSimpleList, User, UserList},
};
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

/// 部门管理
#[async_trait]
pub trait DepartmentManager {
    /// 创建部门
    async fn department_create(&self, params: ParamsCreateDepartment) -> Result<()>;
    /// 更新部门
    async fn department_update(&self, params: ParamsUpdateDepartment) -> Result<()>;
    /// 删除部门
    async fn department_delete(&self, id: u64) -> Result<()>;
    /// 获取部门列表
    async fn department_list(&self, id: Option<u64>) -> Result<DepartmentList<Vec<Department>>>;
    /// 获取子部门ID列表
    async fn department_sample_list(&self, id: Option<u64>) -> Result<DepartmentSimpleList>;
    /// 获取单个部门详情
    async fn department_get(&self, id: u64) -> Result<DepartmentList<Department>>;
}

/// 标签管理
#[async_trait]
pub trait TagManager {}

/// 一部批量接口
#[async_trait]
pub trait AsyncBatchApi {}

/// 通讯录回调通知
#[async_trait]
pub trait ContactCallbackNotifyManager {}

/// 互联企业
#[async_trait]
pub trait LinkedCorpManager {}

/// 异步导出接口
#[async_trait]
pub trait AsyncExportApi {}

mod department;
mod user;
