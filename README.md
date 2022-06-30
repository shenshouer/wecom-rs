# wecom-rs
企业微信Rust版SDK

接口参考：https://developer.work.weixin.qq.com/document/path/90664

## doc
[crates.io](https://crates.io/crates/wecom-rs)

## examples

[examples](./examples)

### run examples

- 将 `.env.sample` 改名为 `.env`, 替换`.env`中的`CORP_ID`与`CORP_SECRET`
- 修改[examples](./examples)对应rs文件中的参数后，使用`cargo run --example xxx `运行，如运行user示例： `cargo run --example user`