<template>
  <div class="info-panel" :style="{ width: width + 'px' }">
    <div 
      class="resize-handle" 
      @mousedown="startResize"
    ></div>
    
    <div class="info-content">
      <h3 class="info-title">Project Information</h3>
      
      <div v-if="selectedProject" class="info-section">
        <InfoItem 
          label="Engine version" 
          :value="getEngineVersionString(selectedProject.engine_association)" 
          icon="⚙️"
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
import { useProjectStore } from '../stores/projectStore'

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

.no-project {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
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