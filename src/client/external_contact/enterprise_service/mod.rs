use crate::Result;
use async_trait::async_trait;

/// 企业服务人员管理
#[async_trait]
pub trait EnterpriseServiceManager {
    /// 获取配置了客户联系功能的成员列表
    async fn get_follow_user_list(&self) -> Result<Vec<String>>;
}

mod enterprise_service;
