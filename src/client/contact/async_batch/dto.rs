use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsBatchUserSync {
    /// 上传的csv文件的media_id
    pub media_id: String,
    /// 是否邀请新建的成员使用企业微信
    /// （将通过微信服务通知或短信或邮件下发邀请，每天自动下发一次，最多持续3个工作日），
    /// 默认值为true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    /// 回调信息。
    /// 如填写该项则任务完成后，通过callback推送事件给企业。
    /// 具体请参考应用回调模式中的相应选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<Callback>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Callback {
    /// 企业应用接收企业微信推送请求的访问协议和地址，支持http或https协议
    pub url: String,
    /// 用于生成签名
    pub token: String,
    /// 用于消息体的加密，是AES密钥的Base64编码
    #[serde(rename = "encodingaeskey")]
    pub encoding_aes_key: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsBatchUserReplace {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsBatchPartyReplace {}
