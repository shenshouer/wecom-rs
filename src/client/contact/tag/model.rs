use serde::{de::Deserializer, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TagUser {
    pub userid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseTagUserGet {
    #[serde(rename = "userlist")]
    pub user_list: Vec<TagUser>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseTagCreate {
    #[serde(rename = "tagid")]
    pub tag_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseTagList {
    #[serde(rename = "taglist")]
    pub tag_list: Vec<Tag>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseTagUser {
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "invalidlist",
        deserialize_with = "str_to_str_vec"
    )]
    pub invalid_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "invalidparty")]
    pub invalid_party: Option<Vec<i64>>,
}

pub fn str_to_str_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    let x = if let Some(s) = s {
        Some(s.split(',').map(|x| x.to_string()).collect::<Vec<_>>())
    } else {
        None
    };

    Ok(x)
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tag {
    #[serde(rename = "tagid")]
    pub id: i64,
    #[serde(rename = "tagname")]
    pub name: String,
}
