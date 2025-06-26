<template>
  <div class="settings-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">⚙️</span>
        Settings
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <!-- Tabs Navigation -->
      <div class="tabs-nav">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          class="tab-button"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >
          <span class="tab-icon">{{ tab.icon }}</span>
          <span class="tab-title">{{ tab.title }}</span>
        </button>
      </div>

      <!-- Tab Content Container with Fixed Height -->
      <div class="tab-content-container">
        <!-- IDE Programs Tab -->
        <div v-show="activeTab === 'ide'" class="tab-panel">
          <div class="settings-section">
            <h3 class="section-title">IDE Programs</h3>
            <div class="section-description">
              Configure your preferred IDEs for opening C++ projects. Add any IDE or code editor you want to use.
            </div>

            <div class="custom-programs">
              <div class="custom-program-list">
                <div 
                  v-for="(_path, name) in localSettings.ide_programs.custom_programs"
                  :key="name"
                  class="custom-program-item"
                >
                  <div class="program-icon">
                    <img 
                      v-if="programIcons[name]" 
                      :src="programIcons[name]" 
                      :alt="name"
                      class="icon-image"
                      @error="handleIconError(name)"
                    />
                    <span v-else class="fallback-icon">⚙️</span>
                  </div>
                  <input
                    v-model="customProgramNames[name]"
                    type="text"
                    class="custom-name-input"
                    placeholder="Program name..."
                    @blur="updateCustomProgramName(name, customProgramNames[name])"
                  />
                  <input
                    v-model="localSettings.ide_programs.custom_programs[name]"
                    type="text"
                    class="custom-path-input"
                    placeholder="Path to executable..."
                    @change="extractIcon(name, localSettings.ide_programs.custom_programs[name])"
                  />
                  <button
                    class="browse-button"
                    @click="browseForCustomIde(name)"
                    title="Browse for executable"
                  >
                    📂
                  </button>
                  <button
                    class="remove-button"
                    @click="removeCustomProgram(name)"
                    title="Remove program"
                  >
                    🗑️
                  </button>
                </div>
              </div>
              <button class="add-custom-button" @click="addCustomProgram">
                <span class="button-icon">➕</span>
                Add IDE Program
              </button>
            </div>
          </div>
        </div>

        <!-- Cleaning Defaults Tab -->
        <div v-show="activeTab === 'cleaning'" class="tab-panel">
          <div class="settings-section">
            <h3 class="section-title">Cleaning Default Selection</h3>
            <div class="section-description">
              Set the default selection for the clean project popup.
            </div>

            <div class="cleaning-defaults-grid">
              <!-- Project Scanning Column -->
              <div class="defaults-column">
                <h4 class="subsection-title">Project Scanning</h4>
                <div class="checkbox-group">
                  <div class="checkbox-item">
                    <input
                      id="default-ide-files"
                      v-model="localSettings.cleaning_defaults.ide_files"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-ide-files" class="checkbox-label">
                      IDE files (.vs and .idea)
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-binaries"
                      v-model="localSettings.cleaning_defaults.binaries"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-binaries" class="checkbox-label">
                      Binaries
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-build"
                      v-model="localSettings.cleaning_defaults.build"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-build" class="checkbox-label">
                      Build
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-intermediate"
                      v-model="localSettings.cleaning_defaults.intermediate"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-intermediate" class="checkbox-label">
                      Intermediate
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-derived-data-cache"
                      v-model="localSettings.cleaning_defaults.derived_data_cache"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-derived-data-cache" class="checkbox-label">
                      DerivedDataCache
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-saved"
                      v-model="localSettings.cleaning_defaults.saved"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-saved" class="checkbox-label">
                      Saved
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-analyze-plugins"
                      v-model="localSettings.cleaning_defaults.analyze_plugins"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-analyze-plugins" class="checkbox-label">
                      Analyze plugins
                    </label>
                  </div>
                </div>
              </div>

              <!-- Plugins Scanning Column -->
              <div class="defaults-column">
                <h4 class="subsection-title">Plugins Scanning</h4>
                <div class="checkbox-group">
                  <div class="checkbox-item">
                    <input
                      id="default-plugin-binaries"
                      v-model="localSettings.cleaning_defaults.plugin_binaries"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-plugin-binaries" class="checkbox-label">
                      Binaries
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-plugin-intermediate"
                      v-model="localSettings.cleaning_defaults.plugin_intermediate"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-plugin-intermediate" class="checkbox-label">
                      Intermediate
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-plugin-node-size-cache"
                      v-model="localSettings.cleaning_defaults.plugin_node_size_cache"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-plugin-node-size-cache" class="checkbox-label">
                      NodeSizeCache
                    </label>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <button class="cancel-button" @click="$emit('close')">
        Cancel
      </button>
      <button class="save-button" @click="saveSettings" :disabled="isSaving">
        <span class="button-icon">{{ isSaving ? '⏳' : '💾' }}</span>
        {{ isSaving ? 'Saving...' : 'Save Settings' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useLogStore } from '../../stores/logStore'

interface AppSettings {
  ide_programs: {
    custom_programs: Record<string, string>
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
}

interface Tab {
  id: string
  title: string
  icon: string
}

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const activeTab = ref('ide')
const isSaving = ref(false)
const customProgramNames = ref<Record<string, string>>({})
const programIcons = ref<Record<string, string>>({})

const tabs: Tab[] = [
  { id: 'ide', title: 'IDE Programs', icon: '💻' },
  { id: 'cleaning', title: 'Cleaning Defaults', icon: '🧹' }
]

const localSettings = reactive<AppSettings>({
  ide_programs: {
    custom_programs: {}
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
  }
})

const loadSettings = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    
    // Update local settings
    localSettings.ide_programs.custom_programs = { ...settings.ide_programs.custom_programs }
    localSettings.cleaning_defaults = { ...settings.cleaning_defaults }
    
    // Initialize custom program names
    Object.keys(localSettings.ide_programs.custom_programs).forEach(name => {
      customProgramNames.value[name] = name
      extractIcon(name, localSettings.ide_programs.custom_programs[name])
    })
    
  } catch (error) {
    console.error('Failed to load settings:', error)
    addLog('Failed to load settings', 'error')
  }
}

