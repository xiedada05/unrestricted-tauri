## 9. Run and build

**On Windows**, double-click the two scripts in the repository root:

- `启动chat-web前端.bat`: detects Node.js / pnpm / `node_modules` and installs whatever is missing (winget / corepack / npm), then starts the dev server on http://localhost:8090;
- `启动mcp后端越狱实验台.bat`: detects Python / `.venv` / MCP deps / fastchat / torch / datasets / EasyJailbreak; EasyJailbreak is installed via pip first and falls back to GitHub or mirror clones, then starts the MCP backend on http://127.0.0.1:8765/mcp.

Manual setup (any OS), requires Node.js 22.12+ and pnpm:

```sh
git clone https://github.com/xiaohuangbo/chat-demo-unrestricted.git
cd chat-demo-unrestricted
pnpm install
pnpm dev        # http://localhost:8090
```

Build the static site:

```sh
pnpm build      # output in dist/
```
Local models bundle `@mlc-ai/web-llm@0.2.87` into the build and emit an extra Web Worker chunk (~6 MB); model weights and wasm libraries are not in the bundle and are downloaded from Hugging Face / GitHub Raw on first use into browser storage.

Local models must run on an http(s) page: under `file://` both the Worker and WebGPU are restricted, and an MLC-LLM API endpoint must allow cross-origin requests.


`dist/` can be deployed to any static host, or opened locally by double-clicking `dist/index.html` (the browser must allow storage on `file://` pages and cross-origin requests to your API origin). Enter your API key under Settings → Data before the first chat.

| Command       | Purpose                      |
| ------------- | ---------------------------- |
| `pnpm dev`    | Start the dev server on 8090 |
| `pnpm build`  | Build into `dist/`           |
| `pnpm type`   | TypeScript type check        |
| `pnpm lint`   | Lint with oxlint             |
| `pnpm format` | Format with oxfmt            |

The API origin is set in [`rspack.config.ts`](../../rspack.config.ts) via `createConfig({ apiOrigin })`; it defaults to `https://api.deepseek.com`.

WebVM (CheerpX) needs cross-origin isolation. On static hosting such as GitHub Pages, where response headers cannot be customised, the first WebVM enable registers `coi-sw.js` and reloads once, after which the service worker adds the headers to same-origin responses. **Android WebView does not support cross-origin isolation (Chromium issue 40914606), so WebVM cannot run inside the APK; use Chrome on the phone or a desktop browser for the online version.**

Unkernel does not need cross-origin isolation in the default main-thread mode; multiple CPUs do, and a static site registers `coi-sw.js` and reloads once. Its full page, UP/SMP kernel wasm and the Alpine image are copied from `public/unkernel/` into `dist/unkernel/`; the Android asset server answers `Range` requests for the 3.9 MiB `alpine.erofs`. It needs JSPI, so an old WebView shows a boot error instead of a terminal.

When rebuilding `public/unkernel/`, keep the vendored `dropLegacyScope` check: Unkernel's boot drops the legacy root-scope service worker, and the page must only unregister `/vm-sw.js`, never the host's `coi-sw.js`; otherwise the multi-CPU iframe reload loses COEP and Firefox blocks it with `NS_ERROR_DOM_COEP_FAILED`. You can also configure `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: credentialless` directly in Nginx / Vercel and skip that reload.

DSH go in the launcher is built from `cmd/dsh-go` in the `go-deepseek` checkout (a 24 MiB static aarch64 Go binary) and checked in at `public/unkernel/demos/dsh-go/dsh-go-linux-arm64`; `web/src/demos/dsh.ts` pins its SHA-256 and byte size, so a rebuild must update both or the page refuses to push the file into the guest.

The built page assumes it is deployed at the origin root: distro images point at `/distros/`, the preview Service Worker registers `/vm-sw.js` / `/vm/`, and the manifest scope is `/`. The app serves the page under `/unkernel/`, so after syncing into `public/unkernel/` run `pnpm patch:unkernel` (or let `pnpm build` do it); the script prefixes the paths and validates the distro URLs, Service Worker and manifest, so a missed rewrite cannot turn into a 404 at boot.

### 9.1 Android APK

