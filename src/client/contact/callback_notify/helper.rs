use serde::{de::Deserializer, de::Error, Deserialize};

pub fn str_to_i8_vec<'de, D>(deserializer: D) -> Result<Vec<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let x = s
        .split(',')
        .map(|x| x.parse::<i8>())
        .collect::<core::result::Result<Vec<i8>, _>>()
        .map_err(D::Error::custom)?;

    Ok(x)
}

pub fn str_to_option_i64_vec<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    let x = if let Some(s) = s {
        Some(
            s.split(',')
                .map(|x| x.parse::<i64>())
                .collect::<core::result::Result<Vec<i64>, _>>()
                .map_err(D::Error::custom)?,
        )
    } else {
        None
    };

    Ok(x)
}

pub fn str_to_option_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    let x = if let Some(s) = s {
        Some(s.split(',').map(|x| x.to_string()).collect())
    } else {
        None
    };
    Ok(x)
}

pub fn str_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.parse().map_err(D::Error::custom)?)
}

pub fn str_to_option_i8<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Some(s.parse().map_err(D::Error::custom)?));
    }
    Ok(None)
}
