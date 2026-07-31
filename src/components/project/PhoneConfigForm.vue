<template>
    <div>
        <!-- 安全区 -->
        <div class="form-section">
            <div class="form-section-title">{{ t('phone.safeArea') }}</div>
            <el-form label-width="100px" inline>
                <el-form-item :label="t('phone.safeArea') + '(Top)'">
                    <el-input-number v-model="phone.safeArea.top" :min="0" />
                </el-form-item>
                <el-form-item label="Bottom">
                    <el-input-number v-model="phone.safeArea.bottom" :min="0" />
                </el-form-item>
                <el-form-item label="Left">
                    <el-input-number v-model="phone.safeArea.left" :min="0" />
                </el-form-item>
                <el-form-item label="Right">
                    <el-input-number v-model="phone.safeArea.right" :min="0" />
                </el-form-item>
            </el-form>
        </div>

        <!-- 顶部导航 -->
        <div class="form-section">
            <div class="form-section-title">{{ t('phone.header') }}</div>
            <el-form label-width="120px">
                <el-form-item :label="t('phone.show')">
                    <el-switch v-model="phone.header.show" />
                </el-form-item>
                <template v-if="phone.header.show">
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item :label="t('phone.title')">
                                <el-input v-model="phone.header.title" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item :label="t('phone.fontSize')">
                                <el-input-number v-model="phone.header.fontSize" :min="10" :max="30" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item :label="t('phone.fontWeight')">
                                <el-select v-model="phone.header.fontWeight">
                                    <el-option label="normal" value="normal" />
                                    <el-option label="bold" value="bold" />
                                </el-select>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item :label="t('phone.backgroundColor')">
                                <el-color-picker v-model="phone.header.backgroundColor" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item :label="t('phone.color')">
                                <el-color-picker v-model="phone.header.color" />
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-form-item :label="'工具栏'">
                        <el-switch v-model="phone.header.toolBar" />
                    </el-form-item>
                </template>
            </el-form>
        </div>

        <!-- 底部标签栏 -->
        <div class="form-section">
            <div class="form-section-title">{{ t('phone.tabBar') }}</div>
            <el-form label-width="120px">
                <el-form-item :label="t('phone.show')">
                    <el-switch v-model="phone.tabBar.show" />
                </el-form-item>
                <template v-if="phone.tabBar.show">
                    <el-row :gutter="20">
                        <el-col :span="8">
                            <el-form-item :label="t('phone.backgroundColor')">
                                <el-color-picker v-model="phone.tabBar.backgroundColor" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="8">
                            <el-form-item :label="t('phone.color')">
                                <el-color-picker v-model="phone.tabBar.color" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="8">
                            <el-form-item :label="t('phone.activeColor')">
                                <el-color-picker v-model="phone.tabBar.activeColor" />
                            </el-form-item>
                        </el-col>
                    </el-row>

                    <!-- Tab 项列表 -->
                    <div v-for="(item, idx) in phone.tabBar.tabBarItem" :key="idx" style="display: flex; gap: 8px; margin-bottom: 8px">
                        <el-input v-model="item.title" placeholder="标题" style="flex: 1" />
                        <el-input v-model="item.icon" placeholder="图标URL" style="flex: 1" />
                        <el-input v-model="item.url" placeholder="跳转URL" style="flex: 1" />
                        <el-button text type="danger" @click="phone.tabBar.tabBarItem.splice(idx, 1)">
                            <el-icon><Delete /></el-icon>
                        </el-button>
                    </div>
                    <el-button @click="phone.tabBar.tabBarItem.push({ title: '', icon: '', url: '' })">
                        <el-icon><Plus /></el-icon>
                        {{ t('phone.addItem') }}
                    </el-button>
                </template>
            </el-form>
        </div>

        <!-- WebView 设置 -->
        <div class="form-section">
            <div class="form-section-title">{{ t('phone.webview') }}</div>
            <el-form label-width="140px">
                <el-form-item :label="t('phone.userAgent')">
                    <el-input v-model="phone.webview.userAgent" placeholder="留空使用默认" />
                </el-form-item>
                <el-row :gutter="20">
                    <el-col :span="6">
                        <el-form-item :label="t('phone.javaScript')">
                            <el-switch v-model="phone.webview.javaScriptEnabled" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="6">
                        <el-form-item :label="t('phone.domStorage')">
                            <el-switch v-model="phone.webview.domStorageEnabled" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="6">
                        <el-form-item :label="t('phone.allowFileAccess')">
                            <el-switch v-model="phone.webview.allowFileAccess" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="6">
                        <el-form-item :label="t('phone.clearCache')">
                            <el-switch v-model="phone.webview.clearCache" />
                        </el-form-item>
                    </el-col>
                </el-row>
            </el-form>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Delete, Plus } from '@element-plus/icons-vue'
import type { ProjectConfig } from '@/types'

const { t } = useI18n()
const props = defineProps<{ config: ProjectConfig }>()
const phone = computed(() => props.config.phone)
</script>
