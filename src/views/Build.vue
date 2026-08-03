<template>
    <div v-if="project">
        <!-- 打包方式选择 -->
        <el-tabs v-model="buildMode" class="config-tabs">
            <!-- 本地打包 -->
            <el-tab-pane :label="t('build.local')" name="local">
                <div class="pp-card">
                    <el-alert :title="t('build.localTip')" type="info" :closable="false" show-icon style="margin-bottom: 16px" />

                    <el-form label-width="120px">
                        <el-form-item :label="t('build.targetDir')">
                            <el-input v-model="localConfig.targetDir" placeholder="选择输出目录" readonly>
                                <template #append>
                                    <el-button @click="selectTargetDir">{{ t('build.selectDir') }}</el-button>
                                </template>
                            </el-input>
                        </el-form-item>

                        <el-form-item label="调试模式">
                            <el-switch v-model="localConfig.debug" />
                            <span style="margin-left: 12px; color: #909399; font-size: 13px">开启后打包的应用包含 vConsole 调试工具</span>
                        </el-form-item>

                        <el-form-item>
                            <el-button
                                type="primary"
                                @click="startLocalBuild"
                                :loading="localBuilding"
                                :disabled="!canLocalBuild"
                            >
                                <el-icon><Promotion /></el-icon>
                                {{ localBuilding ? t('build.building') : t('build.start') }}
                            </el-button>
                            <el-button v-if="localConfig.targetDir" @click="openOutputDir">
                                <el-icon><FolderOpened /></el-icon>
                                {{ t('build.openOutput') }}
                            </el-button>
                        </el-form-item>
                    </el-form>

                    <!-- 进度条 -->
                    <div v-if="localBuilding || localProgress > 0" class="build-progress">
                        <el-progress :percentage="localProgress" :status="localProgress === 100 ? 'success' : ''" />
                    </div>

                    <!-- 日志 -->
                    <div v-if="buildLogs.length" class="log-box" style="margin-top: 16px">
                        <div v-for="(log, i) in buildLogs" :key="i" :class="log.type">
                            {{ log.text }}
                        </div>
                    </div>
                </div>
            </el-tab-pane>

            <!-- 云端打包 -->
            <el-tab-pane :label="t('build.cloud')" name="cloud">
                <div class="pp-card">
                    <el-alert :title="t('build.cloudTip')" type="info" :closable="false" show-icon style="margin-bottom: 16px" />

                    <!-- Token 状态 -->
                    <div v-if="!settingsStore.githubToken" style="margin-bottom: 16px">
                        <el-alert title="尚未配置 GitHub Token" type="warning" :closable="false" show-icon>
                            <template #default>
                                云端打包需要 GitHub Token，
                                <el-link type="primary" @click="$router.push('/settings')">前往设置</el-link>
                            </template>
                        </el-alert>
                    </div>

                    <template v-else>
                        <!-- Fork 状态 -->
                        <div class="build-status-card">
                            <div style="display: flex; align-items: center; gap: 12px">
                                <el-icon size="20" :color="forkStatus === 'forked' ? '#67c23a' : '#909399'">
                                    <CircleCheck v-if="forkStatus === 'forked'" />
                                    <Loading v-else-if="forkStatus === 'checking'" />
                                    <CircleClose v-else />
                                </el-icon>
                                <div>
                                    <div style="font-weight: 500">{{ t('build.forkRepo') }}</div>
                                    <div style="font-size: 12px; color: #909399">
                                        {{ forkStatus === 'forked' ? `已Fork: ${settingsStore.username}/PakePlus` : '需要Fork模板仓库' }}
                                    </div>
                                </div>
                            </div>
                            <el-button
                                v-if="forkStatus === 'not-forked'"
                                type="primary"
                                @click="doFork"
                                :loading="forking"
                            >
                                {{ t('build.fork') }}
                            </el-button>
                        </div>

                        <!-- 平台选择 -->
                        <div class="form-section">
                            <div class="form-section-title">{{ t('build.platform') }}</div>
                            <el-checkbox-group v-model="cloudPlatforms">
                                <el-space wrap>
                                    <el-checkbox value="build_windows_x86_64">{{ t('build.winX64') }}</el-checkbox>
                                    <el-checkbox value="build_windows_aarch64">{{ t('build.winArm') }}</el-checkbox>
                                    <el-checkbox value="build_macos_x86_64">{{ t('build.macX64') }}</el-checkbox>
                                    <el-checkbox value="build_macos_aarch64">{{ t('build.macArm') }}</el-checkbox>
                                    <el-checkbox value="build_linux_x86_64">{{ t('build.linuxX64') }}</el-checkbox>
                                    <el-checkbox value="build_linux_aarch64">{{ t('build.linuxArm') }}</el-checkbox>
                                </el-space>
                            </el-checkbox-group>
                        </div>

                        <!-- 触发构建 -->
                        <el-button
                            type="primary"
                            @click="startCloudBuild"
                            :loading="cloudBuilding"
                            :disabled="forkStatus !== 'forked' || cloudPlatforms.length === 0"
                        >
                            <el-icon><Promotion /></el-icon>
                            {{ t('build.triggerBuild') }}
                        </el-button>
                    </template>

                    <!-- 构建历史 -->
                    <div v-if="workflowRuns.length" style="margin-top: 24px">
                        <div class="form-section-title">{{ t('build.buildStatus') }}</div>
                        <div v-for="run in workflowRuns" :key="run.id" class="build-status-card">
                            <div style="display: flex; align-items: center; gap: 12px">
                                <el-tag :type="runStatusType(run.status, run.conclusion)" size="small">
                                    {{ runStatusText(run.status, run.conclusion) }}
                                </el-tag>
                                <div>
                                    <div style="font-size: 14px">{{ run.name }}</div>
                                    <div style="font-size: 12px; color: #909399">{{ run.head_branch }} · {{ formatTime(run.created_at) }}</div>
                                </div>
                            </div>
                            <el-button text @click="openRunUrl(run.html_url)">
                                <el-icon><Link /></el-icon>
                                {{ t('build.viewBuild') }}
                            </el-button>
                        </div>
                    </div>

                    <!-- Artifacts 下载 -->
                    <div v-if="artifacts.length" style="margin-top: 24px">
                        <div class="form-section-title">{{ t('build.downloadApp') }}</div>
                        <div v-for="art in artifacts" :key="art.id" class="build-status-card">
                            <div style="display: flex; align-items: center; gap: 12px; flex: 1">
                                <div>
                                    <div style="font-size: 14px; font-weight: 500">{{ art.name }}</div>
                                    <div style="font-size: 12px; color: #909399">
                                        {{ formatSize(art.size_in_bytes) }} · {{ formatTime(art.created_at) }}
                                    </div>
                                </div>
                            </div>
                            <el-button type="primary" size="small" @click="downloadArtifact(art)">
                                <el-icon><Download /></el-icon>
                                下载
                            </el-button>
                        </div>
                    </div>

                    <!-- Release 下载（兼容） -->
                    <div v-if="releases.length && !artifacts.length" style="margin-top: 24px">
                        <div class="form-section-title">{{ t('build.downloadApp') }}</div>
                        <div v-for="rel in releases" :key="rel.id" class="build-status-card" style="flex-direction: column; align-items: flex-start">
                            <div style="font-weight: 600; font-size: 15px">{{ rel.tag_name }}</div>
                            <div style="font-size: 13px; color: #909399; margin: 4px 0">{{ rel.name }}</div>
                            <div style="display: flex; flex-wrap: wrap; gap: 8px; margin-top: 8px">
                                <el-button
                                    v-for="asset in rel.assets"
                                    :key="asset.id"
                                    size="small"
                                    @click="downloadAsset(asset)"
                                >
                                    <el-icon><Download /></el-icon>
                                    {{ asset.name }} ({{ formatSize(asset.size) }})
                                </el-button>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import {
    Promotion, FolderOpened, CircleCheck, CircleClose, Loading, Link, Download,
} from '@element-plus/icons-vue'
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { useProjectStore } from '@/stores/project'
import { useSettingsStore } from '@/stores/settings'
import * as tauriApi from '@/api/tauri'
import * as githubApi from '@/api/github'
import { isTauriEnv } from '@/utils/storage'
import type { WorkflowRun, GitHubRelease, ReleaseAsset, GitHubArtifact } from '@/types'

