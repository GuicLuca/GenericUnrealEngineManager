<template>
  <div class="project-bar">
    <div class="project-section">
      <label class="project-label">Project:</label>
      <select 
        class="project-dropdown" 
        :value="selectedProject?.path || ''"
        @change="handleProjectChange"
      >
        <option value="">Select Project</option>
        <option 
          v-for="project in sortedProjects" 
          :key="project.path" 
          :value="project.path"
        >
          {{ project.name }}
        </option>
      </select>
      <button 
        class="manage-projects-btn"
        @click="openProjectManager"
        title="Add projects"
      >
        ➕
      </button>
    </div>
    <div class="project-path">
      <span class="path-text">{{ selectedProject?.path || 'Select a project on the project selector to start.' }}</span>
      <FileExplorerButton
          v-if="selectedProject"
          :project-path="selectedProject.path"
          :project-name="selectedProject.name"
          :disabled="!selectedProject"
      />
    </div>
    <div class="top-bar-actions">
      <button 
        class="settings-btn"
        @click="openSettings"
        title="Settings"
      >
        ⚙️
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectStore } from '../stores/projectStore'
import { useLogStore } from '../stores/logStore'
import { usePopup } from '../composables/usePopup'
import FileExplorerButton from './FileExplorerButton.vue'

const { selectedProject, projects, setSelectedProject, findProjectByPath } = useProjectStore()
const { addLog } = useLogStore()
const { showPopup } = usePopup()

// Sort projects alphabetically by name
const sortedProjects = computed(() => {
  return [...projects.value].sort((a, b) => a.name.localeCompare(b.name))
})

const handleProjectChange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  const projectPath = target.value
  
  if (projectPath) {
    const project = findProjectByPath(projectPath)
    setSelectedProject(project || null)
    if (project) {
      addLog(`Selected project: ${project.name}`)
    }
  } else {
    setSelectedProject(null)
    addLog('No project selected')
  }
}

const openProjectManager = () => {
  showPopup({
    id: 'project-manager',
    component: 'ProjectManager',
    props: {}
  })
}

const openSettings = () => {
  showPopup({
    id: 'settings',
    component: 'Settings',
    props: {}
  })
}
</script>

<style scoped>
.project-bar {
  background-color: var(--surface-color);
  padding: var(--spacing-sm) var(--spacing-md);
  display: flex;
  align-items: center;
  border-bottom: var(--border-width) solid var(--border-color);
  gap: var(--spacing-lg);
  min-height: 3rem;
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

.manage-projects-btn,
.settings-btn {
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

.manage-projects-btn:hover,
.settings-btn:hover {
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

.top-bar-actions {
  display: flex;
  gap: var(--spacing-xs);
}
</style>