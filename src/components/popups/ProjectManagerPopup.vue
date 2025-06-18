<template>
  <div class="project-manager-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">📁</span>
        Manage Tracked Projects
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="projects-section">
        <div class="section-header">
          <h3 class="section-title">Tracked Projects ({{ projectCount }})</h3>
          <div class="header-actions">
            <button 
              class="discover-btn"
              @click="openProjectDiscovery"
              :disabled="isLoading"
              title="Discover new projects"
            >
              {{ isLoading ? '⏳' : '🔍' }}
            </button>
            <button 
              class="refresh-btn"
              @click="handleRefresh"
              :disabled="isLoading"
              title="Refresh project list"
            >
              {{ isLoading ? '⏳' : '🔄' }}
            </button>
          </div>
        </div>

        <div v-if="projects.length === 0" class="no-projects">
          <div class="no-projects-icon">📂</div>
          <div class="no-projects-text">No projects tracked yet</div>
          <div class="no-projects-subtext">Use the Discover button to find projects</div>
        </div>

        <div v-else class="projects-list">
          <div 
            v-for="project in projects"
            :key="project.path"
            class="project-item"
            :class="{ selected: selectedProject?.path === project.path }"
            @click="selectProject(project)"
          >
            <div class="project-info">
              <div class="project-name">{{ project.name }}</div>
              <div class="project-path">{{ project.path }}</div>
              <div class="project-meta">
                <span class="engine-version">{{ getEngineVersionString(project.engine_association) }}</span>
                <span class="has-cpp">{{ project.has_cpp ? 'C++' : 'Blueprint' }}</span>
                <span class="plugin-count">{{ project.plugins.length }} plugin(s)</span>
                <span class="size-on-disk">{{ formatSize(project.size_on_disk) }}</span>
                <span class="last-modified">{{ getTimeSince(project.last_scan_date) }}</span>
              </div>
            </div>
            <div class="project-actions">
              <FileExplorerButton
                :project-path="project.path"
                :project-name="project.name"
                size="small"
                :disabled="isLoading"
              />
              <button 
                class="action-btn remove-btn"
                @click.stop="confirmRemoveProject(project)"
                title="Remove project from tracking"
                :disabled="isLoading"
              >
                🗑️
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import {useProjectStore, type Project} from '../../stores/projectStore'
import { useLogStore } from '../../stores/logStore'
import { usePopup } from '../../composables/usePopup'
import FileExplorerButton from '../FileExplorerButton.vue'
import {formatSize, timeSince} from "../../utils.ts";

const { 
  projects, 
  selectedProject, 
  projectCount,
  isLoading,
  setSelectedProject, 
  removeProjects,
  refreshProjects,
  getEngineVersionString 
} = useProjectStore()

const { addLog } = useLogStore()
const { showPopup } = usePopup()

// Timer for updating time-based fields
let timeUpdateInterval: number | null = null
const forceUpdate = ref(0)

// Function to get time since with forced reactivity
const getTimeSince = (date: number) => {
  // Access forceUpdate to trigger reactivity
  forceUpdate.value
  return timeSince(date)
}

const selectProject = (project: Project) => {
  setSelectedProject(project)
  addLog(`Selected project: ${project.name}`)
}

const confirmRemoveProject = async (project: Project) => {
  if (confirm(`Are you sure you want to remove "${project.name}" from tracked projects?`)) {
    try {
      await removeProjects([project.path])
    } catch (error) {
      // Ignore errors, the backend will handle it.
    }
  }
}

const handleRefresh = async () => {
  try {
    await refreshProjects()
  } catch (error) {
    // Ignore errors, the backend will handle it.
  }
}

const openProjectDiscovery = () => {
  showPopup({
    id: 'project-discovery',
    component: 'ProjectDiscovery',
    props: {}
  })
}

// Setup timer for updating time-based fields
onMounted(() => {
  // Update every minute (60000ms)
  timeUpdateInterval = window.setInterval(() => {
    forceUpdate.value++
  }, 60000)
})

onUnmounted(() => {
  if (timeUpdateInterval) {
    clearInterval(timeUpdateInterval)
    timeUpdateInterval = null
  }
})
</script>

<style scoped>
.project-manager-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 56rem;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.popup-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
}

.popup-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.title-icon {
  font-size: var(--icon-size-lg);
}

.close-button {
  background: none;
  border: none;
  font-size: var(--font-size-lg);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.popup-content {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-lg);
}

.projects-section {
  margin-bottom: var(--spacing-lg);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md);
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.header-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.discover-btn,
.refresh-btn {
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

.discover-btn:hover:not(:disabled),
.refresh-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.discover-btn:disabled,
.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.no-projects {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.no-projects-icon {
  font-size: var(--icon-size-xl);
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.no-projects-text {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-projects-subtext {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  opacity: 0.7;
}

.projects-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.project-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  transition: all var(--transition-fast);
  cursor: pointer;
}

.project-item:hover {
  border-color: var(--accent-color);
  background-color: var(--hover-color);
}

.project-item.selected {
  border-color: var(--accent-color);
  background-color: var(--accent-color-alpha);
}

.project-info {
  flex-grow: 1;
  margin-right: var(--spacing-lg);
  min-width: 0;
}

.project-name {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.project-path {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  margin-bottom: var(--spacing-xs);
  word-break: break-all;
  overflow-wrap: break-word;
}

.project-meta {
  display: flex;
  gap: var(--spacing-md);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  flex-wrap: wrap;
}

.project-actions {
  display: flex;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.action-btn {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  width: 1.75rem;
  height: 1.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.remove-btn:hover:not(:disabled) {
  border-color: #e53e3e;
  background-color: #fed7d7;
}
</style>