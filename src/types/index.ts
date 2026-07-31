// PakePlus 项目配置类型定义

// 平台标识: 1-1=Win x64, 1-2=Win arm64, 2-1=macOS x86, 2-2=macOS arm, 3-1=Linux x64, 3-2=Linux arm, 4-1=Android, 4-2=iOS
export type PlatformKey = '1-1' | '1-2' | '2-1' | '2-2' | '3-1' | '3-2' | '4-1' | '4-2'

// 窗口配置（对应 Tauri WindowConfig）
export interface WindowConfig {
    label?: string
    title: string
    url: string
    userAgent?: string
    width: number
    height: number
    theme?: string | null
    resizable?: boolean
    fullscreen?: boolean
    maximized?: boolean
    minWidth?: number
    minHeight?: number
    maxWidth?: number
    maxHeight?: number
    decorations?: boolean
    transparent?: boolean
    titleBarStyle?: string
    visible?: boolean
    focus?: boolean
    closable?: boolean
    minimizable?: boolean
    maximizable?: boolean
    alwaysOnTop?: boolean
    alwaysOnBottom?: boolean
    center?: boolean
    skipTaskbar?: boolean
    dragDropEnabled?: boolean
    browserExtensionsEnabled?: boolean
    devtools?: boolean
    contentProtected?: boolean
    hiddenTitle?: boolean
    incognito?: boolean
    zoomHotkeysEnabled?: boolean
    acceptFirstMouse?: boolean
}

// 手机端 - 安全区
export interface SafeArea {
    top: number
    bottom: number
    left: number
    right: number
}

// 手机端 - 头部
export interface PhoneHeader {
    show: boolean
    title: string
    backgroundColor: string
    color: string
    fontSize: number
    fontWeight: string
    loading: boolean
    toolBar: boolean
    toolBarBackgroundColor: string
    toolBarColor: string
    toolBarFontSize: number
    toolBarFontWeight: string
}

// 手机端 - 侧边栏
export interface SiderMenu {
    show: boolean
    width: number
    backgroundColor: string
    color: string
    fontSize: number
    fontWeight: string
    title: string
    titleColor: string
    titleFontSize: number
    titleFontWeight: string
}

// 手机端 - TabBar 项
export interface TabBarItem {
    title: string
    icon: string
    url: string
}

// 手机端 - TabBar
export interface TabBar {
    show: boolean
    backgroundColor: string
    color: string
    activeColor: string
    fontSize: number
    fontWeight: string
    tabBarItem: TabBarItem[]
}

// 手机端 - WebView
export interface PhoneWebview {
    userAgent: string
    javaScriptEnabled: boolean
    domStorageEnabled: boolean
    allowFileAccess: boolean
    loadWithOverviewMode: boolean
    setSupportZoom: boolean
    clearCache: boolean
}

// 手机端配置
export interface PhoneConfig {
    safeArea: SafeArea
    header: PhoneHeader
    siderMenu: SiderMenu
    tabBar: TabBar
    webview: PhoneWebview
}

// iOS 打包配置
export interface IosConfig {
    name: string
    showName: string
    version: string
    webUrl: string
    id: string
    icon: string
    desc: string
    pubBody: string
    isHtml: boolean
    debug: boolean
}

// Android 打包配置
export interface AndroidConfig {
    name: string
    showName: string
    version: string
    webUrl: string
    id: string
    icon: string
    input: string
    output: string
    rounded: boolean
    copyTo: string
    androidResDir: string
    desc: string
    pubBody: string
    isHtml: boolean
    debug: boolean
}

// 桌面端打包配置
export interface DesktopConfig {
    name: string
    showName: string
    version: string
    id: string
    desc: string
    webUrl: string
    iconPath: string
    inputPath: string
    tempPath: string
    icnsPath: string
    pubBody: string
    isHtml: boolean
    single: boolean
    state: boolean
    injectJq: boolean
    tauriApi: boolean
    buildMethod: 'local' | 'github'
    debug: boolean
}

// 完整项目配置（对应 ppconfig.json）
export interface ProjectConfig {
    name: string
    url: string
    showName: string
    appid: string
    icon: string // base64
    iconRound: boolean
    state: boolean
    single: boolean
    injectJq: boolean
    tauriApi: boolean
    devbug: boolean
    version: string
    preview: 'desktop' | 'phone'
    platform: PlatformKey[]
    width: number
    height: number
    desc: string
    jsFile: string[]
    filterCss: string
    customJs: string
    isHtml: boolean
    htmlPath: string
    htmlFiles: string[]
    more: {
        windows: WindowConfig
    }
    phone: PhoneConfig
    ios: IosConfig
    android: AndroidConfig
    desktop: DesktopConfig
}

// 运行时 man.json 配置（对应 src-tauri/data/man.json）
export interface ManConfig {
    name: string
    version: string
    description: string
    author: string
    license: string
    window: WindowConfig
    debug: boolean
    icon: string
    visible?: boolean
    langs?: Record<string, unknown>
}

// 项目信息（含元数据）
export interface Project {
    id: string
    config: ProjectConfig
    createdAt: number
    updatedAt: number
}

