use crate::error::{new_http_error, Result};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, Method, Response,
};
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

/// 获取http client，如果设置了PROXY环境变量，则将使用PROXY代理
fn get_http_client() -> Result<Client> {
    let mut client_builder = reqwest::Client::builder();
    if let Ok(proxy) = std::env::var("PROXY") {
        debug!("PROXY is {}", &proxy);
        client_builder = client_builder.proxy(reqwest::Proxy::https(&proxy)?);
    }
    Ok(client_builder.build()?)
}

#[derive(Debug, Default)]
pub struct PostParameters {
    form: Option<Value>,
    json: Option<Value>,
}

impl PostParameters {
    // pub fn form(form: Value) -> Self {
    //     PostParameters {
    //         form: Some(form),
    //         json: None,
    //     }
    // }

    pub fn json(json: Value) -> Self {
        PostParameters {
            form: None,
            json: Some(json),
        }
    }
}

#[tracing::instrument]
pub async fn do_http(
    method: Method,
    req_url: &str,
    headers: Option<HashMap<HeaderName, String>>,
    query: Option<Value>,
    body: Option<PostParameters>,
) -> Result<Response> {
    debug!("request url: {}", req_url);

    // debug!(
    //     "url: {}, method: {}, headers: {:?}, query: {:?}, body:{:?}",
    //     req_url, method, headers, query, body
    // );

    let client = get_http_client()?;

    let mut req_builder = client.get(req_url);
    // .header(reqwest::header::CONTENT_TYPE, "application/json");

    // set query params
    if let Some(query) = query {
        req_builder = req_builder.query(&query);
    }

    // post params setting
    if let Some(params) = body {
        if let Some(body) = params.json {
            req_builder = req_builder.json(&body);
        }
        if let Some(form) = params.form {
            req_builder = req_builder.form(&form)
        }
    }

    let mut req = req_builder.build()?;

    // set request addons headers
    if let Some(headers) = headers {
        for (key, value) in headers.iter() {
            req.headers_mut().append(key, HeaderValue::from_str(value)?);
        }
    }

    // change method
    let m = req.method_mut();
    *m = method;

    let resp = client.execute(req).await?;

    let status_code = resp.status();
    if !status_code.is_success() {
        let msg = resp.text().await?;
        return Err(new_http_error(req_url.to_owned(), status_code, msg));
    }

    Ok(resp)
}
