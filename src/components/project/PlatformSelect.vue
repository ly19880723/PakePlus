<template>
    <div>
        <div class="form-section-title">{{ t('config.platform') }}</div>
        <p style="color: #909399; font-size: 13px; margin-bottom: 16px">
            选择需要打包的目标平台，云端打包支持全部平台，本地打包仅支持当前系统
        </p>

        <div class="platform-grid">
            <div
                v-for="p in platforms"
                :key="p.key"
                class="platform-item"
                :class="{ active: isSelected(p.key) }"
                @click="togglePlatform(p.key)"
            >
                <el-icon size="24" style="margin-bottom: 8px">
                    <component :is="p.icon" />
                </el-icon>
                <div style="font-size: 13px; font-weight: 500">{{ p.label }}</div>
                <div style="font-size: 11px; color: #909399; margin-top: 2px">{{ p.desc }}</div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { ProjectConfig, PlatformKey } from '@/types'

const { t } = useI18n()
const props = defineProps<{ config: ProjectConfig }>()

const platforms = [
    { key: '1-1' as PlatformKey, label: 'Windows x64', desc: 'Intel/AMD 64位', icon: 'Monitor' },
    { key: '1-2' as PlatformKey, label: 'Windows ARM', desc: 'ARM64架构', icon: 'Monitor' },
    { key: '2-1' as PlatformKey, label: 'macOS Intel', desc: 'Intel芯片', icon: 'Platform' },
    { key: '2-2' as PlatformKey, label: 'macOS ARM', desc: 'M1/M2/M3芯片', icon: 'Platform' },
    { key: '3-1' as PlatformKey, label: 'Linux x64', desc: 'Intel/AMD 64位', icon: 'Platform' },
    { key: '3-2' as PlatformKey, label: 'Linux ARM', desc: 'ARM64架构', icon: 'Platform' },
    { key: '4-1' as PlatformKey, label: 'Android', desc: 'APK安装包', icon: 'Cellphone' },
    { key: '4-2' as PlatformKey, label: 'iOS', desc: 'IPA安装包', icon: 'Cellphone' },
]

function isSelected(key: PlatformKey): boolean {
    return props.config.platform.includes(key)
}

function togglePlatform(key: PlatformKey) {
    const idx = props.config.platform.indexOf(key)
    if (idx === -1) {
        props.config.platform.push(key)
    } else {
        props.config.platform.splice(idx, 1)
    }
}
</script>
