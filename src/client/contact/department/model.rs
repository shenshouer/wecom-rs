use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DepartmentList<T> {
    pub department: T,
}

impl FromIterator<Department> for DepartmentList<Vec<Department>> {
    fn from_iter<I: IntoIterator<Item = Department>>(iter: I) -> Self {
        Self {
            department: Vec::from_iter(iter),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Department {
    ///	创建的部门id
    pub id: u64,
    ///	部门名称，代开发自建应用需要管理员授权才返回；此字段从2019年12月30日起，对新创建第三方应用不再返回，2020年6月30日起，对所有历史第三方应用不再返回name，返回的name字段使用id代替，后续第三方仅通讯录应用可获取，未返回名称的情况需要通过通讯录展示组件来展示部门名称
    pub name: String,
    ///	英文名称，此字段从2019年12月30日起，对新创建第三方应用不再返回，2020年6月30日起，对所有历史第三方应用不再返回该字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    ///	部门负责人的UserID；第三方仅通讯录应用可获取
    pub department_leader: Vec<String>,
    ///	父部门id。根部门为1
    pub parentid: u64,
    ///	在父部门中的次序值。order值大的排序靠前。值范围是[0, 2^32)
    pub order: u64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DepartmentSimpleList {
    pub department_id: Vec<DepartmentSimple>,
}

impl FromIterator<DepartmentSimple> for DepartmentSimpleList {
    fn from_iter<I: IntoIterator<Item = DepartmentSimple>>(iter: I) -> Self {
        Self {
            department_id: Vec::from_iter(iter),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DepartmentSimple {
    ///	创建的部门id
    pub id: u64,
    ///	父部门id。根部门为1。
    #[serde(rename = "parentid")]
    pub parent_id: u64,
    ///	在父部门中的次序值。order值大的排序靠前。值范围是[0, 2^32)。
    pub order: u64,
}