const route = useRoute()
const { t } = useI18n()
const projectStore = useProjectStore()
const settingsStore = useSettingsStore()

const project = computed(() => projectStore.getProject(route.params.id as string))

const buildMode = ref('local')

// 本地打包
const localConfig = ref({ targetDir: '', debug: false })
const localBuilding = ref(false)
const localProgress = ref(0)
const buildLogs = ref<{ text: string; type: string }[]>([])

// 云端打包
const forkStatus = ref<'checking' | 'forked' | 'not-forked'>('checking')
const forking = ref(false)
const cloudBuilding = ref(false)
const cloudPlatforms = ref<string[]>([
    'build_windows_x86_64',
    'build_macos_aarch64',
])
const workflowRuns = ref<WorkflowRun[]>([])
const releases = ref<GitHubRelease[]>([])
const artifacts = ref<GitHubArtifact[]>([])

let unlistenProgress: UnlistenFn | null = null
let pollTimer: ReturnType<typeof setInterval> | null = null

const canLocalBuild = computed(() => {
    return project.value && (project.value.config.url || project.value.config.isHtml) && localConfig.value.targetDir
})

onMounted(async () => {
    // 监听本地构建进度
    unlistenProgress = await tauriApi.onLocalProgress((progress) => {
        localProgress.value = parseInt(progress)
        addLog(`构建进度: ${progress}%`, 'log-info')
        if (parseInt(progress) === 100) {
            localBuilding.value = false
            addLog(t('build.success'), 'log-success')
            tauriApi.notification('PakePlus', t('build.success'))
        }
    })

    // 检查 fork 状态
    if (settingsStore.githubToken && settingsStore.username) {
        await checkForkStatus()
        // 启动轮询机制
        startPolling()
    }
})

