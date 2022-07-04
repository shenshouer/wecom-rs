use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponeTransferCustomer {
    /// 客户的external_userid
    pub external_userid: String,
    /// 对此客户进行分配的结果, 具体可参考全局错误码, 0表示成功发起接替,待24小时后自动接替,并不代表最终接替成功
    #[serde(rename = "errcode")]
    pub err_code: i64,
}
