use serde::{Deserialize, Serialize};

pub trait Responser: Default {
    fn error_code(&self) -> u64;
    fn error_message(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response<T> {
    #[serde(rename = "errcode")]
    err_code: u64,
    #[serde(rename = "errmsg")]
    err_msg: String,
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T> Responser for Response<T>
where
    T: Default,
{
    fn error_code(&self) -> u64 {
        self.err_code
    }

    fn error_message(&self) -> String {
        self.err_msg.clone()
    }
}

impl<T> Default for Response<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            err_code: Default::default(),
            err_msg: Default::default(),
            data: Default::default(),
        }
    }
}

/// token
mod token;
pub(crate) use token::Token;

mod user;
pub use user::{User, UserList};

mod department;
pub use department::{Department, DepartmentList, DepartmentSimpleList};
