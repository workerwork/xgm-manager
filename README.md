# XGM Manager

[![Build](https://github.com/workerwork/xgm-manager/actions/workflows/tauri.yml/badge.svg)](https://github.com/workerwork/xgm-manager/actions)
[![Release](https://img.shields.io/github/v/release/workerwork/xgm-manager?include_prereleases)](https://github.com/workerwork/xgm-manager/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)](https://github.com/workerwork/xgm-manager/releases)

XGM Manager 是一款基于 Tauri 2、Vue 3、Vite 和 TypeScript 的跨平台桌面管理工具，专为 XGM 设备的远程配置、监控和管理而设计。

## 安装与运行

### 环境准备

确保你已经安装以下环境：

- Node.js (>= 16)
- Rust 工具链
- Tauri 构建依赖（详见：[https://tauri.app/v2/guides/getting-started/prerequisites](https://tauri.app/v2/guides/getting-started/prerequisites)）

### 安装依赖并运行

```bash
# 克隆仓库
git clone https://github.com/workerwork/xgm-manager.git

cd xgm-manager

# 安装依赖
npm install

# 启动开发模式
npm run tauri dev
```

## 发布版本

本项目已发布 0.1.0 版本，详见 GitHub Releases 页面：

👉 [XGM Manager Releases](https://github.com/workerwork/xgm-manager/releases)

在 Releases 页面可以下载各平台安装包，查看版本更新日志及发布说明，方便快速获取最新稳定版本。

## 功能列表

- ✅ 用户登录
- ✅ 自动接收 XGM 设备广播
- ✅ 显示 XGM 设备列表和状态，支持过滤查询
- ✅ 远程配置 XGM 设备参数
- ✅ 远程修改 XGM 设备地址
- ✅ 主机 python 包列表显示和版本升级
- ✅ quark 进程输出和错误日志显示
- ✅ 集成 Jupyternotebook 环境
- ✅ 支持跨平台构建（Windows / Linux / macOS）

## 配套后端服务

XGM Manager 依赖于 xgm-server 后端服务进行设备管理和数据交互。

xgm-server: [https://github.com/workerwork/xgm-server](https://github.com/workerwork/xgm-server)
