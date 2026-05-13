//! 通过 Tauri WebView 绕过 B站 API 签名获取数据
//!
//! B站 manga API 2026年新增 ultra_sign/m2 签名，非浏览器环境无法计算。
//! 使用 Tauri WebView2（等同 Chrome），B站前端 JS 自行处理签名。
//! 我们注入脚本拦截 API 响应并写入 document.title，Rust 端轮询读取。

use crate::responses::BiliResp;
use crate::types::Comic;
use anyhow::{anyhow, Context};
use std::time::Duration;
use tauri::{AppHandle, Url, WebviewUrl, WebviewWindowBuilder};

/// 注入脚本：拦截 fetch/XHR，将匹配 API 的响应写入全局变量
fn interceptor_script(api_name: &str) -> String {
    format!(
        r#"
(function() {{
    const KEY = '__scraped_{api_name}__';
    // 拦截 fetch
    window._origFetch = window.fetch;
    window.fetch = function(...args) {{
        const urlStr = typeof args[0] === 'string' ? args[0] : (args[0]?.url || '');
        if (urlStr.includes('{api_name}')) {{
            return window._origFetch.apply(this, args).then(r => {{
                if (r.ok) {{ r.clone().text().then(t => {{ window[KEY] = t; }}); }}
                return r;
            }});
        }}
        return window._origFetch.apply(this, args);
    }};
    // 拦截 XHR
    const origOpen = XMLHttpRequest.prototype.open;
    const origSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.open = function(method, url) {{
        this.__url = url; return origOpen.apply(this, arguments);
    }};
    XMLHttpRequest.prototype.send = function(body) {{
        if (this.__url && this.__url.includes('{api_name}')) {{
            this.addEventListener('load', function() {{
                if (this.status === 200) {{ window[KEY] = this.responseText; }}
            }});
        }}
        return origSend.apply(this, arguments);
    }};
}})();
"#
    )
}

/// 创建隐藏 WebView 加载页面，等待注入脚本捕获到数据
fn scrape_api(app: &AppHandle, url: &str, api_name: &str, timeout_secs: u64) -> anyhow::Result<String> {
    let parsed_url = Url::parse(url).context("解析URL失败")?;
    let label = format!("scraper_{}", rand::random::<u32>());
    let init_script = interceptor_script(api_name);

    // 轮询脚本：检查数据并写入 title
    let key = format!("__scraped_{api_name}__");
    let poll_script = format!(
        "setInterval(function(){{ var d=window['{key}']; if(d){{ document.title='SCRAPED:'+d; }} }}, 300);"
    );
    let full_script = format!("{}\n{}", init_script, poll_script);

    let webview = WebviewWindowBuilder::new(app, &label, WebviewUrl::External(parsed_url))
        .title("")
        .inner_size(1.0, 1.0)
        .visible(false)
        .initialization_script(&full_script)
        .build()
        .context("创建WebView失败")?;

    let start = std::time::Instant::now();
    let result = loop {
        if start.elapsed() > Duration::from_secs(timeout_secs) {
            break Err(anyhow!("操作超时（{}秒）", timeout_secs));
        }
        if let Ok(title) = webview.title() {
            if title.starts_with("SCRAPED:") {
                let json_str = title[8..].to_string();
                break Ok(json_str);
            }
        }
        std::thread::sleep(Duration::from_millis(300));
    };

    let _ = webview.close();
    result
}

pub fn get_comic_via_webview(app: &AppHandle, comic_id: i64) -> anyhow::Result<Comic> {
    let url = format!("https://manga.bilibili.com/detail/mc{comic_id}");
    let json_str = scrape_api(app, &url, "ComicDetail", 60)?;

    let bili_resp: BiliResp = serde_json::from_str(&json_str)?;
    if bili_resp.code != 0 {
        return Err(anyhow!("ComicDetail错误: code={} msg={}", bili_resp.code, bili_resp.msg));
    }
    let data = bili_resp.data.context("data为空")?;
    let comic_data: crate::responses::ComicRespData = serde_json::from_value(data)?;
    Ok(Comic::from(app, comic_data))
}

use crate::responses::SearchRespData;
pub fn search_via_webview(app: &AppHandle, keyword: &str, page_num: i64) -> anyhow::Result<SearchRespData> {
    let encoded: String = percent_encoding::utf8_percent_encode(keyword, percent_encoding::NON_ALPHANUMERIC).collect();
    let url = format!("https://manga.bilibili.com/search?keyword={encoded}&page={page_num}");
    let json_str = scrape_api(app, &url, "Search", 30)?;

    let bili_resp: BiliResp = serde_json::from_str(&json_str)?;
    if bili_resp.code != 0 {
        return Err(anyhow!("Search错误: code={} msg={}", bili_resp.code, bili_resp.msg));
    }
    let data = bili_resp.data.context("data为空")?;
    Ok(serde_json::from_value(data)?)
}
