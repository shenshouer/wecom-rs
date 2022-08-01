use crate::client::common::model::ExtAttributes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseAgentPermList {
    /// 可见的userids，是用 CorpId + ’/‘ + USERID 拼成的字符串
    #[serde(skip_serializing_if = "Option::is_none", rename = "userids")]
    pub user_ids: Option<Vec<String>>,
    /// 可见的department_ids，是用 linkedid + ’/‘ + department_id 拼成的字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserInfo {
    /// 成员UserID。对应管理端的帐号，企业内必须唯一。不区分大小写，长度为1~64个字节
    #[serde(rename = "userid")]
    pub user_id: String,
    ///	成员真实名称
    pub name: String,
    /// 手机号码
    pub mobile: String,
    ///	成员所属部门id列表，这个字段会返回在应用可见范围内，该用户所在的所有互联企业的部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<String>>,
    /// 职务信息
    pub position: String,
    /// 邮箱
    pub email: String,
    ///	座机
    pub telephone: String,
    ///	所属企业的corpid
    pub corpid: String,
    ///	扩展属性
    pub extattr: Option<ExtAttributes>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserSimpleInfo {
    /// 成员UserID。对应管理端的帐号，企业内必须唯一。不区分大小写，长度为1~64个字节
    #[serde(rename = "userid")]
    pub user_id: String,
    ///	成员真实名称
    pub name: String,
    ///	成员所属部门id列表，这个字段会返回在应用可见范围内，该用户所在的所有互联企业的部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<String>>,
    pub corpid: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DepartmentInfo {
    #[serde(rename = "department_id")]
    pub id: String,
    #[serde(rename = "department_name")]
    pub name: String,
    #[serde(rename = "parentid")]
    pub parent_id: String,
    pub order: i64,
}
