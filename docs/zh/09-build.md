## 9. 运行与构建

**Windows 用户**可以直接双击仓库根目录的两个脚本：

- `启动chat-web前端.bat`：自动检测 Node.js / pnpm / `node_modules`，缺失时通过 winget / corepack / npm 自动安装，然后启动 http://localhost:8090；
- `启动mcp后端越狱实验台.bat`：自动检测 Python / `.venv` / MCP 依赖 / fastchat / torch / datasets / EasyJailbreak，缺失时自动安装；EasyJailbreak 会先尝试 pip 安装，失败再尝试 GitHub 与镜像克隆，最后启动 http://127.0.0.1:8765/mcp。

手动方式（任意系统），准备 Node.js 22.12+ 与 pnpm：

```sh
git clone https://github.com/xiaohuangbo/chat-demo-unrestricted.git
cd chat-demo-unrestricted
pnpm install
pnpm dev        # http://localhost:8090
```

生成静态产物：

```sh
pnpm build      # 产物在 dist/
```
本地模型用到的 `@mlc-ai/web-llm@0.2.87` 会打进构建产物，并额外产出一个 Web Worker chunk（约 6 MB）；模型权重与 wasm 库不在包里，首次使用时从 Hugging Face / GitHub Raw 下载并写入浏览器缓存。


`dist/` 可以直接部署到静态服务器，也可以双击 `dist/index.html` 本地打开（需要浏览器允许 `file://` 页面使用存储，并允许跨域请求你的 API 地址）。首次使用在设置 → 数据管理里填写 API Key。

本地模型必须在 http(s) 页面下运行：`file://` 打开时 Worker 与 WebGPU 都受限制；MLC-LLM API 也需要目标服务允许跨域。

| 命令          | 用途                   |
| ------------- | ---------------------- |
| `pnpm dev`    | 启动开发服务器（8090） |
| `pnpm build`  | 构建到 `dist/`         |
| `pnpm type`   | TypeScript 类型检查    |
| `pnpm lint`   | oxlint 检查            |
| `pnpm format` | oxfmt 格式化           |
| `pnpm tauri dev`   | 启动桌面壳开发模式（先跑 `pnpm dev`）     |
| `pnpm tauri build` | 打包桌面安装包到 `src-tauri/target/release/bundle/` |

API 地址在 [`rspack.config.ts`](../../rspack.config.ts) 的 `createConfig({ apiOrigin })` 里配置，默认 `https://api.deepseek.com`。

WebVM（CheerpX）需要跨源隔离。浏览器版如果部署在 GitHub Pages 这类不能自定义响应头的静态托管上，第一次启用 WebVM 时会注册 `coi-sw.js` 并刷新一次，之后由 Service Worker 为同源响应补头。**Android WebView 不支持跨源隔离（Chromium issue 40914606），所以 APK 内无法运行 WebVM；请在手机 Chrome 或桌面浏览器里打开在线版。** 也可以自己在 Nginx / Vercel 上直接配置 `Cross-Origin-Opener-Policy: same-origin` 与 `Cross-Origin-Embedder-Policy: credentialless`，省掉这次刷新。

Unkernel 默认在主线程运行，不需要跨源隔离；多 CPU 需要跨源隔离，静态站点会注册 `coi-sw.js` 并刷新一次。完整页面、UP/SMP 内核 wasm 与 Alpine 镜像从 `public/unkernel/` 复制到 `dist/unkernel/`，Android 资源服务器会对 3.9 MiB 的 `alpine.erofs` 响应 `Range` 请求。它需要 JSPI，旧 WebView 会显示启动错误而不是终端。

重建 `public/unkernel/` 时注意保留 vendored 版本的 `dropLegacyScope` 检查：Unkernel 启动时会清理旧的根作用域 Service Worker，页面只能注销 `/vm-sw.js`，不能把宿主应用的 `coi-sw.js` 一起注销；否则多 CPU 的 iframe 重载会因缺少 COEP 被 Firefox 拦成 `NS_ERROR_DOM_COEP_FAILED`。

启动器里的 DSH go 是 `go-deepseek` 仓库 `cmd/dsh-go` 的构建产物（aarch64 静态 Go 二进制，约 24 MiB），放在 `public/unkernel/demos/dsh-go/dsh-go-linux-arm64`；`web/src/demos/dsh.ts` 固定了它的 SHA-256 和字节数，重新编译后要同步更新这两个值，否则页面会拒绝把文件推入 guest。

页面构建产物假设自己部署在站点根目录：发行版镜像写成 `/distros/`，预览 Service Worker 注册到 `/vm-sw.js` / `/vm/`，manifest 的 scope 是 `/`。应用把页面挂在 `/unkernel/` 下，所以同步到 `public/unkernel/` 后要运行 `pnpm patch:unkernel`（`pnpm build` 会自动执行）；脚本会统一加前缀并校验发行版 URL、Service Worker 和 manifest，漏改会在启动时 404。

### 9.1 Android APK

