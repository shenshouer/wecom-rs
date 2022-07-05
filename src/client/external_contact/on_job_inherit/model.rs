use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponeTransferCustomer {
    /// 客户的external_userid
    pub external_userid: String,
    /// 对此客户进行分配的结果, 具体可参考全局错误码, 0表示成功发起接替,待24小时后自动接替,并不代表最终接替成功
    #[serde(rename = "errcode")]
    pub err_code: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponeTransferResult {
    /// 转接客户的外部联系人userid
    pub external_userid: String,
    /// 接替状态， 1-接替完毕 2-等待接替 3-客户拒绝 4-接替成员客户达到上限 5-无接替记录
    pub status: i64,
    /// 接替客户的时间，如果是等待接替状态，则为未来的自动接替时间
    pub takeover_time: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponeTransferResultList {
    pub customer: Vec<ResponeTransferResult>,
    /// 下个分页的起始cursor
    pub next_cursor: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponeTransferOnjobGroupChatList {
    /// 没能成功继承的群
    pub failed_chat_list: Vec<FailedChat>,
}

/// 没能成功继承的群
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FailedChat {
    pub chat_id: String,
    #[serde(rename = "errcode")]
    pub err_code: i64,
    #[serde(rename = "errmsg")]
    pub err_msg: String,
}
