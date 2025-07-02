<template>
  <div class="settings-popup">
    <div class="popup-header">
      <div class="header-content">
        <h2 class="popup-title">
          <span class="title-icon">⚙️</span>
          Settings
        </h2>
        <div class="settings-subtitle">Configure your application preferences</div>
      </div>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="settings-layout">
        <!-- Left Sidebar -->
        <div class="settings-sidebar">
          <nav class="settings-nav">
            <button
              v-for="tab in tabs"
              :key="tab.id"
              class="nav-item"
              :class="{ active: activeTab === tab.id }"
              @click="setActiveTab(tab.id)"
            >
              <span class="nav-icon">{{ tab.icon }}</span>
              <span class="nav-label">{{ tab.label }}</span>
            </button>
          </nav>
        </div>

        <!-- Right Content Area -->
        <div class="settings-content">
          <div class="content-header">
            <h3 class="content-title">{{ getCurrentTab()?.label }}</h3>
            <div class="content-description">{{ getCurrentTab()?.description }}</div>
          </div>

          <div class="content-body">
            <!-- General Settings -->
            <GeneralSettings
              v-if="activeTab === 'general'"
              :settings="settings"
              @update="handleSettingsUpdate"
            />

            <!-- IDE Programs -->
            <IdeSettings
              v-if="activeTab === 'ide'"
              :settings="settings"
              @update="handleSettingsUpdate"
            />

            <!-- Engine Programs -->
            <EngineSettings
              v-if="activeTab === 'engines'"
              :settings="settings"
              @update="handleSettingsUpdate"
            />

            <!-- Cleaning Defaults -->
            <CleaningSettings
              v-if="activeTab === 'cleaning'"
              :settings="settings"
              @update="handleSettingsUpdate"
            />

            <!-- Compression Settings -->
            <CompressionSettings
              v-if="activeTab === 'compression'"
              :settings="settings"
              @update="handleSettingsUpdate"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <div class="left-actions">
        <button
          class="action-button secondary-button"
          @click="resetToDefaults"
          :disabled="isSaving"
        >
          <span class="button-icon">🔄</span>
          Reset to Defaults
        </button>
      </div>
      
      <div class="right-actions">
        <button
          class="action-button secondary-button"
          @click="$emit('close')"
          :disabled="isSaving"
        >
          Cancel
        </button>
        <button
          class="action-button primary-button"
          @click="saveSettings"
          :disabled="isSaving"
        >
          <span class="button-icon">{{ isSaving ? '⏳' : '💾' }}</span>
          {{ isSaving ? 'Saving...' : 'Save Settings' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogStore } from '../../stores/logStore'
import GeneralSettings from './settings/GeneralSettings.vue'
import IdeSettings from './settings/IdeSettings.vue'
import EngineSettings from './settings/EngineSettings.vue'
import CleaningSettings from './settings/CleaningSettings.vue'
import CompressionSettings from './settings/CompressionSettings.vue'

interface AppSettings {
  ide_programs: {
    custom_programs: Record<string, string>
  }
  engine_programs: {
    custom_engines: Record<string, string>
  }
  cleaning_defaults: {
    ide_files: boolean
    binaries: boolean
    build: boolean
    intermediate: boolean
    derived_data_cache: boolean
    saved: boolean
    analyze_plugins: boolean
    plugin_binaries: boolean
    plugin_intermediate: boolean
    plugin_node_size_cache: boolean
  }
  general: {
    autostart_enabled: boolean
    show_welcome_popup: boolean
  }
  compression: {
    filename_format: string
    custom_presets: Record<string, string>
  }
}

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const activeTab = ref('general')
const isSaving = ref(false)

const tabs = [
  {
    id: 'general',
    label: 'General',
    icon: '🏠',
    description: 'General application settings and preferences'
  },
  {
    id: 'ide',
    label: 'IDE Programs',
    icon: '💻',
    description: 'Configure IDE programs for opening projects'
  },
  {
    id: 'engines',
    label: 'Engine Programs',
    icon: '⚙️',
    description: 'Manage Unreal Engine installations'
  },
  {
    id: 'cleaning',
    label: 'Cleaning Defaults',
    icon: '🧹',
    description: 'Default settings for project cleaning operations'
  },
  {
    id: 'compression',
    label: 'Compression',
    icon: '🗜️',
    description: 'Compression and archiving preferences'
  }
]

const settings = reactive<AppSettings>({
  ide_programs: {
    custom_programs: {}
  },
  engine_programs: {
    custom_engines: {}
  },
  cleaning_defaults: {
    ide_files: true,
    binaries: true,
    build: true,
    intermediate: true,
    derived_data_cache: false,
    saved: false,
    analyze_plugins: false,
    plugin_binaries: false,
    plugin_intermediate: false,
    plugin_node_size_cache: false
  },
  general: {
    autostart_enabled: false,
    show_welcome_popup: true
  },
  compression: {
    filename_format: '[Project]_[YYYY][MM][DD][HH][mm]',
    custom_presets: {}
  }
})

const setActiveTab = (tabId: string) => {
  activeTab.value = tabId
}

const getCurrentTab = () => {
  return tabs.find(tab => tab.id === activeTab.value)
}

const handleSettingsUpdate = (updatedSettings: Partial<AppSettings>) => {
  Object.assign(settings, updatedSettings)
}

const loadSettings = async () => {
  try {
    const loadedSettings = await invoke('get_settings') as AppSettings
    Object.assign(settings, loadedSettings)
  } catch (error) {
    console.error('Failed to load settings:', error)
    addLog('Failed to load settings', 'error')
  }
}

const saveSettings = async () => {
  try {
    isSaving.value = true
    await invoke('save_settings', { settings })
    addLog('Settings saved successfully')
    emit('close')
  } catch (error) {
    console.error('Failed to save settings:', error)
    addLog('Failed to save settings', 'error')
  } finally {
    isSaving.value = false
  }
}

const resetToDefaults = async () => {
  if (confirm('Are you sure you want to reset all settings to their default values? This action cannot be undone.')) {
    // Reset to default values
    Object.assign(settings, {
      ide_programs: {
        custom_programs: {}
      },
      engine_programs: {
        custom_engines: {}
      },
      cleaning_defaults: {
        ide_files: true,
        binaries: true,
        build: true,
        intermediate: true,
        derived_data_cache: false,
        saved: false,
        analyze_plugins: false,
        plugin_binaries: false,
        plugin_intermediate: false,
        plugin_node_size_cache: false
      },
      general: {
        autostart_enabled: false,
        show_welcome_popup: true
      },
      compression: {
        filename_format: '[Project]_[YYYY][MM][DD][HH][mm]',
        custom_presets: {
          'Default': '[Project]_[YYYY][MM][DD][HH][mm]',
          'Default Extended': '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
          'Simple': '[Project]_[Type]'
        }
      }
    })
    
    addLog('Settings reset to defaults')
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.settings-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 56rem;
  min-width: 56rem;
  max-height: 85vh;
  min-height: 40rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.popup-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: var(--spacing-lg);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
  flex-shrink: 0;
}

.header-content {
  flex-grow: 1;
}

.popup-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.title-icon {
  font-size: var(--icon-size-lg);
}

.settings-subtitle {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
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
  flex-shrink: 0;
}

.close-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.popup-content {
  flex-grow: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.settings-layout {
  display: flex;
  flex-grow: 1;
  overflow: hidden;
}

.settings-sidebar {
  width: 12rem;
  min-width: 12rem;
  background-color: var(--surface-color);
  border-right: var(--border-width) solid var(--border-color);
  overflow-y: auto;
}

.settings-nav {
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border: none;
  background: none;
  text-align: left;
  cursor: pointer;
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
  width: 100%;
}

.nav-item:hover {
  background-color: var(--hover-color);
}

.nav-item.active {
  background-color: var(--accent-color-alpha);
  color: var(--accent-color);
  font-weight: var(--font-weight-medium);
}

.nav-icon {
  font-size: var(--font-size-md);
  flex-shrink: 0;
}

.nav-label {
  flex-grow: 1;
}

.settings-content {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  padding: var(--spacing-lg);
  border-bottom: var(--border-width) solid var(--border-color);
  flex-shrink: 0;
}

.content-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
}

.content-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
}

.content-body {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-lg);
}

.popup-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  flex-shrink: 0;
}

.left-actions,
.right-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.action-button {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  border: var(--border-width) solid;
}

.secondary-button {
  background-color: transparent;
  border-color: var(--border-color);
  color: var(--text-secondary);
}

.secondary-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.primary-button {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
  color: white;
}

.primary-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .settings-popup {
    min-width: 90vw;
    max-width: 90vw;
  }
  
  .settings-sidebar {
    width: 10rem;
    min-width: 10rem;
  }
  
  .nav-label {
    font-size: var(--font-size-xs);
  }
}
</style>