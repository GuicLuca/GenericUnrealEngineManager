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
      <div class="settings-section">
        <h3 class="section-title">IDE Programs</h3>
        <div class="section-description">
          Configure your preferred IDEs for opening C++ projects. Add any IDE or code editor you want to use.
        </div>

        <!-- Custom Programs Section -->
        <div class="custom-programs">
          <div class="custom-program-list">
            <div 
              v-for="(path, name) in localSettings.ide_programs.custom_programs"
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
}

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const isSaving = ref(false)
const customProgramNames = ref<Record<string, string>>({})
const programIcons = ref<Record<string, string>>({})

const localSettings = reactive<AppSettings>({
  ide_programs: {
    custom_programs: {}
  }
})

const loadSettings = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    
    // Update local settings
    localSettings.ide_programs.custom_programs = { ...settings.ide_programs.custom_programs }
    
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
      }
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
  // Remove the failed icon URL so fallback icon is shown
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
  width: 100%;
  max-width: 42rem;
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

.settings-section {
  margin-bottom: var(--spacing-lg);
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

.popup-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
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