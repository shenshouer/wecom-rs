use crate::client::common::model::ExternalProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExternalContact {
    /// 外部联系人的userid
    pub external_userid: String,
    ///	外部联系人的名称
    pub name: String,
    ///	外部联系人头像，代开发自建应用需要管理员授权才可以获取，第三方不可获取，上游企业不可获取下游企业客户该字段
    pub avatar: String,
    /// 外部联系人的类型，1表示该外部联系人是微信用户，2表示该外部联系人是企业微信用户
    #[serde(rename = "type")]
    pub kind: i32,
    /// 外部联系人性别 0-未知 1-男性 2-女性。第三方不可获取，上游企业不可获取下游企业客户该字段，返回值为0，表示未定义
    pub gender: String,
    /// 外部联系人在微信开放平台的唯一身份标识（微信unionid），
    /// 通过此字段企业可将外部联系人与公众号/小程序用户关联起来。
    /// 仅当联系人类型是微信用户，且企业绑定了微信开发者ID有此字段。查看绑定方法。
    /// 第三方不可获取，上游企业不可获取下游企业客户的unionid字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    /// 外部联系人的职位，如果外部企业或用户选择隐藏职位，则不返回，仅当联系人类型是企业微信用户时有此字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 外部联系人所在企业的简称，仅当联系人类型是企业微信用户时有此字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_name: Option<String>,
    /// 外部联系人所在企业的主体名称，仅当联系人类型是企业微信用户时有此字段。
    /// 仅企业自建应用可获取；
    /// 第三方应用、代开发应用、上下游应用不可获取，返回内容为企业名称，即corp_name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_full_name: Option<String>,
    ///	外部联系人的自定义展示信息，可以有多个字段和多种类型，包括文本，网页和小程序，
    /// 仅当联系人类型是企业微信用户时有此字段，字段详情见对外属性；
    pub external_profile: Option<ExternalProfile>,
    /// 跟进人
    pub follow_user: Option<Vec<FollowUser>>,
    /// 分页的cursor，当跟进人多于500人时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FollowUser {
    /// 添加了此外部联系人的企业成员userid
    pub userid: String,
    /// 该成员对此外部联系人的备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 该成员对此外部联系人的描述
    pub description: Option<String>,

    /// 该成员添加此外部联系人的时间
    pub createtime: i64,
    pub tags: Vec<Tag>,
    /// 该成员对此微信客户备注的企业名称（仅微信客户有该字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark_corp_name: Option<String>,
    /// 该成员对此客户备注的手机号码，
    /// 代开发自建应用需要管理员授权才可以获取，第三方不可获取，
    /// 上游企业不可获取下游企业客户该字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark_mobiles: Option<Vec<String>>,
    /// 该成员添加此客户的来源，具体含义详见来源定义
    pub add_way: i32,
    /// 该成员添加此客户的来源add_way为10时，对应的视频号信息
    pub wechat_channels: Option<WechatChannel>,
    /// 发起添加的userid，
    /// 如果成员主动添加，为成员的userid；
    /// 如果是客户主动添加，则为客户的外部联系人userid；
    /// 如果是内部成员共享/管理员分配，则为对应的成员/管理员userid
    pub oper_userid: String,
    /// 企业自定义的state参数，用于区分客户具体是通过哪个「联系我」添加，由企业通过创建「联系我」方式指定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WechatChannel {
    /// 视频号名称
    pub nickname: String,
    /// 视频号添加场景，
    /// 0-未知
    /// 1-视频号主页
    /// 2-视频号直播间（微信版本要求：iOS ≥ 8.0.20，Android ≥ 8.0.21，且添加时间不早于2022年4月21日。否则添加场景值为0）
    pub source: u8,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Tag {
    /// 该成员添加此外部联系人所打标签的分组名称（标签功能需要企业微信升级到2.7.5及以上版本）
    pub group_name: String,
    /// 该成员添加此外部联系人所打标签名称
    pub tag_name: String,
    /// 该成员添加此外部联系人所打标签类型, 1-企业设置，2-用户自定义，3-规则组标签（仅系统应用返回）
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    /// 该成员添加此外部联系人所打企业标签的id，用户自定义类型标签（type=2）不返回
    pub tag_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseExternalContactCustomerStrategy {
    /// 规则组
    pub strategy: Vec<StrategyId>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct StrategyId {
    /// 规则组id
    pub strategy_id: i64,
}
