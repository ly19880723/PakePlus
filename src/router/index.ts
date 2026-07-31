import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        component: () => import('@/components/layout/Layout.vue'),
        children: [
            {
                path: '',
                name: 'home',
                component: () => import('@/views/Home.vue'),
            },
            {
                path: 'project/:id',
                name: 'edit',
                component: () => import('@/views/ProjectEdit.vue'),
            },
            {
                path: 'build/:id',
                name: 'build',
                component: () => import('@/views/Build.vue'),
            },
            {
                path: 'settings',
                name: 'settings',
                component: () => import('@/views/Settings.vue'),
            },
        ],
    },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router
