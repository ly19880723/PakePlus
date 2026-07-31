// Tauri 命令封装 - 对接 src-tauri 后端所有命令
// 在非 Tauri 环境（浏览器开发调试）中，所有调用会抛出友好错误，不影响 UI 渲染
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { isTauriEnv } from '@/utils/storage'

function notSupported(name: string): Error {
    return new Error(`「${name}」需要在 Tauri 桌面应用中运行，浏览器开发环境不支持`)
}

// 启动本地静态文件服务器
export async function startServer(path: string, port: number = 0): Promise<number> {
    if (!isTauriEnv) throw notSupported('startServer')
    return invoke<number>('start_server', { path, port })
}

// 停止本地服务器
export async function stopServer(): Promise<void> {
    if (!isTauriEnv) throw notSupported('stopServer')
    return invoke('stop_server')
}

// 预览应用
export async function previewFromConfig(
    resize: boolean,
    config: Record<string, unknown>,
    jsContent: string,
    devbug: boolean,
    iconBase64: string
): Promise<void> {
    if (!isTauriEnv) throw notSupported('预览应用')
    return invoke('preview_from_config', {
        resize,
        config,
        jsContent,
        devbug,
        iconBase64,
    })
}

// 打开 URL
export async function openUrl(url: string): Promise<void> {
    if (!isTauriEnv) {
        // 浏览器环境降级：用 window.open
        window.open(url, '_blank')
        return
    }
    return invoke('open_url', { url })
}

// 打开开发者工具
export async function openDevtools(): Promise<void> {
    if (!isTauriEnv) throw notSupported('openDevtools')
    return invoke('open_devtools')
}

// 更新 init.rs 配置
export async function updateInitRs(config: string, state: boolean): Promise<string> {
    if (!isTauriEnv) throw notSupported('updateInitRs')
    return invoke<string>('update_init_rs', { config, state })
}

// 获取机器唯一标识
export async function getMachineUid(): Promise<string> {
    if (!isTauriEnv) throw notSupported('getMachineUid')
    return invoke<string>('get_machine_uid')
}

// 压缩文件夹
export async function compressFolder(source: string, destination: string): Promise<void> {
    if (!isTauriEnv) throw notSupported('compressFolder')
    return invoke('compress_folder', { source, destination })
}

// 解压文件
export async function decompressFile(source: string, destination: string): Promise<void> {
    if (!isTauriEnv) throw notSupported('decompressFile')
    return invoke('decompress_file', { source, destination })
}

// 下载文件
export async function downloadFile(url: string, savePath: string, fileId: string): Promise<void> {
    if (!isTauriEnv) throw notSupported('downloadFile')
    return invoke('download_file', { url, savePath, fileId })
}

// 系统通知
export async function notification(title: string, body: string, icon: string = ''): Promise<void> {
    if (!isTauriEnv) {
        // 浏览器环境降级：使用 Notification API
        if ('Notification' in window) {
            if (Notification.permission === 'granted') {
                new Notification(title, { body })
            } else if (Notification.permission !== 'denied') {
                Notification.requestPermission().then((p) => {
                    if (p === 'granted') new Notification(title, { body })
                })
            }
        }
        return
    }
    return invoke('notification', { params: { title, body, icon } })
}

// 执行系统命令
export async function runCommand(command: string): Promise<string> {
    if (!isTauriEnv) throw notSupported('runCommand')
    return invoke<string>('run_command', { command })
}

// 获取环境变量
export async function getEnvVar(name: string): Promise<string> {
    if (!isTauriEnv) throw notSupported('getEnvVar')
    return invoke<string>('get_env_var', { name })
}

// 查找可用端口
export async function findPort(): Promise<number> {
    if (!isTauriEnv) throw notSupported('findPort')
    return invoke<number>('find_port')
}