const saveSettings = async () => {
  try {
    isSaving.value = true
    
    // Clean up empty values
    const settingsToSave: AppSettings = {
      ide_programs: {
        custom_programs: { ...localSettings.ide_programs.custom_programs }
      },
      cleaning_defaults: { ...localSettings.cleaning_defaults }
    }
    
    await invoke('save_settings', { settings: settingsToSave })
    addLog('Settings saved successfully')
    emit('close')
    
  } catch (error) {
    console.error('Failed to save settings:', error)
    addLog('Failed to save settings', 'error')
  } finally {
    isSaving.value = false
  }
}

const browseForCustomIde = async (programName: string) => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: `Select ${programName} executable`,
      filters: [{
        name: 'Executable',
        extensions: ['exe', 'app', 'AppImage']
      }]
    })
    
    if (selected && typeof selected === 'string') {
      localSettings.ide_programs.custom_programs[programName] = selected
      extractIcon(programName, selected)
    }
  } catch (error) {
    console.error('Failed to browse for custom IDE:', error)
    addLog('Failed to browse for custom IDE', 'error')
  }
}

const addCustomProgram = () => {
  const newName = `IDE Program ${Object.keys(localSettings.ide_programs.custom_programs).length + 1}`
  localSettings.ide_programs.custom_programs[newName] = ''
  customProgramNames.value[newName] = newName
}

const removeCustomProgram = (programName: string) => {
  delete localSettings.ide_programs.custom_programs[programName]
  delete customProgramNames.value[programName]
  delete programIcons.value[programName]
}

const updateCustomProgramName = (oldName: string, newName: string) => {
  if (oldName === newName || !newName.trim()) return
  
  const path = localSettings.ide_programs.custom_programs[oldName]
  const icon = programIcons.value[oldName]
  
  delete localSettings.ide_programs.custom_programs[oldName]
  delete customProgramNames.value[oldName]
  delete programIcons.value[oldName]
  
  localSettings.ide_programs.custom_programs[newName] = path
  customProgramNames.value[newName] = newName
  if (icon) {
    programIcons.value[newName] = icon
  }
}

const extractIcon = (programName: string, executablePath: string) => {
  if (!executablePath) return
  
  // Try to extract icon using file:// protocol for local files
  // This is a simplified approach - in a real application you might want to use
  // a more sophisticated icon extraction method
  try {
    // For now, we'll use a simple file:// URL approach
    // Note: This might not work in all browsers due to security restrictions
    const iconUrl = `file://${executablePath}`
    programIcons.value[programName] = iconUrl
  } catch (error) {
    console.warn('Failed to extract icon for', programName, error)
    // Fallback to default icon
    delete programIcons.value[programName]
  }
}

const handleIconError = (programName: string) => {
  // Remove the failed icon URL so the fallback icon is shown
  delete programIcons.value[programName]
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
  width: 48rem;
  height: 36rem;
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
  flex-shrink: 0;
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
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tabs-nav {
  display: flex;
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
  flex-shrink: 0;
}

.tab-button {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  background-color: transparent;
  border-right: var(--border-width) solid var(--border-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  font-weight: var(--font-weight-medium);
}

.tab-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.tab-button.active {
  background-color: var(--background-color);
  color: var(--text-primary);
  border-bottom: 2px solid var(--accent-color);
}

.tab-icon {
  font-size: var(--font-size-sm);
}

.tab-content-container {
  flex-grow: 1;
  overflow: hidden;
  position: relative;
}

.tab-panel {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.settings-section {
  margin-bottom: var(--spacing-xl);
}

.settings-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-lg);
}

.custom-programs {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.custom-program-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.custom-program-item {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
}

.program-icon {
  flex: 0 0 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-image {
  width: 1.5rem;
  height: 1.5rem;
  object-fit: contain;
}

.fallback-icon {
  font-size: var(--font-size-lg);
}

.custom-name-input {
  flex: 0 0 8rem;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
}

.custom-path-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
}

.browse-button,
.remove-button {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  min-width: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.browse-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.remove-button:hover {
  background-color: #fed7d7;
  border-color: #e53e3e;
}

.add-custom-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
}

.add-custom-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.cleaning-defaults-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-xl);
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.defaults-column {
  display: flex;
  flex-direction: column;
}

.subsection-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.checkbox-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
}

.checkbox-input {
  margin-top: 2px;
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
  flex-shrink: 0;
}

.checkbox-label {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  line-height: var(--line-height-normal);
  flex-grow: 1;
}

.popup-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  flex-shrink: 0;
}

.cancel-button,
.save-button {
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

.cancel-button {
  background-color: transparent;
  border: var(--border-width) solid var(--border-color);
  color: var(--text-secondary);
}

.cancel-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.save-button {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.save-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.save-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>