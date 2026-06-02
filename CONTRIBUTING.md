# 贡献指南

感谢你对 RedsWhale 项目的关注！本文档提供贡献指南和说明。

## 快速开始

### 前置条件

- Rust 1.88 或更高版本（edition 2024）
- Cargo 包管理器
- Git

### 设置开发环境

1. Fork 并克隆仓库：
   ```bash
   git clone https://your-username/Red-DS-TUI.git
   cd Red-DS-TUI
   ```

2. 构建项目：
   ```bash
   cargo build
   ```

3. 运行测试：
   ```bash
   cargo test
   ```

4. 开发模式运行：
   ```bash
   cargo run
   ```

## 开发工作流

### 代码风格

- 提交前运行 `cargo fmt` 确保格式一致
- 运行 `cargo clippy` 并处理所有警告
- 遵循 Rust 命名规范（函数/变量用 snake_case，类型用 CamelCase）
- 为公共 API 添加文档注释

### 测试

- 为新功能编写测试
- 确保所有现有测试通过：`cargo test --workspace --all-features`
- 将单元测试放在代码旁边（标准 Rust `#[cfg(test)]` 模块），集成测试放在 crate 的 `tests/` 目录下

### 提交信息

使用约定式提交格式：
- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码格式（不影响功能）
- `refactor:` 重构
- `test:` 测试相关
- `chore:` 构建/工具相关

示例：`feat: 添加 Ollama 本地模型支持`

## 项目结构

```
Red-DS-TUI/
├── crates/
│   ├── cli/          # CLI 分发器（RedsWhale 命令）
│   ├── tui/          # TUI 运行时（RedsWhale-tui）
│   ├── config/       # 配置管理
│   ├── core/         # 核心引擎
│   ├── agent/        # 代理功能
│   ├── mcp/          # MCP 协议支持
│   ├── tools/        # 工具集
│   ├── state/        # 状态管理
│   ├── secrets/      # 密钥管理
│   ├── hooks/        # 钩子系统
│   ├── protocol/     # 协议定义
│   ├── execpolicy/   # 执行策略
│   ├── tui-core/     # TUI 核心组件
│   └── app-server/   # 应用服务器
└── docs/             # 文档
```

## 运行命令

```bash
# 构建
cargo build

# 测试
cargo test --workspace --all-features

# Lint
cargo clippy --workspace --all-targets --all-features

# 格式化
cargo fmt --all

# 运行 TUI
cargo run --bin RedsWhale

# 运行 CLI
cargo run --bin RedsWhale
```

## 报告问题

1. 在 GitHub 上搜索现有 issue
2. 如果没有找到，创建新 issue
3. 提供清晰的标题和描述
4. 包含复现步骤、预期行为和实际行为
5. 附上环境信息（OS、Rust 版本等）

## 提交 PR

1. 从 `main` 分支创建功能分支
2. 进行修改并测试
3. 确保所有测试通过
4. 更新相关文档
5. 提交 PR 并描述更改内容

## 行为准则

- 尊重所有参与者
- 接受建设性批评
- 专注于对社区最有利的事情
- 对其他社区成员表示同理心

## 许可证

贡献即表示你同意你的贡献将在 MIT 许可证下发布。

---

**英文版**: [CONTRIBUTING.en.md](CONTRIBUTING.en.md)
