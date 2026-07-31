<template>
    <div>
        <!-- 基础信息 -->
        <div class="form-section">
            <div class="form-section-title">{{ t('config.basic') }}</div>
            <el-form label-width="120px" label-position="right">
                <el-row :gutter="20">
                    <el-col :span="12">
                        <el-form-item :label="t('config.name')">
                            <el-input v-model="config.name" placeholder="MyApp" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="12">
                        <el-form-item :label="t('config.showName')">
                            <el-input v-model="config.showName" placeholder="我的应用" />
                        </el-form-item>
                    </el-col>
                </el-row>
                <el-row :gutter="20">
                    <el-col :span="12">
                        <el-form-item :label="t('config.url')">
                            <el-input v-model="config.url" placeholder="https://example.com" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="6">
                        <el-form-item :label="t('config.version')">
                            <el-input v-model="config.version" placeholder="0.0.1" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="6">
                        <el-form-item :label="t('config.appid')">
                            <el-input v-model="config.appid" placeholder="com.example.app" />
                        </el-form-item>
                    </el-col>
                </el-row>
                <el-form-item :label="t('config.desc')">
                    <el-input v-model="config.desc" type="textarea" :rows="2" />
                </el-form-item>
            </el-form>
        </div>

        <!-- 窗口尺寸 -->
        <div class="form-section">
            <div class="form-section-title">{{ t('config.previewMode') }}</div>
            <el-form label-width="120px">
                <el-row :gutter="20">
                    <el-col :span="8">
                        <el-form-item :label="t('config.width')">
                            <el-input-number v-model="config.width" :min="200" :max="3840" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.height')">
                            <el-input-number v-model="config.height" :min="200" :max="2160" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.previewMode')">
                            <el-radio-group v-model="config.preview">
                                <el-radio-button value="desktop">{{ t('config.desktop') }}</el-radio-button>
                            <el-radio-button value="phone">{{ t('config.mobile') }}</el-radio-button>
                            </el-radio-group>
                        </el-form-item>
                    </el-col>
                </el-row>
            </el-form>
        </div>

        <!-- 功能开关 -->
        <div class="form-section">
            <div class="form-section-title">功能选项</div>
            <el-form label-width="160px">
                <el-row :gutter="20">
                    <el-col :span="8">
                        <el-form-item :label="t('config.iconRound')">
                            <el-switch v-model="config.iconRound" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.state')">
                            <el-switch v-model="config.state" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.single')">
                            <el-switch v-model="config.single" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.injectJq')">
                            <el-switch v-model="config.injectJq" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.tauriApi')">
                            <el-switch v-model="config.tauriApi" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="8">
                        <el-form-item :label="t('config.devbug')">
                            <el-switch v-model="config.devbug" />
                        </el-form-item>
                    </el-col>
                </el-row>
            </el-form>
        </div>

        <!-- 静态文件打包 -->
        <div class="form-section">
            <div class="form-section-title">静态文件打包</div>
            <el-form label-width="160px">
                <el-form-item :label="t('config.isHtml')">
                    <el-switch v-model="config.isHtml" />
                    <span style="margin-left: 12px; color: #909399; font-size: 13px">
                        打包本地 Vue/React 编译后的 dist 目录
                    </span>
                </el-form-item>
                <el-form-item v-if="config.isHtml" :label="t('config.htmlPath')">
                    <el-input v-model="config.htmlPath" placeholder="选择本地 dist 目录" readonly>
                        <template #append>
                            <el-button @click="selectHtmlDir">{{ t('config.selectDir') }}</el-button>
                        </template>
                    </el-input>
                </el-form-item>
            </el-form>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import { isTauriEnv } from '@/utils/storage'
import type { ProjectConfig } from '@/types'

const { t } = useI18n()
const props = defineProps<{ config: ProjectConfig }>()

async function selectHtmlDir() {
    if (!isTauriEnv) {
        const dir = window.prompt('请输入静态文件目录路径（浏览器开发模式）')
        if (dir) props.config.htmlPath = dir
        return
    }
    const selected = await open({ directory: true, multiple: false })
    if (selected) {
        props.config.htmlPath = selected as string
    }
}
</script>
