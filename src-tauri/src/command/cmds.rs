use crate::command::model::ServerState;
use base64::prelude::*;
use notify_rust::Notification;
use serde::Serialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::WindowEvent;
use tauri::{
    path::BaseDirectory, utils::config::WindowConfig, AppHandle, Emitter, LogicalSize, Manager,
    Url, WebviewUrl,
};
use ureq;
use walkdir::WalkDir;
use warp::Filter;
use zip::write::FileOptions;
use zip::ZipArchive;
use zip::ZipWriter;

#[tauri::command]
pub async fn start_server(
    app: AppHandle,
    state: tauri::State<'_, Arc<Mutex<ServerState>>>,
    path: String,
    port: u16,
) -> Result<u16, String> {
    let mut state = state.lock().unwrap();
    // if server is running, stop it
    if let Some(handle) = state.server_handle.take() {
        handle.abort();
    }
    let path_clone = path.clone();
    // if port is 0, find a free port
    let port = if port == 0 {
        find_port().unwrap()
    } else {
        port
    };
    // println!("port: {}", port);
    let server_handle = tokio::spawn(async move {
        let static_files = warp::fs::dir(path_clone);

        let oauth_callback = warp::path("callback")
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .map(move |params: std::collections::HashMap<String, String>| {
                // println!("OAuth params: {:?}", params);
                let _ = app.emit("callback", serde_json::json!(params));
                // return a simple page
                warp::reply::html(format!(
                    r#"
                <html>
                    <body>
                        <h2>Login Success ✅</h2>
                        <p>You can close this window.</p>
                        <script>
                            window.close();
                        </script>
                    </body>
                </html>
                "#
                ))
            });

        // routes
        let routes = oauth_callback
            .or(static_files)
            .map(|reply| {
                warp::reply::with_header(
                    reply,
                    "Cache-Control",
                    "no-store, no-cache, must-revalidate, max-age=0",
                )
            })
            .map(|reply| warp::reply::with_header(reply, "Vary", "*"))
            .map(|reply| warp::reply::with_header(reply, "Surrogate-Control", "no-store"))
            .map(|reply| warp::reply::with_header(reply, "Pragma", "no-cache"))
            .map(|reply| warp::reply::with_header(reply, "Expires", "0"));
        // start server
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    });
    state.server_handle = Some(server_handle);
    Ok(port)
}

#[tauri::command]
pub async fn stop_server(state: tauri::State<'_, Arc<Mutex<ServerState>>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if let Some(handle) = state.server_handle.take() {
        handle.abort();
        Ok(())
    } else {
        Err("Server is not running".into())
    }
}

#[tauri::command]
pub async fn preview_from_config(
    handle: AppHandle,
    resize: bool,
    config: WindowConfig,
    js_content: String,
    devbug: bool,
    icon_base64: String,
) {
    let window_label = "PreView";
    println!("[PakePlus] preview_from_config: url={}, title={}, size={}x{}", 
             config.url, config.title, config.width, config.height);
    
    if let Some(existing_window) = handle.get_webview_window(window_label) {
        if resize {
            let new_size = LogicalSize::new(config.width, config.height);
            match existing_window.set_size(new_size) {
                Ok(_) => println!("Window resized to {}x{}", config.width, config.height),
                Err(e) => eprintln!("Failed to resize window: {}", e),
            }
        } else {
            // existing_window.eval(js)
            existing_window.close().unwrap();
            let start = Instant::now();
            while handle.get_webview_window(window_label).is_some() {
                if start.elapsed().as_secs() > 2 {
                    println!("Window close took too long. Aborting.");
                    return;
                }
                std::thread::yield_now();
            }
        }
    }
    let mut contents = String::new();
    // custom js
    contents += js_content.as_str();
    if !resize {
        // 修复：将被错误反序列化为 AssetUrl 的外部URL转换为 External
        let mut url = config.url.clone();
        if let WebviewUrl::App(ref path) = url {
            let url_str = path.to_string_lossy().to_string();
            if url_str.starts_with("http://") || url_str.starts_with("https://") {
                if let Ok(external_url) = Url::parse(&url_str) {
                    println!("[PakePlus] preview_from_config: URL修复 AssetUrl -> External({})", url_str);
                    url = WebviewUrl::External(external_url);
                }
            }
        }
        let title = config.title.clone();
        let width = config.width;
        let height = config.height;
        
        println!("[PakePlus] Creating preview window with url: {:?}", url);
        
        let mut builder = tauri::WebviewWindowBuilder::new(&handle, window_label, url)
            .title(title)
            .initialization_script_for_all_frames(contents.as_str())
            .inner_size(width, height)
            .resizable(config.resizable);
        
        if config.center {
            builder = builder.center();
        }
        
        if let Some(ref ua) = config.user_agent {
            if !ua.is_empty() {
                builder = builder.user_agent(ua);
            }
        }
        
        let pre_window = builder.build().unwrap();
        println!("[PakePlus] Preview window created successfully");
        
        if icon_base64.len() > 0 {
            use tauri::image::Image;
            let icon_decode =
                BASE64_STANDARD.decode(icon_base64.replace("data:image/png;base64,", "").trim());
            let icon_bytes = icon_decode.unwrap();
            let png_image = Image::from_bytes(&icon_bytes).unwrap();
            pre_window.set_icon(png_image).unwrap();
        }
        if devbug {
            pre_window.open_devtools();
        }
        pre_window.on_window_event(move |event| {
            if let WindowEvent::Destroyed = event {
                handle.emit("stop_server", "0").unwrap();
            }
        });
    }
}

