use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsTransferCustomer {
    /// 原跟进成员的userid
    pub handover_userid: String,
    /// 接替成员的userid
    pub takeover_userid: String,
    /// 客户的external_userid列表，每次最多分配100个客户
    pub external_userid: Vec<String>,
    /// 转移成功后发给客户的消息，最多200个字符，不填则使用默认文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_success_msg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsTransferResult {
    /// 原跟进成员的userid
    pub handover_userid: String,
    /// 接替成员的userid
    pub takeover_userid: String,
    /// 分页查询的cursor，每个分页返回的数据不会超过1000条；不填或为空表示获取第一个分页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
