use async_trait::async_trait;

// TODO: 其他暂未实现

/// 客户账号管理
#[async_trait]
pub trait CustomServiceContactManager {}

/// 接待人员管理
#[async_trait]
pub trait ReceptionContactManager {}

/// 会话分配与消息收发
#[async_trait]
pub trait SessionMessageManager {}

/// [升级服务]配置
#[async_trait]
pub trait UpgradeServiceManager {}

/// 其他基础信息获取
#[async_trait]
pub trait OtherBasicInfoManager {}

/// 统计管理
#[async_trait]
pub trait StatisticalManager {}