A native WebView shell is included; the built artifact is `android/deepseek-unrestricted-2.7.11.apk` (about 112 MB — over GitHub's 100 MB blob limit, so it is gitignored and uploaded as a Release asset):

- The app starts a local static server on `http://127.0.0.1:18765` and loads `assets/public`, giving localStorage / IndexedDB / fetch a stable secure origin;
- File selection, camera capture (system camera plus a content:// hand-off), microphone permission, the back key and external links are wired up;
- Building requires Android SDK 34, build-tools 35.0.1+ (build-tools 34.0.0's d8 can crash on JDK 21 bytecode) and JDK 21/17. A one-click script at the repo root reads the `package.json` version (currently 2.7.9 / 279), builds the web app and the signed APK:
  ```sh
  KS_PASS=... ./构建APK.sh
  ```
  Or run `android/build-apk.sh` directly (interactive by default: version, keystore, output, whether to build the web app or install):
  ```sh
  export ANDROID_HOME=$HOME/android-sdk
  export JAVA_HOME=/usr/lib/jvm/java-21-openjdk
  ./android/build-apk.sh
  # non-interactive: KS_PASS=... ./android/build-apk.sh --yes --no-build-web
  ```
- The APK is signed with `xiaohuangbo.jks`, alias `xiaohuangbo`, versionCode 281;
- **The Python backend is not executed inside the APK**: `android/assets/backend` bundles the `mcp-server` and EasyJailbreak sources for reference only; the lab still needs a backend on a PC or server, with its endpoint set to `http://<PC-IP>:8765/mcp`. The phone page origin is `http://127.0.0.1:18765`, which the backend already allows.

### 9.2 Desktop client (Tauri)

`src-tauri/` is a Tauri 2 desktop shell: it opens one window that loads the same `dist/`, adds no chat features and exposes no Tauri API to the frontend.

```sh
pnpm tauri dev     # development: starts pnpm dev, then opens the desktop window with live reload
pnpm tauri build   # packaging: runs pnpm build first; installers land in src-tauri/target/release/bundle/
```

- The app version follows `package.json` (`version` in `tauri.conf.json` points to it), so there is nothing to maintain separately;
- The page origin is `tauri://localhost` (Linux / macOS) or `http://tauri.localhost` (Windows), and the dev server under `tauri dev`. The API key and conversations live in the webview's own localStorage / IndexedDB, separate from the online version's data in a browser, so the API key has to be entered again on first use;
- The shell only handles four things:
  - **External links**: any off-app address goes to the system browser; the app window only loads the app's own addresses (including its `blob:` / `data:` previews, which open in a child window for image / PDF attachments);
  - **Downloads**: Linux and Windows use the webview's built-in flow and save straight into the Downloads folder; WKWebView drops downloads by default, so on macOS the shell catches them and saves to the same place;
  - **Permissions**: microphone, clipboard read, notifications and geolocation are granted without a prompt; everything else keeps the platform default;
  - **Relaunch**: a second launch opens no second window and just brings the running one to the front — all frontend data lives in one webview store, and two processes writing to it would overwrite each other;
- `app.security.headers` sets the same COOP / COEP values as `coi-sw.js`, so the desktop client never registers the service worker. Those headers only take effect on WebView2 (Windows); WebKit does not make `tauri://` cross-origin isolated;
- Icons are generated from `src-tauri/app-icon.png`: `pnpm tauri icon src-tauri/app-icon.png -o src-tauri/icons`;
- On Linux the package name and the desktop entry file name come from the product name, while the window's app_id / WM class comes from the executable name `deepseek-unrestricted`; the taskbar can only link a window to its launcher when the two agree. So `tauri.linux.conf.json` changes the product name to `deepseek-unrestricted` on Linux only, and `src-tauri/linux/desktop.hbs` restores the display name `DeepSeek Unrestricted`; Windows / macOS keep `DeepSeek Unrestricted`.

Building on Linux needs `libwebkit2gtk-4.1-dev` and `patchelf`:

```sh
sudo apt install libwebkit2gtk-4.1-dev patchelf
```

There are three Linux packages: deb and rpm are about 104 MiB each and rely on the system's WebKitGTK; the AppImage is about 208 MiB because it bundles the build machine's WebKitGTK libraries, so it runs on machines where WebKitGTK is missing or has a different version.

For the Windows and macOS prerequisites (WebView2 Runtime, Xcode Command Line Tools and so on), see the [Tauri documentation](https://v2.tauri.app/start/prerequisites/).

---

---

[← 返回 README](../../README.md) · [全部文档](README.md)
