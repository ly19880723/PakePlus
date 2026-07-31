// 通用存储工具
// 在 Tauri 环境中使用 tauri-plugin-store，在浏览器开发环境中使用 localStorage 作为后备

// 检测是否在 Tauri 环境中
function isTauri(): boolean {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

// localStorage 的命名空间前缀，避免冲突
const LS_PREFIX = 'pakeplus:'

// Tauri LazyStore 实例缓存（仅 Tauri 环境下使用）
interface LazyStoreLike {
    get(key: string): Promise<unknown>
    set(key: string, value: unknown): Promise<void>
    save(): Promise<void>
}
const tauriStores = new Map<string, LazyStoreLike>()

async function getTauriStore(filename: string): Promise<LazyStoreLike> {
    const cached = tauriStores.get(filename)
    if (cached) return cached
    // 动态导入，避免在浏览器环境中加载 Tauri 插件
    const mod = await import('@tauri-apps/plugin-store')
    const store = new mod.LazyStore(filename)
    const wrapped = store as unknown as LazyStoreLike
    tauriStores.set(filename, wrapped)
    return wrapped
}

export const storage = {
    async get(filename: string, key: string): Promise<unknown> {
        if (isTauri()) {
            try {
                const store = await getTauriStore(filename)
                return await store.get(key)
            } catch (e) {
                console.error('[PakePlus] Tauri store get 失败 (' + filename + '/' + key + '):', e)
                return null
            }
        }
        try {
            const raw = localStorage.getItem(LS_PREFIX + filename + ':' + key)
            return raw ? JSON.parse(raw) : null
        } catch {
            return null
        }
    },

    async set(filename: string, key: string, value: unknown): Promise<void> {
        if (isTauri()) {
            try {
                const store = await getTauriStore(filename)
                await store.set(key, value)
            } catch (e) {
                console.error('[PakePlus] Tauri store set 失败 (' + filename + '/' + key + '):', e)
            }
            return
        }
        try {
            localStorage.setItem(LS_PREFIX + filename + ':' + key, JSON.stringify(value))
        } catch (e) {
            console.error('[PakePlus] localStorage set 失败 (' + filename + '/' + key + '):', e)
        }
    },

    async save(filename: string): Promise<void> {
        if (isTauri()) {
            try {
                const store = await getTauriStore(filename)
                await store.save()
            } catch (e) {
                console.error('[PakePlus] Tauri store save 失败 (' + filename + '):', e)
            }
        }
    },
}

export const isTauriEnv = isTauri()
