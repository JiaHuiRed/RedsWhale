# 安全策略

RedsWhale 是一个可直接访问文件操作、shell 执行和网络的编程智能体。我们重视安全问题的披露。

## 支持的版本

只有最新的稳定版会收到安全补丁。不支持旧版本的向后移植。

| 版本 | 是否支持 |
|---|---|
| 最新稳定版 | :white_check_mark: |
| < 最新版 | :x: |

查看 [releases 页面](https://github.com/Hmbown/DeepSeek-TUI/releases) 获取当前版本。

## 报告漏洞

**不要为安全漏洞创建公开的 GitHub issue。**

请通过以下方式私下报告：

- **GitHub 私有安全通告**: [github.com/Hmbown/DeepSeek-TUI/security/advisories/new](https://github.com/Hmbown/DeepSeek-TUI/security/advisories/new)
- **邮件**: [security@deepseek-tui.com](mailto:security@deepseek-tui.com) — 主题请加 `[SECURITY]` 前缀

报告中请包含：

- 漏洞描述及其被利用后的影响
- 复现步骤或概念验证
- 受影响的版本和配置详情
- 建议的缓解措施（可选）

## 响应时间

| 阶段 | 目标 |
|---|---|
| 确认收到 | 收到后 48 小时内 |
| 评估 | 5 天内 — 判定严重程度、影响范围和修复方案 |
| 补丁（严重） | 评估后 14 天内 |
| 补丁（中等/低） | 下次功能发布或按维护者时间线 |
| 公开披露 | 补丁发布且用户有时间更新后 |

每个阶段都会收到状态更新。如果时间线有变动，我们会说明原因和修订后的预估。

## 范围

### 在范围内

- 通过构造的 prompt 或模型响应实现远程代码执行
- 沙箱逃逸 — 突破 YOLO 模式工作区边界或 shell `cwd` 限制
- 凭证泄露 — 窃取 API key、token 或环境变量中的密钥
- 在预期工作区之外的任意文件读写（`PathEscape` 绕过）
- 通过 `fetch_url` 或 `web_search` 对内网端点发起的 SSRF
- 未授权的 MCP 服务器访问或工具调用

### 不在范围内

- 对维护者或贡献者进行社会工程攻击
- 对 DeepSeek API 的拒绝服务 / 速率限制耗尽攻击
- 第三方依赖的漏洞（请向对应上游项目报告）
- 需要物理接触受害者机器的攻击
- 未在 RedsWhale 环境中验证的理论性 ML 模型注入攻击

如果你不确定某个 bug 是否在范围内，仍然请报告。我们会进行分类并回复。

## 致谢榜

我们维护一个致谢榜，记录提交已验证安全漏洞的报告者。如需署名，请在报告中注明你希望使用的名称/昵称。

*暂无条目 — 成为第一个吧。*

---

**English**: [SECURITY.en.md](SECURITY.en.md)
