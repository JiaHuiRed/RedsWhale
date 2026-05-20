# RedsTui

> **Fork 自 [Hmbown/DeepSeek-TUI](https://github.com/Hmbown/DeepSeek-TUI)，感谢原作者的出色工作。**
> 本仓库为 Red 的个人定制版，专为简体中文用户优化。

[![版本](https://img.shields.io/badge/版本-v0.1.0-blue)](CHANGELOG.zh-CN.md)
[![上游同步](https://img.shields.io/badge/上游-v0.8.39-green)](https://github.com/Hmbown/DeepSeek-TUI)
[![许可证](https://img.shields.io/badge/许可证-MIT-lightgrey)](LICENSE)

[English README](README.en.md) | [日本語 README](README.ja-JP.md)

---

## 这是什么？

面向 DeepSeek / Ollama 等大模型 API 的终端原生对话客户端，基于 Rust + Ratatui 构建。支持流式输出、Markdown 渲染、MCP 工具调用等功能。

**与原项目的主要差异：**

| 改动 | 说明 |
|------|------|
| 块引用渲染 | Markdown `>` 语法显示 `▌` 竖线轨道 |
| 版本独立 | 使用自有版本号体系（v0.x.x） |
| 中文优先 | 文档以简体中文为主版本 |

---

## 从源码构建

本仓库**不提供预编译二进制**，需要自行编译。

### 前置条件

- Rust 1.88+（通过 [rustup.rs](https://rustup.rs) 安装）
- Windows 用户需安装 [WinLibs MinGW-w64](https://winlibs.com)（提供 GNU 工具链）

### 编译步骤

```bash
# 克隆仓库
git clone https://github.com/JiaHuiRed/Red-DS-TUI.git
cd Red-DS-TUI

# 检查编译（快速验证）
cargo check -p deepseek-tui   # crate 内部名，暂保持原名

# 完整构建
cargo build --release

# 可执行文件位于
# Windows: target/release/redstui.exe
# Linux/macOS: target/release/redstui
```

### 保持与上游同步

```bash
git remote add upstream https://github.com/Hmbown/DeepSeek-TUI.git
git fetch upstream
git merge upstream/main
```

---

## 配置

首次运行会在以下位置创建配置文件：

- **Windows**：`%APPDATA%\deepseek\config.toml`
- **Linux/macOS**：`~/.deepseek/config.toml`

参考 [`config.example.toml`](config.example.toml) 查看完整配置项。

### DeepSeek API

```toml
[providers.deepseek]
api_key = "sk-your-key-here"
```

或通过环境变量：`DEEPSEEK_API_KEY=sk-your-key-here`

### Ollama 本地模型

已安装 [Ollama](https://ollama.com) 并在本地运行时，在 `config.toml` 中添加：

```toml
provider = "ollama"

[providers.ollama]
base_url = "http://localhost:11434/v1"
model = "deepseek-r1:8b"   # 替换为 ollama list 中显示的模型名
# api_key 可选
```

切换提供商也可在运行时输入 `/provider ollama`。

---

## 快捷键

| 按键 | 功能 |
|------|------|
| `Enter` | 发送消息 |
| `Ctrl+J` | 插入换行 |
| `Ctrl+C` | 中止生成 |
| `Esc` | 退出 |
| `/help` | 所有命令 |
| `/provider` | 切换提供商 |
| `/model` | 切换模型 |

完整快捷键见 [`docs/KEYBINDINGS.md`](docs/KEYBINDINGS.md)。

---

## 更新日志

见 [CHANGELOG.zh-CN.md](CHANGELOG.zh-CN.md)。

---

## 致谢

- 原项目：[Hmbown/DeepSeek-TUI](https://github.com/Hmbown/DeepSeek-TUI)
- 许可证：[MIT](LICENSE)，版权归原作者所有
