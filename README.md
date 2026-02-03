# Claude Code Monitor

一个轻量级的跨平台桌面应用，用于统筹管理和查看 Claude Code 运行实例及其执行情况。

## 🎨 Vibecoding

本项目采用 **Vibecoding** 开发模式 —— 由 AI (Claude) 主导开发，人类提供需求和反馈。所有的开发对话、需求沟通和迭代过程都完整记录在 `vibe/` 文件夹中，形成了一份真实的 AI 协作开发档案。

通过查看 `vibe/` 目录下的对话记录，你可以了解：
- 项目从概念到实现的完整演进过程
- AI 如何理解需求并做出技术决策
- 遇到的问题以及解决方案的推导过程
- 人机协作开发的实际案例

## 功能特性

- **实例监控**: 实时显示系统中运行的所有 Claude Code 实例
- **资源监控**: 查看 CPU、内存使用情况
- **自动安装**: 内置 Claude Code 自动安装功能
- **Hook 集成**: 从工具内部管理和配置 Claude Code Hooks
- **历史记录**: 长期保存会话历史，支持搜索
- **简洁界面**: 黑白灰极简设计风格

## 技术架构

- **框架**: Tauri (Rust + Web)
- **前端**: Vanilla TypeScript + 自定义 CSS
- **数据库**: SQLite (嵌入式)
- **进程监控**: sysinfo crate
- **Hook 服务**: Axum HTTP 服务器

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows / macOS / Linux

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri:dev
```

### 构建发布版本

```bash
npm run tauri:build
```

## 项目结构

```
F:\hw\hw-2602\
├── src-tauri/           # Rust 后端代码
│   ├── src/
│   │   ├── main.rs      # 程序入口
│   │   ├── lib.rs       # 模块组织
│   │   ├── commands.rs  # Tauri 命令
│   │   ├── database.rs  # SQLite 数据库
│   │   ├── hook_server.rs # Hook HTTP 服务
│   │   ├── installer.rs # Claude Code 安装器
│   │   ├── monitor.rs   # 进程监控
│   │   └── models.rs    # 数据模型
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src/                 # 前端代码
│   ├── main.ts          # 主入口
│   ├── style.css        # 样式
│   └── types.d.ts       # 类型声明
├── hooks/               # Hook 脚本模板
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 使用说明

### 1. 检查/安装 Claude Code

打开应用后，先前往"安装管理"页面：
- 自动检测 Claude Code 是否已安装
- 一键安装 Node.js (如未安装)
- 一键安装 Claude Code

### 2. 监控实例

在"实例监控"页面：
- 自动扫描系统中运行的 Claude Code 进程
- 每 5 秒自动刷新
- 显示 PID、工作目录、CPU/内存占用
- 支持终止指定实例

### 3. 配置 Hook

在"Hook 配置"页面：
- 安装 Hook 脚本到 `~/.claude-monitor/`
- 自动生成 wrapper 脚本
- 可选择全局启用或手动使用

### 4. 查看历史

在"历史记录"页面：
- 搜索历史会话
- 按工作区/项目查看统计

## Hook 机制

工具通过以下方式捕获 Claude Code 事件：

1. **Hook 脚本** (`~/.claude-monitor/hook.sh`): 发送 HTTP 请求到本地服务
2. **Wrapper 脚本** (`~/.claude-monitor/claude-with-hook`): 包装 claude 命令，自动设置环境变量

使用方法：
```bash
# 方法 1: 使用 wrapper
cd ~/.claude-monitor
./claude-with-hook

# 方法 2: 手动设置环境变量
export CLAUDE_CODE_HOOKS=~/.claude-monitor/hook.sh
claude
```

## 数据库

SQLite 数据库默认存储在：
- Windows: `%APPDATA%/claude-code-monitor/data.db`
- macOS: `~/Library/Application Support/claude-code-monitor/data.db`
- Linux: `~/.local/share/claude-code-monitor/data.db`

## 配置

Hook 服务端口：9876 (可修改)

## 跨平台支持

| 功能 | Windows | macOS | Linux |
|------|---------|-------|-------|
| 进程监控 | ✓ | ✓ | ✓ |
| 资源监控 | ✓ | ✓ | ✓ |
| 自动安装 | ✓ | ✓ | ✓ |
| Hook 集成 | ✓ | ✓ | ✓ |

## 注意事项

1. 某些系统可能需要管理员权限才能监控其他用户进程的资源使用情况
2. Hook 机制依赖于 Claude Code 的环境变量支持
3. 首次安装可能需要重启终端使 `claude` 命令生效

## License

MIT