// 获取可执行文件目录
export async function getExeDir(parent: boolean = true): Promise<string> {
    if (!isTauriEnv) throw notSupported('getExeDir')
    return invoke<string>('get_exe_dir', { parent })
}

// 本地打包构建
export async function buildLocal(
    targetDir: string,
    projectName: string,
    exeName: string,
    config: Record<string, unknown>,
    base64Png: string,
    debug: boolean,
    customJs: string,
    htmlPath: string
): Promise<void> {
    if (!isTauriEnv) throw notSupported('本地打包')
    return invoke('build_local', {
        targetDir,
        projectName,
        exeName,
        config,
        base64Png,
        debug,
        customJs,
        htmlPath,
    })
}

// PNG 转 ICNS（macOS）
export async function pngToIcns(base64Png: string, outputDir: string): Promise<void> {
    if (!isTauriEnv) throw notSupported('pngToIcns')
    return invoke('png_to_icns', { base64Png, outputDir })
}

// 获取内置工作流 YML 文件内容
export async function getWorkflowYml(): Promise<string> {
    if (!isTauriEnv) throw notSupported('获取工作流文件')
    return invoke<string>('get_workflow_yml')
}

// Windows 本地构建
export async function windowsBuild(
    baseDir: string,
    exeName: string,
    config: string,
    customJs: string,
    htmlPath: string,
    scriptPath: string
): Promise<void> {
    if (!isTauriEnv) throw notSupported('windowsBuild')
    return invoke('windows_build', {
        baseDir,
        exeName,
        config,
        customJs,
        htmlPath,
        scriptPath,
    })
}

// macOS 本地构建
export async function macosBuild(
    baseDir: string,
    exeName: string,
    config: string,
    base64Png: string,
    customJs: string,
    htmlPath: string
): Promise<void> {
    if (!isTauriEnv) throw notSupported('macosBuild')
    return invoke('macos_build', {
        baseDir,
        exeName,
        config,
        base64Png,
        customJs,
        htmlPath,
    })
}

// 复制目录
export async function copyDir(src: string, dst: string): Promise<void> {
    if (!isTauriEnv) throw notSupported('copyDir')
    return invoke('copy_dir', { src, dst })
}

// 获取 www 目录并启动服务
export async function getWwwDir(baseDir: string): Promise<string> {
    if (!isTauriEnv) throw notSupported('getWwwDir')
    return invoke<string>('get_www_dir', { baseDir })
}

// 获取自定义 JS 配置
export async function getConfigJs(baseDir: string): Promise<string> {
    if (!isTauriEnv) throw notSupported('getConfigJs')
    return invoke<string>('get_config_js', { baseDir })
}

// 监听本地构建进度
export async function onLocalProgress(callback: (progress: string) => void): Promise<UnlistenFn> {
    if (!isTauriEnv) {
        // 浏览器环境返回一个 no-op 的 unlisten 函数
        return (() => {}) as UnlistenFn
    }
    return listen<string>('local-progress', (event) => {
        callback(event.payload)
    })
}

// 监听下载进度
export async function onDownloadProgress(
    callback: (data: { fileId: string; downloaded: number; total: number }) => void
): Promise<UnlistenFn> {
    if (!isTauriEnv) {
        return (() => {}) as UnlistenFn
    }
    return listen('download_progress', (event) => {
        callback(event.payload as { fileId: string; downloaded: number; total: number })
    })
}

// 监听预览窗口关闭
export async function onStopServer(callback: () => void): Promise<UnlistenFn> {
    if (!isTauriEnv) {
        return (() => {}) as UnlistenFn
    }
    return listen<string>('stop_server', () => {
        callback()
    })
}

// 监听 OAuth 回调
export async function onCallback(
    callback: (params: Record<string, string>) => void
): Promise<UnlistenFn> {
    if (!isTauriEnv) {
        return (() => {}) as UnlistenFn
    }
    return listen('callback', (event) => {
        callback(event.payload as Record<string, string>)
    })
}
