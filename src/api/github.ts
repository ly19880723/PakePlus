// GitHub API 封装 - 用于云端打包（fork仓库、推送配置、触发Actions、下载产物）
// 在 Tauri 环境使用 @tauri-apps/plugin-http 的 fetch（绕过 CORS），浏览器环境使用原生 fetch
import { fetch as tauriFetch } from '@tauri-apps/plugin-http'
import { isTauriEnv } from '@/utils/storage'
import type { GitHubRepo, GitHubRelease, WorkflowRun } from '@/types'

const GITHUB_API = 'https://api.github.com'

// 上游模板仓库
const UPSTREAM_REPO = 'Sjj1024/PakePlus'

// 根据环境选择 fetch
const fetchFn: typeof globalThis.fetch = isTauriEnv ? (tauriFetch as unknown as typeof globalThis.fetch) : globalThis.fetch

function headers(token: string): Record<string, string> {
    return {
        Authorization: `Bearer ${token}`,
        Accept: 'application/vnd.github+json',
        'X-GitHub-Api-Version': '2022-11-28',
    }
}

// 获取当前用户信息
export async function getUser(token: string) {
    const resp = await fetchFn(`${GITHUB_API}/user`, {
        headers: headers(token),
    })
    if (!resp.ok) throw new Error(`获取用户信息失败: ${resp.status}`)
    return resp.json()
}

// 检查是否已 fork PakePlus 仓库
export async function checkFork(token: string, username: string): Promise<GitHubRepo | null> {
    const resp = await fetchFn(`${GITHUB_API}/repos/${username}/${UPSTREAM_REPO.split('/')[1]}`, {
        headers: headers(token),
    })
    if (resp.status === 404) return null
    if (!resp.ok) throw new Error(`检查仓库失败: ${resp.status}`)
    return resp.json()
}

// Fork PakePlus 模板仓库
export async function forkRepo(token: string): Promise<GitHubRepo> {
    const resp = await fetchFn(`${GITHUB_API}/repos/${UPSTREAM_REPO}/forks`, {
        method: 'POST',
        headers: headers(token),
    })
    if (!resp.ok) throw new Error(`Fork 仓库失败: ${resp.status}`)
    return resp.json()
}

// 获取仓库的默认分支信息
export async function getRepo(token: string, owner: string, repo: string): Promise<GitHubRepo> {
    const resp = await fetchFn(`${GITHUB_API}/repos/${owner}/${repo}`, {
        headers: headers(token),
    })
    if (!resp.ok) throw new Error(`获取仓库信息失败: ${resp.status}`)
    return resp.json()
}

// 获取文件内容（SHA）
export async function getFileSha(
    token: string,
    owner: string,
    repo: string,
    path: string,
    branch?: string
): Promise<string | null> {
    const url = `${GITHUB_API}/repos/${owner}/${repo}/contents/${path}${branch ? `?ref=${branch}` : ''}`
    const resp = await fetchFn(url, { headers: headers(token) })
    if (resp.status === 404) return null
    if (!resp.ok) throw new Error(`获取文件SHA失败: ${resp.status}`)
    const data = await resp.json()
    return data.sha
}

// 更新或创建文件
export async function updateFile(
    token: string,
    owner: string,
    repo: string,
    path: string,
    content: string,
    message: string,
    branch?: string,
    sha?: string
): Promise<void> {
    const body: Record<string, unknown> = {
        message,
        content: btoa(unescape(encodeURIComponent(content))),
    }
    if (branch) body.branch = branch
    if (sha) body.sha = sha

    const resp = await fetchFn(`${GITHUB_API}/repos/${owner}/${repo}/contents/${path}`, {
        method: 'PUT',
        headers: { ...headers(token), 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
    })
    if (!resp.ok) {
        const err = await resp.json()
        throw new Error(`更新文件失败: ${err.message || resp.status}`)
    }
}

// 触发 GitHub Actions 工作流
export async function dispatchWorkflow(
    token: string,
    owner: string,
    repo: string,
    workflowId: string,
    ref: string,
    inputs: Record<string, unknown> = {}
): Promise<void> {
    const body: Record<string, unknown> = { ref }
    if (Object.keys(inputs).length > 0) {
        body.inputs = inputs
    }
    const resp = await fetchFn(
        `${GITHUB_API}/repos/${owner}/${repo}/actions/workflows/${workflowId}/dispatches`,
        {
            method: 'POST',
            headers: { ...headers(token), 'Content-Type': 'application/json' },
            body: JSON.stringify(body),
        }
    )
    if (!resp.ok) {
        const err = await resp.json()
        const msg = err.message || resp.statusText || String(resp.status)
        throw new Error(`触发工作流失败(${resp.status}): ${msg}`)
    }
}

// 获取工作流运行列表
export async function getWorkflowRuns(
    token: string,
    owner: string,
    repo: string,
    perPage: number = 10
): Promise<WorkflowRun[]> {
    const resp = await fetchFn(
        `${GITHUB_API}/repos/${owner}/${repo}/actions/runs?per_page=${perPage}`,
        { headers: headers(token) }
    )
    if (!resp.ok) throw new Error(`获取工作流运行失败: ${resp.status}`)
    const data = await resp.json()
    return data.workflow_runs || []
}

// 获取 Release 列表
export async function getReleases(
    token: string,
    owner: string,
    repo: string
): Promise<GitHubRelease[]> {
    const resp = await fetchFn(`${GITHUB_API}/repos/${owner}/${repo}/releases`, {
        headers: headers(token),
    })
    if (!resp.ok) throw new Error(`获取Release失败: ${resp.status}`)
    return resp.json()
}

// 给仓库加 star
export async function starRepo(token: string, owner: string, repo: string): Promise<void> {
    const resp = await fetchFn(`${GITHUB_API}/user/starred/${owner}/${repo}`, {
        method: 'PUT',
        headers: headers(token),
    })
    if (!resp.ok) throw new Error(`Star 仓库失败: ${resp.status}`)
}

// 获取分支列表
export async function getBranches(
    token: string,
    owner: string,
    repo: string
): Promise<{ name: string; commit: { sha: string } }[]> {
    const resp = await fetchFn(`${GITHUB_API}/repos/${owner}/${repo}/branches`, {
        headers: headers(token),
    })
    if (!resp.ok) throw new Error(`获取分支失败: ${resp.status}`)
    return resp.json()
}

// 创建分支
export async function createBranch(
    token: string,
    owner: string,
    repo: string,
    branchName: string,
    fromSha: string
): Promise<void> {
    const resp = await fetchFn(`${GITHUB_API}/repos/${owner}/${repo}/git/refs`, {
        method: 'POST',
        headers: { ...headers(token), 'Content-Type': 'application/json' },
        body: JSON.stringify({
            ref: `refs/heads/${branchName}`,
            sha: fromSha,
        }),
    })
    if (!resp.ok) {
        const err = await resp.json()
        throw new Error(`创建分支失败: ${err.message || resp.status}`)
    }
}
