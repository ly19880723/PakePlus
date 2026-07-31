// 项目管理 Store - 使用通用 storage 工具持久化（Tauri 环境 / 浏览器开发环境）
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { storage } from '@/utils/storage'
import { createDefaultConfig, type Project, type ProjectConfig } from '@/types'

const STORE_FILE = 'projects.json'

export const useProjectStore = defineStore('project', () => {
    const projects = ref<Project[]>([])
    const currentProject = ref<Project | null>(null)
    const loading = ref(false)

    // 初始化加载项目列表
    async function loadProjects() {
        loading.value = true
        try {
            const data = await storage.get(STORE_FILE, 'projects') as Project[] | null
            projects.value = data || []
        } catch (e) {
            console.error('[PakePlus] 加载项目列表失败:', e)
            projects.value = []
        } finally {
            loading.value = false
        }
    }

    // 保存项目列表到本地
    async function saveProjects() {
        try {
            await storage.set(STORE_FILE, 'projects', projects.value)
            await storage.save(STORE_FILE)
        } catch (e) {
            console.error('[PakePlus] 保存项目列表失败:', e)
            // 不抛错，让上层逻辑能继续执行
        }
    }

    // 创建新项目
    async function createProject(name?: string): Promise<Project> {
        const config = createDefaultConfig()
        if (name) {
            config.name = name
            config.showName = name
            config.more.windows.title = name
        }
        const project: Project = {
            id: Date.now().toString(36) + Math.random().toString(36).slice(2, 8),
            config,
            createdAt: Date.now(),
            updatedAt: Date.now(),
        }
        projects.value.unshift(project)
        await saveProjects()
        return project
    }

    // 删除项目
    async function deleteProject(id: string) {
        const idx = projects.value.findIndex((p) => p.id === id)
        if (idx !== -1) {
            projects.value.splice(idx, 1)
            await saveProjects()
        }
    }

    // 复制项目
    async function duplicateProject(id: string): Promise<Project | null> {
        const original = projects.value.find((p) => p.id === id)
        if (!original) return null
        const copy: Project = {
            id: Date.now().toString(36) + Math.random().toString(36).slice(2, 8),
            config: JSON.parse(JSON.stringify(original.config)),
            createdAt: Date.now(),
            updatedAt: Date.now(),
        }
        copy.config.name = `${original.config.name}_copy`
        copy.config.showName = `${original.config.showName}_copy`
        projects.value.unshift(copy)
        await saveProjects()
        return copy
    }

    // 更新项目
    async function updateProject(id: string, config: ProjectConfig) {
        const idx = projects.value.findIndex((p) => p.id === id)
        if (idx !== -1) {
            projects.value[idx].config = config
            projects.value[idx].updatedAt = Date.now()
            await saveProjects()
        }
    }

    // 获取单个项目
    function getProject(id: string): Project | undefined {
        return projects.value.find((p) => p.id === id)
    }

    // 设置当前编辑项目
    function setCurrentProject(project: Project | null) {
        currentProject.value = project
    }

    return {
        projects,
        currentProject,
        loading,
        loadProjects,
        saveProjects,
        createProject,
        deleteProject,
        duplicateProject,
        updateProject,
        getProject,
        setCurrentProject,
    }
})
