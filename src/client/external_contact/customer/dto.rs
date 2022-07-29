use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsExternalContactBatchGetByUser {
    /// 企业成员的userid列表，字符串类型，最多支持100个
    pub userid_list: Vec<String>,
    /// 用于分页查询的游标，字符串类型，由上一次调用返回，首次调用可不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// 返回的最大记录数，整型，最大值100，默认值50，超过最大值时取最大值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// remark_company只在此外部联系人为微信用户时有效。
/// remark，description，remark_company，remark_mobiles和remark_pic_mediaid不可同时为空。
/// 如果填写了remark_mobiles，将会覆盖旧的备注手机号。
/// 如果要清除所有备注手机号,请在remark_mobiles填写一个空字符串("")。
/// remark_pic_mediaid可以通过素材管理接口获得
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsExternalContactRemark {
    /// 企业成员的userid
    pub userid: String,
    /// 外部联系人userid
    pub external_userid: String,
    /// 此用户对外部联系人的备注，最多20个字符
    pub remark: Option<String>,
    /// 此用户对外部联系人的描述，最多150个字符
    pub description: Option<String>,
    /// 此用户对外部联系人备注的所属公司名称，最多20个字符
    pub remark_company: Option<String>,
    /// 此用户对外部联系人备注的手机号
    pub remark_mobiles: Option<Vec<String>>,
    /// 备注图片的mediaid
    pub remark_pic_mediaid: Option<String>,
}

impl ParamsExternalContactRemark {
    pub fn check(&self) -> bool {
        if self.remark.is_none()
            && self.description.is_none()
            && self.remark_company.is_none()
            && self.remark_mobiles.is_none()
            && self.remark_pic_mediaid.is_none()
        {
            return false;
        }

        true
    }
}
