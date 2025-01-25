[English](README.md) | **中文**

# FOSScope Toolkit

为开源观察贡献者设计的工具箱

## 项目结构

本项目使用 Tauri 作为框架，使用 Rust 作为后端，以及 React + TypeScript 作为前端。

### 推荐的开发环境

- IntelliJ IDEA + Rust 插件
- [VS Code](https://code.visualstudio.com/) + [Tauri 插件](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer 插件](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- Node.js 20.x
- npm 10.x
- 最新版的 Rust
    - 通过 `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh` 安装
- Cargo
- Tauri CLI
    - 通过 `cargo install tauri-cli` 安装

### 调试

运行 `cargo tauri dev` 即可自动启动前端和后端。

你也可以通过 `npm run dev` 来启动前端，然后通过运行 Rust target `fosscopetoolkit` 来启动后端+程序窗口。

### 文件结构

- `.vscode`：VS Code 配置文件，用于自动配置插件（Tauri 自动生成）
- `public`：前端静态资源
- `src`：前端源码，基于 Vite，使用 React + TypeScript 编写
- `src-rust`：所有相关的 Rust 源码
    - `tauri-backend`：Tauri 后端代码，即为默认生成模板中的 `src-tauri` 目录
    - `toolkit-core`：核心库，用于提供核心与基础功能的实现
    - `toolkit-cli`：命令行版本的 FOSScope Toolkit，可以单独发布，也可用于在前端未开发完毕的情况下测试核心功能

## 贡献者

- [@Cubik65536](https://github.com/Cubik65536) - 首席开发者 & 项目经理
- [@cys2004](https://github.com/cys2004) - 副项目经理 & 开发者，HTML 处理机制
- [@sureau2020](https://github.com/sureau2020) - 前端开发者 & 前端设计师
