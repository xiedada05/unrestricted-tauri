<p align="center">
  <img src="./brand.png" alt="A blue-haired maid holds a whale icon and a padlock icon; the caption reads that countless geniuses want to prove themselves under the whale, while under the padlock genius is only the entry threshold." width="49%"><img src="./brand_2.jpg" alt="Give up every client app in the world and try the strongest adversarial play; this is your only chance to duel with a top open-source model." width="49%">
</p>

<h1 align="center">Deepseek Unrestricted Chat Harness</h1>

<p align="center">
  Deepseek Unrestricted Chat Harness — a static DeepSeek chat front end with an unrestricted mode, editable replies, and direct control over model parameters<br>
  English | <a href="./README.zh-CN.md">简体中文</a>
</p>

> This is an **unofficial fork** of the official demo [deepseek-ai/chat-demo](https://github.com/deepseek-ai/chat-demo), MIT licensed.
> Every change lives in the browser: how the request body is assembled, how replies are stored, and how they are rendered.
> There is no proxy and no relay service. Your API key and conversations stay in your own browser.
>
> 🔗Live demo: <https://chat-unrestricted.fur0.com/>
>
> 📦Repository: <https://github.com/xiaohuangbo/chat-demo-unrestricted>
>
> 🎥Intro video: <https://www.bilibili.com/video/BV1m3s66WESg/>
>
> 📚Reference paper: <https://arxiv.org/pdf/2403.12171>

<p align="center">
  <img src="./Demo%20Picture.png" alt="Chat demo — technical Q&A in unrestricted mode" width="860">
</p>

---

## What this fork adds

- **Unrestricted mode and prompt engineering**: jailbreak mode, system-prompt presets, per-round injection, Chinese jailbreak prompts and custom chain of thought;
- **Browser agent tools** (off by default, per-tool switches): date & time, location, weather, notifications, clipboard, page fetch, file generation, open link and visualization widgets;
- **Virtual workspace**: a local file store with any-format import/export, the OPFS file system, and app storage on Android;
- **NanoVM Slim**: a built-in WASM Shell Engine with 130 busybox commands, single-file import (device / workspace, zip and binaries included, `nanovm_import` agent tool) and a PyBot toolchain (640k lines of pure-Python rules, no weights), docked right on desktop and a bottom panel on phones; Python / Node / Lua / SQLite / esbuild / tsc / Sass / Terser / Prettier / make / Git install on demand, and a virtual network inside the VM runs Node and Python HTTP / WebSocket services;
- **WebVM (CheerpX)**: a second terminal running a real x86 Debian 10 with bash / gcc / g++ / Python 3 / Node.js / Ruby / Lua / make / git / vim; it runs alongside NanoVM with free switching, import/export and model tools, and keeps disk changes in an IndexedDB overlay;
- **Unkernel Linux**: a third VM running a real Linux 7.x kernel compiled to WebAssembly — Alpine 3.24 / Debian 13.7, `apk` / `apt`, bash, SMP multi-CPU, Debian defaulting to 2 GiB memory, networking over Wisp (public anura.pro / mercurywork.shop proxies by default), Web Audio sound, a framebuffer/evdev display, a persistent IndexedDB disk (resizable and re-creatable from Settings) and the full unkernel page (its UI localized to Chinese); the chat workspace is mounted at `/mnt/workspace`, and the model gets ten `unkernel_*` tools (shell, list, read/write files, workspace transfer, Python, system info, open, restart, status); the launcher runs **DSH go**, a 24 MiB single-binary Go build of DeepSeek Harness with a Chinese REPL and a working `bash` tool (no Node.js and no multi-hundred-MB runtime), and NanoVM / WebVM gain `fastfetch`;
- **Chat modes**: Chat only / Web agent / PTC / Creator decide which client tools and round limit apply;
- **Local models**: a port of mlc-ai/web-llm-chat — WebGPU models (Llama / Qwen / Gemma / Phi / DeepSeek) run in the browser, downloaded weights stay cached on device, the chat bar switches between them, `.mlcmodel` bundles import/export, and a self-hosted MLC-LLM REST endpoint is supported; local models also write long-term memories, use a search pre-pass, read text attachments and stay Chat-only;
- **Characters and account**: role cards (persona, avatar, auto memory), My profile, a compression threshold and DeepSeek balance plus usage overview, ported from Remember and wired into the existing memory and stats systems;
- **Keyword refusal**: configure keywords, case sensitivity and a custom message under Settings → Model; a match refuses locally without calling the API, even without a key;
- **Statistics and billing**: usage ledger, context breakdown, per-model pricing and the free server-search switch;
- **Android**: APK builds, in-app update check and a built-in network proxy for NanoVM; WebVM stays browser-only because Android WebView has no cross-origin isolation, while Unkernel runs in the bundled page when the WebView supports JSPI;
- **Desktop**: a Tauri 2 shell (`pnpm tauri build`) that loads the same build in one window and adds no features.

## Documentation

The README is now split into chapters under `docs/en/`; this page keeps the intro and the index. Chinese docs live in [`docs/zh/`](docs/zh/README.md).

| Chapter                         | Contents                                                             | File                                                   |
| ------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------ |
| 1. What this is                 | Scope, original features and everything this fork adds               | [01-what-is-this.md](docs/en/01-what-is-this.md)       |
| 2. Restrictions in the original | What was hard-coded on the request and UI side                       | [02-original-limits.md](docs/en/02-original-limits.md) |
| 3. What this fork changes       | 3.1–3.39: every settings group, NanoVM Slim, WebVM, Unkernel and chat modes in detail | [03-changes.md](docs/en/03-changes.md)                 |
| 4. Behaviour matrix             | Original / default / unrestricted mode, side by side                 | [04-behaviour.md](docs/en/04-behaviour.md)             |
| 5. Settings reference           | Defaults and localStorage keys for every setting                     | [05-settings.md](docs/en/05-settings.md)               |
| 6. What the request contains    | The JSON that actually goes out                                      | [06-request.md](docs/en/06-request.md)                 |
| 7. Implementation index         | Which source file implements what                                    | [07-implementation.md](docs/en/07-implementation.md)   |
| 8. Data, privacy and network    | Where data lives and where requests go                               | [08-privacy.md](docs/en/08-privacy.md)                 |
| 9. Run and build                | Development, production build, the Android APK and the Tauri desktop packages | [09-build.md](docs/en/09-build.md)                     |
| 10. Known limitations           | The 86 boundaries and trade-offs                                     | [10-known-limits.md](docs/en/10-known-limits.md)       |
| 11. License and credits         | MIT and upstream                                                     | [11-license.md](docs/en/11-license.md)                 |

## Quick start

```sh
pnpm install
pnpm dev                       # dev server (http://localhost:8090)
pnpm build                     # static build into dist/
pnpm tauri dev                 # desktop shell in development (window + live reload)
pnpm tauri build               # package the desktop installers, see 9.2
KS_PASS=<keystore-password> ./构建APK.sh   # build and sign the Android APK
```

Full details (env vars, signing, fonts, offline behaviour) are in [9. Run and build](docs/en/09-build.md).

Prebuilt artifact: [GitHub Releases](https://github.com/xiaohuangbo/chat-demo-unrestricted/releases) — `deepseek-unrestricted-2.7.11.apk` (112 MB, versionCode 281,
SHA-256 `a816e584883066dc1d3b496aa183c033260c46eced3c582309b936dacf71f5da`); see [android/README.md](android/README.md). The APK is a Release asset, not a Git blob (GitHub rejects files over 100 MB).
Note: Android WebView has no WebGPU, so in-browser WebLLM models only work in a desktop browser; inside the APK use the MLC-LLM API.

## Compatibility

Last regression run (2026-11, v2.7.11, both `compat` and `compat-prod` matrices):

| Area             | Coverage                                                 | Result |
| ---------------- | -------------------------------------------------------- | ------ |
| Browsers         | Chromium, Firefox                                        | 13/13  |
| Viewports        | 320 / 390 / 768 / 1024 / 1440 / 1920                     | 13/13  |
| Theme and locale | Light / dark, zh-CN / en-US                              | 13/13  |
| Production build | The same suite against the static `dist/` output         | 13/13  |
| NanoVM           | Terminal, quotas, tool calls, 11 toolchains, virtual net | 21/21  |
| WebVM / CheerpX  | Cross-origin isolation, boot, xterm input, scrolling, import/export, switching, agent tools | pass |
| Keyword refusal | Switch, case sensitivity, keyless local reply, no API request | pass |
| Local models    | WebGPU on Intel Arc: Qwen2.5-0.5B q4f16 downloaded (277 MB in 57 s) and answered at ~21 tok/s; 277 MB bundle exported and imported into a fresh profile, then ran offline with zero cloud API calls | pass |
| Local extras    | Text attachment read into the prompt, server search pre-pass injected, role prompt applied, Chat-only mode enforced, local memory extraction with zero Flash calls | pass |
| Characters / account | Character create/edit/select, profile name, compression threshold, balance query (`/user/balance`) and stats shortcut | pass |
| Upstream 0.1.3  | Oversized upload image resize and stale `file_id` re-upload path | pass (typecheck + build) |
| Chat modes       | Tool groups and round limits for all four modes          | 10/10  |
| Mobile layout    | 320 / 360 / 390 / 414 / 599 / 767 plus 768 / 1024 / 1440 | 49/49  |
| Android APK      | Android 14 WebView (320dp): boot, terminal, virtual net; WebVM is limited by WebView COI | pass |
| Unkernel Linux   | iframe boot, Alpine + Debian shell, 2-CPU SMP, workspace mount / write-back, file import/export, resizing, settings | pass |

Boundaries and trade-offs are listed in [10. Known limitations](docs/en/10-known-limits.md).

## Recent versions

- **2.7.11**: fixes the real cause of the "token too large" attachment failure — any DeepSeek extension field (`tools` / `safety_check` / `title`) makes the server cap user input at 32768 tokens; turns above that now drop the three fields and read attachments through the plain Anthropic-compatible path up to 1M, retrying once without them if the estimate was too low. A first large turn falls back to the first 24 characters as the title and usage is still recorded.

- **2.7.10**: Settings → Attachments gains "Handle large text attachments": plain-text files above 32768 tokens are split into `.partN` parts under the same limit at pick time and sent together, plus an experimental switch that pastes the text straight into the turn using the model 1M context; Settings → Data → Import chat history now shows a progress bar with processed / total entries, imported chats and the percentage.

- **2.7.9**: fixes mobile WebVM drift when the soft keyboard opens; ports Unkernel Linux (real Linux 7.x kernel in WebAssembly, Alpine 3.24 and Debian 13.7, SMP multi-CPU, Debian defaulting to 2 GiB memory) with networking, sound, display, persistent disk, import/export, a full-screen page, a `/mnt/workspace` mount shared with the chat workspace, a fully Chinese page UI, file import/export and Restart in a separate Chinese settings section, ten `unkernel_*` Agent tools, a data disk that Settings can grow or reset, a **DSH go** launcher demo (a 24 MiB static Go binary with a Chinese REPL and a `bash` tool, replacing the former DSH + dsh-TUI installer) and `fastfetch` in NanoVM / WebVM; the desktop right-hand VM dock slides in like the left sidebar, and pulling down at the top of the main page opens Unkernel.

- **2.7.8**: fixes DeepSeek V4 Pro requests — server-side web tools and cloud titles are no longer sent, `latest_reminder` is folded into system, the search toggle points users to Flash and new sessions take their title from the first question; the README acknowledgements now include the Linux DO community.
- **2.7.7**: settings are regrouped into Jailbreak and Attachments — unrestricted mode, prompt presets, auto jailbreak, obfuscation, AutoTune, anti-retraction, skip safety check and continue-after-refusal now live on Jailbreak, and attachment poisoning moves under Attachments; DeepChat memory-update notices are ported (a toast shows what was written, switchable under Settings → Memory); avatar upload opens a circular crop dialog (drag / zoom, dimmed outside the circle, 384×384 output); inside the APK the composer paperclip becomes a + tray for camera / photos / files, and the camera opens the system camera app; the mobile sidebar becomes a DeepChat-style swipe drawer, with a slide-in animation on desktop;
- **2.7.6**: NanoVM gains single-file import for users and the agent — write any file from this device or the workspace straight into the VM without base64 shell commands, with automatic disk growth; new `nanovm_import` tool; PyBot toolchain (pure-Python rule chatbot) installs offline from a bundled snapshot and supports an interactive `pybot -i` chat with `/reset` and `/exit`; known limits now 80.
- **2.7.5**: merges upstream 0.1.3 (resize oversized Files API images before reuse); local models can extract long-term memories (with a selectable extraction model), borrow a server-side search pre-pass, read text attachments and are restricted to Chat mode; local thinking merges into the composer’s Deep thinking control; ports Remember’s characters, My profile, compression threshold and account/usage overview; the local model settings icon becomes a chip; on mobile, auto-jailbreak and input perturbation keep their text labels instead of collapsing into circles; system prompts and chain-of-thought gain multiple named presets with create / clone / rename / delete and one-click switching; known limits now 78.
- **2.7.4**: ports mlc-ai/web-llm-chat — Settings → Local models runs WebGPU models in the browser, downloads and caches weights, switches from the chat bar, imports/exports `.mlcmodel` bundles and talks to an MLC-LLM REST server; local tokens are free (speech, search and parsing still billed); the page title is now “网页里的deepseek unrestricted Chat Harness”; known limits now 73.
- **2.7.3**: fixed the stretched NanoVM panel on mobile and the unscrollable WebVM terminal; added keyword refusal under Settings → Model (local match, works without a key, no API request); known limits now 67; WebVM gains sudo (root password password) and a one-click Root switch.
- **2.7.2**: new WebVM / CheerpX terminal — a real x86 Debian 10 in the browser with gcc / g++ / Python 3 / Node.js / Ruby / Lua / make / git / vim; it runs alongside NanoVM with free switching, import/export, an environment check and the `webvm_shell` / `webvm_ls` / `webvm_status` model tools.
- **2.7.1**: a virtual network inside the VM (`vm serve` + `serve` + `wscat`, curl to 127.0.0.1 routes automatically); Node gains `http`/`ws`, Python gains `pip install` and Flask/WSGI; new tsc / Sass / Terser / Prettier toolchains and 12 developer commands.
- **2.7.0**: the mobile input bar moves chain-of-thought / search / deep thinking to a second row with visible labels (desktop unchanged); 26 new NanoVM commands plus a jq subset; Lua / SQLite / esbuild / make toolchains in settings.
- **2.6.9**: “Check for updates” in Settings → About (hands Android users the APK link); fixed ghost cursors in the terminal; workspace storage on Android now means app storage with larger quotas; README split into chapters.
- **2.6.8**: NanoVM Slim (WASM shell + busybox + on-demand toolchains), four chat modes, VM networking (built-in reverse proxy in the APK), keyboard switching and the red reset.
- Older entries live in the in-app changelog (Settings → Changelog).

## Star History

<!-- star-history:start -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/star-history/star-history-dark.svg" />
  <source media="(prefers-color-scheme: light)" srcset="assets/star-history/star-history-light.svg" />
  <img alt="Star History Chart" src="assets/star-history/star-history-light.svg" />
</picture>
<!-- star-history:end -->

## Acknowledgements

- **Linux DO community**: <https://linux.do/> — thanks for the discussion, feedback and testing support.
- Upstream projects, reference implementations and licensing details are listed in [11. License and credits](docs/en/11-license.md).

## License

MIT — see [11. License and credits](docs/en/11-license.md).
