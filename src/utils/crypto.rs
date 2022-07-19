use crate::{error::Error, Result};
use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes256,
};
use sha1::{Digest, Sha1};

const AES_ENCODE_KEY_LEN: usize = 43;

pub struct WecomCrypto {
    token: String,
    receive_id: String,
    byte_aes_key: Vec<u8>,
}

impl WecomCrypto {
    pub fn new(token: &str, receive_id: &str, aes_key: &str) -> Result<Self> {
        if aes_key.len() != AES_ENCODE_KEY_LEN {
            return Err(Error::General(String::from("不合法的EncodingAESKey")));
        }

        let byte_aes_key = base64::decode_config(
            format!("{}=", aes_key),
            base64::STANDARD.decode_allow_trailing_bits(true),
        )
        .unwrap();

        Ok(Self {
            token: token.to_string(),
            receive_id: receive_id.to_string(),
            byte_aes_key,
        })
    }

    /// 消息加密
    /// @params:
    ///   msg: 消息体明文
    ///   timestamp: 时间戳
    ///   nonce: 随机字符串
    /// @return
    ///   0: 加密消息
    ///   1: 签名
    pub fn encrypt_msg(&self, msg: &str, timestamp: &str, nonce: &str) -> Result<(String, String)> {
        let size = (msg.len() as u32).to_be_bytes();

        let msg = format!(
            "{}{}{}{}",
            random_string(16),
            String::from_utf8_lossy(&size),
            msg,
            self.receive_id
        );

        let cipher_msg = base64::encode(self.encrypt(&msg));
        let signature = self.create_signature(&cipher_msg, &timestamp, &nonce);

        Ok((cipher_msg, signature))
    }

    /// 加密明文字符串
    fn encrypt(&self, msg: &str) -> Vec<u8> {
        let padded_message = pad_pkcs7(msg, 16);
        let msg_bytes = padded_message.as_bytes();
        let iv = &self.byte_aes_key[..16].to_vec();

        let b_key_slice = &self.byte_aes_key[..];
        let cipher = Aes256::new(b_key_slice.into());

        let mut encrypted_blocks: Vec<Vec<u8>> = Vec::new();
        (0..msg.len()).step_by(16).for_each(|x| {
            let last = encrypted_blocks.last().unwrap_or(iv);
            let xor_block = xor_bytes(last, &msg_bytes[x..x + 16]);
            let mut block = GenericArray::clone_from_slice(&xor_block);

            cipher.encrypt_block(&mut block);
            encrypted_blocks.push(block.into_iter().collect::<Vec<u8>>());
        });

        encrypted_blocks.into_iter().flatten().collect::<Vec<u8>>()
    }

    pub fn decrypt_msg(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        cipher_msg: &str,
    ) -> Result<String> {
        if !self.verification_signature(msg_signature, timestamp, nonce, cipher_msg) {
            return Err(Error::General("签名不匹配".to_string()));
        }

        let msg = self.decrypt(cipher_msg);
        let size = u32::from_be_bytes(msg[16..16 + 4].try_into().unwrap()) as usize;
        let plant_text = &msg[16 + 4..];
        let receive_id = String::from_utf8_lossy(&plant_text[size..]);

        assert_eq!(self.receive_id, receive_id);

        Ok(format!("{}", String::from_utf8_lossy(&plant_text[..size])))
    }

    /// 解密消息数据
    pub fn decrypt(&self, secret_msg: &str) -> Vec<u8> {
        let encrypted_bytes = base64::decode(secret_msg).unwrap();
        let iv = &self.byte_aes_key[..16].to_vec();
        let b_key_slice = &self.byte_aes_key[..];

        let cipher = Aes256::new(b_key_slice.into());
        let mut decrypted_blocks: Vec<Vec<u8>> = Vec::new();
        (0..encrypted_bytes.len()).step_by(16).for_each(|x| {
            let last = if x == 0 {
                &iv
            } else {
                &encrypted_bytes[x - 16..x]
            };

            let mut block = GenericArray::clone_from_slice(&encrypted_bytes[x..x + 16]);
            cipher.decrypt_block(&mut block);
            let decrypted_block = block.into_iter().collect::<Vec<u8>>();
            let xor_block = xor_bytes(last, &decrypted_block);
            decrypted_blocks.push(xor_block);
        });

        // Get number of padding bytes applied during encryption & remove padding
        let padding_byte = *decrypted_blocks.last().unwrap().last().unwrap() as usize;

        decrypted_blocks
            .into_iter()
            .flatten()
            .take(encrypted_bytes.len() - padding_byte)
            .collect::<Vec<u8>>()
    }

    fn verification_signature(
        &self,
        signature: &str,
        timestamp: &str,
        nonce: &str,
        msg: &str,
    ) -> bool {
        self.create_signature(msg, timestamp, nonce) == signature
    }

    fn create_signature(&self, msg: &str, timestamp: &str, nonce: &str) -> String {
        let mut params: Vec<String> = vec![];
        params.push(self.token.clone());
        params.push(timestamp.to_string());
        params.push(nonce.to_string());
        params.push(msg.to_string());
        params.sort();
        let x = params.join("");
        let mut hasher = Sha1::new();
        hasher.update(x.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect()
}

/// 生成随机字符串
fn random_string(len: usize) -> String {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// 加密补位
fn pad_pkcs7(message: &str, block_size: usize) -> String {
    let padding_size = block_size - message.len() % block_size;
    let padding_char = padding_size as u8 as char;
    let padding: String = (0..padding_size).map(|_| padding_char).collect();
    format!("{}{}", message, padding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_msg_decrypt() -> Result<()> {
        let token = "QDG6eK";
        let aes_key = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C";
        let corp_id = "wx5823bf96d3bd56c7";
        let content = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";
        let signature = "477715d11cdb4164915debcba66cb864d751f3e6";
        let timestamp = "1409659813";
        let nonce = "1372623149";

        let c = WecomCrypto::new(token, corp_id, aes_key)?;
        let data = c.decrypt_msg(signature, timestamp, nonce, content)?;
        assert_eq!(
            r#"<xml><ToUserName><![CDATA[wx5823bf96d3bd56c7]]></ToUserName>
<FromUserName><![CDATA[mycreate]]></FromUserName>
<CreateTime>1409659813</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[hello]]></Content>
<MsgId>4561255354251345929</MsgId>
<AgentID>218</AgentID>
</xml>"#,
            data
        );
        Ok(())
    }

    #[test]
    fn test_msg_encrypt() -> Result<()> {
        let token = "123456";
        let aes_key = "4g5j64qlyl3zvetqxz5jiocdr586fn2zvjpa8zls3ij";
        let corp_id = "suite4xxxxxxxxxxxxxxx221";
        let timestamp = "1445827045067";
        let content = "succ111ess";
        let nonce = "nEXhMP4r";
        let c = WecomCrypto::new(token, corp_id, aes_key)?;
        let (_cipher_msg, _signature) = c.encrypt_msg(content, timestamp, nonce)?;
        println!("signature:{_signature}, cipher_msg:{_cipher_msg}");
        Ok(())
    }
}
