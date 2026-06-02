# 项目指令

本文件为在此项目上工作的 AI 助手提供上下文。

## 项目类型: Rust

### 命令
- 构建: `cargo build`（默认成员包含 `deepseek` 分发器）
- 测试: `cargo test --workspace --all-features`
- Lint: `cargo clippy --workspace --all-targets --all-features`
- 格式化: `cargo fmt --all`
- 运行（规范）: `deepseek` — 使用 **`deepseek` 二进制文件**，而非 `deepseek-tui`。分发器委托给 TUI 进行交互使用，是所有流程的支持入口（`deepseek`、`deepseek -p "..."`、`deepseek doctor`、`deepseek mcp` 等）。
- 从源码运行: `cargo run --bin deepseek`（或 `cargo run -p deepseek-tui-cli`）。
- 本地开发简写: `cargo build --release` 后运行 `./target/release/deepseek`。
- **两个二进制文件，两次安装。** `deepseek`（CLI 分发器，`crates/cli`）和 `deepseek-tui`（TUI 运行时，`crates/tui`）作为**独立可执行文件**发布。分发器在 PATH 中查找并启动 `deepseek-tui` 作为同级进程进行交互使用，因此只安装 CLI 会导致 TUI 过时。每当修改 `crates/tui/` 下的内容时，需要同时安装两个：
  ```bash
  cargo install --path crates/cli --locked --force
  cargo install --path crates/tui --locked --force
  ```
  发布流水线会同时打包两者 — 只有手动维护者安装才会遗漏。如果刚做的修复"没有生效"，先检查 `stat -f '%Sm' ~/.cargo/bin/deepseek-tui` 再去用 `tracing::debug!`。

### 构建依赖
- **Rust** 1.88+（工作空间声明 `rust-version = "1.88"`，因为我们在 `if`/`while` 条件中使用了 `let_chains`，该特性在 1.88 中稳定）。

### 仅使用稳定版 Rust — 不使用 nightly 功能

此 crate 必须在稳定版 Rust 上编译。**永远不要**引入需要 `#![feature(...)]`、`cargo +nightly` 或任何不稳定语言/库功能的代码。常见陷阱：

- **`if let` guards in match arms**（`if_let_guard`，tracking issue #51114）— 在 Rust < 1.94 上是 nightly-only。重写为普通 match guard，在 arm body 中嵌套 `if let`：
  ```rust
  // 错误 — 在 stable rustc < 1.94 上编译失败 (E0658)
  match key {
      KeyCode::Char(c) if cond && let Some(x) = find(c) => { … }
  }
  ```
  改为：
  ```rust
  // 正确 — 在所有支持的 rustc 上都能工作
  match key {
      KeyCode::Char(c) if cond => {
          if let Some(x) = find(c) { … }
      }
  }
  ```
- `let_chains` in `if`/`while`（`&& let Some(_) = …`）自 Rust 1.88 起**已稳定**，可以使用。
- 自定义 `#![feature(...)]` 属性 — 永远不要使用。

提交 PR 前，运行 `cargo build`（不是 `cargo +nightly build`）确保工作空间声明的 `rust-version` 足够编译。

### 文档
项目概览见 README.md，内部架构见 docs/ARCHITECTURE.md。

## DeepSeek 相关说明

- **思考 Token**: DeepSeek 模型在最终回答前输出思考块（`ContentBlock::Thinking`）。TUI 以视觉区分方式流式显示这些内容。
- **推理模型**: `deepseek-v4-pro` 和 `deepseek-v4-flash` 是正式的 V4 模型 ID。旧版 `deepseek-chat` 和 `deepseek-reasoner` 是 `deepseek-v4-flash` 的兼容别名。
- **大上下文窗口**: DeepSeek V4 模型支持 100 万 token 上下文窗口。使用搜索工具高效导航。
- **API**: OpenAI 兼容的 Chat Completions（`/chat/completions`）是 DeepSeek 的正式 API 路径。Base URL 在全局和 `deepseek-cn` 预设中使用官方主机 `api.deepseek.com`；旧版拼写错误主机 `api.deepseeki.com` 仍保持向后兼容。`/v1` 为 OpenAI SDK 兼容接受，`/beta` 仅用于 beta 功能（如 strict tool mode、chat prefix completion 和 FIM completion）。
- **思考 + 工具调用**: 在 V4 思考模式下，包含工具调用的 assistant 消息必须在所有后续请求中重放其 `reasoning_content`，否则 API 返回 HTTP 400。

## 会话持久性（关键）

DeepSeek TUI 中的长时间会话如果顺序工作将会降级并崩溃。会话在 `api_messages` 和 `history` 中累积每条消息和工具结果，**没有自动修剪**（自 v0.6.6 起自动压缩默认禁用）。会话保存会将整个膨胀的数组序列化到磁盘。

**多小时冲刺的生存策略：**

1. **尽早委派独立工作。** 对于只读侦察、有界实现切片、测试验证或 issue 分类等不需要阻塞下一步本地操作的任务，为每个任务打开一个专注的 `agent_open` 会话。你是协调者；将父级会话保留给决策、集成和面向用户的综合。

2. **批量独立读取/搜索。** 避免一个 `read_file`、等待、一个 `grep_files`、等待。将回答同一问题的读取/搜索一起发出，然后总结证据，而不是让重复的工具行充斥会话记录。

3. **积极压缩。** 在 60% 上下文使用率时建议 `/compact`，而不是 80%。一个保持快速的压缩会话总是胜过一个死掉的会话。

4. **在 3 个连续父级轮次后重新评估。** 如果同一功能仍需要广泛阅读、issue 分类或并行验证，将工作拆分到子代理或 RLM 会话中，而不是继续串行父线程爬取。

5. **每 3 个轮次后检查：** 上下文低于 60%？子代理还在运行？PR 准备推送？`cargo check` 还通过吗？

---

**English**: [AGENTS.en.md](AGENTS.en.md)
