use crate::{
    error::Result,
    model::{Department, DepartmentList, DepartmentSimpleList},
};
use async_trait::async_trait;

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

mod department;
mod dto;
pub use dto::*;
