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
          <button 
            class="add-project-btn"
            @click="showAddProject = true"
            title="Add new project"
          >
            <span class="button-icon">➕</span>
            Add Project
          </button>
        </div>

        <div v-if="projects.length === 0" class="no-projects">
          <div class="no-projects-icon">📂</div>
          <div class="no-projects-text">No projects tracked yet</div>
          <div class="no-projects-subtext">Add projects to start managing them</div>
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
                class="action-btn edit-btn"
                @click="editProject(project)"
                title="Edit project"
              >
                ✏️
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

      <!-- Add Project Form -->
      <div v-if="showAddProject" class="add-project-form">
        <h4 class="form-title">Add New Project</h4>
        <form @submit.prevent="handleAddProject">
          <div class="form-group">
            <label class="form-label">Project Path</label>
            <div class="path-input-group">
              <input
                v-model="newProject.path"
                type="text"
                class="path-input"
                placeholder="Select .uproject file..."
                required
              />
              <button
                type="button"
                class="browse-btn"
                @click="selectProjectFile"
                title="Browse for .uproject file"
              >
                📂
              </button>
            </div>
          </div>
          
          <div class="form-group">
            <label class="form-label">Description (Optional)</label>
            <textarea
              v-model="newProject.description"
              class="description-input"
              placeholder="Enter project description..."
              rows="3"
            ></textarea>
          </div>

          <div class="form-actions">
            <button
              type="button"
              class="cancel-btn"
              @click="cancelAddProject"
            >
              Cancel
            </button>
            <button
              type="submit"
              class="add-btn"
              :disabled="!newProject.path.trim()"
            >
              <span class="button-icon">➕</span>
              Add Project
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useProjectStore, type Project } from '../../stores/projectStore'
import { useLogStore } from '../../stores/logStore'

interface Emits {
  (e: 'close'): void
}

const emit = defineEmits<Emits>()

const { projects, selectedProject, setSelectedProject, addProject, removeProject } = useProjectStore()
const { addLog } = useLogStore()

const showAddProject = ref(false)
const newProject = reactive({
  path: '',
  description: ''
})

const selectProject = (project: Project) => {
  setSelectedProject(project)
  addLog(`Selected project: ${project.name}`)
}

const editProject = (project: Project) => {
  // TODO: Implement edit functionality
  addLog(`Edit project: ${project.name} (not implemented yet)`)
}

const confirmRemoveProject = (project: Project) => {
  if (confirm(`Are you sure you want to remove "${project.name}" from tracked projects?`)) {
    removeProject(project.id)
    addLog(`Removed project: ${project.name}`)
  }
}

const selectProjectFile = async () => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: 'Select Unreal Engine Project File',
      filters: [{
        name: 'Unreal Project',
        extensions: ['uproject']
      }]
    })
    
    if (selected && typeof selected === 'string') {
      newProject.path = selected
    }
  } catch (error) {
    console.error('Failed to open file dialog:', error)
    addLog('Error: Failed to open file dialog', 'error')
  }
}

const handleAddProject = () => {
  if (!newProject.path.trim()) return

  // Extract project name from path
  const pathParts = newProject.path.replace(/\\/g, '/').split('/')
  const fileName = pathParts[pathParts.length - 1]
  const projectName = fileName.replace('.uproject', '')

  const project: Project = {
    id: Date.now().toString(),
    name: projectName,
    path: newProject.path,
    engineVersion: 'Unknown', // TODO: Parse from .uproject file
    description: newProject.description || 'No description provided',
    lastScan: new Date().toLocaleString()
  }

  addProject(project)
  addLog(`Added project: ${project.name}`)
  
  cancelAddProject()
}

const cancelAddProject = () => {
  showAddProject.value = false
  newProject.path = ''
  newProject.description = ''
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

.add-project-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--accent-color);
  border: none;
  border-radius: var(--border-radius-sm);
  color: white;
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: background-color var(--transition-fast);
}

.add-project-btn:hover {
  background-color: #2c5aa0;
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

.add-project-form {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.form-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.form-group {
  margin-bottom: var(--spacing-md);
}

.form-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
}

.path-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.path-input,
.description-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  transition: border-color var(--transition-fast);
}

.path-input:focus,
.description-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.description-input {
  resize: vertical;
  min-height: 4rem;
}

.browse-btn {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-md);
  transition: all var(--transition-fast);
  min-width: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.browse-btn:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-lg);
}

.cancel-btn,
.add-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.cancel-btn {
  background-color: transparent;
  border: var(--border-width) solid var(--border-color);
  color: var(--text-secondary);
}

.cancel-btn:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.add-btn {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.add-btn:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.add-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>