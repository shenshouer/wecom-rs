use crate::Result;
use async_trait::async_trait;

/// 客户群管理
#[async_trait]
pub trait ConstomerGroupManager {
    /// 获取客户群列表
    /// https://developer.work.weixin.qq.com/document/path/92120
    async fn group_chat_list(&self, params: ParamsGroupChatListParams)
        -> Result<RespGroupChatList>;
    /// 获取客户群详情
    /// https://developer.work.weixin.qq.com/document/path/92122
    async fn group_chat_get(&self, params: ParamsGroupChatGetParams) -> Result<GroupChat>;
    /// 客户群opengid转换
    /// https://developer.work.weixin.qq.com/document/path/94822
    /// @params open_gid: 小程序在微信获取到的群ID，参见wx.getGroupEnterInfo
    async fn group_chat_opengid_to_chatid(&self, open_gid: &str) -> Result<ChatId>;
}

mod dto;
pub use dto::*;

mod model;
pub use model::*;