onUnmounted(() => {
    unlistenProgress?.()
    stopPolling()
})

function addLog(text: string, type: string = 'log-info') {
    buildLogs.value.push({ text, type })
}

async function selectTargetDir() {
    if (!isTauriEnv) {
        // 浏览器环境降级：使用 prompt
        const dir = window.prompt('请输入输出目录路径（浏览器开发模式）')
        if (dir) localConfig.value.targetDir = dir
        return
    }
    const selected = await openDialog({ directory: true, multiple: false })
    if (selected) {
        localConfig.value.targetDir = selected as string
    }
}

async function startLocalBuild() {
    if (!project.value) return
    const cfg = project.value.config

    localBuilding.value = true
    localProgress.value = 0
    buildLogs.value = []
    addLog('开始本地打包...', 'log-info')
    addLog(`应用名称: ${cfg.name || '未命名'}`, 'log-info')
    addLog(`目标目录: ${localConfig.value.targetDir}`, 'log-info')

    try {
        // 同步配置
        const win = cfg.more.windows
        win.title = cfg.showName || cfg.name || 'MyApp'
        if (cfg.isHtml && cfg.htmlPath) {
            // HTML 模式：先启动本地服务器获取 URL
            const port = await tauriApi.startServer(cfg.htmlPath, 0)
            win.url = `http://127.0.0.1:${port}`
        } else {
            // URL 模式：补全协议前缀
            let url = cfg.url || ''
            if (url && !/^https?:\/\//i.test(url)) {
                url = 'https://' + url
            }
            win.url = url
        }
        win.width = cfg.width
        win.height = cfg.height
        addLog(`目标URL: ${win.url}`, 'log-info')

        await tauriApi.buildLocal(
            localConfig.value.targetDir,
            cfg.name || 'MyApp',
            cfg.name || 'MyApp',
            win as unknown as Record<string, unknown>,
            cfg.icon ? cfg.icon.replace('data:image/png;base64,', '') : '',
            localConfig.value.debug,
            cfg.customJs || '',
            cfg.isHtml ? cfg.htmlPath : ''
        )

        if (localProgress.value === 0) {
            localProgress.value = 100
            localBuilding.value = false
            addLog(t('build.success'), 'log-success')
            ElMessage.success(t('build.success'))
            tauriApi.notification('PakePlus', t('build.success'))
        }
    } catch (e) {
        localBuilding.value = false
        let msg: string
        if (typeof e === 'string') {
            msg = e
        } else if (e instanceof Error && e.message) {
            msg = e.message
        } else {
            try {
                msg = JSON.stringify(e)
            } catch {
                msg = String(e)
            }
        }
        addLog(t('build.failed') + ': ' + msg, 'log-error')
        ElMessage.error(t('build.failed') + ': ' + msg)
        tauriApi.notification('PakePlus', t('build.failed'))
    }
}

async function openOutputDir() {
    if (localConfig.value.targetDir) {
        await tauriApi.openUrl(localConfig.value.targetDir)
    }
}

