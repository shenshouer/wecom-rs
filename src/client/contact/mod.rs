use async_trait::async_trait;

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
pub use department::*;
mod user;
pub use user::*;
/// 通讯录回调通知
mod callback_notify;
