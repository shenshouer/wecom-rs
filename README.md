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
- [✓] 加解密 [实现](./src/utils/crypto.rs)
- [ ] 推广二维码
- [WIP] 通讯录管理
    - [✓] 成员管理 [实现](./src/client/contact/user/mod.rs) [示例](./examples/user.rs)
    - [✓] 部门管理 [实现](./src/client/contact/department/mod.rs) [示例](./examples/department.rs)
    - [✓] 标签管理 [实现](./src/client/contact/tag/mod.rs) [示例](./examples/tag.rs)
    - [WIP] 异步批量接口 [实现](./src/client/contact/async_batch/mod.rs)
    - [ ] 通讯录回调通知
    - [WIP] 互联企业 [实现](./src/client/contact/linked_corp/mod.rs)
    - [WIP] 异步导出接口 [实现](./src/client/contact/async_export/mod.rs)
- [WIP] 客户联系
    - [✓] 企业服务人员管理 [实现](./src/client/external_contact/enterprise_service/mod.rs) [示例](./examples/external_contact.rs)
    - [WIP] 客户管理 [实现](./src/client/external_contact/customer/mod.rs)
    - [WIP] 客户标签管理 [实现](./src/client/external_contact/customer_tag/mod.rs)
    - [✓] 在职继承 [实现](./src/client/external_contact/on_job_inherit/mod.rs) [示例](./examples/external_contact.rs)
    - [✓] 离职继承 [实现](./src/client/external_contact/leave_job_inherit/mod.rs) [示例](./examples/external_contact.rs)
    - [WIP] 客户群管理  [实现](./src/client/external_contact/group_chat/mod.rs)
    - [ ] 联系我与客户入群方式
    - [ ] 客户朋友圈
    - [ ] 消息推送
    - [ ] 统计管理
    - [ ] 变更回调
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