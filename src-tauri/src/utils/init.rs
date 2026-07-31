use crate::command::cmds::{get_config_js, get_exe_dir, get_www_dir, load_man};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};
use tauri::{utils::config::WindowConfig, App, AppHandle, Url, WebviewUrl, WindowEvent};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Man {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub window: Option<WindowConfig>,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub icon: String,
}

impl Default for Man {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            author: String::new(),
            license: String::new(),
            window: None,
            debug: false,
            icon: String::new(),
        }
    }
}

pub fn append_param(original_url: &str, value: &str) -> String {
    let separator = if original_url.contains('?') { "&" } else { "?" };
    format!("{}{}args={}", original_url, separator, url_encode(value))
}

pub fn url_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| match b {
            b'-' | b'_' | b'.' | b'~' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                (b as char).to_string()
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
}

// 同步版本：必须在主线程上调用（WebView 创建要求主线程）
pub fn resolve_setup_sync(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let args_str = args[1..].join("|");
    let args_base64 = BASE64_STANDARD.encode(args_str.as_bytes());
    let window_json = r#"
        {
            "label": "main",
            "title": "PakePlus",
            "visible": true,
            "url": "index.html",
            "width": 1024,
            "height": 720
        }
    "#;
    let mut json_value: Value = serde_json::from_str(window_json)?;
    if !args_base64.is_empty() {
        if let Some(url) = json_value.get_mut("url") {
            if let Some(original_url) = url.as_str() {
                let new_url = append_param(original_url, args_base64.as_str());
                *url = Value::String(new_url);
            }
        }
    }
    let mut store_name = "app_data.json".to_string();
    let mut config: WindowConfig = serde_json::from_value(json_value)?;

    let startup_dir = get_exe_dir(true);
    let man_result = load_man(&startup_dir);
    let man_content = match man_result {
        Ok(content) => content,
        Err(e) => {
            println!("[PakePlus] 加载配置文件失败: {}, 使用默认配置", e);
            String::new()
        }
    };

    let mut contents = String::new();
    #[cfg(target_os = "windows")]
    let mut icon_bytes: Vec<u8> = Vec::new();

    if man_content.len() > 0 {
        println!("[PakePlus] 发现配置文件，正在解析...");
        match serde_json::from_str::<Man>(&man_content) {
            Ok(mut man_config) => {
                println!("[PakePlus] 配置解析成功: name={}, window.url={:?}", 
                         man_config.name, 
                         man_config.window.as_ref().map(|w| &w.url));

                let www_dir = get_www_dir(&startup_dir);
                let www_dir_str = match www_dir {
                    Ok(dir) => dir,
                    Err(_) => String::new(),
                };

                if let Some(ref mut window_config) = man_config.window {
                    let url = window_config.url.clone();
                    if let WebviewUrl::App(path) = url {
                        let url_str = path.to_string_lossy().to_string();
                        if url_str.starts_with("http://") || url_str.starts_with("https://") {
                            if let Ok(external_url) = Url::parse(&url_str) {
                                println!("[PakePlus] URL修复: AssetUrl -> External({})", url_str);
                                window_config.url = WebviewUrl::External(external_url);
                            }
                        }
                    }

                    window_config.label = "main".to_string();
                    window_config.visible = false;

                    store_name = format!("{}.json", man_config.name.as_str());

                    if www_dir_str.len() > 0 {
                        match Url::parse(&www_dir_str) {
                            Ok(url) => {
                                window_config.url = WebviewUrl::External(url);
                                println!("[PakePlus] 使用本地静态文件: {}", www_dir_str);
                            }
                            Err(e) => {
                                println!("[PakePlus] URL解析失败: {}", e);
                            }
                        }
                    } else {
                        println!("[PakePlus] 使用配置中的URL: {:?}", window_config.url);
                    }

                    config = window_config.clone();
                } else {
                    println!("[PakePlus] 配置中无窗口设置，使用默认配置");
                }

                match get_config_js(&startup_dir) {
                    Ok(custom_js) => {
                        contents = custom_js;
                        println!("[PakePlus] 加载自定义JS ({}字节)", contents.len());
                    }
                    Err(_) => {
                        println!("[PakePlus] 无自定义JS");
                    }
                }

                if man_config.debug {
                    contents += "var vConsole = new window.VConsole();";
                    println!("[PakePlus] 调试模式已开启");
                }

                #[cfg(target_os = "windows")]
                if man_config.icon.len() > 0 {
                    match BASE64_STANDARD.decode(man_config.icon.trim()) {
                        Ok(bytes) => {
                            icon_bytes = bytes;
                            println!("[PakePlus] 加载图标 ({}字节)", icon_bytes.len());
                        }
                        Err(e) => {
                            println!("[PakePlus] 图标解码失败: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("[PakePlus] 配置解析失败: {}, 使用默认配置", e);
            }
        }
    } else {
        println!("[PakePlus] 无配置文件，使用默认PakePlus前端");
    }

    println!("[PakePlus] 创建窗口: title={:?}, url={:?}, size={}x{}", 
             config.title, 
             config.url,
             config.width,
             config.height);

    // 使用 WebviewWindowBuilder::new 创建窗口（Tauri v2 推荐方式）
    let window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "main",
        config.url.clone(),
    )
    .title(&config.title)
    .visible(config.visible)
    .inner_size(config.width, config.height)
    .initialization_script(contents.as_str())
    .build()?;
    println!("[PakePlus] 窗口创建成功: title={:?}, visible={}", config.title, config.visible);

    // store
    let store = app_handle.store(store_name)?;
    let window_size: Option<serde_json::Value> = store.get("window_size");
    let mut width = 0.0;
    let mut height = 0.0;
    if let Some(window_size) = window_size {
        let size = window_size.as_object().unwrap();
        width = size["width"].as_f64().unwrap();
        height = size["height"].as_f64().unwrap();
    }

    #[cfg(target_os = "windows")]
    if icon_bytes.len() > 0 {
        use tauri::image::Image;
        match Image::from_bytes(&icon_bytes) {
            Ok(png_image) => {
                let _ = window.set_icon(png_image);
                println!("[PakePlus] 窗口图标已设置");
            }
            Err(e) => {
                println!("[PakePlus] 图标设置失败: {}", e);
            }
        }
    }

    let window_position: Option<serde_json::Value> = store.get("window_position");
    let mut x = 0.0;
    let mut y = 0.0;

    if let Some(window_position) = window_position {
        let position = window_position.as_object().unwrap();
        x = position["x"].as_f64().unwrap();
        y = position["y"].as_f64().unwrap();
    }

    // position
    if config.center || x <= 0.0 || y <= 0.0 {
        let _ = window.center();
    } else {
        let _ = window
            .set_position(tauri::PhysicalPosition::new(x, y));
    }

    if config.fullscreen
        || store
            .get("fullscreen")
            .unwrap_or(serde_json::Value::Bool(false))
            .as_bool()
            .unwrap()
    {
        let _ = window.set_fullscreen(true);
    } else if config.maximized
        || store
            .get("maximized")
            .unwrap_or(serde_json::Value::Bool(false))
            .as_bool()
            .unwrap()
    {
        let _ = window.maximize();
    } else if width > 0.0 && height > 0.0 {
        let _ = window
            .set_size(tauri::PhysicalSize::new(width, height));
    }

    // 延迟显示窗口，确保所有设置就绪后再 show()
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Resized(size) = event {
            if window_clone.is_maximized().unwrap_or(false) {
                let _ = store.set("maximized", true);
            } else if size.width > 0
                && size.height > 0
                && !window_clone.is_minimized().unwrap_or(false)
            {
                let _ = store.set(
                    "window_size",
                    json!({
                        "width": size.width,
                        "height": size.height
                    }),
                );
                let _ = store.set("maximized", false);
            }
            if window_clone.is_fullscreen().unwrap_or(false) {
                let _ = store.set("fullscreen", true);
            } else {
                let _ = store.set("fullscreen", false);
            }
        } else if let WindowEvent::Moved(position) = event {
            if position.x > 0
                && position.y > 0
                && !window_clone.is_minimized().unwrap_or(false)
                && !window_clone.is_maximized().unwrap_or(false)
            {
                let _ = store.set(
                    "window_position",
                    json!({ "x": position.x, "y": position.y }),
                );
            }
        } else if let WindowEvent::DragDrop(drag_drop) = event {
            println!("drag_drop: {:?}", drag_drop);
        }
    });

    // 确保窗口显示并获取焦点
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
    println!("[PakePlus] 窗口已显示并获取焦点");

    Ok(())
}

// handle something when start app (legacy, kept for compatibility)
pub async fn resolve_setup(app: &mut App) -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let args_str = args[1..].join("|");
    let args_base64 = BASE64_STANDARD.encode(args_str.as_bytes());
    let app_handle = app.handle();
    let window_json = r#"
        {
            "label": "main",
            "title": "PakePlus",
            "visible": true,
            "url": "index.html",
            "width": 1024,
            "height": 720
        }
    "#;
    let mut json_value: Value = serde_json::from_str(window_json)?;
    if !args_base64.is_empty() {
        if let Some(url) = json_value.get_mut("url") {
            if let Some(original_url) = url.as_str() {
                let new_url = append_param(original_url, args_base64.as_str());
                *url = Value::String(new_url);
            }
        }
    }
    let mut store_name = "app_data.json".to_string();
    let mut config: WindowConfig = serde_json::from_value(json_value).unwrap();

    // load man
    let startup_dir = get_exe_dir(true);
    let man_result = load_man(&startup_dir);
    let man_content = match man_result {
        Ok(content) => content,
        Err(e) => {
            println!("[PakePlus] 加载配置文件失败: {}, 使用默认配置", e);
            String::new()
        }
    };

    // custom js
    let mut contents = String::new();
    #[cfg(target_os = "windows")]
    let mut icon_bytes: Vec<u8> = Vec::new();

    if man_content.len() > 0 {
        println!("[PakePlus] 发现配置文件，正在解析...");
        match serde_json::from_str::<Man>(&man_content) {
            Ok(mut man_config) => {
                println!("[PakePlus] 配置解析成功: name={}, window.url={:?}", 
                         man_config.name, 
                         man_config.window.as_ref().map(|w| &w.url));

                let www_dir = get_www_dir(&startup_dir);
                let www_dir_str = match www_dir {
                    Ok(dir) => dir,
                    Err(_) => String::new(),
                };

                if let Some(ref mut window_config) = man_config.window {
                    // 修复：将被错误反序列化为 AssetUrl 的外部URL转换为 External
                    // Tauri v2 的 WebviewUrl serde 实现会将字符串默认为 AssetUrl
                    let url = window_config.url.clone();
                    if let WebviewUrl::App(path) = url {
                        let url_str = path.to_string_lossy().to_string();
                        if url_str.starts_with("http://") || url_str.starts_with("https://") {
                            if let Ok(external_url) = Url::parse(&url_str) {
                                println!("[PakePlus] URL修复: AssetUrl -> External({})", url_str);
                                window_config.url = WebviewUrl::External(external_url);
                            }
                        }
                    }

                    window_config.label = "main".to_string();
                    window_config.visible = false;

                    store_name = format!("{}.json", man_config.name.as_str());

                    if www_dir_str.len() > 0 {
                        match Url::parse(&www_dir_str) {
                            Ok(url) => {
                                window_config.url = WebviewUrl::External(url);
                                println!("[PakePlus] 使用本地静态文件: {}", www_dir_str);
                            }
                            Err(e) => {
                                println!("[PakePlus] URL解析失败: {}", e);
                            }
                        }
                    } else {
                        println!("[PakePlus] 使用配置中的URL: {:?}", window_config.url);
                    }

                    config = window_config.clone();
                } else {
                    println!("[PakePlus] 配置中无窗口设置，使用默认配置");
                }

                // custom js
                match get_config_js(&startup_dir) {
                    Ok(custom_js) => {
                        contents = custom_js;
                        println!("[PakePlus] 加载自定义JS ({}字节)", contents.len());
                    }
                    Err(_) => {
                        println!("[PakePlus] 无自定义JS");
                    }
                }

                // debug
                if man_config.debug {
                    contents += "var vConsole = new window.VConsole();";
                    println!("[PakePlus] 调试模式已开启");
                }

                // icon
                #[cfg(target_os = "windows")]
                if man_config.icon.len() > 0 {
                    match BASE64_STANDARD.decode(man_config.icon.trim()) {
                        Ok(bytes) => {
                            icon_bytes = bytes;
                            println!("[PakePlus] 加载图标 ({}字节)", icon_bytes.len());
                        }
                        Err(e) => {
                            println!("[PakePlus] 图标解码失败: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("[PakePlus] 配置解析失败: {}, 使用默认配置", e);
            }
        }
    } else {
        println!("[PakePlus] 无配置文件，使用默认PakePlus前端");
    }

    println!("[PakePlus] 创建窗口: title={:?}, url={:?}, size={}x{}", 
             config.title, 
             config.url,
             config.width,
             config.height);

    // init window
    let window = tauri::WebviewWindowBuilder::new(
        app_handle,
        "main",
        config.url.clone(),
    )
    .title(&config.title)
    .visible(config.visible)
    .inner_size(config.width, config.height)
    .initialization_script(contents.as_str())
    .build()
    .unwrap();
    println!("[PakePlus] 窗口创建成功: title={:?}, visible={:?}", config.title, config.visible);
    let store = app.store(store_name).unwrap();
    // store.clear();
    let window_size: Option<serde_json::Value> = store.get("window_size");
    let mut width = 0.0;
    let mut height = 0.0;
    if let Some(window_size) = window_size {
        let size = window_size.as_object().unwrap();
        width = size["width"].as_f64().unwrap();
        height = size["height"].as_f64().unwrap();
    }

    #[cfg(target_os = "windows")]
    if icon_bytes.len() > 0 {
        use tauri::image::Image;
        match Image::from_bytes(&icon_bytes) {
            Ok(png_image) => {
                let _ = window.set_icon(png_image);
                println!("[PakePlus] 窗口图标已设置");
            }
            Err(e) => {
                println!("[PakePlus] 图标设置失败: {}", e);
            }
        }
    }

    let window_position: Option<serde_json::Value> = store.get("window_position");
    let mut x = 0.0;
    let mut y = 0.0;

    if let Some(window_position) = window_position {
        let position = window_position.as_object().unwrap();
        x = position["x"].as_f64().unwrap();
        y = position["y"].as_f64().unwrap();
    }

    // position
    if config.center || x <= 0.0 || y <= 0.0 {
        let _ = window.center();
    } else {
        let _ = window
            .set_position(tauri::PhysicalPosition::new(x, y));
    }

    if config.fullscreen
        || store
            .get("fullscreen")
            .unwrap_or(serde_json::Value::Bool(false))
            .as_bool()
            .unwrap()
    {
        let _ = window.set_fullscreen(true);
    } else if config.maximized
        || store
            .get("maximized")
            .unwrap_or(serde_json::Value::Bool(false))
            .as_bool()
            .unwrap()
    {
        let _ = window.maximize();
    } else if width > 0.0 && height > 0.0 {
        let _ = window
            .set_size(tauri::PhysicalSize::new(width, height));
    }
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Resized(size) = event {
            if window_clone.is_maximized().unwrap_or(false) {
                let _ = store.set("maximized", true);
            } else if size.width > 0
                && size.height > 0
                && !window_clone.is_minimized().unwrap_or(false)
            {
                let _ = store.set(
                    "window_size",
                    json!({
                        "width": size.width,
                        "height": size.height
                    }),
                );
                let _ = store.set("maximized", false);
            }
            if window_clone.is_fullscreen().unwrap_or(false) {
                let _ = store.set("fullscreen", true);
            } else {
                let _ = store.set("fullscreen", false);
            }
        } else if let WindowEvent::Moved(position) = event {
            if position.x > 0
                && position.y > 0
                && !window_clone.is_minimized().unwrap_or(false)
                && !window_clone.is_maximized().unwrap_or(false)
            {
                let _ = store.set(
                    "window_position",
                    json!({ "x": position.x, "y": position.y }),
                );
            }
        } else if let WindowEvent::DragDrop(drag_drop) = event {
            println!("drag_drop: {:?}", drag_drop);
        }
    });
    let _ = window.unminimize();
    println!("[PakePlus] 窗口已 unminimize");
    let _ = window.show();
    println!("[PakePlus] 窗口已 show()");
    let _ = window.set_focus();
    println!("[PakePlus] 窗口已 set_focus()");
    Ok(())
}
