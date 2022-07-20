use crate::{
    error::{new_api_error, Result},
    model::{Responser, Token},
    utils::http::{do_http, PostParameters},
};
use reqwest::Method;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{json, Value};
use std::env;
use tracing::debug;

const BASE_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin";

#[derive(Debug)]
pub struct Client {
    pub(crate) corp_id: String,
    pub(crate) corp_secret: String,
    /// 客户联系专用
    pub(crate) custom_contact_secret: Option<String>,
}

impl Client {
    pub fn new(
        corp_id: String,
        corp_secret: String,
        custom_contact_secret: Option<String>,
    ) -> Client {
        Self {
            corp_id,
            corp_secret,
            custom_contact_secret,
        }
    }

    pub fn new_from_env() -> Result<Self> {
        let custom_contact_secret = if let Ok(ccs) = env::var("CUSTOM_CONTACT_SECRET") {
            Some(ccs)
        } else {
            None
        };
        Ok(Self {
            corp_id: env::var("CORP_ID")?,
            corp_secret: env::var("CORP_SECRET")?,
            custom_contact_secret,
        })
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

    async fn custom_contact_access_token(&self) -> Result<String> {
        let query_body = json!({
            "corpid": self.corp_id,
            "corpsecret": self.custom_contact_secret,
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
    async fn request<R: Responser + DeserializeOwned + Serialize + Default>(
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

        debug!("\nurl:{url} \nresponse: {}", serde_json::to_string(&resp)?);
        if resp.error_code() != 0 {
            return Err(new_api_error(resp.error_code(), resp.error_message()));
        }
        Ok(resp)
    }
}

/// 通讯录管理
mod contact;
pub use contact::*;

/// 客户联系管理
mod external_contact;
pub use external_contact::*;
/// 微信客服
mod wechat_custom_service;
pub use wechat_custom_service::*;
/// 身份验证
mod auth;
pub use auth::*;
/// 应用管理
mod app;
pub use app::*;
/// 消息推送
mod message_push;
pub use message_push::*;
/// 事件定义
pub mod event;
// TODO: 其他功能模块接口实现
