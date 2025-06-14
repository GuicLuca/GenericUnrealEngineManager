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
          <h3 class="section-title">Tracked Projects</h3>
        </div>

        <div v-if="projects.length === 0" class="no-projects">
          <div class="no-projects-icon">📂</div>
          <div class="no-projects-text">No projects tracked yet</div>
          <div class="no-projects-subtext">Use the Discover button to find projects</div>
        </div>

        <div v-else class="projects-list">
          <div 
            v-for="project in projects" 
            :key="project.id"
            class="project-item"
            :class="{ selected: selectedProject?.id === project.id }"
          >
            <div class="project-info">
              <div class="project-name">{{ project.name }}</div>
              <div class="project-path">{{ project.path }}</div>
              <div class="project-meta">
                <span class="engine-version">{{ project.engineVersion }}</span>
                <span class="last-scan">Last scan: {{ project.lastScan }}</span>
              </div>
            </div>
            <div class="project-actions">
              <button 
                class="action-btn select-btn"
                @click="selectProject(project)"
                :disabled="selectedProject?.id === project.id"
                title="Select this project"
              >
                {{ selectedProject?.id === project.id ? '✓' : '👆' }}
              </button>
              <button 
                class="action-btn remove-btn"
                @click="confirmRemoveProject(project)"
                title="Remove project"
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
import { useProjectStore, type Project } from '../../stores/projectStore'
import { useLogStore } from '../../stores/logStore'

interface Emits {
  (e: 'close'): void
}

const emit = defineEmits<Emits>()

const { projects, selectedProject, setSelectedProject, removeProject } = useProjectStore()
const { addLog } = useLogStore()

const selectProject = (project: Project) => {
  setSelectedProject(project)
  addLog(`Selected project: ${project.name}`)
}

const confirmRemoveProject = (project: Project) => {
  if (confirm(`Are you sure you want to remove "${project.name}" from tracked projects?`)) {
    removeProject(project.id)
    addLog(`Removed project: ${project.name}`)
  }
}
</script>

<style scoped>
.project-manager-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 48rem;
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
}

.project-meta {
  display: flex;
  gap: var(--spacing-md);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.project-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.action-btn {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
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

.select-btn:hover:not(:disabled) {
  border-color: var(--accent-color);
}

.remove-btn:hover:not(:disabled) {
  border-color: #e53e3e;
  background-color: #fed7d7;
}
</style>