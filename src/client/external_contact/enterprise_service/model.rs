use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RespContactWayCreate {
    pub config_id: String,
    pub qr_code: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ContactWay {
    /// 联系方式的配置id
    pub config_id: String,
    /// 联系方式类型,1-单人, 2-多人
    #[serde(rename = "type")]
    pub kind: i32,
    /// 场景，1-在小程序中联系，2-通过二维码联系
    pub scene: i32,
    /// 在小程序中联系时使用的控件样式，详见附表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<i32>,
    /// 联系方式的备注信息，用于助记，不超过30个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 外部客户添加时是否无需验证，默认为true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_verify: Option<bool>,
    /// 企业自定义的state参数，用于区分不同的添加渠道，在调用“获取外部联系人详情”时会返回该参数值，不超过30个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 使用该联系方式的用户userID列表，在type为1时为必填，且只能有一个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    /// 使用该联系方式的部门id列表，只在type为2时有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Vec<i32>>,
    /// 是否临时会话模式，true表示使用临时会话模式，默认为false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_temp: Option<bool>,
    /// 临时会话二维码有效期，以秒为单位。该参数仅在is_temp为true时有效，默认7天，最多为14天
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// 临时会话有效期，以秒为单位。该参数仅在is_temp为true时有效，默认为添加好友后24小时，最多为14天
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_expires_in: Option<i64>,
    /// 可进行临时会话的客户unionid，该参数仅在is_temp为true时有效，如不指定则不进行限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    /// 结束语，会话结束时自动发送给客户，可参考“结束语定义”，仅在is_temp为true时有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusions: Option<super::Conclusion>,
}
