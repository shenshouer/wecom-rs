use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsTransferCustomer {
    /// 原跟进成员的userid
    /// 必须是已离职用户
    pub handover_userid: String,
    /// 接替成员的userid
    pub takeover_userid: String,
    /// 客户的external_userid列表，最多一次转移100个客户
    /// external_userid必须是handover_userid的客户（即配置了客户联系功能的成员所添加的联系人）
    pub external_userid: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsUnassignedList {
    /// 分页查询，要查询页号，从0开始
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_id: Option<i64>,
    /// 分页查询游标，字符串类型，适用于数据量较大的情况，如果使用该参数则无需填写page_id，该参数由上一次调用返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// 每次返回的最大记录数，默认为1000，最大值为1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
