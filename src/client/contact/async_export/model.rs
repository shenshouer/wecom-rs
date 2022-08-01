use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExportResult {
    /// 数据下载链接,支持指定Range头部分段下载。有效期2个小时
    pub url: String,
    /// 密文数据大小
    pub size: usize,
    /// 密文数据md5
    pub md5: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseExportResultGet {
    /// 任务状态:0-未处理，1-处理中，2-完成，3-异常失败
    pub status: i32,
    /// 数据文件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_list: Option<Vec<ExportResult>>,
}
