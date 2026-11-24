// 发布构建下不在 Windows 上额外弹一个控制台窗口。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! DeepSeek Unrestricted 的桌面壳。
//!
//! 只做这几件事：开窗口加载 `dist/`、把外链交给系统浏览器、按平台处理下载、
//! 放行前端会用到的几项浏览器权限、重复启动时复用已经开着的窗口。
//! 聊天、设置和存储全部仍由前端完成，这里不含任何业务逻辑，也没有向前端暴露 Tauri 接口。

use std::{
    io::{self, Write as _},
    sync::atomic::{AtomicU32, Ordering},
};

use tauri::{App, Manager, Url, WebviewUrl, WebviewWindowBuilder, webview::{
    NewWindowResponse, PermissionKind, PermissionResponse,
}};

const MAIN: &str = "main";

/// 预览窗口的标签序号；图片 / PDF 附件在新窗口里打开，每次都要一个新标签
static PREVIEW_SEQ: AtomicU32 = AtomicU32::new(0);

fn main() {
    tauri::Builder::default()
        // 必须最先注册：再次启动时只把已经开着的窗口提到前台。
        // 前端把 API Key 与全部对话都存在 webview 的 localStorage / IndexedDB 里，
        // 两个进程同时写同一份存储会互相覆盖。
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window(MAIN) {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            // 窗口尺寸、标题写在 tauri.conf.json 里，但 create 为 false：
            // 得在这里自己建，才能把下面这些处理器挂上去
            let config = app
                .config()
                .app
                .windows
                .iter()
                .find(|window| window.label == MAIN)
                .cloned()
                .ok_or("tauri.conf.json 里没有 main 窗口")?;
            let origin = app_origin(app);

            let nav_origin = origin.clone();
            let popup_app = app.handle().clone();
            let popup_origin = origin;
            let preview_title = config.title.clone();

            let builder = WebviewWindowBuilder::from_config(app, &config)?
                // 站外地址一律不在这个 webview 里加载，交给系统浏览器
                .on_navigation(move |url| {
                    if is_internal(url, &nav_origin) {
                        true
                    } else {
                        open_external(url);
                        false
                    }
                })
                // 聊天里的链接都带 target="_blank" / window.open：
                // 站外的同样交给系统浏览器，blob: / data: 的附件预览开子窗口
                .on_new_window(move |url, features| {
                    if !is_internal(&url, &popup_origin) {
                        open_external(&url);
                        return NewWindowResponse::Deny;
                    }
                    let label = format!("preview-{}", PREVIEW_SEQ.fetch_add(1, Ordering::Relaxed));
                    // 先给一个默认尺寸，window_features 再按 window.open 的参数覆盖
                    let opened = WebviewWindowBuilder::new(
                        &popup_app,
                        label,
                        WebviewUrl::External(url.clone()),
                    )
                    .title(preview_title.clone())
                    .inner_size(1024.0, 720.0)
                    .window_features(features)
                    .build();
                    match opened {
                        Ok(window) => NewWindowResponse::Create { window },
                        Err(error) => {
                            let _ = writeln!(io::stderr(), "预览窗口创建失败 {url}: {error}");
                            NewWindowResponse::Deny
                        }
                    }
                })
                // 不挂处理器时 Linux / macOS 的麦克风请求没有可用的授权流程，语音输入会直接失败。
                // 这里只放行前端主动用到的能力，其余保持各平台默认行为。
                .on_permission_request(|_webview, kind| match kind {
                    PermissionKind::Microphone
                    | PermissionKind::ClipboardRead
                    | PermissionKind::Geolocation
                    | PermissionKind::Notifications => PermissionResponse::Allow,
                    _ => PermissionResponse::Default,
                });

            // Linux 的 WebKitGTK 和 Windows 的 WebView2 自带下载流程（直接存进「下载」目录），
            // WKWebView 不挂处理器却会把下载直接丢掉，所以只在 macOS 上接住，落点同样是「下载」目录
            #[cfg(target_os = "macos")]
            let builder = builder.on_download(|_webview, _event| true);

            builder.build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 应用自身页面的来源：`tauri dev` 时是开发服务器，打包后 Linux / macOS 为
/// `tauri://localhost`，Windows 为 `http://tauri.localhost`。
fn app_origin(app: &App) -> String {
    match &app.config().build.dev_url {
        Some(url) if cfg!(dev) => url.origin().ascii_serialization(),
        _ if cfg!(windows) => "http://tauri.localhost".to_string(),
        _ => "tauri://localhost".to_string(),
    }
}

/// 应用自己的地址：主文档、同源资源，以及它们的 `blob:` / `data:` 预览页。
/// `tauri://` 不是标准协议，`Url::origin()` 会把它当成不透明来源，所以按字符串比前缀。
fn is_internal(url: &Url, origin: &str) -> bool {
    let text = url.as_str();
    match url.scheme() {
        "about" | "data" => true,
        "blob" => text.strip_prefix("blob:").is_some_and(|inner| has_origin(inner, origin)),
        _ => has_origin(text, origin),
    }
}

/// 来源之后只能直接接路径、查询串或片段，避免 `http://tauri.localhost.example` 冒充
fn has_origin(text: &str, origin: &str) -> bool {
    text.strip_prefix(origin).is_some_and(|rest| {
        matches!(rest.as_bytes().first(), None | Some(b'/' | b'?' | b'#'))
    })
}

/// 交给系统浏览器。只接 http(s)：其余协议由网页发起时浏览器本来就会拦下。
/// 用 `open` 库直接调系统打开器（Linux 是 xdg-open），不开进程僵尸、也不暴露 IPC
fn open_external(url: &Url) {
    if !matches!(url.scheme(), "http" | "https") {
        return;
    }
    // 写 stderr 且忽略错误：Windows 发布版没有控制台，不能让它把进程带崩
    let mut stderr = io::stderr().lock();
    if let Err(error) = open::that_detached(url.as_str()) {
        let _ = writeln!(stderr, "打开外链失败 {url}: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::{has_origin, is_internal};
    use tauri::Url;

    fn url(text: &str) -> Url {
        text.parse().expect("测试地址应当能被解析")
    }

    #[test]
    fn accepts_the_app_itself() {
        let origin = "tauri://localhost";
        for text in [
            "tauri://localhost/",
            "tauri://localhost/index.html",
            "tauri://localhost/unkernel/index.html?distro=alpine&cpus=2#x",
            "tauri://localhost/static/js/async/chunk.js",
            "blob:tauri://localhost/8c1f-4c2e",
            "data:image/png;base64,AAAA",
            "about:blank",
            "about:srcdoc",
        ] {
            assert!(is_internal(&url(text), origin), "{text}");
        }
    }

    #[test]
    fn accepts_the_windows_and_dev_origins() {
        assert!(is_internal(&url("http://tauri.localhost/index.html"), "http://tauri.localhost"));
        assert!(is_internal(&url("blob:http://tauri.localhost/1"), "http://tauri.localhost"));
        assert!(is_internal(&url("http://localhost:8090/"), "http://localhost:8090"));
        assert!(is_internal(&url("blob:http://localhost:8090/1"), "http://localhost:8090"));
    }

    #[test]
    fn rejects_everything_else() {
        let origin = "tauri://localhost";
        for text in [
            "https://example.com/",
            "http://example.com/",
            "https://tauri.localhost/",
            "tauri://localhost.evil.example/",
            "tauri://evilhost/",
            "blob:https://example.com/1",
            "blob:tauri://evilhost/1",
            "file:///etc/passwd",
            "javascript:alert(1)",
            "mailto:a@example.com",
        ] {
            assert!(!is_internal(&url(text), origin), "{text}");
        }
        assert!(!is_internal(&url("http://tauri.localhost.example.com/"), "http://tauri.localhost"));
    }

    #[test]
    fn origin_must_end_on_a_boundary() {
        assert!(has_origin("http://tauri.localhost", "http://tauri.localhost"));
        assert!(has_origin("http://tauri.localhost/?a=1", "http://tauri.localhost"));
        assert!(!has_origin("http://tauri.localhost:8080/", "http://tauri.localhost"));
        assert!(!has_origin("xhttp://tauri.localhost/", "http://tauri.localhost"));
    }
}