async function checkForkStatus() {
    forkStatus.value = 'checking'
    try {
        const repo = await githubApi.checkFork(settingsStore.githubToken, settingsStore.username)
        forkStatus.value = repo ? 'forked' : 'not-forked'
        if (repo) {
            await loadWorkflowRuns()
            await loadReleases()
        }
    } catch {
        forkStatus.value = 'not-forked'
    }
}

async function doFork() {
    forking.value = true
    try {
        await githubApi.forkRepo(settingsStore.githubToken)
        await githubApi.starRepo(settingsStore.githubToken, 'Sjj1024', 'PakePlus')
        ElMessage.success('Fork 成功，请等待几秒后重新检查')
        setTimeout(() => checkForkStatus(), 5000)
    } catch (e) {
        ElMessage.error('Fork 失败: ' + (e as Error).message)
    } finally {
        forking.value = false
    }
}

async function startCloudBuild() {
    if (!project.value) return
    cloudBuilding.value = true
    try {
        const token = settingsStore.githubToken
        const owner = settingsStore.username
        const repo = 'PakePlus'

        // 获取 fork 的默认分支
        const repoInfo = await githubApi.getRepo(token, owner, repo)
        const defaultBranch = repoInfo.default_branch || 'main'
        addLog(`使用分支: ${defaultBranch}`, 'log-info')

        // 先确保 workflow 文件是最新的（从本地上传到 fork）
        let syncOk = false
        try {
            const localBuildYml = await tauriApi.getWorkflowYml()
            const wfSha = await githubApi.getFileSha(token, owner, repo, '.github/workflows/build.yml', defaultBranch)
            await githubApi.updateFile(
                token, owner, repo,
                '.github/workflows/build.yml',
                localBuildYml,
                `update workflow for cloud build`,
                defaultBranch,
                wfSha || undefined
            )
            addLog('工作流文件已同步到仓库', 'log-info')
            syncOk = true
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e)
            addLog(`工作流同步失败: ${msg}`, 'log-error')
            addLog('尝试直接触发（如果 fork 上已有最新工作流则不受影响）', 'log-info')
        }

        // 推送项目配置到仓库
        const cfg = project.value.config
        const simplifiedConfig = {
            name: cfg.name || 'PakePlus',
            showName: cfg.showName || cfg.name || 'PakePlus',
            appid: cfg.appid || 'com.pakeplus.app',
            url: cfg.url || 'https://juejin.cn/',
            version: cfg.version || '0.0.1',
            width: cfg.width || 800,
            height: cfg.height || 600,
        }
        const configContent = JSON.stringify(simplifiedConfig, null, 2)
        try {
            const sha = await githubApi.getFileSha(token, owner, repo, 'scripts/ppconfig.json', defaultBranch)
            await githubApi.updateFile(
                token, owner, repo,
                'scripts/ppconfig.json',
                configContent,
                `update config for ${simplifiedConfig.showName}`,
                defaultBranch,
                sha || undefined
            )
            addLog(`项目配置已推送到仓库 (${simplifiedConfig.showName})`, 'log-info')
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e)
            addLog(`配置推送失败: ${msg}`, 'log-error')
        }

        // 触发构建工作流
        // 如果同步成功，使用选中的平台作为 inputs
        // 如果同步失败，不发送 inputs，让工作流使用默认值
        const inputs: Record<string, boolean> = {
            build_windows_x86_64: false,
            build_windows_aarch64: false,
            build_macos_x86_64: false,
            build_macos_aarch64: false,
            build_linux_x86_64: false,
            build_linux_aarch64: false,
        }
        if (syncOk) {
            cloudPlatforms.value.forEach((p) => {
                inputs[p] = true
            })
        }

        addLog(`触发工作流: build.yml, 分支: ${defaultBranch}${syncOk ? ', inputs: ' + JSON.stringify(inputs) : '（无 inputs，使用默认配置）'}`, 'log-info')

        await githubApi.dispatchWorkflow(
            token, owner, repo,
            'build.yml',
            defaultBranch,
            inputs
        )

        ElMessage.success('已触发云端构建')
        addLog('已触发 GitHub Actions 构建', 'log-success')

        // 立即刷新状态并启动轮询
        await loadWorkflowRuns()
        startPolling()
    } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        ElMessage.error('云端构建失败: ' + msg)
        addLog('云端构建失败: ' + msg, 'log-error')
    } finally {
        cloudBuilding.value = false
    }
}