仓库内置一个原生 WebView 壳，APK 产物为 `android/deepseek-unrestricted-2.7.11.apk`（约 112 MB，超过 GitHub 100 MB 单文件限制，已加入 `.gitignore`，请作为 Release 资产上传）：

- 应用在 `http://127.0.0.1:18765` 启动本地静态服务器，再加载 `assets/public`，给 localStorage / IndexedDB / fetch 一个稳定的安全来源；
- 支持文件选择、拍照（系统相机 + content:// 回传）、麦克风权限、返回键和外部链接跳转；
- 构建需要 Android SDK 34、build-tools 35.0.1+（34.0.0 的 d8 在 JDK 21 下可能崩溃）与 JDK 21/17。仓库根目录提供一键脚本，会读取 `package.json` 的版本（当前 2.7.9 / 279），依次构建 Web、APK 并签名：
  ```sh
  KS_PASS=... ./构建APK.sh
  ```
  也可以直接运行 `android/build-apk.sh`（默认交互式，询问版本、签名文件、输出路径、是否构建 Web / 安装）：
  ```sh
  export ANDROID_HOME=$HOME/android-sdk
  export JAVA_HOME=/usr/lib/jvm/java-21-openjdk
  ./android/build-apk.sh
  # 非交互：KS_PASS=... ./android/build-apk.sh --yes --no-build-web
  ```
- APK 使用 `xiaohuangbo.jks`、别名 `xiaohuangbo` 签名，versionCode 281；
- **Python 后端不会在 APK 内执行**：`android/assets/backend` 只打包 `mcp-server` 与 EasyJailbreak 源码供参考；实验台仍需在电脑或服务器上运行后端，并在「实验台」里把地址改为 `http://<电脑IP>:8765/mcp`。手机页面的 Origin 是 `http://127.0.0.1:18765`，已在后端允许来源内。

### 9.2 桌面客户端（Tauri）

`src-tauri/` 是一个 Tauri 2 桌面壳：只开一个窗口加载同一份 `dist/`，不新增聊天功能，也不向前端暴露 Tauri 接口。

```sh
pnpm tauri dev     # 开发：先启动 pnpm dev，再打开桌面窗口，前端改动即时生效
pnpm tauri build   # 打包：先执行 pnpm build，安装包输出到 src-tauri/target/release/bundle/
```

- 应用版本跟随 `package.json`（`tauri.conf.json` 的 `version` 指向它），不需要单独维护；
- 页面来源是 `tauri://localhost`（Linux / macOS）或 `http://tauri.localhost`（Windows），`tauri dev` 时是开发服务器。API Key 与对话记录存在 webview 自己的 localStorage / IndexedDB 里，和浏览器里在线版的数据互不相通，首次使用要重新填写 API Key；
- 壳只处理四件事：
  - **外链**：站外地址一律交给系统浏览器，应用窗口里只加载应用自己的地址（含它的 `blob:` / `data:` 预览页；图片 / PDF 附件的预览开子窗口）；
  - **下载**：Linux 与 Windows 用 webview 自带的下载流程，直接存进「下载」目录；WKWebView 默认会丢弃下载，macOS 上由壳接住，落点相同；
  - **权限**：麦克风、剪贴板读取、通知、定位直接允许、不弹询问，其余权限沿用各平台默认行为；
  - **重复启动**：不再开第二个窗口，而是把已开着的窗口提到前台——前端的数据全在同一份 webview 存储里，两个进程同时写会互相覆盖；
- `app.security.headers` 配了和 `coi-sw.js` 相同的 COOP / COEP，所以桌面端不需要注册 Service Worker。这组头只在 WebView2（Windows）上生效，WebKit 不会为 `tauri://` 建立跨源隔离；
- 图标由 `src-tauri/app-icon.png` 生成：`pnpm tauri icon src-tauri/app-icon.png -o src-tauri/icons`；
- Linux 的包名和桌面入口文件名取自产品名，而窗口的 app_id / WM class 取自可执行文件名 `deepseek-unrestricted`，两者一致任务栏才能把窗口关联到启动器。所以 `tauri.linux.conf.json` 只在 Linux 上把产品名改成 `deepseek-unrestricted`，显示名由 `src-tauri/linux/desktop.hbs` 恢复成 `DeepSeek Unrestricted`；Windows / macOS 仍用 `DeepSeek Unrestricted`。

Linux 构建需要 `libwebkit2gtk-4.1-dev` 和 `patchelf`：

```sh
sudo apt install libwebkit2gtk-4.1-dev patchelf
```

Linux 有三种安装包：deb 和 rpm 约 104 MiB，依赖系统自带的 WebKitGTK；AppImage 约 208 MiB，把构建机上的 WebKitGTK 运行库一并打了进去，可以在没装或版本不匹配的机器上直接运行。

Windows、macOS 的前置条件（WebView2 Runtime、Xcode Command Line Tools 等）见 [Tauri 官方文档](https://v2.tauri.app/start/prerequisites/)。

---

---

[← 返回 README](../../README.md) · [全部文档](README.md)
