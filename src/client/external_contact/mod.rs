use async_trait::async_trait;

// TODO: 其他暂未实现

/// 企业服务人员管理
mod enterprise_service;
pub use enterprise_service::*;

/// 客户管理
mod customer;
pub use customer::*;

/// 客户标签管理
mod customer_tag;
pub use customer_tag::*;

/// 在职继承
mod on_job_inherit;
pub use on_job_inherit::*;

/// 离职继承
mod leave_job_inherit;
pub use leave_job_inherit::*;

/// 客户群管理
mod group_chat;
pub use group_chat::*;

/// 联系我与客户入群方式
#[async_trait]
pub trait ContactAndInGroupManager {}

/// 客户朋友圈管理
#[async_trait]
pub trait ConstomerMomentManager {}

/// 消息推送
#[async_trait]
pub trait MessagePushManager {}

/// 统计管理
#[async_trait]
pub trait StatisticalManager {}

/// 变更回调
#[async_trait]
pub trait ChangedCallbackManager {}
