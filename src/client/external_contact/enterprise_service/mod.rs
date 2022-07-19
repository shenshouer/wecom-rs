use crate::Result;
use async_trait::async_trait;

/// 企业服务人员管理
#[async_trait]
pub trait EnterpriseServiceManager {
    /// 获取配置了客户联系功能的成员列表
    /// https://developer.work.weixin.qq.com/document/path/92571
    async fn get_follow_user_list(&self) -> Result<Vec<String>>;

    // 客户联系[联系我]管理
    // https://developer.work.weixin.qq.com/document/path/92577

    /// 配置客户联系「联系我」方式
    async fn contact_way_create(
        &self,
        params: ParamsContactWayCreate,
    ) -> Result<RespContactWayCreate>;

    /// 获取企业已配置的「联系我」方式
    async fn contact_way_get(&self, config_id: &str) -> Result<ContactWay>;

    /// 获取企业已配置的「联系我」列表
    async fn contact_way_list(
        &self,
        params: Option<ParamsContactWayList>,
    ) -> Result<Vec<ContactWay>>;

    /// 更新企业已配置的「联系我」方式
    async fn contact_way_update(&self, params: ParamsContactWayUpdate) -> Result<()>;

    /// 删除企业已配置的「联系我」方式
    async fn contact_way_delete(&self, config_id: &str) -> Result<()>;
    /// 结束临时会话
    async fn temp_chat_close(&self, userid: &str, external_userid: &str) -> Result<()>;
}

mod dto;
pub use dto::*;

mod model;
pub use model::*;

mod enterprise_service;
