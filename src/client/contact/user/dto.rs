use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsCreateUser {
    /// 成员UserID。对应管理端的帐号，企业内必须唯一。长度为1~64个字节。只能由数字、字母和“_-@.”四种字符组成，且第一个字符必须是数字或字母。系统进行唯一性检查时会忽略大小写。
    #[serde(rename = "userid")]
    pub user_id: String,
    /// 成员名称。长度为1~64个utf8字符
    pub name: String,
    ///	成员别名。长度1~64个utf8字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    ///	手机号码。企业内必须唯一，mobile/email二者不能同时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    ///	成员所属部门id列表，不超过100个
    pub department: Vec<u64>,
    ///	部门内的排序值，默认为0，成员次序以创建时间从小到大排列。个数必须和参数department的个数一致，数值越大排序越前面。有效的值范围是[0, 2^32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<u64>>,
    ///	职务信息。长度为0~128个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    ///	性别。1表示男性，2表示女性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    ///	邮箱。长度6~64个字节，且为有效的email格式。企业内必须唯一，mobile/email二者不能同时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///	企业邮箱。仅对开通企业邮箱的企业有效。长度6~64个字节，且为有效的企业邮箱格式。企业内必须唯一。未填写则系统会为用户生成默认企业邮箱（由系统生成的邮箱可修改一次，2022年4月25日之后创建的成员需通过企业管理后台-协作-邮件-邮箱管理-成员邮箱修改）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_mail: Option<String>,
    ///	座机。32字节以内，由纯数字、“-”、“+”或“,”组成。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    ///	个数必须和参数department的个数一致，表示在所在的部门内是否为部门负责人。1表示为部门负责人，0表示非部门负责人。在审批(自建、第三方)等应用里可以用来标识上级审批人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leader_in_dept: Option<Vec<u64>>,
    ///	直属上级UserID，设置范围为企业内成员，可以设置最多5个上级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_leader: Option<String>,
    ///	成员头像的mediaid，通过素材管理接口上传图片获得的mediaid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_mediaid: Option<String>,
    ///	启用/禁用成员。1表示启用成员，0表示禁用成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<u8>,
    ///	自定义字段。自定义字段需要先在WEB管理端添加，见扩展属性添加方法，否则忽略未知属性的赋值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extattr: Option<ExtAttributes>,
    /// 是否邀请该成员使用企业微信（将通过微信服务通知或短信或邮件下发邀请，每天自动下发一次，最多持续3个工作日），默认值为true。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    ///	成员对外属性，字段详情见对外属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_profile: Option<ExternalProfile>,
    ///	对外职务，如果设置了该值，则以此作为对外展示的职务，否则以position来展示。长度12个汉字内
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_position: Option<String>,
    ///	视频号名字（设置后，成员将对外展示该视频号）。须从企业绑定到企业微信的视频号中选择，可在“我的企业”页中查看绑定的视频号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    ///	地址。长度最大128个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///	主部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_department: Option<u64>,
}

impl ParamsCreateUser {
    pub fn new_simple(user_id: String, name: String, mobile: String, department: Vec<u64>) -> Self {
        Self {
            user_id,
            name,
            mobile: Some(mobile),
            department,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsUpdateUser {
    /// 成员UserID。对应管理端的帐号，企业内必须唯一。长度为1~64个字节。只能由数字、字母和“_-@.”四种字符组成，且第一个字符必须是数字或字母。系统进行唯一性检查时会忽略大小写。
    #[serde(rename = "userid")]
    pub user_id: String,
    /// 如果userid由系统自动生成，则仅允许修改一次。新值可由new_userid字段指定
    #[serde(rename = "new_userid")]
    pub new_user_id: Option<String>,
    /// 成员名称。长度为1~64个utf8字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///	成员别名。长度1~64个utf8字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    ///	手机号码。企业内必须唯一。若成员已激活企业微信，则需成员自行修改（此情况下该参数被忽略，但不会报错）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    ///	成员所属部门id列表，不超过100个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<u64>>,
    ///	部门内的排序值，默认为0，成员次序以创建时间从小到大排列。个数必须和参数department的个数一致，数值越大排序越前面。有效的值范围是[0, 2^32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<u64>>,
    ///	职务信息。长度为0~128个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    ///	性别。1表示男性，2表示女性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    ///	邮箱。长度不超过64个字节，且为有效的email格式。企业内必须唯一。若是绑定了腾讯企业邮箱的企业微信，则需要在腾讯企业邮箱中修改邮箱（此情况下该参数被忽略，但不会报错）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///	邮箱。企业邮箱。仅对开通企业邮箱的企业有效。长度6~64个字节，且为有效的企业邮箱格式。企业内必须唯一。
    /// 仅2022年4月25日以前创建且企业邮箱为系统自动生成的成员可修改一次
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_mail: Option<String>,
    ///	座机。32字节以内，由纯数字、“-”、“+”或“,”组成。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    ///	个数必须和参数department的个数一致，表示在所在的部门内是否为部门负责人。1表示为部门负责人，0表示非部门负责人。在审批(自建、第三方)等应用里可以用来标识上级审批人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leader_in_dept: Option<Vec<u64>>,
    ///	直属上级UserID，设置范围为企业内成员，可以设置最多5个上级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_leader: Option<String>,
    ///	成员头像的mediaid，通过素材管理接口上传图片获得的mediaid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_mediaid: Option<String>,
    ///	启用/禁用成员。1表示启用成员，0表示禁用成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<u8>,
    ///	自定义字段。自定义字段需要先在WEB管理端添加，见扩展属性添加方法，否则忽略未知属性的赋值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extattr: Option<ExtAttributes>,
    /// 是否邀请该成员使用企业微信（将通过微信服务通知或短信或邮件下发邀请，每天自动下发一次，最多持续3个工作日），默认值为true。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    ///	成员对外属性，字段详情见对外属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_profile: Option<ExternalProfile>,
    ///	对外职务，如果设置了该值，则以此作为对外展示的职务，否则以position来展示。长度12个汉字内
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_position: Option<String>,
    ///	视频号名字（设置后，成员将对外展示该视频号）。须从企业绑定到企业微信的视频号中选择，可在“我的企业”页中查看绑定的视频号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    ///	地址。长度最大128个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///	主部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_department: Option<u64>,
}

impl ParamsUpdateUser {
    pub fn is_empty(&self) -> bool {
        if self.new_user_id.is_none()
            && self.name.is_none()
            && self.alias.is_none()
            && self.mobile.is_none()
            && self.department.is_none()
            && self.order.is_none()
            && self.position.is_none()
            && self.gender.is_none()
            && self.email.is_none()
            && self.biz_mail.is_none()
            && self.telephone.is_none()
            && self.is_leader_in_dept.is_none()
            && self.direct_leader.is_none()
            && self.avatar_mediaid.is_none()
            && self.enable.is_none()
            && self.extattr.is_none()
            && self.external_profile.is_none()
            && self.external_position.is_none()
            && self.nickname.is_none()
            && self.address.is_none()
            && self.main_department.is_none()
        {
            return true;
        }
        false
    }
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
    #[serde(rename = "miniprogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub nickname: String,
}
