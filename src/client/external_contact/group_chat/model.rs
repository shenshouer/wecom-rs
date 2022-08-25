use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RespGroupChatList {
    /// 客户群列表
    pub group_chat_list: Vec<GroupChatStatus>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GroupChatStatus {
    /// 客户群ID
    pub chat_id: String,
    /// 客户群跟进状态。
    /// 0 - 跟进人正常
    /// 1 - 跟进人离职
    /// 2 - 离职继承中
    /// 3 - 离职继承完成
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GroupChat {
    /// 客户群ID
    pub chat_id: String,
    /// 群名
    pub name: String,
    /// 群主ID
    pub owner: String,
    /// 群的创建时间
    pub create_time: i64,
    /// 群公告
    pub notice: String,
    /// 群成员列表
    pub member_list: Vec<Member>,
    pub admin_list: Vec<Userid>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Member {
    /// 群成员id
    pub userid: String,
    /// 成员类型。
    /// 1 - 企业成员
    /// 2 - 外部联系人
    #[serde(rename = "type")]
    pub kind: i64,
    /// 外部联系人在微信开放平台的唯一身份标识（微信unionid），
    /// 通过此字段企业可将外部联系人与公众号/小程序用户关联起来。
    /// 仅当群成员类型是微信用户（包括企业成员未添加好友），
    /// 且企业绑定了微信开发者ID有此字段（查看绑定方法）。
    /// 第三方不可获取，上游企业不可获取下游企业客户的unionid字段
    pub unionid: Option<String>,
    /// 入群时间
    pub join_time: i64,
    /// 入群方式。
    /// 1 - 由群成员邀请入群（直接邀请入群）
    /// 2 - 由群成员邀请入群（通过邀请链接入群）
    /// 3 - 通过扫描群二维码入群
    pub join_scene: i32,
    /// 邀请者。目前仅当是由本企业内部成员邀请入群时会返回该值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitor: Option<Vec<Userid>>,
    /// 在群里的昵称
    pub group_nickname: String,
    /// 名字。仅当 need_name = 1 时返回
    /// 如果是微信用户，则返回其在微信中设置的名字
    /// 如果是企业微信联系人，则返回其设置对外展示的别名或实名
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Userid {
    pub userid: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ChatId {
    /// 客户群ID，可以用来调用获取客户群详情
    pub chat_id: String,
}
