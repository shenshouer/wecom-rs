use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RespExternalContactCorpTagList {
    /// 标签组列表
    pub tag_group: Vec<CorpTag>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CorpTag {
    /// 标签组列表
    pub group_id: String,
    /// 标签组列表
    pub group_name: String,
    /// 标签组列表
    pub create_time: i64,
    /// 标签组排序的次序值，order值大的排序靠前。有效的值范围是[0, 2^32)
    pub order: i64,
    /// 标签组是否已经被删除，只在指定tag_id进行查询时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// 标签组所属的规则组id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<i64>,
    /// 标签组内的标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Tag {
    /// 标签id
    pub id: String,
    /// 标签名称
    pub name: String,
    /// 标签名称
    pub create_time: i64,
    /// 标签排序的次序值，order值大的排序靠前。有效的值范围是[0, 2^32)
    pub order: i64,
    /// 标签是否已经被删除，只在指定tag_id/group_id进行查询时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