function startPolling() {
    stopPolling()
    pollTimer = setInterval(async () => {
        await loadWorkflowRuns()
        // 检查是否有进行中的构建
        const hasActive = workflowRuns.value.some(
            r => r.status === 'in_progress' || r.status === 'queued'
        )
        if (!hasActive) {
            // 所有构建完成，加载 artifacts
            await loadArtifacts()
            stopPolling()
        }
    }, 10000) // 每10秒轮询一次
}

function stopPolling() {
    if (pollTimer) {
        clearInterval(pollTimer)
        pollTimer = null
    }
}

async function loadWorkflowRuns() {
    try {
        workflowRuns.value = await githubApi.getWorkflowRuns(
            settingsStore.githubToken,
            settingsStore.username,
            'PakePlus',
            10
        )
    } catch {
        // ignore
    }
}

async function loadArtifacts() {
    try {
        artifacts.value = await githubApi.getArtifacts(
            settingsStore.githubToken,
            settingsStore.username,
            'PakePlus'
        )
    } catch {
        // ignore
    }
}

async function loadReleases() {
    try {
        releases.value = await githubApi.getReleases(
            settingsStore.githubToken,
            settingsStore.username,
            'PakePlus'
        )
    } catch {
        // ignore
    }
}

function runStatusType(status: string, conclusion: string | null): string {
    if (status === 'completed') {
        if (conclusion === 'success') return 'success'
        if (conclusion === 'cancelled') return 'info'
        return 'danger'
    }
    return 'warning'
}

function runStatusText(status: string, conclusion: string | null): string {
    if (status === 'completed') {
        if (conclusion === 'success') return '成功'
        if (conclusion === 'cancelled') return '已取消'
        return '失败'
    }
    if (status === 'in_progress') return '构建中'
    if (status === 'queued') return '排队中'
    return status
}

function formatTime(iso: string): string {
    const d = new Date(iso)
    return `${d.getMonth() + 1}/${d.getDate()} ${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

function formatSize(bytes: number): string {
    if (bytes < 1024) return bytes + 'B'
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + 'KB'
    return (bytes / 1024 / 1024).toFixed(1) + 'MB'
}

function openRunUrl(url: string) {
    tauriApi.openUrl(url)
}

async function downloadArtifact(artifact: GitHubArtifact) {
    const token = settingsStore.githubToken
    const downloadUrl = artifact.archive_download_url

    if (!isTauriEnv) {
        // 浏览器环境：使用 fetch 下载
        try {
            const resp = await fetch(downloadUrl, {
                headers: {
                    Authorization: `Bearer ${token}`,
                    Accept: 'application/vnd.github+json',
                }
            })
            if (!resp.ok) throw new Error(`下载失败: ${resp.status}`)
            const blob = await resp.blob()
            const url = URL.createObjectURL(blob)
            const a = document.createElement('a')
            a.href = url
            a.download = `${artifact.name}.zip`
            a.click()
            URL.revokeObjectURL(url)
            ElMessage.success('下载完成: ' + artifact.name)
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e)
            ElMessage.error('下载失败: ' + msg)
        }
        return
    }

    // Tauri 环境：先通过 fetch 获取 blob，再保存
    try {
        const savePath = await saveDialog({ defaultPath: `${artifact.name}.zip` })
        if (!savePath) return

        // 使用 Tauri HTTP 插件下载
        const { fetch: tauriFetch } = await import('@tauri-apps/plugin-http')
        const resp = await tauriFetch(downloadUrl, {
            headers: {
                Authorization: `Bearer ${token}`,
                Accept: 'application/vnd.github+json',
            }
        })
        if (!resp.ok) throw new Error(`下载失败: ${resp.status}`)

        const arrayBuffer = await resp.arrayBuffer()
        const { writeBinaryFile } = await import('@tauri-apps/plugin-fs')
        await writeBinaryFile(savePath as string, arrayBuffer)
        ElMessage.success('下载完成: ' + artifact.name)
    } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        ElMessage.error('下载失败: ' + msg)
    }
}

async function downloadAsset(asset: ReleaseAsset) {
    if (!isTauriEnv) {
        // 浏览器环境降级：直接打开下载链接
        window.open(asset.browser_download_url, '_blank')
        return
    }
    try {
        const savePath = await saveDialog({ defaultPath: asset.name })
        if (savePath) {
            await tauriApi.downloadFile(asset.browser_download_url, savePath as string, asset.name)
            ElMessage.success('下载完成: ' + asset.name)
        }
    } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        ElMessage.error('下载失败: ' + msg)
    }
}
</script>
