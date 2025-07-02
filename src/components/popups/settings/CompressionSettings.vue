<template>
  <div class="compression-settings">
    <div class="settings-section">
      <h4 class="section-title">Filename Format</h4>
      <div class="section-description">
        Configure the default filename format for compressed project archives. Select a preset to apply its format.
      </div>
      
      <div class="format-group">
        <label class="form-label">Default Format</label>
        <div class="format-input-group">
          <select
            v-model="selectedPreset"
            class="preset-dropdown"
            @change="applyPreset"
          >
            <option 
              v-for="(_format, name) in sortedAvailableFormats"
              :key="name"
              :value="name"
            >
              {{ name }}
            </option>
          </select>
          <input
            v-model="localCompression.filename_format"
            type="text"
            class="format-input readonly"
            placeholder="Format will be set by preset selection..."
            readonly
          />
        </div>
        <div class="format-preview">
          <span class="preview-label">Preview:</span>
          <span class="preview-filename">{{ previewFilename }}</span>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <div class="section-header-content">
          <h4 class="section-title">Custom Presets</h4>
          <div class="section-description">
            Create and manage custom filename format presets for quick access.
          </div>
        </div>
        <button class="add-preset-btn" @click="openAddPresetPopup">
          <span class="button-icon">➕</span>
          Add Custom Preset
        </button>
      </div>
      
      <div class="presets-list">
        <div 
          v-for="(format, name) in sortedCustomPresets"
          :key="name"
          class="preset-item"
        >
          <div class="preset-info">
            <div class="preset-name">{{ name }}</div>
            <div class="preset-format">{{ format }}</div>
          </div>
          <div class="preset-actions">
            <button
              class="action-btn edit-btn"
              @click="editPreset(name as string, format)"
              title="Edit preset"
            >
              ✏️
            </button>
            <button
              class="action-btn remove-btn"
              @click="removePreset(name as string)"
              title="Remove preset"
              :disabled="name === 'Default'"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="Object.keys(localCompression.custom_presets).length === 0" class="no-presets">
          <div class="no-presets-icon">🗜️</div>
          <div class="no-presets-text">No custom presets</div>
          <div class="no-presets-subtext">Create presets for commonly used filename formats</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { usePopup } from '../../../composables/usePopup'
import { useSettingsStore } from '../../../stores/settingsStore'

const { 
  getSettings, 
  addCompressionPreset, 
  removeCompressionPreset, 
  updateCompressionPreset, 
  setCompressionFormat 
} = useSettingsStore()

const { showPopup } = usePopup()

const localCompression = reactive({ ...getSettings('compression') })

const selectedPreset = ref('Default')
const systemInfo = ref({
  username: 'john_doe',
  hostname: 'DESKTOP-PC'
})

