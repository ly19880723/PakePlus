<template>
    <div class="pp-layout">
        <!-- 侧边栏 -->
        <div class="pp-sidebar" :class="{ collapsed: collapsed }">
            <div class="pp-logo">
                <img src="/app.svg" alt="logo" />
                <span v-if="!collapsed">PakePlus</span>
            </div>
            <div class="pp-menu">
                <div
                    v-for="item in menuItems"
                    :key="item.path"
                    class="pp-menu-item"
                    :class="{ active: isActive(item.path) }"
                    @click="navigate(item.path)"
                >
                    <el-icon><component :is="item.icon" /></el-icon>
                    <span v-if="!collapsed">{{ item.label }}</span>
                </div>
            </div>
            <div class="pp-menu" style="flex: 0">
                <div class="pp-menu-item" @click="toggleCollapse">
                    <el-icon><Fold v-if="!collapsed" /><Expand v-else /></el-icon>
                    <span v-if="!collapsed">收起</span>
                </div>
            </div>
        </div>
        <!-- 主内容 -->
        <div class="pp-main">
            <div class="pp-header">
                <div style="display: flex; align-items: center; gap: 12px">
                    <el-button text @click="goBack" v-if="showBack">
                        <el-icon><ArrowLeft /></el-icon>
                        返回
                    </el-button>
                    <h3 style="font-weight: 600">{{ pageTitle }}</h3>
                </div>
                <div style="display: flex; align-items: center; gap: 12px">
                    <el-tag :type="settingsStore.tokenValid ? 'success' : 'info'" size="small">
                        {{ settingsStore.tokenValid ? 'Token已验证' : 'Token未设置' }}
                    </el-tag>
                    <el-dropdown @command="changeLocale">
                        <el-button text>
                            <el-icon><Promotion /></el-icon>
                            {{ locale === 'zh' ? '中文' : 'English' }}
                        </el-button>
                        <template #dropdown>
                            <el-dropdown-menu>
                                <el-dropdown-item command="zh">中文</el-dropdown-item>
                                <el-dropdown-item command="en">English</el-dropdown-item>
                            </el-dropdown-menu>
                        </template>
                    </el-dropdown>
                </div>
            </div>
            <div class="pp-content">
                <router-view />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '@/stores/settings'

const route = useRoute()
const router = useRouter()
const { t, locale } = useI18n()
const settingsStore = useSettingsStore()
const collapsed = ref(false)

const menuItems = computed(() => [
    { path: '/', label: t('menu.home'), icon: 'HomeFilled' },
    { path: '/settings', label: t('menu.settings'), icon: 'Setting' },
])

const pageTitle = computed(() => {
    if (route.name === 'home') return t('menu.home')
    if (route.name === 'edit') return t('menu.edit')
    if (route.name === 'build') return t('menu.build')
    if (route.name === 'settings') return t('menu.settings')
    return ''
})

const showBack = computed(() => route.name === 'edit' || route.name === 'build')

function isActive(path: string): boolean {
    return route.path === path
}

function navigate(path: string) {
    router.push(path)
}

function goBack() {
    router.push('/')
}

function toggleCollapse() {
    collapsed.value = !collapsed.value
}

async function changeLocale(cmd: string) {
    locale.value = cmd as 'zh' | 'en'
    await settingsStore.saveLocale(cmd as 'zh' | 'en')
}
</script>
