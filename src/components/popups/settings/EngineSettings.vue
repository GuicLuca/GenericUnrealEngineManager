<template>
  <div class="engine-settings">
    <div class="settings-section">
      <h4 class="section-title">Custom Engine Installations</h4>
      <div class="section-description">
        Manage custom Unreal Engine installations. These can be source builds or custom engine versions.
      </div>

      <div class="engines-list">
        <div
            v-for="(path, name) in localEngines"
            :key="name"
            class="engine-item"
        >
          <div class="engine-info">
            <div class="engine-name">{{ name }}</div>
            <div class="engine-path">{{ path }}</div>
          </div>
          <div class="engine-actions">
            <button
                class="action-btn edit-btn"
                @click="editEngine(name as string, path)"
                title="Edit engine"
            >
              ✏️
            </button>
            <button
                class="action-btn remove-btn"
                @click="handleRemoveEngine(name as string)"
                title="Remove engine"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="Object.keys(localEngines).length === 0" class="no-engines">
          <div class="no-engines-icon">⚙️</div>
          <div class="no-engines-text">No custom engines configured</div>
          <div class="no-engines-subtext">Add custom engine installations or source builds</div>
        </div>
      </div>

      <div class="engine-actions-row">
        <button class="add-engine-btn" @click="openAddEnginePopup">
          <span class="button-icon">➕</span>
          Add Custom Engine
        </button>

        <button class="auto-detect-btn" @click="autoDetectEngines">
          <span class="button-icon">🔍</span>
          Auto-Detect Engines
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {reactive, watch, onMounted, onUnmounted} from 'vue'
import {usePopup} from '../../../composables/usePopup'
import {useSettingsStore} from '../../../stores/settingsStore'
import {useTaskStore} from '../../../stores/taskStore'

const {
  getSettings,
  addEngineProgram,
  removeEngineProgram,
  updateEngineProgram,
  saveSettings
} = useSettingsStore()

const {showPopup} = usePopup()
const {isEngineDetectionRunning, currentEngineDetectionTask} = useTaskStore()

const localEngines = reactive({...getSettings('engine_programs').custom_engines})

const editEngine = (name: string, path: string) => {
  showPopup({
    id: 'engine-form',
    component: 'EngineForm',
    props: {
      editingEngine: name,
      initialPath: path,
      onSave: handleEngineSave
    }
  })
}

const handleRemoveEngine = (name: string) => {
  removeEngineProgram(name)
  delete localEngines[name]
  saveSettings()
}

const openAddEnginePopup = () => {
  showPopup({
    id: 'engine-form',
    component: 'EngineForm',
    props: {
      onSave: handleEngineSave
    }
  })
}

const handleEngineSave = (data: { name: string; path: string; isEdit: boolean; originalName?: string }) => {
  if (data.isEdit && data.originalName) {
    updateEngineProgram(data.originalName, data.name, data.path)
    if (data.originalName !== data.name) {
      delete localEngines[data.originalName]
    }
  } else {
    addEngineProgram(data.name, data.path)
  }

  localEngines[data.name] = data.path

  saveSettings()
}

const autoDetectEngines = () => {
  // Check if engine detection is already running
  if (isEngineDetectionRunning.value) {
    // Show popup with current scan state
    showPopup({
      id: 'engine-detection',
      component: 'EngineDetection',
      props: {
        reconnectToRunningTask: true,
        currentTask: currentEngineDetectionTask.value
      }
    })
  } else {
    // Show popup to start new scan
    showPopup({
      id: 'engine-detection',
      component: 'EngineDetection',
      props: {}
    })
  }
}

const showEngineDetectionResults = () => {
  showPopup({
    id: 'engine-detection',
    component: 'EngineDetection',
    props: {
      showResults: true
    }
  })
}

const refreshEngineList = () => {
  // Refresh the local engines from the store
  const currentEngines = getSettings('engine_programs').custom_engines

  // Clear current engines
  Object.keys(localEngines).forEach(key => {
    delete localEngines[key]
  })

  // Add updated engines
  Object.assign(localEngines, currentEngines)
}

const handleEnginesUpdated = () => {
  // Force refresh the engine list when engines are updated
  setTimeout(() => {
    refreshEngineList()
    
    // Show results popup if user is not in settings anymore
    const currentPopup = document.querySelector('.settings-popup')
    if (!currentPopup) {
      showEngineDetectionResults()
    }
  }, 500) // Small delay to ensure backend has processed the changes
}

// Watch for external changes to settings
watch(() => getSettings('engine_programs').custom_engines, (newEngines) => {
  Object.assign(localEngines, newEngines)
}, {deep: true})

onMounted(() => {
  // Listen for engine updates from the detection popup
  window.addEventListener('engines-updated', handleEnginesUpdated)
})

onUnmounted(() => {
  // Cleanup event listener
  window.removeEventListener('engines-updated', handleEnginesUpdated)
})
</script>

<style scoped>
.engine-settings {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.settings-section {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
}

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0 0 var(--spacing-md) 0;
  line-height: var(--line-height-normal);
}

.engines-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.engine-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.engine-info {
  flex-grow: 1;
  min-width: 0;
}

.engine-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.engine-path {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  word-break: break-all;
}

.engine-actions {
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

.action-btn:hover {
  background-color: var(--hover-color);
}

.remove-btn:hover {
  border-color: #e53e3e;
  background-color: #fed7d7;
}

.no-engines {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-xl);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.no-engines-icon {
  font-size: var(--icon-size-lg);
  margin-bottom: var(--spacing-sm);
  opacity: 0.5;
}

.no-engines-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-engines-subtext {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  opacity: 0.7;
}

.engine-actions-row {
  display: flex;
  gap: var(--spacing-sm);
  flex-wrap: wrap;
}

.add-engine-btn,
.auto-detect-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-fast);
  border: var(--border-width) solid;
}

.add-engine-btn {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
  color: white;
}

.add-engine-btn:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.auto-detect-btn {
  background-color: transparent;
  border-color: var(--border-color);
  color: var(--text-secondary);
}

.auto-detect-btn:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
  border-color: var(--accent-color);
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>