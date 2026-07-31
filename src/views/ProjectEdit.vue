<template>
    <div class="pp-card" v-if="project">
        <!-- 顶部操作 -->
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px">
            <div>
                <h2 style="font-size: 20px; font-weight: 600">{{ config.name || '未命名项目' }}</h2>
                <p style="color: #909399; font-size: 13px; margin-top: 4px">{{ config.url || '未设置URL' }}</p>
            </div>
            <div style="display: flex; gap: 8px">
                <el-button @click="handlePreview">
                    <el-icon><View /></el-icon>
                    {{ t('config.preview') }}
                </el-button>
                <el-button type="primary" @click="handleSave">
                    <el-icon><Check /></el-icon>
                    {{ t('config.save') }}
                </el-button>
            </div>
        </div>

        <!-- 配置标签页 -->
        <el-tabs v-model="activeTab" class="config-tabs">
            <el-tab-pane :label="t('config.basic')" name="basic">
                <BasicConfig :config="config" />
            </el-tab-pane>
            <el-tab-pane :label="t('config.icon')" name="icon">
                <IconEditor :config="config" />
            </el-tab-pane>
            <el-tab-pane :label="t('config.window')" name="window">
                <WindowConfigForm :config="config" />
            </el-tab-pane>
            <el-tab-pane :label="t('config.phone')" name="phone">
                <PhoneConfigForm :config="config" />
            </el-tab-pane>
            <el-tab-pane :label="t('config.jsInject')" name="js">
                <JsEditor :config="config" />
            </el-tab-pane>
            <el-tab-pane :label="t('config.platform')" name="platform">
                <PlatformSelect :config="config" />
            </el-tab-pane>
        </el-tabs>
    </div>
    <div v-else class="empty-state">
        <el-icon><Warning /></el-icon>
        <p v-if="projectStore.loading">项目加载中...</p>
        <p v-else>项目不存在</p>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import { View, Check, Warning } from '@element-plus/icons-vue'
import { useProjectStore } from '@/stores/project'
import * as tauriApi from '@/api/tauri'
import BasicConfig from '@/components/project/BasicConfig.vue'
import IconEditor from '@/components/project/IconEditor.vue'
import WindowConfigForm from '@/components/project/WindowConfigForm.vue'
import PhoneConfigForm from '@/components/project/PhoneConfigForm.vue'
import JsEditor from '@/components/project/JsEditor.vue'
import PlatformSelect from '@/components/project/PlatformSelect.vue'
import { createDefaultConfig, type ProjectConfig } from '@/types'

const route = useRoute()
const router = useRouter()
const { t } = useI18n()
const projectStore = useProjectStore()

const activeTab = ref('basic')
// 初始化为默认配置，避免 null 引用
const config = ref<ProjectConfig>(createDefaultConfig())

const project = computed(() => projectStore.getProject(route.params.id as string))

// 监听 project 变化（异步加载完成后会赋值），同步一份 config 副本用于编辑
watch(
    project,
    (p) => {
        if (p) {
            config.value = JSON.parse(JSON.stringify(p.config))
        }
    },
    { immediate: true }
)

async function handleSave() {
    if (project.value) {
        await projectStore.updateProject(project.value.id, config.value)
        ElMessage.success(t('config.saved'))
    }
}

async function handlePreview() {
    try {
        const win = config.value.more.windows
        win.title = config.value.showName || config.value.name
        win.width = config.value.width
        win.height = config.value.height

        if (config.value.isHtml && config.value.htmlPath) {
            // HTML 模式：先启动本地静态文件服务器
            const port = await tauriApi.startServer(config.value.htmlPath, 0)
            win.url = `http://127.0.0.1:${port}`
            win.userAgent = config.value.more.windows.userAgent || ''
        } else {
            // URL 模式：直接使用用户配置的 URL，确保包含协议前缀
            let url = config.value.url || ''
            if (url && !/^https?:\/\//i.test(url)) {
                url = 'https://' + url
            }
            win.url = url
        }

        console.log('[PakePlus] 预览配置:', JSON.stringify({
            url: win.url,
            title: win.title,
            width: win.width,
            height: win.height,
        }))

        await tauriApi.previewFromConfig(
            false,
            win as unknown as Record<string, unknown>,
            config.value.customJs || '',
            config.value.devbug,
            config.value.icon || ''
        )
        ElMessage.success('预览窗口已打开')
    } catch (e) {
        ElMessage.error('预览失败: ' + (e as Error).message)
    }
}
</script>
