<p align="center">
  <img src="./brand.png" alt="在这个客户端中，有无数的天才想要在肥鱼图标里，证明自己是天才。可是在带锁图标里，天才只是门槛。" width="49%"><img src="./brand_2.jpg" alt="你应该放弃这世间所有的C端客户端，来体验宇宙最强对抗路神作，这是你唯一和顶级开源模型厮杀的机会" width="49%">
</p>

<h1 align="center">deepseek unrestricted chat harness</h1>

<p align="center">
  deepseek unrestricted chat harness —— 基于 DeepSeek API 的静态聊天前端，带解除限制模式、回复改写与参数直调<br>
  <a href="./README.md">English</a> | 简体中文
</p>

> 本项目是官方示例 [deepseek-ai/chat-demo](https://github.com/deepseek-ai/chat-demo) 的**非官方修改版**（fork），MIT 许可。
> 所有改动都在浏览器里完成：请求体怎么拼、回复怎么存、界面怎么显示。没有代理、没有中转服务，
> API Key 与对话记录仍只保存在你自己的浏览器里。
>
> 🔗在线版：<https://chat-unrestricted.fur0.com/>
>
> 📦仓库：<https://github.com/xiaohuangbo/chat-demo-unrestricted>
>
> 🎥介绍视频：<https://www.bilibili.com/video/BV1m3s66WESg/>
>
> 📚参考论文：<https://arxiv.org/pdf/2403.12171>

<p align="center">
  <img src="./Demo%20Picture.png" alt="聊天界面演示：解除限制模式下的技术问答" width="860">
</p>

---

## 本仓库新增了什么

- **解除限制与提示词工程**：越狱模式、系统提示词预设、每轮注入、中文越狱提示词、自定义思维链；
- **浏览器 Agent 工具**（默认关闭，逐项开关）：时间、定位、天气、通知、剪贴板、抓网页、生成文件、打开链接、可视化组件；
- **虚拟工作区**：浏览器本地文件库，任意格式导入导出、OPFS 真文件系统、安卓端为应用存储；
- **NanoVM Slim**：内置 WASM Shell Engine 与 130 个 busybox 命令，支持从本机或工作区直接导入单个文件（含 zip / 二进制，另有 `nanovm_import` Agent 工具），并新增 PyBot 工具链（64 万行纯 Python 规则聊天机器人，无模型权重），桌面端右侧停靠、手机端底部面板，Python / Node / Lua / SQLite / esbuild / tsc / Sass / Terser / Prettier / make / Git 按需安装，虚拟机内自带虚拟网络（Node 与 Python 的 HTTP / WebSocket 服务）；
- **WebVM（CheerpX）**：第二个终端跑真正的 x86 Debian 10，自带 bash / gcc / g++ / Python 3 / Node.js / Ruby / Lua / make / git / vim；与 NanoVM 同时运行、自由切换，支持导入导出与模型调用，磁盘改动写进 IndexedDB 覆盖层；
- **Unkernel Linux**：第三个虚拟机，跑真正编译成 WebAssembly 的 Linux 7.x 内核——Alpine 3.24 / Debian 13.7、`apk` / `apt`、bash、SMP 多 CPU、Wisp 联网（默认公共 anura.pro / mercurywork.shop 代理）、Web Audio 声音、framebuffer/evdev 显示、IndexedDB 持久磁盘（设置里可扩容、可重置）、文件导入导出和完整 unkernel 页面（界面已汉化）；聊天工作区挂到 `/mnt/workspace`，模型可用 shell、列目录、读写文件、工作区互传、Python、系统信息、打开、重启、状态共十个 `unkernel_*` 工具；启动器内置 **DSH go**——24 MiB 的 Go 单文件 DeepSeek Harness，自带中文 REPL 和可用的 `bash` 工具，不需要 Node.js 和几百 MB 的运行时；NanoVM / WebVM 新增 `fastfetch`；
- **聊天模式**：仅聊天 / 网页 Agent / PTC / 创造，决定下发哪些客户端工具与往返上限；
- **关键词拒答**：设置 → 模型里配置关键词、大小写与自定义话术，命中用户输入时本地直接拒答，不请求接口，未填 API Key 也能触发；
- **统计与计费**：用量账本、上下文分类、按模型计费、服务端搜索免费开关；
- **安卓端**：APK 构建、应用内更新检测、NanoVM 联网内置代理；WebVM 因 Android WebView 不支持跨源隔离而只能在浏览器里运行，Unkernel 在 WebView 支持 JSPI 时可直接使用；
- **桌面端**：Tauri 2 壳（`pnpm tauri build`），开一个窗口加载同一份构建产物，不新增功能。

## 文档导航

README 已经拆成 `docs/zh/` 下的分册，首页只保留简介与入口；英文文档在 [`docs/en/`](docs/en/README.md)。

| 章节                  | 内容                                                    | 文件                                                   |
| --------------------- | ------------------------------------------------------- | ------------------------------------------------------ |
| 1. 这是什么           | 项目定位、原版功能与本仓库新增的能力清单                | [01-what-is-this.md](docs/zh/01-what-is-this.md)       |
| 2. 原版做了哪些限制   | 请求侧与界面侧被写死的地方                              | [02-original-limits.md](docs/zh/02-original-limits.md) |
| 3. 本仓库做了哪些改动 | 3.1–3.39：设置页每一组功能、NanoVM Slim、WebVM、Unkernel、聊天模式的细节 | [03-changes.md](docs/zh/03-changes.md)                 |
| 4. 行为对照表         | 原版 / 默认 / 越狱模式逐项对比                          | [04-behaviour.md](docs/zh/04-behaviour.md)             |
| 5. 设置项速查         | 每一项的默认值与 localStorage 键名                      | [05-settings.md](docs/zh/05-settings.md)               |
| 6. 请求体实际内容     | 实际发出去的 JSON 长什么样                              | [06-request.md](docs/zh/06-request.md)                 |
| 7. 实现位置索引       | 每个能力对应的源码文件                                  | [07-implementation.md](docs/zh/07-implementation.md)   |
| 8. 数据、隐私与网络   | 数据都存在哪、请求发往哪里                              | [08-privacy.md](docs/zh/08-privacy.md)                 |
| 9. 运行与构建         | 开发、构建、Android APK 与 Tauri 桌面打包               | [09-build.md](docs/zh/09-build.md)                     |
| 10. 已知限制          | 86 条边界与取舍                                         | [10-known-limits.md](docs/zh/10-known-limits.md)       |
| 11. 许可与致谢        | MIT 许可与上游                                          | [11-license.md](docs/zh/11-license.md)                 |

## 快速开始

```sh
pnpm install
pnpm dev                       # 本地开发（http://localhost:8090）
pnpm build                     # 构建静态产物到 dist/
pnpm tauri dev                 # 桌面壳开发模式（窗口 + 热更新）
pnpm tauri build               # 打包桌面安装包，见 9.2
KS_PASS=<keystore密码> ./构建APK.sh   # 构建并签名 Android APK
```

完整说明（环境变量、签名、字体与离线细节）见 [9. 运行与构建](docs/zh/09-build.md)。

预构建产物：[GitHub Releases](https://github.com/xiaohuangbo/chat-demo-unrestricted/releases) — `deepseek-unrestricted-2.7.11.apk`（112 MB，versionCode 281，
SHA-256 `a816e584883066dc1d3b496aa183c033260c46eced3c582309b936dacf71f5da`），详情见 [android/README.md](android/README.md)。APK 作为 Release 资产分发，不进 Git（GitHub 拒绝超过 100 MB 的文件）。
注意：Android WebView 没有 WebGPU，浏览器内 WebLLM 模型只能在桌面浏览器使用；APK 内请改用 MLC-LLM API。

## 兼容性

最近一次兼容性回归（2026-11，v2.7.11，`compat` / `compat-prod` 两套矩阵）：

| 范围            | 覆盖                                                       | 结果  |
| --------------- | ---------------------------------------------------------- | ----- |
| 浏览器          | Chromium、Firefox                                          | 13/13 |
| 视口            | 320 / 390 / 768 / 1024 / 1440 / 1920                       | 13/13 |
| 主题与语言      | 明 / 暗、zh-CN / en-US                                     | 13/13 |
| 生产静态构建    | `dist/` 产物跑同一套用例                                   | 13/13 |
| NanoVM / 工具链 | 终端、配额、工具调用、11 条工具链、虚拟网络                | 21/21 |
| WebVM / CheerpX | 跨源隔离、启动、xterm 输入、滚动、导入导出、切换、Agent 工具 | 通过  |
| 关键词拒答      | 开关、大小写、无 Key 本地回复、不请求接口                   | 通过  |
| 聊天模式        | 仅聊天 / 网页 Agent / PTC / 创造 的工具分组与往返上限      | 10/10 |
| 移动端布局      | 320 / 360 / 390 / 414 / 599 / 767 与桌面 768 / 1024 / 1440 | 49/49 |
| 安卓 APK        | Android 14 WebView（320dp）：启动、终端、虚拟网络、控制台；WebVM 受 WebView 跨源隔离限制 | 通过  |
| Unkernel Linux  | iframe 启动、Alpine / Debian shell、2 CPU SMP、工作区挂载/写回、文件导入/导出、窗口缩放、设置页 | 通过  |

浏览器与桌面/移动端的边界、已知取舍见 [10. 已知限制](docs/zh/10-known-limits.md)。

## 最近版本

- **2.7.11**：修好附件「token 过大」的真正原因——请求里只要带 `tools` / `safety_check` / `title` 任一 DeepSeek 扩展字段，服务端就把用户输入限制在 32768 token；本轮输入超过 32768 token 时自动去掉这三项、走纯 Anthropic 兼容路径直读附件（上限 1M），估算偏小被拒时也会去掉后自动重试一次；同时首轮大输入改用提问前 24 字作标题、用量照常记账。

- **2.7.10**：设置 → 附件新增「大文本附件自动处理」——超过 32768 token 的纯文本附件选择时自动拆成 `.partN` 分片（每片仍不超过 32768 token）分别上传、一起发送，另有实验性开关把大文本直接拼进本轮提问、占用模型的 1M 上下文；设置 → 数据管理 → 导入历史对话新增进度条，显示已处理条目 / 总条目、已导入对话数与百分比。

- **2.7.9**：修复手机网页端 WebVM 打开输入法时漂移；移植 Unkernel Linux（真正编译成 WebAssembly 的 Linux 7.x 内核，Alpine 3.24 / Debian 13.7，支持 SMP 多 CPU、Debian 默认 2 GiB 内存），支持联网、声音、显示、持久磁盘、导入导出、全屏页面，页面界面已全面汉化；工作区挂到 `/mnt/workspace`，设置页新增文件导入/下载/保存到工作区与重启虚拟机，Agent 工具扩展到十个，数据盘可在设置里扩容或一键重置，启动器内置 **DSH go**（24 MiB 静态 Go 二进制、中文 REPL 与 `bash` 工具，取代原来的 DSH + dsh-TUI 安装入口），NanoVM / WebVM 新增 `fastfetch`；桌面右侧虚拟机面板与左侧侧边栏同样滑入，主页顶部下拉可进入 Unkernel。

- **2.7.8**：修复 DeepSeek V4 Pro 调用报错——请求不再下发服务端联网工具与联网标题，`latest_reminder` 折进 system，智能搜索提示改用 Flash，新会话用首问自动生成标题；README 鸣谢新增 Linux DO 社区。
- **2.7.7**：设置重组成「破甲」与「附件」两页——越狱相关的解除限制、提示词预设、自动越狱、扰动、自动调参、防撤回、跳过安全审核、拒绝后继续生成集中到「破甲」，附件投毒并入「附件」；移植 DeepChat 的记忆更新提示（气泡显示本次写入内容，可在设置 → 记忆关闭）；头像上传改为圆形取景框裁剪（拖动 / 缩放、圆外变暗、输出 384×384）；APK 输入栏的回形针换成「+」展开拍照 / 相册 / 文件，拍照直接调用系统相机；移动端侧边栏改为 DeepChat 式右滑抽屉，桌面展开带滑入动画；
- **2.7.6**：NanoVM 支持用户与 Agent 导入单个文件——本机或工作区的任意文件直接写入虚拟机，不再需要 base64 命令行，空间不足自动扩容；新增 `nanovm_import` 工具与 PyBot 工具链（内置离线快照，支持 `pybot -i` 连续对话与 `/reset`、`/exit`）；已知限制补到 80 条。
- **2.7.5**：合并上游 0.1.3（复用 Files API 图片前按 4096 长边缩放）；本地模型支持自动提炼长期记忆（可选提炼模型）、借用服务端预检索联网、读取文本附件，并限定仅聊天模式；本地「思考模式」并入输入栏「深度思考」；移植 Remember 的角色、我的资料、压缩触发阈值与账户用量概览；本地模型设置图标换成芯片；手机端自动越狱与扰动保留文字标签、不再折叠成圆；系统提示词与思维链支持多套命名预设，可新建 / 克隆 / 重命名 / 删除并一键切换；已知限制补到 78 条。
- **2.7.4**：移植 mlc-ai/web-llm-chat——设置新增「本地模型」，浏览器用 WebGPU 直接运行模型，权重在线下载并缓存，聊天栏随时切换，支持 `.mlcmodel` 模型包导入导出与自建 MLC-LLM REST 服务；本地模型 token 不计费（语音、搜索、解析照常计费）；网页小标题改为「网页里的deepseek unrestricted Chat Harness」；已知限制补到 73 条。
- **2.7.3**：修复手机端 NanoVM 面板被标签条撑高、WebVM 终端无法滚动的问题；设置 → 模型新增关键词拒答（本地匹配、无 Key 可用、不请求接口）；已知限制补到 67 条；WebVM 新增 sudo（root 密码 password）与 Root 一键切换。
- **2.7.2**：新增 WebVM / CheerpX 终端：浏览器里的 x86 Debian 10，自带 gcc / g++ / Python 3 / Node.js / Ruby / Lua / make / git / vim；与 NanoVM 同时运行、自由切换，支持导入导出、编程环境检测与 `webvm_shell` / `webvm_ls` / `webvm_status` 三个模型工具。
- **2.7.1**：虚拟机虚拟网络（`vm serve` + `serve` + `wscat`，curl 访问 127.0.0.1 自动路由）；Node 补齐 `http`/`ws`，Python 支持 `pip install` 与 Flask/WSGI；新增 tsc / Sass / Terser / Prettier 工具链与 12 条开发命令。
- **2.7.0**：手机端输入栏把思维链 / 智能搜索 / 深度思考挪到第二排并显示文字标签（桌面不变）；NanoVM Slim 新增 26 条命令与 jq 子集；设置里新增 Lua / SQLite / esbuild / make 按需工具链。
- **2.6.9**：设置 → 关于新增「检查更新」（安卓端直接给 APK 链接）；修复终端光标残影；安卓端工作区改为应用存储语义并放宽配额；README 拆分为分册。
- **2.6.8**：NanoVM Slim（WASM shell + busybox + 按需工具链）、聊天模式四档、虚拟机联网（APK 内置反向代理）、切换输入法与红色重置。
- 更早的版本见应用内 **设置 → 更新日志**。

## Star 历史

<!-- star-history:start -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/star-history/star-history-dark.svg" />
  <source media="(prefers-color-scheme: light)" srcset="assets/star-history/star-history-light.svg" />
  <img alt="Star History Chart" src="assets/star-history/star-history-light.svg" />
</picture>
<!-- star-history:end -->

## 鸣谢

- **Linux DO 社区**：<https://linux.do/> —— 感谢社区提供的交流、反馈与测试支持。
- 上游项目、参考实现与许可信息见 [11. 许可与致谢](docs/zh/11-license.md)。

## 许可

MIT，见 [11. 许可与致谢](docs/zh/11-license.md)。