#[tauri::command]
pub async fn open_url(_: tauri::AppHandle, url: String) {
    open::that(url).unwrap();
}

// open devtools
#[tauri::command]
pub async fn open_devtools(handle: AppHandle) {
    let _ = handle.get_webview_window("main").unwrap().open_devtools();
}

#[tauri::command]
pub async fn update_init_rs(handle: tauri::AppHandle, config: String, state: bool) -> String {
    let resource_path = handle
        .path()
        .resolve("data/init.rs", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut main_rust = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    main_rust.read_to_string(&mut contents).unwrap();
    contents = contents.replace("WINDOWCONFIG", config.as_str());
    // 替换state
    if state {
        println!("state: true");
    } else {
        contents = contents.replace("if true {", "if false {");
    }
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn run_command(command: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let output = tokio::process::Command::new("powershell")
        .arg("-Command")
        .arg(&command)
        .creation_flags(0x08000000)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let output = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        #[cfg(target_os = "windows")]
        {
            use encoding_rs::GBK;
            let (decoded, _, _) = GBK.decode(&output.stdout);
            Ok(decoded.into_owned())
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    } else {
        #[cfg(target_os = "windows")]
        {
            use encoding_rs::GBK;
            let (decoded, _, _) = GBK.decode(&output.stderr);
            Err(decoded.into_owned())
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

#[tauri::command]
pub fn get_machine_uid() -> String {
    let uid: String = machine_uid::get().unwrap();
    uid
}

fn zip_folder(src_path: &str, dst_path: &str) -> std::io::Result<()> {
    let file = File::create(dst_path)?;
    let mut zip = ZipWriter::new(file);
    print!("src_path = {src_path}");
    print!("dst_path = {dst_path}");
    let options: FileOptions<()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let src_path = Path::new(src_path);
    let walkdir = WalkDir::new(src_path);
    let it = walkdir.into_iter();

    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(src_path).unwrap().to_str().unwrap();

        if path.is_file() {
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        } else if !name.is_empty() {
            zip.add_directory(name, options)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn unzip_file(src_path: &str, dst_path: &str) -> std::io::Result<()> {
    let file = File::open(src_path)?;
    let mut archive = ZipArchive::new(file)?;
    let dst_path = Path::new(dst_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dst_path.join(file.mangled_name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn compress_folder(source: String, destination: String) -> Result<(), String> {
    zip_folder(&source, &destination).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn decompress_file(source: String, destination: String) -> Result<(), String> {
    unzip_file(&source, &destination).map_err(|e| e.to_string())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadProgress {
    file_id: String,
    downloaded: u64,
    total: u64,
}

#[tauri::command]
pub async fn download_file(
    app: AppHandle,
    url: String,
    save_path: String,
    file_id: String,
) -> Result<(), String> {
    // 用 ureq 同步下载（简单 GET 请求，ureq 足够用）
    let resp = ureq::get(&url)
        .call()
        .map_err(|e| e.to_string())?;
    // 从 header 获取 content-length
    let total_size: u64 = resp
        .header("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let mut reader = resp.into_reader();
    
    // if save path is empty
    let mut save_path = save_path;
    let file_name = url.split('/').last().unwrap();
    if save_path.is_empty() {
        let file_path = app
            .path()
            .resolve(file_name, BaseDirectory::Download)
            .expect("failed to resolve resource");
        save_path = file_path.to_str().unwrap().to_string();
    }
    // if file exists, add number to file name
    if Path::new(&save_path).exists() {
        let mut i = 1;
        while Path::new(&save_path).exists() {
            save_path = save_path.split('.').nth(0).unwrap().to_string()
                + &i.to_string()
                + "."
                + save_path.split('.').nth(1).unwrap();
            i += 1;
        }
    }
    // 确保父目录存在
    if let Some(parent) = Path::new(&save_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建下载目录失败: {}", e))?;
        }
    }
    
    let mut file = File::create(&save_path).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];
    loop {
        let n = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        file.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
        downloaded += n as u64;
        app.emit(
            "download_progress",
            DownloadProgress {
                file_id: file_id.clone(),
                downloaded,
                total: total_size,
            },
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct NotificationParams {
    title: String,
    body: String,
    icon: String,
}

#[tauri::command]
pub fn notification(app: AppHandle, params: NotificationParams) -> Result<(), String> {
    let mut notifi_app = Notification::new();
    #[cfg(target_os = "macos")]
    {
        let _ = notify_rust::set_application(if tauri::is_dev() {
            "com.apple.Terminal"
        } else {
            &app.config().identifier
        });
    }
    #[cfg(windows)]
    {
        use std::path::MAIN_SEPARATOR as SEP;
        let curr_dir = get_exe_dir(true);
        // set the notification's System.AppUserModel.ID only when running the installed app
        if !(curr_dir.ends_with(format!("{SEP}target{SEP}debug").as_str())
            || curr_dir.ends_with(format!("{SEP}target{SEP}release").as_str()))
        {
            notifi_app.app_id(&app.config().identifier);
        }
    }
    if !params.icon.is_empty() {
        notifi_app.icon(&params.icon);
    } else {
        notifi_app.auto_icon();
    }
    tauri::async_runtime::spawn(async move {
        let _ = notifi_app
            .summary(&params.title)
            .body(&params.body)
            .show()
            .expect("show notification failed");
    });
    Ok(())
}

#[tauri::command]
pub fn get_exe_dir(parent: bool) -> String {
    let exe_dir = env::current_exe().unwrap();
    if parent {
        exe_dir.parent().unwrap().to_str().unwrap().to_string()
    } else {
        exe_dir.to_str().unwrap().to_string()
    }
}

// load man.json
pub fn load_man(base_dir: &str) -> Result<String, io::Error> {
    let mut man_path = PathBuf::from(base_dir);
    man_path.push("config");
    man_path.push("man");
    match fs::read_to_string(&man_path) {
        Ok(man_base64) => match BASE64_STANDARD.decode(man_base64.trim()) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(decoded_str) => Ok(decoded_str),
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            },
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e),
    }
}

// server config www dir
#[tauri::command]
pub fn get_www_dir(base_dir: &str) -> Result<String, io::Error> {
    let mut www_dir = PathBuf::from(base_dir);
    www_dir.push("config");
    www_dir.push("www");
    if fs::metadata(&www_dir).is_ok() {
        let files = fs::read_dir(&www_dir)?;
        if files.count() > 0 {
            let port = find_port().unwrap();
            let route = warp::fs::dir(www_dir);
            tokio::spawn(async move {
                warp::serve(route).run(([127, 0, 0, 1], port)).await;
            });
            return Ok(format!("http://127.0.0.1:{}", port));
        } else {
            return Ok(String::new());
        }
    }
    Ok(String::new())
}

// get config custom js
#[tauri::command]
pub fn get_config_js(base_dir: &str) -> Result<String, io::Error> {
    let mut config_dir = PathBuf::from(base_dir);
    config_dir.push("config");
    config_dir.push("inject");
    config_dir.push("custom.js");
    if fs::metadata(&config_dir).is_ok() {
        let content = fs::read_to_string(&config_dir)?;
        Ok(content)
    } else {
        Ok(String::new())
    }
}

#[tauri::command]
pub fn get_env_var(name: String) -> Result<String, String> {
    std::env::var(name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn find_port() -> Result<u16, String> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    Ok(port)
}

// copy dir all
#[tauri::command]
pub fn copy_dir(src: &Path, dst: &Path) -> Result<(), String> {
    if dst.starts_with(src) {
        return Err("Destination cannot be inside source directory".into());
    }
    if !dst.exists() {
        fs::create_dir_all(dst).expect("create dst dir failed");
    }
    for entry in fs::read_dir(src).expect("read src dir failed") {
        let entry = entry.expect("read src dir entry failed");
        let ty = entry
            .file_type()
            .expect("read src dir entry file type failed");
        if ty.is_symlink() {
            // skip symlink
            continue;
        }
        if ty.is_dir() {
            copy_dir(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name())).expect("copy file failed");
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn windows_build(
    base_dir: &str,
    exe_name: &str,
    config: String,
    custom_js: String,
    html_path: String,
    _script_path: String,
    base64_png: String,
) -> Result<(), String> {
    println!("[PakePlus] windows_build: base_dir={}, exe_name={}", base_dir, exe_name);

    let base_path = Path::new(base_dir).join(exe_name);
    if !base_path.exists() {
        fs::create_dir_all(&base_path).map_err(|e| format!("创建目标目录失败: {}", e))?;
    }
    let config_dir = base_path.join("config").join("inject");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;
    }
    let www_dir = base_path.join("config").join("www");
    if !html_path.is_empty() {
        let html_dir = Path::new(&html_path);
        if html_dir.exists() {
            copy_dir(html_dir, &www_dir).map_err(|e| format!("复制静态文件失败: {}", e))?;
        }
    }
    let custom_js_path = config_dir.join("custom.js");
    fs::write(&custom_js_path, custom_js).map_err(|e| format!("写入 custom.js 失败: {}", e))?;
    let man_path = base_path.join("config").join("man");
    fs::write(&man_path, config).map_err(|e| format!("写入 man 配置失败: {}", e))?;

    // 复制 PakePlus.exe 并重命名为目标 exe 名
    let exe_path = env::current_exe().map_err(|e| format!("获取当前 exe 路径失败: {}", e))?;
    let target_exe = base_path.join(format!("{}.exe", exe_name));
    fs::copy(&exe_path, &target_exe).map_err(|e| format!("复制主程序失败: {}", e))?;
    println!("[PakePlus] 主程序已复制到: {:?}", target_exe);

    // ---- 修改PE头子系统=WINDOWS (2)，消灭cmd黑窗口 ----
    if let Err(e) = set_pe_subsystem_windows(&target_exe) {
        println!("[PakePlus] 警告: 修改子系统失败（不影响使用）: {}", e);
    } else {
        println!("[PakePlus] 已切换 PE 子系统为 Windows（无cmd黑窗口）");
    }

    // ---- Resource Hacker 替换 ICON 和 VERSIONINFO ----
    // 先把 rh.exe 复制到输出目录，这样后续运行 RH 时能找到它
    let exe_dir = exe_path.parent().unwrap();
    let rhexe_src = exe_dir.join("data").join("rh.exe");
    let rh_local = base_path.join("rh.exe");
    if rhexe_src.exists() {
        let _ = fs::copy(&rhexe_src, &rh_local);
        println!("[PakePlus] rh.exe 已复制到输出目录");
    }

    if rh_local.exists() {
        let do_icon = !base64_png.is_empty();

        // 生成 ICO 临时文件
        let icon_ico_path = base_path.join("app.ico");
        let mut icon_ok = false;
        if do_icon {
            match generate_ico_from_png(&base64_png, &icon_ico_path) {
                Ok(_) => {
                    println!("[PakePlus] 图标ICO已生成: {:?}", icon_ico_path);
                    icon_ok = true;
                }
                Err(e) => println!("[PakePlus] 警告: 图标生成失败: {}", e),
            }
        }

        // 生成 VERSINFO 临时文件
        let version_rc_path = base_path.join("version.rc");
        let now = time::OffsetDateTime::now_utc();
        let exe_name_escaped = exe_name.replace("\"", "\"\"");
        let version_rc_content = format!(
            "1 VERSIONINFO\r\nFILEVERSION 1,0,0,0\r\nPRODUCTVERSION 1,0,0,0\r\nFILEFLAGSMASK 0x3fL\r\nFILEFLAGS 0x0L\r\nFILEOS 0x40004L\r\nFILETYPE 0x1L\r\nFILESUBTYPE 0x0L\r\nBEGIN\r\n  BLOCK \"StringFileInfo\"\r\n  BEGIN\r\n    BLOCK \"040904b0\"\r\n    BEGIN\r\n      VALUE \"CompanyName\", \"PakePlus\"\r\n      VALUE \"FileDescription\", \"{}\"\r\n      VALUE \"FileVersion\", \"1.0.0.0\"\r\n      VALUE \"LegalCopyright\", \"Copyright (C) {}\"\r\n      VALUE \"ProductName\", \"{}\"\r\n      VALUE \"ProductVersion\", \"1.0.0.0\"\r\n    END\r\n  END\r\n  BLOCK \"VarFileInfo\"\r\n  BEGIN\r\n    VALUE \"Translation\", 0x409, 1200\r\n  END\r\nEND\r\n",
            exe_name_escaped,
            now.year(),
            exe_name_escaped
        );
        let _ = fs::write(&version_rc_path, version_rc_content);

        // 编写 Resource Hacker 脚本（路径用单反斜杠，RH脚本格式要求）
        let target_exe_str = target_exe.to_string_lossy().to_string();
        let icon_ico_str = icon_ico_path.to_string_lossy().to_string();
        let version_rc_str = version_rc_path.to_string_lossy().to_string();
        let log_str = base_path.join("rh.log").to_string_lossy().to_string();

        let mut rh_script_content = String::new();
        rh_script_content.push_str(&format!(
            "[FILENAMES]\nExe=\"{}\"\nSaveAs=\"{}\"\nLog=\"{}\"\n\n[COMMANDS]\n",
            target_exe_str, target_exe_str, log_str
        ));

        if icon_ok {
            rh_script_content.push_str(&format!(
                "-addoverwrite \"{}\", ICONGROUP,MAINICON,1033\n",
                icon_ico_str
            ));
        }
        rh_script_content.push_str(&format!(
            "-add \"{}\", VERSIONINFO,1,1033\n",
            version_rc_str
        ));

        // 写入 RH 脚本到输出目录（避免 AppData 路径问题）
        let rh_script_path = base_path.join("rhscript.txt");
        let _ = fs::write(&rh_script_path, &rh_script_content);
        println!("[PakePlus] RH 脚本已生成: {:?}", rh_script_path);

        // 在输出目录本地执行 rh.exe
        let rh_cmd = format!(
            "& \"{}\" -script \"{}\"",
            rh_local.to_string_lossy(),
            rh_script_path.to_string_lossy()
        );
        println!("[PakePlus] 执行 RH 命令: {}", rh_cmd);
        match run_command(rh_cmd).await {
            Ok(_) => println!("[PakePlus] Resource Hacker 资源替换成功（图标+版本）"),
            Err(e) => println!("[PakePlus] Resource Hacker 警告: {}", e),
        }
    } else {
        println!("[PakePlus] 未找到 rh.exe，跳过资源替换");
    }

    println!("[PakePlus] windows_build 完成");
    Ok(())
}

// ========== 辅助函数 ==========

// 修改 PE 文件头的 Subsystem 字段为 IMAGE_SUBSYSTEM_WINDOWS_GUI (2)
fn set_pe_subsystem_windows(exe_path: &std::path::Path) -> Result<(), String> {
    use std::io::{Read, Seek, SeekFrom, Write};
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(exe_path)
        .map_err(|e| e.to_string())?;

    // 1. 读取 e_lfanew (MZ 头偏移 0x3C 处的 4 字节，指向 PE 签名位置)
    let mut buf = [0u8; 2];
    file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    file.read_exact(&mut buf).map_err(|e| e.to_string())?;
    if buf[0] != b'M' || buf[1] != b'Z' {
        return Err("不是有效的 MZ PE 文件".into());
    }
    let mut e_lfanew_buf = [0u8; 4];
    file.seek(SeekFrom::Start(0x3C))
        .map_err(|e| e.to_string())?;
    file.read_exact(&mut e_lfanew_buf).map_err(|e| e.to_string())?;
    let pe_off = u32::from_le_bytes(e_lfanew_buf) as u64;

    // 2. 验证 PE 签名
    let mut sig = [0u8; 4];
    file.seek(SeekFrom::Start(pe_off)).map_err(|e| e.to_string())?;
    file.read_exact(&mut sig).map_err(|e| e.to_string())?;
    if &sig != b"PE\0\0" {
        return Err("不是有效的 PE 文件".into());
    }

    // 3. 跳到 Subsystem 字段:
    //    COFF header 20 bytes (4 signature already used, + 20 = 0x18)
    //    = COFF header 结束位置 pe_off + 4 + 20 = pe_off + 24
    //    Standard (optional) header 起点 pe_off + 24
    //    Subsystem 在 Standard header 偏移 0x44 处，即绝对 = pe_off + 24 + 0x44
    //    = pe_off + 0x5C
    let subsys_offset = pe_off + 24 + 0x44;
    file.seek(SeekFrom::Start(subsys_offset))
        .map_err(|e| e.to_string())?;
    // 写入 IMAGE_SUBSYSTEM_WINDOWS_GUI = 2 (2 字节，little-endian)
    file.write_all(&[2u8, 0]).map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;
    Ok(())
}

// 将 base64 PNG 解码并生成 .ico 文件（内含 16/32/48/64/128/256 多尺寸）
fn generate_ico_from_png(base64_png: &str, ico_output: &std::path::Path) -> Result<(), String> {
    let png_bytes = if base64_png.starts_with("data:image/png;base64,") {
        &base64_png[22..]
    } else {
        base64_png
    };
    let raw = BASE64_STANDARD
        .decode(png_bytes.trim())
        .map_err(|e| format!("图标 base64 解码失败: {}", e))?;

    // 直接使用 image crate 解码 PNG 并缩放多种尺寸，再组装 ICO
    let img =
        image::load_from_memory(&raw).map_err(|e| format!("图标 PNG 解码失败: {}", e))?;

    // 需要的 ICO 尺寸 (边长)
    let sizes: &[u32] = &[16, 24, 32, 48, 64, 96, 128, 256];
    let mut icon_images: Vec<(u32, u32, Vec<u8>)> = Vec::new(); // (w,h, bmp_png_bytes)
    let filter = image::imageops::FilterType::Lanczos3;
    for &s in sizes {
        let resized = img.resize(s, s, filter);
        // 256 及以下保存为 PNG（ICO 格式支持 PNG 编码的子图，256 尤其推荐）
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut buf);
            resized
                .write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| format!("图标尺寸 {} 编码失败: {}", s, e))?;
        }
        icon_images.push((s, s, buf));
    }

    // ---- 组装 ICO 文件 ----
    // ICONDIR (6 bytes)
    //   idReserved: u16 = 0
    //   idType:     u16 = 1 (ICO)
    //   idCount:    u16 = N
    // ICONDIRENTRY * N (每项目 16 bytes)
    //   bWidth / bHeight / bColorCount / bReserved / wPlanes / wBitCount / dwBytesInRes / dwImageOffset
    // 然后按顺序拼接每副图的原始 bytes
    let count = icon_images.len() as u16;
    let header_len = 6usize + 16 * count as usize;
    let mut out = Vec::<u8>::with_capacity(header_len + icon_images.iter().map(|i| i.2.len()).sum::<usize>());
    out.extend_from_slice(&0u16.to_le_bytes()); // reserved
    out.extend_from_slice(&1u16.to_le_bytes()); // type = 1 (ICO)
    out.extend_from_slice(&count.to_le_bytes());
    let mut offset = header_len as u32;
    for (w, h, bytes) in &icon_images {
        // 宽/高: 256 存 0，否则存实际值
        out.push((*w).min(256) as u8);
        out.push((*h).min(256) as u8);
        out.push(0u8); // color count (0=真彩色)
        out.push(0u8); // reserved
        out.extend_from_slice(&1u16.to_le_bytes()); // planes
        out.extend_from_slice(&32u16.to_le_bytes()); // bit count (ARGB)
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += bytes.len() as u32;
    }
    for (_, _, bytes) in &icon_images {
        out.extend_from_slice(bytes);
    }
    fs::write(ico_output, &out).map_err(|e| format!("写入 ico 文件失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn macos_build(
    base_dir: &str,
    exe_name: &str,
    config: String,
    base64_png: String,
    custom_js: String,
    html_path: String,
) -> Result<(), String> {
    let base_path = Path::new(base_dir).join(exe_name);
    let app_dir = base_path.join("Contents");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("create app dir failed");
    }
    let config_dir = base_path.join("Contents/MacOS/config/inject");
    let resources_dir = base_path.join("Contents/Resources");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("create config dir failed");
    }
    if !resources_dir.exists() {
        fs::create_dir_all(&resources_dir).expect("create resources dir failed");
    }
    let www_dir = base_path.join("Contents/MacOS/config/www");
    if !html_path.is_empty() {
        let html_dir = Path::new(&html_path);
        if html_dir.exists() {
            copy_dir(html_dir, &www_dir).expect("copy html dir failed");
        }
    }
    let custom_js_path = config_dir.join("custom.js");
    fs::write(custom_js_path, custom_js).expect("write custom.js failed");
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let exe_parent_dir = exe_dir.parent().unwrap();
    let info_plist_source = exe_parent_dir.join("Info.plist");
    let info_plist_target = base_path.join("Contents/Info.plist");
    fs::copy(&info_plist_source, &info_plist_target).expect("copy info.plist failed");
    let pp_app_target = base_path.join("Contents/MacOS/PakePlus");
    fs::copy(&exe_path, &pp_app_target).expect("copy PakePlus app failed");
    let man_path = base_path.join("Contents/MacOS/config/man");
    fs::write(man_path, config).expect("write man failed");
    if !base64_png.is_empty() {
        let _ = png_to_icns(
            base64_png.replace("data:image/png;base64,", ""),
            resources_dir.to_str().unwrap().to_string(),
        )
        .expect("convert png to icns failed");
    }
    let base_app = Path::new(base_dir).join(format!("{}.app", exe_name));
    if base_app.exists() {
        fs::remove_dir_all(&base_app).expect("delete old app failed");
    }
    fs::rename(base_path, base_app).expect("rename app failed");
    Ok(())
}

#[tauri::command]
pub async fn linux_build(
    base_dir: &str,
    exe_name: &str,
    config: String,
    base64_png: String,
    custom_js: String,
    html_path: String,
) -> Result<(), String> {
    println!("base_dir: {}", base_dir);
    println!("exe_name: {}", exe_name);
    println!("config: {}", config);
    println!("base64_png: {}", base64_png);
    println!("custom_js: {}", custom_js);
    println!("html_path: {}", html_path);
    Ok(())
}

#[tauri::command]
pub async fn build_local(
    handle: AppHandle,
    target_dir: &str,
    project_name: &str,
    exe_name: &str,
    mut config: WindowConfig,
    base64_png: String,
    debug: bool,
    custom_js: String,
    html_path: String,
) -> Result<(), String> {
    // 修复：将被错误反序列化为 AssetUrl 的外部URL转换为 External
    // Tauri v2 的 WebviewUrl serde 实现会将字符串默认为 AssetUrl，
    // 导致外部链接被当作本地资源加载
    let url = config.url.clone();
    if let WebviewUrl::App(path) = url {
        let url_str = path.to_string_lossy().to_string();
        if url_str.starts_with("http://") || url_str.starts_with("https://") {
            if let Ok(external_url) = Url::parse(&url_str) {
                println!("[PakePlus] URL修复: AssetUrl -> External({})", url_str);
                config.url = WebviewUrl::External(external_url);
            }
        }
    }

    handle.emit("local-progress", "10").unwrap();
    let resource_path = handle
        .path()
        .resolve("data/man.json", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    handle.emit("local-progress", "20").unwrap();
    let man_json = fs::read_to_string(&resource_path).expect("read man.json failed");
    handle.emit("local-progress", "30").unwrap();
    let mut man_json =
        serde_json::from_str::<serde_json::Value>(&man_json).expect("parse man.json failed");
    man_json["window"] = serde_json::to_value(config).unwrap();
    man_json["debug"] = serde_json::to_value(debug).unwrap();
    man_json["name"] = serde_json::to_value(project_name).unwrap();
    man_json["visible"] = serde_json::to_value(false).unwrap();
    #[cfg(target_os = "windows")]
    {
        if !base64_png.is_empty() {
            man_json["icon"] =
                serde_json::to_value(base64_png.replace("data:image/png;base64,", "")).unwrap();
        }
    }
    let man_json_base64 = BASE64_STANDARD.encode(man_json.to_string());
    handle.emit("local-progress", "40").unwrap();
    #[cfg(target_os = "windows")]
    {
        let script_path = handle
            .path()
            .resolve("rhscript.txt", BaseDirectory::AppData)
            .expect("failed to resolve resource");

        // 修复：从 Tauri 资源系统解析 rh.exe，而不是依赖 exe_dir/data/
        // Tauri v2 将 data 目录嵌入资源，不保证物理文件存在于 exe 旁边
        let output_base = Path::new(target_dir).join(exe_name);
        if !output_base.exists() {
            let _ = fs::create_dir_all(&output_base);
        }
        let rh_target = output_base.join("rh.exe");
        let rh_from_resource = handle
            .path()
            .resolve("data/rh.exe", BaseDirectory::Resource)
            .ok();
        if let Some(rh_src) = rh_from_resource {
            if rh_src.exists() {
                match fs::copy(&rh_src, &rh_target) {
                    Ok(_) => println!("[PakePlus] rh.exe 已从资源复制到输出目录"),
                    Err(e) => println!("[PakePlus] rh.exe 复制失败: {}", e),
                }
            }
        }

        windows_build(
            target_dir,
            exe_name,
            man_json_base64,
            custom_js,
            html_path,
            script_path.to_str().unwrap().to_string(),
            base64_png.clone(),
        )
        .await?;
    }
    handle.emit("local-progress", "60").unwrap();
    #[cfg(target_os = "macos")]
    macos_build(
        target_dir,
        exe_name,
        man_json_base64,
        base64_png,
        custom_js,
        html_path,
    )
    .await?;
    handle.emit("local-progress", "80").unwrap();
    #[cfg(target_os = "linux")]
    linux_build(
        target_dir,
        exe_name,
        man_json_base64,
        base64_png,
        custom_js,
        html_path,
    )
    .await?;
    handle.emit("local-progress", "100").unwrap();
    Ok(())
}

#[tauri::command]
pub fn png_to_icns(base64_png: String, output_dir: String) -> Result<(), String> {
    let iconset_path = format!("{}/temp.iconset", output_dir);
    if Path::new(&iconset_path).exists() {
        fs::remove_dir_all(&iconset_path)
            .map_err(|e| format!("delete old iconset dir failed: {}", e))?;
    }
    fs::create_dir_all(&iconset_path).map_err(|e| format!("create iconset dir failed: {}", e))?;
    let png_data = BASE64_STANDARD
        .decode(&base64_png)
        .map_err(|e| format!("decode base64 png failed: {}", e))?;
    let input_png_path = format!("{}/icon.png", output_dir);
    let mut png_file =
        File::create(&input_png_path).map_err(|e| format!("write png failed: {}", e))?;
    png_file
        .write_all(&png_data)
        .map_err(|e| format!("write png content failed: {}", e))?;
    let sizes = vec![16, 32, 128, 256, 512];
    for size in sizes {
        let double = size * 2;
        let filename = format!("{}/icon_{}x{}.png", iconset_path, size, size);
        let filename2x = format!("{}/icon_{}x{}@2x.png", iconset_path, size, size);
        let status1 = Command::new("sips")
            .args([
                "-z",
                &size.to_string(),
                &size.to_string(),
                &input_png_path,
                "--out",
                &filename,
            ])
            .status()
            .map_err(|e| format!("execute sips failed: {}", e))?;
        let status2 = Command::new("sips")
            .args([
                "-z",
                &double.to_string(),
                &double.to_string(),
                &input_png_path,
                "--out",
                &filename2x,
            ])
            .status()
            .map_err(|e| format!("execute sips 2x failed: {}", e))?;
        if !status1.success() || !status2.success() {
            return Err("sips convert failed".into());
        }
    }
    let icns_path = format!("{}/icon.icns", output_dir);
    let status = Command::new("iconutil")
        .args(["-c", "icns", &iconset_path, "-o", &icns_path])
        .status()
        .map_err(|e| format!("execute iconutil failed: {}", e))?;
    if !status.success() {
        return Err("iconutil convert failed".into());
    }
    let _ = fs::remove_file(&input_png_path);
    let _ = fs::remove_dir_all(&iconset_path);
    Ok(())
}

#[tauri::command]
pub async fn get_workflow_yml(handle: AppHandle) -> Result<String, String> {
    let resource_path = handle
        .path()
        .resolve("data/workflow_build.yml", BaseDirectory::Resource)
        .map_err(|e| format!("解析工作流文件路径失败: {}", e))?;
    fs::read_to_string(&resource_path).map_err(|e| format!("读取工作流文件失败: {}", e))
}
