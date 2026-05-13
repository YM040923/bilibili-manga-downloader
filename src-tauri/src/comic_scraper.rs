//! 通过 Tauri WebView 绕过 B站 ultra_sign 签名获取漫画数据
//!
//! 原理：B站漫画 API 添加了 ultra_sign 签名，非浏览器环境无法计算。
//! Tauri 自带的 WebView 使用系统的 WebView2（Windows），行为等同于 Chrome/Edge，
//! B站前端 JS 可以正常计算签名。我们注入脚本拦截 API 响应。

use crate::responses::BiliResp;
use crate::types::Comic;
use anyhow::{anyhow, Context};
use std::time::Duration;
use tauri::{AppHandle, Url, WebviewUrl, WebviewWindowBuilder};

pub fn get_comic_via_webview(app: &AppHandle, comic_id: i64) -> anyhow::Result<Comic> {
    let url_str = format!("https://manga.bilibili.com/detail/mc{comic_id}");
    let url = Url::parse(&url_str).context("解析漫画详情URL失败")?;

    // 注入脚本：在页面加载前拦截 fetch 和 XHR，捕获 ComicDetail 响应
    let init_script = r#"
(function() {
    // 拦截 fetch
    const origFetch = window.fetch;
    window.fetch = function(...args) {
        const resource = args[0];
        const urlStr = typeof resource === 'string' ? resource : (resource.url || '');
        if (urlStr.includes('ComicDetail')) {
            return origFetch.apply(this, args).then(r => {
                if (r.ok) {
                    const cloned = r.clone();
                    cloned.text().then(t => { window.__comic_detail_data__ = t; });
                }
                return r;
            });
        }
        return origFetch.apply(this, args);
    };
    // 拦截 XMLHttpRequest
    const origOpen = XMLHttpRequest.prototype.open;
    const origSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.open = function(method, url) {
        this.__url = url;
        return origOpen.apply(this, arguments);
    };
    XMLHttpRequest.prototype.send = function(body) {
        if (this.__url && this.__url.includes('ComicDetail')) {
            this.addEventListener('load', function() {
                if (this.status === 200) {
                    window.__comic_detail_data__ = this.responseText;
                }
            });
        }
        return origSend.apply(this, arguments);
    };
})();
"#;

    // 创建隐藏 WebView
    let label = format!("comic_scraper_{}", comic_id);
    let webview = WebviewWindowBuilder::new(app, &label, WebviewUrl::External(url))
        .title("")
        .inner_size(1.0, 1.0)
        .visible(false)
        .initialization_script(init_script)
        .build()
        .context("创建WebView失败")?;

    // 轮询等待数据（通过 document.title 变化判断）
    let start = std::time::Instant::now();
    let result = loop {
        if start.elapsed() > Duration::from_secs(60) {
            break Err(anyhow!("获取漫画详情超时（60秒），可能Cookie已过期或网络问题"));
        }

        if let Ok(title) = webview.title() {
            if title.starts_with("SCRAPED:") {
                let json_str = &title[8..];
                break Ok(json_str.to_string());
            } else if title == "TIMEOUT" {
                break Err(anyhow!("页面已加载但未获取到ComicDetail数据，请确认已用完整Cookie登录"));
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    };

    // 清理
    let _ = webview.close();

    let json_str = result?;

    // 解析 BiliResp → ComicRespData → Comic
    let bili_resp: BiliResp =
        serde_json::from_str(&json_str).context("解析ComicDetail API响应失败")?;
    if bili_resp.code != 0 {
        return Err(anyhow!("ComicDetail API返回错误: code={} msg={}", bili_resp.code, bili_resp.msg));
    }
    let data = bili_resp.data.context("ComicDetail API返回data为空")?;
    use crate::responses::ComicRespData;
    let comic_data: ComicRespData =
        serde_json::from_value(data).context("解析ComicRespData失败（API返回格式可能已变化）")?;
    let comic = Comic::from(app, comic_data);

    Ok(comic)
}
