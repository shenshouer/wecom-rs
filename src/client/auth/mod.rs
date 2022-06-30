use async_trait::async_trait;

/// 网页授权登录
#[async_trait]
pub trait WebArantManager {}

/// 扫码授权登录
#[async_trait]
pub trait ScanQrcodeManager {}
