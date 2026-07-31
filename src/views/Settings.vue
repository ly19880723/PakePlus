<template>
    <div style="max-width: 700px">
        <!-- GitHub 设置 -->
        <div class="pp-card" style="margin-bottom: 20px">
            <div class="form-section-title">{{ t('settings.github') }}</div>
            <el-form label-width="140px">
                <el-form-item :label="t('settings.token')">
                    <el-input
                        v-model="tokenInput"
                        type="password"
                        show-password
                        placeholder="ghp_xxxxxxxxxxxxxxxxxxxx"
                    />
                </el-form-item>
                <el-form-item :label="t('settings.tokenStatus')">
                    <el-tag :type="tokenTagType">
                        {{ tokenTagText }}
                    </el-tag>
                </el-form-item>
                <el-form-item>
                    <div style="display: flex; gap: 8px">
                        <el-button @click="verifyToken" :loading="verifying">
                            {{ t('settings.verify') }}
                        </el-button>
                        <el-button type="primary" @click="saveToken">
                            {{ t('settings.save') }}
                        </el-button>
                    </div>
                </el-form-item>
            </el-form>

            <el-alert type="info" :closable="false" style="margin-top: 8px">
                <template #title>
                    <div style="font-size: 13px; line-height: 1.8">
                        Token 权限要求（Classic Token）：<br />
                        - <strong>repo</strong>: Fork和管理仓库文件<br />
                        - <strong>workflow</strong>: 编译和发布应用<br /><br />
                        获取 Token：
                        <el-link type="primary" @click="openUrl('https://github.com/settings/tokens')">
                            GitHub Settings → Tokens (classic)
                        </el-link>
                    </div>
                </template>
            </el-alert>
        </div>

        <!-- 通用设置 -->
        <div class="pp-card" style="margin-bottom: 20px">
            <div class="form-section-title">{{ t('settings.general') }}</div>
            <el-form label-width="140px">
                <el-form-item :label="t('settings.language')">
                    <el-radio-group v-model="localeValue" @change="changeLocale">
                        <el-radio value="zh">中文</el-radio>
                        <el-radio value="en">English</el-radio>
                    </el-radio-group>
                </el-form-item>
            </el-form>
        </div>

        <!-- 关于 -->
        <div class="pp-card">
            <div class="form-section-title">{{ t('settings.about') }}</div>
            <el-descriptions :column="1" border>
                <el-descriptions-item :label="t('settings.version')">
                    PakePlus v{{ appVersion }}
                </el-descriptions-item>
                <el-descriptions-item label="开源协议">
                    MIT License
                </el-descriptions-item>
                <el-descriptions-item label="项目地址">
                    <el-link type="primary" @click="openUrl('https://github.com/Sjj1024/PakePlus')">
                        github.com/Sjj1024/PakePlus
                    </el-link>
                </el-descriptions-item>
                <el-descriptions-item label="技术栈">
                    Rust + Tauri v2 + Vue 3 + Element Plus
                </el-descriptions-item>
            </el-descriptions>
            <div style="margin-top: 16px">
                <el-button @click="checkUpdate" :loading="checkingUpdate">
                    <el-icon><Refresh /></el-icon>
                    {{ t('settings.checkUpdate') }}
                </el-button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh } from '@element-plus/icons-vue'
import { useSettingsStore } from '@/stores/settings'
import * as tauriApi from '@/api/tauri'
import { isTauriEnv } from '@/utils/storage'

// 仅在 Tauri 环境中动态导入 updater 和 process 插件
let check: (() => Promise<any>) | null = null
let relaunch: (() => Promise<void>) | null = null
if (isTauriEnv) {
    import('@tauri-apps/plugin-updater').then((m) => {
        check = m.check
    })
    import('@tauri-apps/plugin-process').then((m) => {
        relaunch = m.relaunch
    })
}

const { t, locale } = useI18n()
const settingsStore = useSettingsStore()

const tokenInput = ref(settingsStore.githubToken)
const verifying = ref(false)
const checkingUpdate = ref(false)
const localeValue = ref(settingsStore.locale)
const appVersion = ref('2.2.8')

const tokenTagType = computed(() => {
    if (settingsStore.tokenValid === true) return 'success'
    if (settingsStore.tokenValid === false) return 'danger'
    return 'info'
})

const tokenTagText = computed(() => {
    if (settingsStore.tokenValid === true) return `${t('settings.tokenValid')} (${settingsStore.username})`
    if (settingsStore.tokenValid === false) return t('settings.tokenInvalid')
    return t('settings.tokenEmpty')
})

async function verifyToken() {
    verifying.value = true
    try {
        await settingsStore.saveGithubToken(tokenInput.value)
        if (settingsStore.tokenValid) {
            ElMessage.success(t('settings.tokenValid') + ': ' + settingsStore.username)
        } else {
            ElMessage.error(t('settings.tokenInvalid'))
        }
    } catch {
        ElMessage.error(t('settings.tokenInvalid'))
    } finally {
        verifying.value = false
    }
}

async function saveToken() {
    await settingsStore.saveGithubToken(tokenInput.value)
    ElMessage.success(t('common.success'))
}

async function changeLocale(val: string) {
    locale.value = val as 'zh' | 'en'
    await settingsStore.saveLocale(val as 'zh' | 'en')
}

function openUrl(url: string) {
    tauriApi.openUrl(url)
}

async function checkUpdate() {
    if (!isTauriEnv) {
        ElMessage.warning('浏览器开发环境不支持检查更新')
        return
    }
    if (!check) {
        ElMessage.warning('更新插件尚未加载完成，请稍后再试')
        return
    }
    checkingUpdate.value = true
    try {
        const update = await check()
        if (update) {
            try {
                await ElMessageBox.confirm(
                    t('settings.updaterBody'),
                    t('settings.updaterTitle') + ` (v${update.version})`,
                    {
                        confirmButtonText: t('settings.updateNow'),
                        cancelButtonText: t('settings.later'),
                        type: 'success',
                    }
                )
                await update.downloadAndInstall()
                if (relaunch) await relaunch()
            } catch {
                // 用户取消
            }
        } else {
            ElMessage.success('当前已是最新版本')
        }
    } catch (e) {
        ElMessage.warning('检查更新失败: ' + (e as Error).message)
    } finally {
        checkingUpdate.value = false
    }
}
</script>
