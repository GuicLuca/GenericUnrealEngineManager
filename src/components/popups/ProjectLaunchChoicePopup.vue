<template>
  <div class="project-launch-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">🚀</span>
        Launch Project
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="project-info">
        <div class="project-name">{{ projectName }}</div>
        <div class="project-note">This is a C++ project. Choose how you want to open it:</div>
      </div>

      <div class="launch-options">
        <button 
          class="launch-option engine-option"
          @click="launchWithEngine"
          :disabled="isLaunching"
        >
          <div class="option-icon">🎮</div>
          <div class="option-content">
            <div class="option-title">Open {{ projectName }} with Unreal Engine</div>
            <div class="option-description">Open the project directly in Unreal Engine</div>
          </div>
        </button>

        <button 
          class="launch-option ide-option"
          @click="showIdeSelection = true"
          :disabled="isLaunching"
        >
          <div class="option-icon">💻</div>
          <div class="option-content">
            <div class="option-title">Open {{ projectName }} with IDE</div>
            <div class="option-description">Open the project solution file in your preferred IDE</div>
          </div>
        </button>

        <!-- Custom Engine Option (only for custom engine projects) -->
        <button 
          v-if="isCustomEngine"
          class="launch-option custom-engine-option"
          @click="showCustomEngineIdeSelection = true"
          :disabled="isLaunching"
        >
          <div class="option-icon">🔧</div>
          <div class="option-content">
            <div class="option-title">Open the custom engine with IDE</div>
            <div class="option-description">Open the custom engine solution file in your preferred IDE</div>
          </div>
        </button>
      </div>

      <!-- IDE Selection for Project -->
      <div v-if="showIdeSelection" class="ide-selection">
        <h3 class="ide-title">Select IDE for {{ projectName }}</h3>
        <div class="ide-list">
          <button 
            v-for="ide in availableIdes"
            :key="ide.name"
            class="ide-item"
            @click="launchProjectWithIde(ide.path)"
            :disabled="isLaunching"
          >
            <div class="ide-icon">{{ ide.icon }}</div>
            <div class="ide-info">
              <div class="ide-name">{{ ide.name }}</div>
              <div class="ide-path">{{ ide.path }}</div>
            </div>
          </button>
          
          <div v-if="availableIdes.length === 0" class="no-ides">
            <div class="no-ides-icon">⚠️</div>
            <div class="no-ides-text">No IDEs configured</div>
            <div class="no-ides-subtext">Configure IDE programs in settings</div>
            <button class="settings-button" @click="openSettings">
              Open Settings
            </button>
          </div>
        </div>
      </div>

      <!-- IDE Selection for Custom Engine -->
      <div v-if="showCustomEngineIdeSelection" class="ide-selection">
        <h3 class="ide-title">Select IDE for Custom Engine</h3>
        <div class="ide-list">
          <button 
            v-for="ide in availableIdes"
            :key="ide.name"
            class="ide-item"
            @click="launchCustomEngineWithIde(ide.path)"
            :disabled="isLaunching"
          >
            <div class="ide-icon">{{ ide.icon }}</div>
            <div class="ide-info">
              <div class="ide-name">{{ ide.name }}</div>
              <div class="ide-path">{{ ide.path }}</div>
            </div>
          </button>
          
          <div v-if="availableIdes.length === 0" class="no-ides">
            <div class="no-ides-icon">⚠️</div>
            <div class="no-ides-text">No IDEs configured</div>
            <div class="no-ides-subtext">Configure IDE programs in settings</div>
            <button class="settings-button" @click="openSettings">
              Open Settings
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogStore } from '../../stores/logStore'
import { usePopup } from '../../composables/usePopup'
import { useProjectStore, type EngineAssociation } from '../../stores/projectStore'

interface Props {
  projectName: string
  projectPath: string
  engineAssociation?: EngineAssociation
}

interface IdeInfo {
  name: string
  path: string
  icon: string
}

