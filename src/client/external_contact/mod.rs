use async_trait::async_trait;

// TODO: 其他暂未实现

/// 企业服务人员管理
#[async_trait]
pub trait EnterpriseServiceManager {}

/// 客户管理
#[async_trait]
pub trait ConstomerManager {}

/// 客户标签管理
#[async_trait]
pub trait ConstomerTagManager {}

/// 在职继承
#[async_trait]
pub trait OnJobInheritManager {}

/// 离职继承
#[async_trait]
pub trait LeaveJobInheritManager {}

/// 客户群管理
#[async_trait]
pub trait ConstomerGroupManager {}

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
