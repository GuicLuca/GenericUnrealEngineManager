<template>
  <div class="sidebar">
    <SidebarItem
      v-for="item in items"
      :key="item.name"
      :icon="item.icon"
      :label="item.name"
      :disabled="!hasSelectedProject && item.requiresProject"
      @click="handleItemClick(item)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import SidebarItem from './SidebarItem.vue'
import { useProjectStore } from '../stores/projectStore'
import { useLogStore } from '../stores/logStore'
import { usePopup } from '../composables/usePopup'
import { invoke } from "@tauri-apps/api/core"

export interface SidebarItem {
  name: string
  icon: string
  action?: string
  requiresProject?: boolean
}

interface Props {
  items: SidebarItem[]
}

const props = defineProps<Props>()

const { selectedProject, hasSelectedProject } = useProjectStore()
const { addLog } = useLogStore()
const { showPopup } = usePopup()

const handleItemClick = (item: SidebarItem) => {
  if (item.requiresProject && !hasSelectedProject.value) {
    addLog(`Action "${item.name}" requires a project to be selected`, 'warn')
    return
  }
  
  // Handle specific actions
  switch (item.action) {
    case 'rescan':
      handleReScan()
      break
    case 'open':
      handleOpen()
      break
    // Add more action handlers as needed
    default:
      console.log(`Action ${item.action} not implemented yet`)
  }
}

const handleReScan = async () => {
  if (!selectedProject.value) return

  await invoke('rescan_projects', {
    projectPaths: [selectedProject.value.path]
  });
}

const handleOpen = async () => {
  if (!selectedProject.value) return

  try {
    // Check if project has C++ code
    const hasCpp = selectedProject.value.has_cpp
    
    if (!hasCpp) {
      // Blueprint project - launch directly with Unreal Engine
      addLog(`Launching Blueprint project: ${selectedProject.value.name}`)
      await invoke('launch_project_with_engine', {
        projectPath: selectedProject.value.path
      })
    } else {
      // C++ project - show popup to choose launch method
      showPopup({
        id: 'project-launch-choice',
        component: 'ProjectLaunchChoice',
        props: {
          projectName: selectedProject.value.name,
          projectPath: selectedProject.value.path
        }
      })
    }
  } catch (error) {
    console.error('Failed to open project:', error)
    addLog('Failed to open project. Check console for details.', 'error')
  }
}
</script>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  min-width: var(--sidebar-width);
  background-color: var(--surface-color);
  border-right: var(--border-width) solid var(--border-color);
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--scrollbar-thumb) var(--scrollbar-track);
}

.sidebar::-webkit-scrollbar {
  width: var(--scrollbar-width);
}

.sidebar::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
}

.sidebar::-webkit-scrollbar-thumb {
  background-color: var(--scrollbar-thumb);
  border-radius: var(--border-radius-sm);
}

.sidebar::-webkit-scrollbar-thumb:hover {
  background-color: var(--scrollbar-thumb-hover);
}
</style>
</script>