<template>
    <div>
        <div class="form-section-title">{{ t('icon.upload') }}</div>
        <p style="color: #909399; font-size: 13px; margin-bottom: 16px">{{ t('icon.tip') }}</p>

        <div class="icon-uploader">
            <!-- 图标预览/上传 -->
            <div class="icon-preview" @click="triggerUpload">
                <img v-if="config.icon" :src="config.icon" alt="icon" />
                <el-icon v-else size="32" color="#c0c4cc"><Plus /></el-icon>
                <input
                    ref="fileInput"
                    type="file"
                    accept="image/png,image/jpeg,image/svg+xml"
                    style="display: none"
                    @change="handleFileChange"
                />
            </div>

            <!-- 配置选项 -->
            <div style="flex: 1">
                <el-form label-width="100px">
                    <el-form-item :label="t('icon.rounded')">
                        <el-switch v-model="config.iconRound" />
                    </el-form-item>
                    <el-form-item>
                        <div style="display: flex; gap: 8px">
                            <el-button @click="generateIcons" :loading="generating" type="primary">
                                <el-icon><MagicStick /></el-icon>
                                {{ t('icon.generate') }}
                            </el-button>
                            <el-button v-if="config.icon" @click="config.icon = ''">
                                {{ t('icon.reset') }}
                            </el-button>
                        </div>
                    </el-form-item>
                </el-form>

                <!-- 图标尺寸预览 -->
                <div v-if="config.icon" style="margin-top: 16px">
                    <div style="font-size: 13px; color: #909399; margin-bottom: 8px">各尺寸预览</div>
                    <div style="display: flex; gap: 16px; align-items: flex-end">
                        <div v-for="size in [16, 32, 64, 128, 256]" :key="size" style="text-align: center">
                            <img
                                :src="config.icon"
                                :style="{
                                    width: size + 'px',
                                    height: size + 'px',
                                    borderRadius: config.iconRound ? '50%' : '8px',
                                    objectFit: 'contain',
                                    border: '1px solid #ebeef5',
                                }"
                            />
                            <div style="font-size: 11px; color: #909399; margin-top: 4px">{{ size }}px</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 生成结果 -->
        <div v-if="generatedIcons" style="margin-top: 20px">
            <el-alert :title="`已生成 ${Object.keys(generatedIcons).length} 个平台图标`" type="success" :closable="false" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import { Plus, MagicStick } from '@element-plus/icons-vue'
import { fileToBase64, generateAllIcons } from '@/utils/icon'
import type { ProjectConfig } from '@/types'

const { t } = useI18n()
const props = defineProps<{ config: ProjectConfig }>()

const fileInput = ref<HTMLInputElement>()
const generating = ref(false)
const generatedIcons = ref<Record<string, string> | null>(null)

function triggerUpload() {
    fileInput.value?.click()
}

async function handleFileChange(e: Event) {
    const input = e.target as HTMLInputElement
    if (!input.files || !input.files[0]) return
    const file = input.files[0]
    if (file.size > 5 * 1024 * 1024) {
        ElMessage.warning('图片大小不能超过5MB')
        return
    }
    const base64 = await fileToBase64(file)
    props.config.icon = base64
    ElMessage.success('图标已上传')
    input.value = ''
}

async function generateIcons() {
    if (!props.config.icon) {
        ElMessage.warning('请先上传图标')
        return
    }
    generating.value = true
    try {
        generatedIcons.value = await generateAllIcons(props.config.icon, props.config.iconRound)
        ElMessage.success(t('common.success'))
    } catch (e) {
        ElMessage.error('生成图标失败: ' + (e as Error).message)
    } finally {
        generating.value = false
    }
}
</script>
