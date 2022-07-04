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
