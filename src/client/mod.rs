pub use crate::dto::*;
use crate::model::{Response, Responser, Token, User};
use crate::{
    error::{new_api_error, Result},
    utils::http::{do_http, PostParameters},
};
use async_trait::async_trait;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

const BASE_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin";

#[derive(Debug)]
pub struct Client {
    pub(crate) corp_id: String,
    pub(crate) corp_secret: String,
}

impl Client {
    pub fn new(corp_id: String, corp_secret: String) -> Client {
        Self {
            corp_id,
            corp_secret,
        }
    }

    async fn access_token(&self) -> Result<String> {
        let query_body = json!({
            "corpid": self.corp_id,
            "corpsecret": self.corp_secret,
        });

        let resp = do_http(
            Method::GET,
            &format!("{}/gettoken", BASE_URL),
            None,
            Some(query_body),
            None,
        )
        .await?;

        let data = resp.json::<Token>().await?;

        Ok(data.access_token)
    }

    // http 请求
    async fn request<R: Responser + DeserializeOwned + Default>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<R> {
        let body = if let Some(data) = body {
            Some(PostParameters::json(data))
        } else {
            None
        };
        let resp = do_http(method, url, None, None, body)
            .await?
            .json::<R>()
            .await?;

        if resp.error_code() != 0 {
            return Err(new_api_error(resp.error_code(), resp.error_message()));
        }

        Ok(resp)

        // 调试使用，验证输出结果
        // let resp = do_http(method, url, None, None, body)
        //     .await?
        //     .text()
        //     .await?;
        // println!("{resp}");
        // Ok(Response::default().data)
    }
}

/// 通讯录管理
#[async_trait]
pub trait ContactManager {
    // 成员管理
    /// 创建成员
    async fn user_create(&self) -> Result<()>;
    /// 读取成员
    async fn user_get(&self, user_id: &str) -> Result<User>;
}

/// 客户管理
pub trait ExternalContactManager {
    // TODO:
}

pub trait CustomerServiceManager {
    // TODO:
}

mod contact;