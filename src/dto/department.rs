use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsCreateDepartment {
    ///	部门名称。同一个层级的部门名称不能重复。长度限制为1~32个字符，字符不能包括:*?"<>｜
    pub name: String,
    ///	英文名称。同一个层级的部门名称不能重复。需要在管理后台开启多语言支持才能生效。长度限制为1~32个字符，字符不能包括:*?"<>｜
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    ///	父部门id，32位整型
    #[serde(rename = "parentid")]
    pub parent_id: u64,
    ///	在父部门中的次序值。order值大的排序靠前。有效的值范围是[0, 2^32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u64>,
    ///	部门id，32位整型，指定时必须大于1。若不填该参数，将自动生成id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

impl ParamsCreateDepartment {
    pub fn new(name: &str, parent_id: u64) -> Self {
        Self {
            name: String::from(name),
            parent_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsUpdateDepartment {
    ///	部门id
    pub id: u64,
    ///	部门名称。长度限制为1~32个字符，字符不能包括:*?"<>｜
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///	英文名称，需要在管理后台开启多语言支持才能生效。长度限制为1~32个字符，字符不能包括:*?"<>｜
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    ///	父部门id
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentid")]
    pub parent_id: Option<u64>,
    ///	在父部门中的次序值。order值大的排序靠前。有效的值范围是[0, 2^32)
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentid")]
    pub order: Option<u64>,
}

impl ParamsUpdateDepartment {
    pub fn is_empty(&self) -> bool {
        if self.name.is_none()
            && self.name_en.is_none()
            && self.parent_id.is_none()
            && self.order.is_none()
        {
            return true;
        }
        false
    }
}
