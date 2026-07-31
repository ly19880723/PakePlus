<template>
    <div>
        <!-- 顶部操作栏 -->
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px">
            <el-input
                v-model="searchQuery"
                :placeholder="t('home.search')"
                style="width: 300px"
                clearable
                :prefix-icon="Search"
            />
            <el-button type="primary" @click="createProject">
                <el-icon><Plus /></el-icon>
                {{ t('home.create') }}
            </el-button>
        </div>

        <!-- 项目列表 -->
        <div v-if="filteredProjects.length === 0" class="empty-state">
            <el-icon><FolderOpened /></el-icon>
            <p>{{ t('home.empty') }}</p>
        </div>

        <div v-else class="project-grid">
            <div
                v-for="project in filteredProjects"
                :key="project.id"
                class="project-card"
                @click="editProject(project.id)"
            >
                <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px">
                    <img
                        v-if="project.config.icon"
                        :src="project.config.icon"
                        style="width: 48px; height: 48px; border-radius: 8px; object-fit: contain"
                    />
                    <div
                        v-else
                        style="width: 48px; height: 48px; border-radius: 8px; background: #f0f2f5; display: flex; align-items: center; justify-content: center"
                    >
                        <el-icon size="24" color="#909399"><Platform /></el-icon>
                    </div>
                    <div style="flex: 1; min-width: 0">
                        <div style="font-weight: 600; font-size: 15px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
                            {{ project.config.name || project.config.showName || '未命名' }}
                        </div>
                        <div style="font-size: 12px; color: #909399; overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
                            {{ project.config.url || '未设置URL' }}
                        </div>
                    </div>
                </div>

                <!-- 平台标签 -->
                <div style="display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 12px">
                    <el-tag
                        v-for="p in project.config.platform"
                        :key="p"
                        size="small"
                        type="info"
                    >
                        {{ platformLabel(p) }}
                    </el-tag>
                </div>

                <!-- 底部操作 -->
                <div style="display: flex; justify-content: space-between; align-items: center; border-top: 1px solid #f0f0f0; padding-top: 10px">
                    <span style="font-size: 12px; color: #909399">
                        {{ formatTime(project.updatedAt) }}
                    </span>
                    <div @click.stop>
                        <el-button text size="small" @click="editProject(project.id)">
                            <el-icon><Edit /></el-icon>
                        </el-button>
                        <el-button text size="small" @click="buildProject(project.id)">
                            <el-icon><Promotion /></el-icon>
                        </el-button>
                        <el-dropdown trigger="click" @command="(cmd: string) => handleCommand(cmd, project)">
                            <el-button text size="small">
                                <el-icon><MoreFilled /></el-icon>
                            </el-button>
                            <template #dropdown>
                                <el-dropdown-menu>
                                    <el-dropdown-item command="duplicate">{{ t('home.duplicate') }}</el-dropdown-item>
                                    <el-dropdown-item command="delete" divided>
                                        <span style="color: #f56c6c">{{ t('home.delete') }}</span>
                                    </el-dropdown-item>
                                </el-dropdown-menu>
                            </template>
                        </el-dropdown>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessageBox, ElMessage } from 'element-plus'
import { Search, Plus, Edit, Promotion, MoreFilled, FolderOpened, Platform } from '@element-plus/icons-vue'
import { useProjectStore } from '@/stores/project'
import type { Project, PlatformKey } from '@/types'

const router = useRouter()
const { t } = useI18n()
const projectStore = useProjectStore()
const searchQuery = ref('')

const filteredProjects = computed(() => {
    if (!searchQuery.value) return projectStore.projects
    const q = searchQuery.value.toLowerCase()
    return projectStore.projects.filter(
        (p) =>
            p.config.name.toLowerCase().includes(q) ||
            p.config.url.toLowerCase().includes(q)
    )
})

function platformLabel(p: PlatformKey): string {
    const labels: Record<PlatformKey, string> = {
        '1-1': 'Win x64',
        '1-2': 'Win ARM',
        '2-1': 'macOS Intel',
        '2-2': 'macOS ARM',
        '3-1': 'Linux x64',
        '3-2': 'Linux ARM',
        '4-1': 'Android',
        '4-2': 'iOS',
    }
    return labels[p]
}

function formatTime(ts: number): string {
    const d = new Date(ts)
    const m = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    const h = String(d.getHours()).padStart(2, '0')
    const min = String(d.getMinutes()).padStart(2, '0')
    return `${m}-${day} ${h}:${min}`
}

async function createProject() {
    try {
        const project = await projectStore.createProject()
        console.log('[PakePlus] 创建项目成功, ID:', project.id)
        await router.push(`/project/${project.id}`)
        console.log('[PakePlus] 路由跳转完成')
    } catch (e) {
        console.error('[PakePlus] 创建项目失败:', e)
        ElMessage.error('创建项目失败: ' + (e as Error).message)
    }
}

function editProject(id: string) {
    router.push(`/project/${id}`)
}

function buildProject(id: string) {
    router.push(`/build/${id}`)
}

async function handleCommand(cmd: string, project: Project) {
    if (cmd === 'duplicate') {
        await projectStore.duplicateProject(project.id)
        ElMessage.success(t('common.success'))
    } else if (cmd === 'delete') {
        try {
            await ElMessageBox.confirm(t('home.deleteConfirm'), t('common.tip'), {
                type: 'warning',
            })
            await projectStore.deleteProject(project.id)
            ElMessage.success(t('common.success'))
        } catch {
            // 取消
        }
    }
}
</script>
