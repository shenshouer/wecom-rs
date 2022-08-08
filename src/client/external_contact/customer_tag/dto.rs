use serde::{Deserialize, Serialize};

/// 请确保external_userid是userid的外部联系人。
/// add_tag和remove_tag不可同时为空。
/// 同一个标签组下现已支持多个标签
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsExternalContactMarkTag {
    /// 添加外部联系人的userid
    pub userid: String,
    /// 外部联系人userid
    pub external_userid: String,
    /// 要标记的标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tag: Option<Vec<String>>,
    /// 要移除的标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tag: Option<Vec<String>>,
}