// GitHub 仓库信息
export interface GitHubRepo {
    id: number
    name: string
    full_name: string
    html_url: string
    default_branch: string
    fork: boolean
    parent?: {
        full_name: string
    }
}

// GitHub Actions 工作流运行状态
export interface WorkflowRun {
    id: number
    name: string
    status: string
    conclusion: string | null
    html_url: string
    created_at: string
    updated_at: string
    head_branch: string
}

// GitHub Release 资源
export interface ReleaseAsset {
    id: number
    name: string
    browser_download_url: string
    size: number
    content_type: string
}

export interface GitHubRelease {
    id: number
    tag_name: string
    name: string
    body: string
    assets: ReleaseAsset[]
    created_at: string
    published_at: string
    html_url: string
}

// 默认项目配置
export function createDefaultConfig(): ProjectConfig {
    return {
        name: '',
        url: '',
        showName: '',
        appid: '',
        icon: '',
        iconRound: true,
        state: true,
        single: true,
        injectJq: true,
        tauriApi: false,
        devbug: false,
        version: '0.0.1',
        preview: 'desktop',
        platform: ['1-1', '2-2'],
        width: 800,
        height: 600,
        desc: '',
        jsFile: [],
        filterCss: '',
        customJs: '',
        isHtml: false,
        htmlPath: '',
        htmlFiles: [],
        more: {
            windows: {
                label: '',
                title: '',
                url: '',
                userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36',
                width: 800,
                height: 600,
                theme: null,
                resizable: true,
                fullscreen: false,
                maximized: false,
                minWidth: 400,
                minHeight: 300,
                maxWidth: 1920,
                maxHeight: 1080,
                decorations: true,
                transparent: false,
                titleBarStyle: 'Visible',
                visible: true,
                focus: true,
                closable: true,
                minimizable: true,
                maximizable: true,
                alwaysOnTop: false,
                alwaysOnBottom: false,
                center: false,
                skipTaskbar: false,
                dragDropEnabled: true,
                browserExtensionsEnabled: false,
                devtools: true,
                contentProtected: false,
                hiddenTitle: false,
                incognito: false,
                zoomHotkeysEnabled: false,
                acceptFirstMouse: false,
            },
        },
        phone: {
            safeArea: { top: 0, bottom: 0, left: 0, right: 0 },
            header: {
                show: false,
                title: '',
                backgroundColor: '',
                color: '',
                fontSize: 16,
                fontWeight: 'bold',
                loading: false,
                toolBar: false,
                toolBarBackgroundColor: '',
                toolBarColor: '',
                toolBarFontSize: 16,
                toolBarFontWeight: 'bold',
            },
            siderMenu: {
                show: false,
                width: 0,
                backgroundColor: '',
                color: '',
                fontSize: 16,
                fontWeight: 'bold',
                title: '',
                titleColor: '',
                titleFontSize: 16,
                titleFontWeight: 'bold',
            },
            tabBar: {
                show: false,
                backgroundColor: '',
                color: '',
                activeColor: '',
                fontSize: 16,
                fontWeight: 'bold',
                tabBarItem: [],
            },
            webview: {
                userAgent: '',
                javaScriptEnabled: true,
                domStorageEnabled: true,
                allowFileAccess: true,
                loadWithOverviewMode: true,
                setSupportZoom: true,
                clearCache: true,
            },
        },
        ios: {
            name: 'PakePlus',
            showName: 'PakePlus',
            version: '0.0.1',
            webUrl: 'https://juejin.cn/',
            id: 'com.xiaoshen.PakePlus.ios',
            icon: './app-icon.png',
            desc: '打包仅限个人使用，请勿传播或商业用途',
            pubBody: '打包仅限个人使用，请勿传播或商业用途',
            isHtml: false,
            debug: false,
        },
        android: {
            name: 'PakePlus',
            showName: 'PakePlus',
            version: '0.0.1',
            webUrl: 'https://juejin.cn/',
            id: 'com.xiaoshen.PakePlus.android',
            icon: './app-icon.png',
            input: './app-icon.png',
            output: './res',
            rounded: true,
            copyTo: './app/src/main/res',
            androidResDir: './app/src/main/res',
            desc: '打包仅限个人使用，请勿传播或商业用途',
            pubBody: '打包仅限个人使用，请勿传播或商业用途',
            isHtml: false,
            debug: false,
        },
        desktop: {
            name: 'PakePlus',
            showName: 'xiaoshen',
            version: '0.0.2',
            id: 'com.xiaoshen.app',
            desc: '打包仅限个人使用，请勿传播或商业用途，否则后果自负',
            webUrl: 'https://juejin.cn/',
            iconPath: '../app-icon.png',
            inputPath: '../app-icon.png',
            tempPath: './processed-image.png',
            icnsPath: '../src-tauri/icons/icon.icns',
            pubBody: '打包仅限个人使用，请勿传播或商业用途，否则后果自负',
            isHtml: false,
            single: true,
            state: true,
            injectJq: false,
            tauriApi: false,
            buildMethod: 'local',
            debug: false,
        },
    }
}
