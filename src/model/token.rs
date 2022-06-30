use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Token {
    pub access_token: String,
    pub expires_in: u32,
}
