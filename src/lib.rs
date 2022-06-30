/// 客户端
mod client;
pub use client::{Client, ContactManager};
/// 参数相关
mod dto;
/// 错误定义
mod error;
pub use error::Result;
/// 返回数据模型
mod model;
/// 辅助工具
mod utils;
