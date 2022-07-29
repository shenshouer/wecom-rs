use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExternalProfile {
    /// 企业简称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_corp_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_channels: Option<WechatChannel>,
    pub external_attr: Vec<ExtAttribute>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WechatChannel {
    /// 视频号名称
    pub nickname: String,
    pub status: u8,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExtAttributes {
    pub attrs: Vec<ExtAttribute>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExtAttribute {
    /// 属性类型: 0-文本 1-网页 2-小程序
    #[serde(rename = "type")]
    pub kind: u8,
    /// 属性名称： 需要先确保在管理端有创建该属性，否则会忽略
    pub name: String,
    /// 文本类型的属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttribute>,
    /// 网页类型的属性，url和title字段要么同时为空表示清除该属性，要么同时不为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WebAttribute>,
    #[serde(rename = "miniprogram", skip_serializing_if = "Option::is_none")]
    pub mini_program: Option<MiniProgramAttribute>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TextAttribute {
    /// 文本属性内容，长度限制64个UTF8字符
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WebAttribute {
    /// 网页的url,必须包含http或者https头
    pub url: String,
    ///	网页的展示标题,长度限制12个UTF8字符
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MiniProgramAttribute {
    #[serde(rename = "appid")]
    pub app_id: String,
    #[serde(rename = "pagepath")]
    pub page_path: String,
    pub title: String,
}
