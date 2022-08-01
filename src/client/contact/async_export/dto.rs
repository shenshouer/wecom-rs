use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsExport {
    /// 需要导出的标签
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagid")]
    pub tag_id: Option<i64>,
    /// base64encode的加密密钥，长度固定为43，
    /// base64decode之后即得到AESKey。
    /// 加密方式采用AES-256-CBC方式，
    /// 数据采用PKCS#7填充至32字节的倍数；
    /// IV初始向量大小为16字节，
    /// 取AESKey前16字节，
    /// 详见：https://datatracker.ietf.org/doc/html/rfc2315
    pub encoding_aeskey: String,
    /// 每块数据的人员数和部门数之和，支持范围[104,106]，默认值为106
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,
}
