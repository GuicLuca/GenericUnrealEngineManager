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
                  v-if="getEngineFolder(selectedProject.engine_association)"
                  :project-path="getEngineFolder(selectedProject.engine_association)!"
                  :project-name="`${getEngineVersionString(selectedProject.engine_association)} Engine`"
                  size="mini"
                  title="Open engine directory"
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
          :value="currentTimeSince" 
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
import {ref, computed, onMounted, onUnmounted} from 'vue'
import { invoke } from '@tauri-apps/api/core'
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

interface AppSettings {
  engine_programs?: {
    custom_engines?: Record<string, string>
  }
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { selectedProject, getEngineVersionString } = useProjectStore()
const isResizing = ref(false)
const settings = ref<AppSettings | null>(null)

// Timer for updating time-based fields
let timeUpdateInterval: number | null = null
let forceUpdate = ref(0)

// Computed property that updates every minute
const currentTimeSince = computed(() => {
  if (!selectedProject.value) return ''
  forceUpdate.value
  return timeSince(selectedProject.value.last_scan_date)
})

const getEngineFolder = (engineAssociation: EngineAssociation): string | null => {
  if (typeof engineAssociation === 'string' && engineAssociation === 'Custom') {
    // For custom engines, try to find a matching registered engine
    if (settings.value?.engine_programs?.custom_engines) {
      // Return the first custom engine path (could be improved to match by name)
      const enginePaths = Object.values(settings.value.engine_programs.custom_engines)
      if (enginePaths.length > 0 && enginePaths[0].trim()) {
        return enginePaths[0]
      }
    }
  } else if (typeof engineAssociation === 'object' && engineAssociation.Standard) {
    // For standard engines, check if there's a registered engine with matching version
    if (settings.value?.engine_programs?.custom_engines) {
      const engineVersion = engineAssociation.Standard
      for (const [name, path] of Object.entries(settings.value.engine_programs.custom_engines)) {
        if (name.includes(engineVersion) && path.trim()) {
          return path
        }
      }
    }
  }
  
  return null
}

const loadSettings = async () => {
  try {
    settings.value = await invoke('get_settings') as AppSettings
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
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

// Setup timer for updating time-based fields
onMounted(() => {
  loadSettings()
  
  // Update every minute (60 000 ms)
  timeUpdateInterval = window.setInterval(async () => {
    forceUpdate.value = (forceUpdate.value + 1) % 60
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