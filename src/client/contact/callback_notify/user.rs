use super::helper::{
    str_to_i64, str_to_i8_vec, str_to_option_i64_vec, str_to_option_i8, str_to_option_vec,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "xml")]
pub struct EventUserCreate {
    /// 企业微信CorpID
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    /// 此事件该值固定为sys，表示该消息由系统生成
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    /// 消息创建时间 （整型）
    #[serde(rename = "CreateTime", deserialize_with = "str_to_i64")]
    pub create_time: i64,
    /// 消息的类型，此时固定为event
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    /// 事件的类型，此时固定为change_contact
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "ChangeType")]
    pub change_type: String,
    /// 成员UserID
    #[serde(rename = "UserID")]
    pub user_id: String,
    /// 新的UserID，变更时推送（userid由系统生成时可更改一次）
    #[serde(rename = "NewUserID")]
    pub new_user_id: String,
    /// 成员名称，此字段从2019年12月30日起，
    /// 对新创建第三方应用不再返回真实name，使用userid代替name，
    /// 2020年6月30日起，对所有历史第三方应用不再返回真实name，使用userid代替name，
    /// 后续第三方仅通讯录应用可获取;
    /// 代开发自建应用需要管理员授权才返回；
    /// 未返回name的情况页面需要通过通讯录展示组件来展示名字
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 更新后成员所在部门列表，仅返回该应用有查看权限的部门id；成员授权模式下，仅返回根部门，即1
    #[serde(
        rename = "Department",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "str_to_option_i64_vec"
    )]
    pub department: Option<Vec<i64>>,
    /// 主部门，成员授权模式下，仅返回根部门，即1
    #[serde(rename = "MainDepartment")]
    pub main_department: i64,
    /// 表示所在部门是否为上级，0-否，1-是，顺序与Department字段的部门逐一对应
    #[serde(rename = "IsLeaderInDept", deserialize_with = "str_to_i8_vec")]
    pub is_leader_in_dept: Vec<i8>,
    /// 直属上级UserID，最多5个，仅通讯录管理应用可获取；代开发的自建应用不返回该字段
    #[serde(
        default,
        rename = "DirectLeader",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "str_to_option_vec"
    )]
    pub direct_leader: Option<Vec<String>>,
    /// 手机号码，仅通讯录管理应用可获取;代开发自建应用需要管理员授权才返回
    #[serde(rename = "Mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 职位信息。长度为0~64个字节，仅通讯录应用可获取;代开发自建应用需要管理员授权才返回
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 性别。0表示未定义，1表示男性，2表示女性。
    /// 代开发自建应用需要管理员授权且成员oauth2授权获取；
    /// 第三方仅通讯录应用可获取；
    /// 对于非第三方创建的成员，第三方通讯录应用也不可获取；
    /// 上游企业不可获取下游企业成员该字段。
    /// 注：不可获取指返回值0
    #[serde(
        default,
        rename = "Gender",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "str_to_option_i8"
    )]
    pub gender: Option<i8>,
    /// 邮箱，仅通讯录管理应用可获取;代开发自建应用需要管理员授权才返回
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 企业邮箱，仅通讯录管理应用可获取;代开发自建应用不返回
    #[serde(rename = "BizMail", skip_serializing_if = "Option::is_none")]
    pub biz_mail: Option<String>,
    /// 激活状态: 1=已激活，2=已禁用，4=未激活，5=退出企业。
    /// 已激活代表已激活企业微信或已关注微信插件（原企业号）。
    /// 未激活代表既未激活企业微信又未关注微信插件（原企业号）。
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 头像url。注：如果要获取小图将url最后的”/0”改成”/100”即可，仅通讯录管理应用可获取
    #[serde(rename = "Avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 成员别名
    #[serde(rename = "Alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// 座机，仅通讯录管理应用可获取;代开发自建应用需要管理员授权才返回
    #[serde(rename = "Telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    /// 地址。代开发自建应用需要管理员授权且成员oauth2授权获取；
    /// 第三方仅通讯录应用可获取；
    /// 对于非第三方创建的成员，第三方通讯录应用也不可获取；
    /// 上游企业不可获取下游企业成员该字段
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    // /// 扩展属性;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(skip_serializing_if = "Option::is_none", rename = "ExtAttr")]
    pub ext_attr: Option<ExtAttr>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtAttr {
    #[serde(rename = "Item")]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub kind: String,
    #[serde(flatten)]
    pub attr: Attr,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Attr {
    Text {
        #[serde(rename = "Value")]
        value: String,
    },
    // #[serde(rename = "Type")]
    // Web {
    //     #[serde(rename = "Title")]
    //     title: String,
    //     #[serde(rename = "Url")]
    //     url: String,
    // },
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use quick_xml::de::from_str;

    #[test]
    fn test_parse_xml_evnet_user_create() -> Result<()> {
        println!("==>>test_parse_xml_evnet_user_create");
        let xml_str = r##"
        <xml>
	<ToUserName><![CDATA[toUser]]></ToUserName>
	<FromUserName><![CDATA[sys]]></FromUserName> 
	<CreateTime>1403610513</CreateTime>
	<MsgType><![CDATA[event]]></MsgType>
	<Event><![CDATA[change_contact]]></Event>
	<ChangeType>update_user</ChangeType>
	<UserID><![CDATA[zhangsan]]></UserID>
	<NewUserID><![CDATA[zhangsan001]]></NewUserID>
	<Name><![CDATA[张三]]></Name>
	<Department><![CDATA[1,2,3]]></Department>
	<MainDepartment>1</MainDepartment>
	<IsLeaderInDept><![CDATA[1,0,0]]></IsLeaderInDept>
	<Position><![CDATA[产品经理]]></Position>
	<Mobile>13800000000</Mobile>
	<Gender>1</Gender>
	<Email><![CDATA[zhangsan@gzdev.com]]></Email>
	<Status>1</Status>
	<Avatar><![CDATA[http://wx.qlogo.cn/mmopen/ajNVdqHZLLA3WJ6DSZUfiakYe37PKnQhBIeOQBO4czqrnZDS79FH5Wm5m4X69TBicnHFlhiafvDwklOpZeXYQQ2icg/0]]></Avatar>
	<Alias><![CDATA[zhangsan]]></Alias>
	<Telephone><![CDATA[020-123456]]></Telephone>
	<Address><![CDATA[广州市]]></Address>
	<ExtAttr>
		<Item>
		<Name><![CDATA[爱好]]></Name>
		<Type>0</Type>
		<Text>
			<Value><![CDATA[旅游]]></Value>
		</Text>
		</Item>
		<Item>
		<Name><![CDATA[卡号]]></Name>
		<Type>1</Type>
		<Web>
			<Title><![CDATA[企业微信]]></Title>
			<Url><![CDATA[https://work.weixin.qq.com]]></Url>
		</Web>
		</Item>
	</ExtAttr>
</xml>"##;
        println!("==>>test_parse_xml_evnet_user_create111");
        let xml: EventUserCreate = from_str(xml_str)?;

        assert_eq!(xml.to_user_name, "toUser");
        Ok(())
    }
}
