<template>
    <div>
        <div class="form-section-title">{{ t('config.customJs') }}</div>
        <p style="color: #909399; font-size: 13px; margin-bottom: 12px">
            注入自定义 JavaScript 代码到目标网页，可用于隐藏广告、自动化操作、调用系统API等
        </p>
        <div ref="editorRef" class="js-editor"></div>

        <div style="margin-top: 12px; display: flex; gap: 8px">
            <el-button @click="insertTemplate('hideAd')">隐藏广告</el-button>
            <el-button @click="insertTemplate('injectStyle')">注入样式</el-button>
            <el-button @click="insertTemplate('systemApi')">系统API调用</el-button>
        </div>

        <div class="form-section" style="margin-top: 20px">
            <div class="form-section-title">{{ t('config.filterCss') }}</div>
            <el-input
                v-model="config.filterCss"
                type="textarea"
                :rows="3"
                placeholder="要过滤的CSS选择器，多个用逗号分隔，如：.ad, #banner, .sidebar"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { EditorState } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import { javascript } from '@codemirror/lang-javascript'
import { oneDark } from '@codemirror/theme-one-dark'
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
import { indentUnit } from '@codemirror/language'
import type { ProjectConfig } from '@/types'

const { t } = useI18n()
const props = defineProps<{ config: ProjectConfig }>()

const editorRef = ref<HTMLDivElement>()
let editorView: EditorView | null = null

const TEMPLATES: Record<string, string> = {
    hideAd: `// 隐藏广告元素
const adSelectors = ['.ad', '#banner', '.advertisement', '[class*="ad-"]'];
adSelectors.forEach(sel => {
    document.querySelectorAll(sel).forEach(el => {
        el.style.display = 'none';
    });
});`,
    injectStyle: `// 注入自定义样式
const style = document.createElement('style');
style.textContent = \`
    body { font-size: 16px !important; }
    .header { background: #1890ff !important; }
\`;
document.head.appendChild(style);`,
    systemApi: `// 调用系统级API（需要开启Tauri API）
// 下载文件示例
async function downloadFile(url, name) {
    try {
        const { invoke } = window.__TAURI__.core;
        await invoke('download_file', { url, savePath: '', fileId: name });
    } catch(e) {
        console.error('下载失败:', e);
    }
}`,
}

onMounted(() => {
    if (!editorRef.value) return
    editorView = new EditorView({
        state: EditorState.create({
            doc: props.config.customJs || '',
            extensions: [
                history(),
                keymap.of([...defaultKeymap, ...historyKeymap]),
                javascript(),
                oneDark,
                indentUnit.of('    '),
                EditorView.lineWrapping,
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        props.config.customJs = update.state.doc.toString()
                    }
                }),
            ],
        }),
        parent: editorRef.value,
    })
})

watch(
    () => props.config,
    () => {
        // 外部更新时不同步编辑器，避免光标跳动
    }
)

function insertTemplate(key: string) {
    if (!editorView) return
    const template = TEMPLATES[key]
    const transaction = editorView.state.replaceSelection(template + '\n')
    editorView.dispatch(transaction)
    props.config.customJs = editorView.state.doc.toString()
}
</script>

<style scoped>
.js-editor {
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid #ebeef5;
}
.js-editor :deep(.cm-editor) {
    height: 320px;
    font-size: 13px;
}
</style>
