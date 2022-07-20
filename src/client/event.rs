use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    /// 通讯录变更通知
    ChangeContact(ChangeContact),
}

/// 通讯录变更通知
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeContact {
    /// 新增成员事件
    CreateUser,
    /// 更新成员事件
    UpdateUser,
    /// 删除成员事件
    DeleteUser,
    /// 新增部门事件
    CreateParty,
    /// 更新部门事件
    UpdateParty,
    /// 删除部门事件
    DeleteParty,
    /// 标签成员变更事件
    UpdateTag,
    /// 异步任务完成通知
    BatchJobResult,
}
