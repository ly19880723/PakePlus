// 设置 Store - GitHub Token、语言等
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { storage } from '@/utils/storage'
import * as githubApi from '@/api/github'

const STORE_FILE = 'settings.json'

export const useSettingsStore = defineStore('settings', () => {
    const githubToken = ref('')
    const username = ref('')
    const tokenValid = ref<boolean | null>(null)
    const locale = ref<'zh' | 'en'>('zh')

    // 初始化加载设置
    async function loadSettings() {
        try {
            githubToken.value = ((await storage.get(STORE_FILE, 'githubToken')) as string) || ''
            username.value = ((await storage.get(STORE_FILE, 'username')) as string) || ''
            locale.value = ((await storage.get(STORE_FILE, 'locale')) as 'zh' | 'en') || 'zh'
        } catch (e) {
            console.error('[PakePlus] 加载设置失败:', e)
            // 默认值
        }
    }

    // 保存 GitHub Token
    async function saveGithubToken(token: string) {
        githubToken.value = token
        try {
            await storage.set(STORE_FILE, 'githubToken', token)
            await storage.save(STORE_FILE)
        } catch (e) {
            console.error('[PakePlus] 保存 GitHub Token 失败:', e)
        }
        await verifyToken()
    }

    // 保存语言设置
    async function saveLocale(lang: 'zh' | 'en') {
        locale.value = lang
        try {
            await storage.set(STORE_FILE, 'locale', lang)
            await storage.save(STORE_FILE)
        } catch (e) {
            console.error('[PakePlus] 保存语言设置失败:', e)
        }
    }

    // 验证 Token
    async function verifyToken(): Promise<boolean> {
        if (!githubToken.value) {
            tokenValid.value = false
            return false
        }
        try {
            const user = await githubApi.getUser(githubToken.value)
            username.value = user.login
            await storage.set(STORE_FILE, 'username', user.login)
            await storage.save(STORE_FILE)
            tokenValid.value = true
            return true
        } catch (e) {
            tokenValid.value = false
            return false
        }
    }

    return {
        githubToken,
        username,
        tokenValid,
        locale,
        loadSettings,
        saveGithubToken,
        saveLocale,
        verifyToken,
    }
})