interface AppSettings {
  ide_programs: {
    custom_programs: Record<string, string>
  }
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()
const { showPopup } = usePopup()
const { selectedProject } = useProjectStore()

const isLaunching = ref(false)
const showIdeSelection = ref(false)
const showCustomEngineIdeSelection = ref(false)
const settings = ref<AppSettings | null>(null)

// Check if this is a custom engine project
const isCustomEngine = computed(() => {
  const engineAssoc = props.engineAssociation || selectedProject.value?.engine_association
  return typeof engineAssoc === 'string' && engineAssoc === 'Custom'
})

const availableIdes = computed(() => {
  if (!settings.value) return []
  
  const ides: IdeInfo[] = []
  
  // Add custom programs
  Object.entries(settings.value.ide_programs.custom_programs).forEach(([name, path]) => {
    if (path.trim()) { // Only include programs with valid paths
      ides.push({
        name,
        path,
        icon: '⚙️'
      })
    }
  })
  
  return ides
})

const launchWithEngine = async () => {
  try {
    isLaunching.value = true
    addLog(`Launching ${props.projectName} with Unreal Engine`)
    
    await invoke('launch_project_with_engine', {
      projectPath: props.projectPath
    })
    
    emit('close')
  } catch (error) {
    console.error('Failed to launch with engine:', error)
    addLog('Failed to launch project with Unreal Engine', 'error')
  } finally {
    isLaunching.value = false
  }
}

const launchProjectWithIde = async (idePath: string) => {
  try {
    isLaunching.value = true
    addLog(`Launching ${props.projectName} with IDE`)
    
    await invoke('launch_project_with_ide', {
      projectPath: props.projectPath,
      idePath
    })
    
    emit('close')
  } catch (error) {
    console.error('Failed to launch project with IDE:', error)
    addLog('Failed to launch project with IDE', 'error')
  } finally {
    isLaunching.value = false
  }
}

const launchCustomEngineWithIde = async (idePath: string) => {
  try {
    isLaunching.value = true
    addLog(`Opening custom engine with IDE`)
    
    // Get the custom engine directory (parent of project directory)
    const projectDir = props.projectPath.replace(/[^/\\]*\.uproject$/, '')
    const pathParts = projectDir.replace(/[/\\]+$/, '').split(/[/\\]/)
    pathParts.pop() // Remove the project directory name
    const customEngineDir = pathParts.join('/')
    
    // Look for .sln file in the custom engine directory
    const result = await invoke('launch_custom_engine_with_ide', {
      customEngineDir,
      idePath
    })
    
    addLog('Custom engine opened with IDE successfully')
    emit('close')
  } catch (error) {
    console.error('Failed to launch custom engine with IDE:', error)
    addLog('Failed to open custom engine with IDE. No .sln file found in the custom engine directory.', 'error')
  } finally {
    isLaunching.value = false
  }
}

const openSettings = () => {
  showPopup({
    id: 'settings',
    component: 'Settings',
    props: {}
  })
}

const loadSettings = async () => {
  try {
    settings.value = await invoke('get_settings')
  } catch (error) {
    console.error('Failed to load settings:', error)
    addLog('Failed to load settings', 'error')
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.project-launch-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 32rem;
  overflow: hidden;
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
  padding: var(--spacing-lg);
  overflow: hidden;
}

.project-info {
  margin-bottom: var(--spacing-lg);
  text-align: center;
}

.project-name {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
}

.project-note {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.launch-options {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-lg);
}

.launch-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  background-color: var(--surface-color);
  cursor: pointer;
  transition: all var(--transition-fast);
  text-align: left;
}

.launch-option:hover:not(:disabled) {
  border-color: var(--accent-color);
  background-color: var(--hover-color);
}

.launch-option:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.custom-engine-option {
  border-color: #f56500;
}

.custom-engine-option:hover:not(:disabled) {
  border-color: #f56500;
  background-color: rgba(245, 101, 0, 0.1);
}

.option-icon {
  font-size: var(--icon-size-xl);
  flex-shrink: 0;
}

.option-content {
  flex-grow: 1;
}

.option-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.option-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.ide-selection {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.ide-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.ide-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.ide-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--background-color);
  cursor: pointer;
  transition: all var(--transition-fast);
  text-align: left;
}

.ide-item:hover:not(:disabled) {
  border-color: var(--accent-color);
  background-color: var(--hover-color);
}

.ide-item:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.ide-icon {
  font-size: var(--font-size-lg);
  flex-shrink: 0;
}

.ide-info {
  flex-grow: 1;
  min-width: 0;
}

.ide-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.ide-path {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  word-break: break-all;
}

.no-ides {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-lg);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.no-ides-icon {
  font-size: var(--icon-size-lg);
  margin-bottom: var(--spacing-sm);
}

.no-ides-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-ides-subtext {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-md);
}

.settings-button {
  padding: var(--spacing-xs) var(--spacing-sm);
  border: var(--border-width) solid var(--accent-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--accent-color);
  color: white;
  font-size: var(--font-size-xs);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.settings-button:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}
</style>