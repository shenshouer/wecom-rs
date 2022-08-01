use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AsyncExporter {
    /// 导出成员
    /// https://developer.work.weixin.qq.com/document/path/94849
    /// @return string 任务ID，可通过获取导出结果接口查询任务结果
    async fn export_user_simple(&self, params: ParamsExport) -> Result<String>;
    /// 导出成员详情
    /// https://developer.work.weixin.qq.com/document/path/94851
    /// @return string 任务ID，可通过获取导出结果接口查询任务结果
    async fn export_user(&self, params: ParamsExport) -> Result<String>;
    /// 导出部门
    /// https://developer.work.weixin.qq.com/document/path/94852
    /// @return string 任务ID，可通过获取导出结果接口查询任务结果
    async fn export_department(&self, params: ParamsExport) -> Result<String>;
    /// 导出标签成员
    /// https://developer.work.weixin.qq.com/document/path/94853
    /// @return string 任务ID，可通过获取导出结果接口查询任务结果
    async fn export_tag_user(&self, params: ParamsExport) -> Result<String>;
    /// 获取导出结果
    /// https://developer.work.weixin.qq.com/document/path/94854
    /// @return string 任务ID，可通过获取导出结果接口查询任务结果
    async fn export_result_get(&self, job_id: &str) -> Result<ResponseExportResultGet>;
}

/// 导出任务完成通知
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExportCompleteNotify {
    /// 企业微信CorpID
    pub to_user_name: String,
    /// 此时固定为sys
    pub from_user_name: String,
    /// 消息创建时间（整型）
    pub create_time: i64,
    /// 消息类型，此时固定为：event
    pub msg_type: String,
    /// 事件类型：batch_job_result
    pub event: String,
    pub batch_job: Option<BatchJob>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase", rename = "xml")]
pub struct BatchJob {
    /// 异步任务id，最大长度为64字符
    pub job_id: String,
    /// 操作类型，字符串，在异步导出的场景下分别有：
    /// export_user(导出成员详情)、
    /// export_simple_user(导出成员）、
    /// export_department(导出部门）、
    /// export_tag(导出标签成员)
    pub job_type: String,
    /// 返回码
    pub err_code: i32,
    /// 对返回码的文本描述内容
    pub err_msg: String,
}

mod dto;
pub use dto::*;

mod model;
pub use model::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use quick_xml::de::from_str;

    #[test]
    fn test_parse_xml_evnet_export_complete_notify() -> Result<()> {
        let xml_str = r#"<xml>
<ToUserName><![CDATA[wx28dbb14e3720FAKE]]></ToUserName>
<FromUserName><![CDATA[FromUser]]></FromUserName>
<CreateTime>1425284517</CreateTime>
<MsgType><![CDATA[event]]></MsgType>
<Event><![CDATA[batch_job_result]]></Event>
<BatchJob>
    <JobId><![CDATA[jobid_S0MrnndvRG5fadSlLwiBqiDDbM143UqTmKP3152FZk4]]></JobId>
    <JobType><![CDATA[export_user]]></JobType>
    <ErrCode>0</ErrCode>
    <ErrMsg><![CDATA[ok]]></ErrMsg>
</BatchJob>
</xml>"#;

        let xml: ExportCompleteNotify = from_str(xml_str)?;
        assert_eq!("ok", xml.batch_job.unwrap().err_msg);
        Ok(())
    }
}