// Sort available formats alphabetically by name
const sortedAvailableFormats = computed(() => {
  const entries = Object.entries(localCompression.custom_presets)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

// Sort custom presets alphabetically by name
const sortedCustomPresets = computed(() => {
  const entries = Object.entries(localCompression.custom_presets)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

const previewFilename = computed(() => {
  return getPreviewForFormat(localCompression.filename_format)
})

const getPreviewForFormat = (format: string): string => {
  const now = new Date()
  let preview = format
  
  // Replace common tags with example values
  const replacements: Record<string, string> = {
    'Project': 'MyAwesomeProject',
    'Type': 'Cpp',
    'Engine': '5-3',
    'SizeMB': '1024',
    'SizeGB': '1',
    'PluginCount': '5',
    'Algorithm': 'ZIP',
    'YYYY': now.getFullYear().toString(),
    'YY': now.getFullYear().toString().slice(-2),
    'MM': (now.getMonth() + 1).toString().padStart(2, '0'),
    'DD': now.getDate().toString().padStart(2, '0'),
    'HH': now.getHours().toString().padStart(2, '0'),
    'mm': now.getMinutes().toString().padStart(2, '0'),
    'ss': now.getSeconds().toString().padStart(2, '0'),
    'Month': now.toLocaleDateString('en-US', { month: 'long' }),
    'Mon': now.toLocaleDateString('en-US', { month: 'short' }),
    'Day': now.toLocaleDateString('en-US', { weekday: 'long' }),
    'Weekday': now.toLocaleDateString('en-US', { weekday: 'short' }),
    'User': systemInfo.value.username,
    'Computer': systemInfo.value.hostname,
    'Timestamp': Math.floor(now.getTime() / 1000).toString()
  }
  
  for (const [key, value] of Object.entries(replacements)) {
    preview = preview.replace(new RegExp(`\\[${key}\\]`, 'g'), value)
  }
  
  if (!preview.includes('.')) {
    preview += '.zip'
  }
  
  return preview
}

const applyPreset = () => {
  if (selectedPreset.value && localCompression.custom_presets[selectedPreset.value]) {
    localCompression.filename_format = localCompression.custom_presets[selectedPreset.value]
    setCompressionFormat(localCompression.filename_format)
  }
}

const openAddPresetPopup = () => {
  showPopup({
    id: 'preset-form',
    component: 'PresetForm',
    props: {
      onSave: handlePresetSave
    }
  })
}

const editPreset = (name: string, format: string) => {
  showPopup({
    id: 'preset-form',
    component: 'PresetForm',
    props: {
      editingPreset: name,
      initialName: name,
      initialFormat: format,
      onSave: handlePresetSave
    }
  })
}

const removePreset = (name: string) => {
  if (name === 'Default') return // Prevent removing the default preset
  
  if (confirm(`Are you sure you want to remove the preset "${name}"?`)) {
    removeCompressionPreset(name)
    delete localCompression.custom_presets[name]
    
    // If the removed preset was selected, switch to Default
    if (selectedPreset.value === name) {
      selectedPreset.value = 'Default'
      applyPreset()
    }
  }
}

const handlePresetSave = (data: { name: string; format: string; isEdit: boolean; originalName?: string }) => {
  if (data.isEdit && data.originalName) {
    updateCompressionPreset(data.originalName, data.name, data.format)
    // Remove old entry if name changed
    if (data.originalName !== data.name) {
      delete localCompression.custom_presets[data.originalName]
    }
  } else {
    addCompressionPreset(data.name, data.format)
  }

  localCompression.custom_presets[data.name] = data.format
  
  // If this is a new preset or the name changed, select it
  selectedPreset.value = data.name
  applyPreset()
}

const loadSystemInfo = async () => {
  try {
    systemInfo.value.username = await invoke('get_system_username') as string
    systemInfo.value.hostname = await invoke('get_system_hostname') as string
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

// Watch for external changes to settings
watch(() => getSettings('compression'), (newCompression) => {
  Object.assign(localCompression, newCompression)
  
  // Ensure Default preset is selected if it exists
  if (localCompression.custom_presets['Default']) {
    selectedPreset.value = 'Default'
  }
}, { deep: true })

onMounted(async () => {
  await loadSystemInfo()
  
  // Set Default as selected preset if it exists
  if (localCompression.custom_presets['Default']) {
    selectedPreset.value = 'Default'
  }
})
</script>

<style scoped>
.compression-settings {
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

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--spacing-md);
}

.section-header-content {
  flex-grow: 1;
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
  margin: 0;
  line-height: var(--line-height-normal);
}

.format-group {
  margin-bottom: var(--spacing-lg);
}

.form-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.format-input-group {
  display: flex;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-sm);
}

.preset-dropdown {
  min-width: 10rem;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  cursor: pointer;
}

.format-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  font-family: var(--font-mono);
}

.format-input.readonly {
  background-color: var(--surface-color);
  cursor: not-allowed;
  color: var(--text-secondary);
}

.format-input:focus:not(.readonly) {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.format-preview {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.preview-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
  flex-shrink: 0;
}

.preview-filename {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
  word-break: break-all;
  flex-grow: 1;
}

.presets-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.preset-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.preset-info {
  flex-grow: 1;
  min-width: 0;
}

.preset-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.preset-format {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  word-break: break-all;
}

.preset-actions {
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

.action-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.remove-btn:hover:not(:disabled) {
  border-color: #e53e3e;
  background-color: #fed7d7;
}

.no-presets {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-lg);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.no-presets-icon {
  font-size: var(--icon-size-lg);
  margin-bottom: var(--spacing-sm);
  opacity: 0.5;
}

.no-presets-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-presets-subtext {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  opacity: 0.7;
}

.add-preset-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.add-preset-btn:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .format-input-group {
    flex-direction: column;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm);
  }
}
</style>