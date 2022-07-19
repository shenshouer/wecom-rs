use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FailedChatList {
    /// 没能成功继承的群
    pub failed_chat_list: Vec<FailedChat>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FailedChat {
    /// 没能成功继承的群ID
    pub chat_id: String,
    /// 没能成功继承的群，错误码
    #[serde(rename = "errcode")]
    pub err_code: String,
    /// 没能成功继承的群，错误描述
    #[serde(rename = "errmsg")]
    pub err_msg: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Customer<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<T>,
    /// 下个分页的起始cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TransferResult {
    /// 转接客户的外部联系人userid
    pub external_userid: String,
    /// 接替状态， 1-接替完毕 2-等待接替 3-客户拒绝 4-接替成员客户达到上限
    pub status: i32,
    /// 接替客户的时间，如果是等待接替状态，则为未来的自动接替时间
    pub takeover_time: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TransferCustomer {
    /// 客户的external_userid
    pub external_userid: String,
    /// 对此客户进行分配的结果, 具体可参考全局错误码, 0表示开始分配流程,待24小时后自动接替,并不代表最终分配成功
    pub errcode: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseUnassignedList {
    ///
    pub info: Vec<UnassignedCustomer>,
    /// 是否是最后一条记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    /// 分页查询游标,已经查完则返回空("")，使用page_id作为查询参数时不返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UnassignedCustomer {
    /// 离职成员的userid
    pub handover_userid: String,
    /// 外部联系人userid
    pub external_userid: String,
    /// 成员离职时间
    pub dimission_time: i64,
}
