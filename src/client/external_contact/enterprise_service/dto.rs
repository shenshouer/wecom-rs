use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsContactWayCreate {
    ///	联系方式类型,1-单人, 2-多人
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
    pub conclusions: Option<Conclusion>,
}

/// text、image、link和miniprogram四者不能同时为空；
/// text与另外三者可以同时发送，此时将会以两条消息的形式触达客户;
/// image、link和miniprogram只能有一个，如果三者同时填，则按image、link、miniprogram的优先顺序取参，也就是说，如果image与link同时传值，则只有image生效;
/// media_id可以通过素材管理接口获得;
/// 构造结束语使用image消息时，只能填写meida_id字段,获取含有image结构的联系我方式时，返回pic_url字段。
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Conclusion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<MiniProgram>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Text {
    /// 消息文本内容,最长为4000字节
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Image {
    /// 图片的media_id
    pub media_id: String,
    /// 图片的url
    pub pic_url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Link {
    /// 图文消息标题，最长为128字节
    pub title: String,
    /// 图文消息封面的url
    #[serde(rename = "picurl")]
    pub pic_url: String,
    /// 图文消息的描述，最长为512字节
    pub desc: String,
    /// 图文消息的链接
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MiniProgram {
    /// 小程序消息标题，最长为64字节
    pub title: String,
    /// 小程序消息封面的mediaid，封面图建议尺寸为520*416
    pub pic_media_id: String,
    /// 小程序appid，必须是关联到企业的小程序应用
    #[serde(rename = "appid")]
    pub app_id: String,
    /// 小程序page路径
    pub page: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsContactWayList {
    /// 联系我」创建起始时间戳, 默认为90天前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 「联系我」创建结束时间戳, 默认为当前时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 分页查询使用的游标，为上次请求返回的 next_cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// 每次查询的分页大小，默认为100条，最多支持1000条
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsContactWayUpdate {
    /// 企业联系方式的配置id
    pub config_id: String,
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
    pub conclusions: Option<Conclusion>,
}
