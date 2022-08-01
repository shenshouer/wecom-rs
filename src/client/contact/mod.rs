use async_trait::async_trait;

/// 异步批量接口
mod async_batch;
pub use async_batch::*;

/// 通讯录回调通知
#[async_trait]
pub trait ContactCallbackNotifyManager {}

/// 互联企业
mod linked_corp;
pub use linked_corp::*;

/// 异步导出接口
#[async_trait]
pub trait AsyncExportApi {}

mod department;
pub use department::*;
mod user;
pub use user::*;
/// 通讯录回调通知
// TODO: 暂未实现，API文档变化频繁
mod callback_notify;
/// 标签管理
mod tag;
pub use tag::*;
