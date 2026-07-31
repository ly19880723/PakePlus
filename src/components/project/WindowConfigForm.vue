<template>
    <div>
        <div class="form-section-title">{{ t('config.window') }}</div>
        <el-form label-width="140px">
            <el-row :gutter="20">
                <el-col :span="12">
                    <el-form-item :label="t('window.title')">
                        <el-input v-model="win.title" />
                    </el-form-item>
                </el-col>
                <el-col :span="12">
                    <el-form-item :label="t('window.titleBarStyle')">
                        <el-select v-model="win.titleBarStyle" style="width: 100%">
                            <el-option label="Visible" value="Visible" />
                            <el-option label="Transparent" value="Transparent" />
                            <el-option label="Overlay" value="Overlay" />
                        </el-select>
                    </el-form-item>
                </el-col>
            </el-row>

            <el-form-item :label="t('window.userAgent')">
                <el-input v-model="win.userAgent" />
            </el-form-item>

            <el-row :gutter="20">
                <el-col :span="6">
                    <el-form-item :label="t('config.width')">
                        <el-input-number v-model="win.width" :min="200" :max="3840" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('config.height')">
                        <el-input-number v-model="win.height" :min="200" :max="2160" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.minWidth')">
                        <el-input-number v-model="win.minWidth" :min="0" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.minHeight')">
                        <el-input-number v-model="win.minHeight" :min="0" />
                    </el-form-item>
                </el-col>
            </el-row>

            <el-row :gutter="20">
                <el-col :span="6">
                    <el-form-item :label="t('window.maxWidth')">
                        <el-input-number v-model="win.maxWidth" :min="0" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.maxHeight')">
                        <el-input-number v-model="win.maxHeight" :min="0" />
                    </el-form-item>
                </el-col>
            </el-row>

            <!-- 开关选项 -->
            <div class="form-section-title" style="margin-top: 8px">窗口行为</div>
            <el-row :gutter="20">
                <el-col :span="6">
                    <el-form-item :label="t('window.resizable')">
                        <el-switch v-model="win.resizable" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.fullscreen')">
                        <el-switch v-model="win.fullscreen" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.maximized')">
                        <el-switch v-model="win.maximized" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.center')">
                        <el-switch v-model="win.center" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.decorations')">
                        <el-switch v-model="win.decorations" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.transparent')">
                        <el-switch v-model="win.transparent" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.alwaysOnTop')">
                        <el-switch v-model="win.alwaysOnTop" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.skipTaskbar')">
                        <el-switch v-model="win.skipTaskbar" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.devtools')">
                        <el-switch v-model="win.devtools" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.dragDrop')">
                        <el-switch v-model="win.dragDropEnabled" />
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item :label="t('window.zoomHotkeys')">
                        <el-switch v-model="win.zoomHotkeysEnabled" />
                    </el-form-item>
                </el-col>
            </el-row>
        </el-form>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { ProjectConfig } from '@/types'

const { t } = useI18n()
const props = defineProps<{ config: ProjectConfig }>()
const win = computed(() => props.config.more.windows)
</script>
