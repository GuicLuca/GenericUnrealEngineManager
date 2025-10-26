<script setup lang="ts">
import { reactive, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useSettingsStore } from '../../../stores/settingsStore'
import { usePopup } from '../../../composables/usePopup'

const { getSettings, updateSettings } = useSettingsStore()
const { showPopup } = usePopup()

const localEngines = reactive({ ...getSettings('engine_programs') })

const engineEntries = computed(() => {
  return Object.entries(localEngines.custom_engines || {})
})

const addEngine = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Unreal Engine installation directory'
    })

    if (selected) {
      showPopup({
        id: 'engine-detection',
        component: 'EngineDetection',
        props: {
          enginePath: selected
        }
      })
    }
  } catch (error) {
    console.error('Failed to select engine directory:', error)
  }
}

const removeEngine = (engineName: string) => {
  if (confirm(`Are you sure you want to remove ${engineName}?`)) {
    const engines = { ...localEngines.custom_engines }
    delete engines[engineName]
    localEngines.custom_engines = engines
    updateSettings('engine_programs', localEngines)
  }
}

const getEngineType = (engineName: string): string => {
  return engineName.startsWith('Custom-') ? 'Custom' : 'Standard'
}
</script>

<template>
  <div class="engine-settings">
    <div class="section-header">
      <h3 class="section-title">Engine Programs</h3>
      <p class="section-description">Manage Unreal Engine installations</p>
    </div>

    <div class="settings-section">
      <div class="section-toolbar">
        <button class="add-engine-button" @click="addEngine">
          <span class="button-icon">➕</span>
          Add Engine
        </button>
      </div>

      <div v-if="engineEntries.length === 0" class="empty-state">
        <div class="empty-icon">🎮</div>
        <div class="empty-title">No engines registered</div>
        <div class="empty-description">Click "Add Engine" to register an Unreal Engine installation</div>
      </div>

      <div v-else class="engine-list">
        <div
          v-for="[name, path] in engineEntries"
          :key="name"
          class="engine-item"
        >
          <div class="engine-info">
            <div class="engine-header">
              <span class="engine-name">{{ name }}</span>
              <span class="engine-badge" :class="getEngineType(name).toLowerCase()">
                {{ getEngineType(name) }}
              </span>
            </div>
            <div class="engine-path">{{ path }}</div>
          </div>
          <button
            class="remove-button"
            @click="removeEngine(name)"
            title="Remove engine"
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.engine-settings {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.section-header {
  margin-bottom: var(--spacing-md);
}

.section-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
}

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
}

.settings-section {
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.section-toolbar {
  display: flex;
  justify-content: flex-end;
}

.add-engine-button {
  padding: var(--spacing-sm) var(--spacing-md);
  border: var(--border-width) solid var(--accent-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--accent-color);
  color: white;
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.add-engine-button:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  text-align: center;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.empty-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.empty-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.engine-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.engine-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  gap: var(--spacing-md);
}

.engine-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  min-width: 0;
}

.engine-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.engine-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.engine-badge {
  padding: 2px var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.engine-badge.standard {
  background-color: #e3f2fd;
  color: #1976d2;
}

.engine-badge.custom {
  background-color: #fff3e0;
  color: #f57c00;
}

.engine-path {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.remove-button {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.remove-button:hover {
  background-color: var(--hover-color);
  border-color: #dc3545;
  color: #dc3545;
}
</style>
