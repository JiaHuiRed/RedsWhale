# 更新日志

本文件记录 Red-DS-TUI 的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循以下规则：小型改动 v0.0.x，中等改动 v0.1.x，大型改动 v1.x.x。

---

## [0.0.4] - 2026-05-22

### 新增

- **Fin 快速执行智能体**：新增 `tool-agent` / `fin` 子智能体类型，使用 V4 Flash 模型并关闭思考模式，专注快速执行简单工具任务，减少等待时间。
- **截图 OCR 工具**：AI 可调用 `image_ocr` 工具对项目目录中的图片文件（PNG/JPEG/TIFF）进行本地 OCR 文字提取。Windows/Linux 需安装 Tesseract，macOS 使用系统 Vision 框架。
- **会话分叉**（`/fork`）：在当前对话任意节点另开一条分支，保留上下文独立推进不同方案。
- **输入框 Home/End 跳转**：多行输入框支持 Home/End 键跳到行首/行尾。
- **model picker 记住推理强度**：切换模型后 effort 设置不再重置。

### 维护

- **同步上游 v0.8.40**：含 Windows 滚动优化（composer 箭头键默认改为滚动模式）、工具调用去重修复、子智能体写权限修复等。
- **Windows 滚动优化**：Windows 下方向键默认触发滚动而非历史导航，与 macOS/Linux 行为对齐。

---

## [0.0.3] - 2026-05-21

### 新增

- **代码块语法高亮**（`markdown_render.rs`）：fenced code block 现在根据语言标识符（rust / python / js / bash 等）渲染彩色语法高亮，使用 syntect 5 + base16-ocean.dark 主题，颜色以 24-bit RGB 输出到终端。

### 修复

- **MinGW UCRT 链接错误**：将 reqwest TLS 后端从 `rustls`（依赖 aws-lc-sys）换为 `native-tls`（Windows SChannel），修复 WinLibs 16.x POSIX UCRT 下 `nanosleep64` 未定义符号导致的链接失败。

---

## [0.0.2] - 2026-05-20

### 新增

- **嵌套列表渲染**（`markdown_render.rs`）：`Block::ListItem` 新增 `depth` 字段，按缩进层级（每层 2 空格）正确缩进显示多级列表。
- **Ollama `/models` 支持**（`ui.rs`）：`/models` 命令现在对 Ollama provider 发起实时查询，列出本地已下载的所有模型。
- **Red 名字动画**（`header.rs`）：标题栏鲸鱼旁显示 R→e→d 逐字母亮起动画，与鲸鱼跃出/落水节奏同步。
- **品牌统一**：二进制重命名为 `redstui` / `redstui-tui`，界面文字、启动画面、信任对话框全部替换为 RedsTui。
- **中文文档主版本**：README.md 升级为中文主版本（emoji 标题风格），附 README.en.md 英文版。
- **版本号体系独立**：启用自有版本号（0.x.x），与上游 0.8.x 分离。

### 维护

- **同步上游 v0.8.39**：合并原项目最新修复（详见下方上游变更记录）。
- **注释规范统一**：将旧格式 `//#YYMMDD` 修正为 `//YYMMDD`。

---

## [0.0.1] - 2026-05-15

### 新增

- **块引用渲染**（`markdown_render.rs`）：支持 Markdown `>` 语法，左侧渲染 `▌` 蓝色竖线，文字以 `TEXT_DIM` 颜色显示。

---

## 上游变更记录（原项目）

以下为同步自 [Hmbown/DeepSeek-TUI](https://github.com/Hmbown/DeepSeek-TUI) 的上游版本变更，供参考。

---

### [上游 0.8.39] - 2026-05-17

#### 修复

- **飞书/Lark 桥接启动顺序已加锁**：`ThreadStore` 现在保证在持久线程状态打开前完成初始化，并增加回归测试防止回退。
- **`/model` 选择器再次即时打开**：还原了 v0.8.38 的实时目录方案，选择器不再在打开时发起阻塞网络请求，恢复显示精选 `auto` / `deepseek-v4-pro` / `deepseek-v4-flash` 条目；`/models` 命令仍可列出完整实时目录。
- **"会话批准" 再次按命令族分组**：会话批准改回使用基于模糊指纹的键值匹配，批准 `cargo build` 后也涵盖 `cargo build --release`；拒绝仍使用精确指纹，不会误拦后续不同调用。
- **Docker 首次运行状态目录可写**：镜像现在预建 `/home/deepseek/.deepseek` 并赋予 `deepseek` 所有权，修复命名卷首次运行时的写入失败。
- **运行时 API 系统提示覆盖在首轮后持续生效**：以 `system_prompt` 覆盖创建的线程在模型请求构建前的模式/上下文刷新中保持该提示。
- **压缩保留工具密集历史中的用户文本查询**：自动压缩现在在仅含工具调用/结果的保留尾部时，固定最近的用户文本消息，避免下次请求的 Jinja 模板错误。
- **翻页器跳转精确落在可见底部**：`G` / End 键不再过冲渲染边界，鼠标滚轮现在直接滚动翻页覆盖层。
- **鼠标滚轮模拟箭头时保留输入草稿**：`composer_arrows_scroll` 开启时，Up/Down 在有草稿的情况下也会滚动转录区，不再替换草稿内容。
- **多行输入框箭头先在行内移动光标**：单行模式下鼠标滚轮模拟箭头行为不变。
- **第三方 `reasoning_content` 流不再损坏文本输出**：非官方支持推理内容语义的提供商将其渲染为普通文本。
- **macOS 系统主题检测识别浅色模式**：`COLORFGBG` 缺失时回退到 macOS 外观检测，缺少 `AppleInterfaceStyle` 键时视为浅色模式。
- **`rlm_open` 接受 schema 中填充的空白来源字段**：空字符串的 `file_path`、`content`、`url` 视为缺失，只提供一个真实来源时不再验证失败。
- **终端缩放后翻页立即可用**：PageUp/PageDown 使用缩放后的视口高度，不再在下次渲染前退化为单行跳转。
- **ACP 响应将 JSON-RPC id 转为字符串**：`serve --acp` 现在即使客户端发送数字 id 也返回字符串 id，符合 Zed 严格的 ACP 客户端预期。

---

### [上游 0.8.38] - 2026-05-12

#### 修复

- **OpenAI 批量响应中保留所有 `tool_calls`**：修复流式响应中工具调用数组合并逻辑，防止多工具调用时丢失条目。
- **循环防护块正确计为失败次数**：修复工具调用循环计数器在防护块触发时不递增的问题。
