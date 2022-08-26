use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JobStatus {
    /// 任务状态，整型，1表示任务开始，2表示任务进行中，3表示任务已完成
    pub status: i32,
    /// 操作类型，字节串，目前分别有：
    /// 1. sync_user(增量更新成员)
    /// 2. replace_user(全量覆盖成员)
    /// 3. replace_party(全量覆盖部门)
    #[serde(rename = "type")]
    pub kind: JobType,
    /// 任务运行总条数
    pub total: i32,
    /// 目前运行百分比，当任务完成时为100
    pub percentage: i32,
    /// 详细的处理结果，具体格式参考下面说明。当任务完成后此字段有效
    pub result: Vec<Job>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum JobType {
    /// 增量更新成员
    #[default]
    #[serde(rename = "sync_user")]
    SyncUser,
    /// 全量覆盖成员
    #[serde(rename = "replace_user")]
    ReplaceUser,
    /// 全量覆盖部门
    #[serde(rename = "replace_party")]
    ReplaceParty,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Job {
    #[serde(rename = "errcode")]
    pub err_code: String,
    #[serde(rename = "errmsg")]
    pub err_msg: i64,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserId>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub party: Option<Party>,
}

/// 用户相关批量异步接口结果
/// type为sync_user、replace_user时：
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserId {
    /// 成员UserID。对应管理端的帐号
    pub userid: String,
}

/// 部门相关批量异步接口结果
/// type为replace_party时：
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Party {
    /// 操作类型（按位或）：
    /// 1 新建部门 ，
    /// 2 更改部门名称，
    /// 4 移动部门，
    /// 8 修改部门排序
    pub action: i32,
    /// 部门ID
    pub partyid: i64,
}
