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

## todo list

- [✓] 获取access_token [实现](./src/client/mod.rs)
- [ ] 通讯录管理
    - [✓] 成员管理 [实现](./src/client/contact/user/mod.rs) [示例](./examples/user.rs)
    - [✓] 部门管理 [实现](./src/client/contact/department/mod.rs) [示例](./examples/department.rs)
    - [ ] 标签管理
    - [ ] 异步批量接口
    - [ ] 互联企业
    - [ ] 异步导出接口
- [ ] 客户联系
- [ ] 微信客户
- [ ] 身份验证
- [ ] 应用管理
- [ ] 消息推送
- [ ] 素材管理
- [ ] OA
- [ ] 效率工具
- [ ] 企业支付
- [ ] 企业互联
- [ ] 上下游
- [ ] 会话内容存档
- [ ] 电子发票
- [ ] 家校沟通
- [ ] 家校应用
- [ ] 政民沟通