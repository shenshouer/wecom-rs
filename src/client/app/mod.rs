use async_trait::async_trait;

/// 获取应用
#[async_trait]
pub trait AppGeter {}

/// 设置应用
#[async_trait]
pub trait AppSetter {}

/// 设置工作台自定义展示
#[async_trait]
pub trait WorkPanelManager {}
