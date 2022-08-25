use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsGroupChatListParams {
    /// 客户群跟进状态过滤。
    /// 0 - 所有列表(即不过滤)
    /// 1 - 离职待继承
    /// 2 - 离职继承中
    /// 3 - 离职继承完成
    /// 默认为0
    pub status_filter: Option<i32>,
    /// 群主过滤。
    /// 如果不填，表示获取应用可见范围内全部群主的数据
    /// （但是不建议这么用，如果可见范围人数超过1000人，
    /// 为了防止数据包过大，会报错 81017）
    pub owner_filter: Option<UseridList>,
    /// 用于分页查询的游标，字符串类型，由上一次调用返回，首次调用不填
    pub cursor: Option<String>,
    /// 分页，预期请求的数据量，取值范围 1 ~ 1000
    pub limit: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UseridList {
    /// 用户ID列表。最多100个
    pub userid_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsGroupChatGetParams {
    /// 客户群ID
    pub chat_id: String,
    /// 是否需要返回群成员的名字group_chat.member_list.name。
    /// 0-不返回；1-返回。默认不返回
    pub need_name: Option<i32>,
}
