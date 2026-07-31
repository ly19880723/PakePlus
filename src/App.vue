<template>
    <router-view />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useSettingsStore } from '@/stores/settings'
import { useI18n } from 'vue-i18n'

const projectStore = useProjectStore()
const settingsStore = useSettingsStore()
const { locale } = useI18n()

onMounted(async () => {
    await Promise.all([projectStore.loadProjects(), settingsStore.loadSettings()])
    locale.value = settingsStore.locale
})
</script>
