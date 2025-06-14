<template>
  <div class="project-bar">
    <div class="project-section">
      <label class="project-label">Project:</label>
      <select 
        class="project-dropdown" 
        :value="selectedProject?.id || ''"
        @change="handleProjectChange"
      >
        <option value="">Select Project</option>
        <option 
          v-for="project in projects" 
          :key="project.id" 
          :value="project.id"
        >
          {{ project.name }}
        </option>
      </select>
      <button 
        class="manage-projects-btn"
        @click="openProjectManager"
        title="Manage tracked projects"
      >
        ⚙️
      </button>
    </div>
    <div class="project-path">
      <span class="path-text">{{ selectedProject?.path || 'No project selected' }}</span>
      <button 
        class="open-folder-btn" 
        @click="handleOpenExplorer" 
        title="Open in file explorer"
        :disabled="!selectedProject"
      >
        📁
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProjectStore } from '../stores/projectStore'
import { useLogStore } from '../stores/logStore'
import { usePopup } from '../composables/usePopup'
import { invoke } from "@tauri-apps/api/core"

const { selectedProject, projects, setSelectedProject } = useProjectStore()
const { addLog } = useLogStore()
const { showPopup } = usePopup()

const handleProjectChange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  const projectId = target.value
  
  if (projectId) {
    const project = projects.value.find(p => p.id === projectId)
    setSelectedProject(project || null)
    if (project) {
      addLog(`Selected project: ${project.name}`)
    }
  } else {
    setSelectedProject(null)
    addLog('No project selected')
  }
}

const handleOpenExplorer = async (): Promise<void> => {
  if (!selectedProject.value) return
  
  try {
    await invoke('open_file_explorer', { path: selectedProject.value.path })
    addLog(`Opened file explorer for: ${selectedProject.value.name}`)
  } catch (error) {
    console.error('Failed to open file explorer:', error)
    addLog('Error: Failed to open file explorer. Check console for details.', 'error')
  }
}

const openProjectManager = () => {
  showPopup({
    id: 'project-manager',
    component: 'ProjectManager',
    props: {}
  })
}
</script>

<style scoped>
.project-bar {
  background-color: var(--surface-color);
  padding: var(--spacing-md);
  display: flex;
  align-items: center;
  border-bottom: var(--border-width) solid var(--border-color);
  gap: var(--spacing-lg);
  min-height: var(--project-bar-height);
}

.project-section {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.project-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
}

.project-dropdown {
  padding: var(--spacing-xs) var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  min-width: 12rem;
}

.project-dropdown:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.manage-projects-btn {
  background: none;
  border: var(--border-width) solid var(--border-color);
  cursor: pointer;
  font-size: var(--font-size-md);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
  background-color: var(--surface-color);
}

.manage-projects-btn:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.project-path {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  padding: var(--spacing-xs) var(--spacing-sm);
  flex-grow: 1;
  border-radius: var(--border-radius-md);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-sm);
  min-height: 2rem;
}

.path-text {
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.open-folder-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: var(--font-size-md);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color var(--transition-fast);
  flex-shrink: 0;
  width: 2rem;
  height: 2rem;
}

.open-folder-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
}

.open-folder-btn:active:not(:disabled) {
  background-color: var(--active-color);
}

.open-folder-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>