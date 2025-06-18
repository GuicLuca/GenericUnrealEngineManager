<template>
  <div class="info-panel" :style="{ width: width + 'px' }">
    <div 
      class="resize-handle" 
      @mousedown="startResize"
    ></div>
    
    <div class="info-content">
      <h3 class="info-title">Project Information</h3>
      
      <div v-if="selectedProject" class="info-section">
        <div class="info-item">
          <div class="info-header">
            <span class="info-icon">⚙️</span>
            <span class="info-label">Engine version</span>
          </div>
          <div class="info-value-with-action">
            <div class="info-value">
              {{ getEngineVersionString(selectedProject.engine_association) }}
              <FileExplorerButton
                  v-if="isCustomEngine(selectedProject.engine_association)"
                  :project-path="getCustomEngineDirectory(selectedProject.path)"
                  :project-name="`${selectedProject.name} (Custom Engine)`"
                  size="mini"
                  title="Open custom engine directory"
                  class="engine-dir-button"
              />
            </div>
          </div>
        </div>
        
        <InfoItem
            label="Size on disk"
            :value="formatSize(selectedProject.size_on_disk)"
            icon="📥"
        />
        <InfoItem 
          label="Has C++ code" 
          :value="selectedProject.has_cpp ? 'Yes' : 'No'" 
          icon="💻"
        />
        <InfoItem 
          label="Description" 
          :value="selectedProject.description || 'No description available'" 
          icon="📝"
          multiline
        />
        <InfoItem 
          label="Plugins" 
          :value="`${selectedProject.plugins.length} plugin(s)`" 
          icon="🔌"
        />
        <InfoItem 
          label="Last scan" 
          :value="timeSince(selectedProject.last_scan_date)" 
          icon="🕒"
        />
      </div>
      
      <div v-else class="no-project">
        <div class="no-project-icon">📂</div>
        <div class="no-project-text">No project selected</div>
        <div class="no-project-subtext">Select a project to view its information</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import InfoItem from './InfoItem.vue'
import FileExplorerButton from './FileExplorerButton.vue'
import { useProjectStore, type EngineAssociation } from '../stores/projectStore'
import {formatSize, timeSince} from '../utils.ts'

interface Props {
  width: number
  minWidth: number
  maxWidth: number
}

interface Emits {
  (e: 'resize', width: number): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { selectedProject, getEngineVersionString } = useProjectStore()
const isResizing = ref(false)

const isCustomEngine = (engineAssociation: EngineAssociation): boolean => {
  return typeof engineAssociation === 'string' && engineAssociation === 'Custom'
}

const getCustomEngineDirectory = (projectPath: string): string => {
  // Extract the parent directory of the project directory
  // Project path format: DISK:\...\CUSTOM_ENGINE_DIRECTORY\PROJECT\PROJECT.uproject
  // We want to get: DISK:\...\CUSTOM_ENGINE_DIRECTORY
  
  // First, get the directory containing the .uproject file
  const projectDir = projectPath.replace(/[^/\\]*\.uproject$/, '')
  
  // Then get the parent of that directory (the custom engine directory)
  const pathParts = projectDir.replace(/[/\\]+$/, '').split(/[/\\]/)
  pathParts.pop() // Remove the project directory name
  
  return pathParts.join('/') || projectDir
}

const startResize = (event: MouseEvent) => {
  isResizing.value = true
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  event.preventDefault()
}

const handleResize = (event: MouseEvent) => {
  if (!isResizing.value) return
  
  const containerRect = document.querySelector('.app-container')?.getBoundingClientRect()
  if (!containerRect) return
  
  const newWidth = containerRect.right - event.clientX
  const clampedWidth = Math.min(Math.max(newWidth, props.minWidth), props.maxWidth)
  emit('resize', clampedWidth)
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
}
</script>

<style scoped>
.info-panel {
  background-color: var(--surface-color);
  border-left: var(--border-width) solid var(--border-color);
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.resize-handle {
  position: absolute;
  top: 0;
  left: -3px;
  bottom: 0;
  width: 6px;
  background-color: transparent;
  cursor: ew-resize;
  z-index: 10;
  transition: background-color var(--transition-fast);
}

.resize-handle:hover {
  background-color: var(--accent-color-alpha);
}

.resize-handle:active {
  background-color: var(--accent-color);
}

.info-content {
  padding: var(--spacing-md);
  overflow-y: auto;
  flex-grow: 1;
}

.info-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
  padding-bottom: var(--spacing-sm);
  border-bottom: var(--border-width) solid var(--border-color);
}

.info-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.info-icon {
  font-size: var(--font-size-sm);
}

.info-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-secondary);
}

.info-value-with-action {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.info-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.4;
  padding: var(--spacing-xs);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
  flex-grow: 1;
  display: flex;
  justify-content: space-between;
}

.engine-dir-button {
  flex-shrink: 0;
}

.no-project {
  position: relative;
  top: 28%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  align-self: center;
  text-align: center;
  padding: var(--spacing-xl);
}

.no-project-icon {
  font-size: var(--icon-size-xl);
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.no-project-text {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-project-subtext {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  opacity: 0.7;
}
</style>