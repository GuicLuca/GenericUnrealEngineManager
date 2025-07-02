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
              v-for="(format, name) in localSettings.compression.custom_presets"
              :key="name"
              :value="name"
            >
              {{ name }}
            </option>
          </select>
          <input
            v-model="localSettings.compression.filename_format"
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
        <button class="add-preset-btn" @click="showAddPresetPopup = true">
          <span class="button-icon">➕</span>
          Add Custom Preset
        </button>
      </div>
      
      <div class="presets-list">
        <div 
          v-for="(format, name) in localSettings.compression.custom_presets"
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

        <div v-if="Object.keys(localSettings.compression.custom_presets).length === 0" class="no-presets">
          <div class="no-presets-icon">🗜️</div>
          <div class="no-presets-text">No custom presets</div>
          <div class="no-presets-subtext">Create presets for commonly used filename formats</div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Preset Popup -->
    <Teleport to="body">
      <Transition name="popup-overlay">
        <div 
          v-if="showAddPresetPopup || editingPreset"
          class="popup-overlay"
          @click="cancelPresetForm"
        >
          <Transition name="popup-content">
            <div 
              class="preset-popup"
              @click.stop
            >
              <div class="popup-header">
                <h4 class="popup-title">{{ editingPreset ? 'Edit' : 'Add' }} Custom Preset</h4>
                <button class="close-button" @click="cancelPresetForm" title="Close">
                  ✕
                </button>
              </div>

              <div class="popup-content">
                <div class="form-group">
                  <label class="form-label">Preset Name</label>
                  <input
                    v-model="presetForm.name"
                    type="text"
                    class="form-input"
                    placeholder="e.g., My Custom Format"
                    :disabled="!!editingPreset"
                  />
                </div>

                <div class="form-group">
                  <label class="form-label">Format Template</label>
                  <input
                    v-model="presetForm.format"
                    type="text"
                    class="form-input"
                    placeholder="e.g., [Project]_[Type]_[YYYY][MM][DD]"
                  />
                  <div class="format-preview">
                    <span class="preview-label">Preview:</span>
                    <span class="preview-filename">{{ getPreviewForFormat(presetForm.format) }}</span>
                  </div>
                </div>

                <div class="tags-section">
                  <h5 class="tags-title">Available Tags</h5>
                  <div class="tags-description">Click on any tag to add it to your format template</div>
                  
                  <div class="tags-grid">
                    <div class="tag-group">
                      <h6 class="tag-group-title">Project Info</h6>
                      <div class="tag-list">
                        <button 
                          v-for="tag in projectTags"
                          :key="tag.code"
                          class="tag-button"
                          @click="addTagToFormat(tag.code)"
                          :title="tag.description"
                        >
                          <code>{{ tag.code }}</code>
                          <span>{{ tag.label }}</span>
                        </button>
                      </div>
                    </div>

                    <div class="tag-group">
                      <h6 class="tag-group-title">Date & Time</h6>
                      <div class="tag-list">
                        <button 
                          v-for="tag in dateTags"
                          :key="tag.code"
                          class="tag-button"
                          @click="addTagToFormat(tag.code)"
                          :title="tag.description"
                        >
                          <code>{{ tag.code }}</code>
                          <span>{{ tag.label }}</span>
                        </button>
                      </div>
                    </div>

                    <div class="tag-group">
                      <h6 class="tag-group-title">System Info</h6>
                      <div class="tag-list">
                        <button 
                          v-for="tag in systemTags"
                          :key="tag.code"
                          class="tag-button"
                          @click="addTagToFormat(tag.code)"
                          :title="tag.description"
                        >
                          <code>{{ tag.code }}</code>
                          <span>{{ tag.label }}</span>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="popup-actions">
                <button
                  class="form-btn cancel-btn"
                  @click="cancelPresetForm"
                >
                  Cancel
                </button>
                <button
                  class="form-btn save-btn"
                  @click="savePreset"
                  :disabled="!presetForm.name.trim() || !presetForm.format.trim()"
                >
                  {{ editingPreset ? 'Update' : 'Add' }} Preset
                </button>
              </div>
            </div>
          </Transition>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Props {
  settings: {
    compression: {
      filename_format: string
      custom_presets: Record<string, string>
    }
  }
}

interface Emits {
  (e: 'update', settings: any): void
}

interface Tag {
  code: string
  label: string
  description: string
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localSettings = reactive({
  compression: {
    filename_format: props.settings.compression.filename_format,
    custom_presets: { ...props.settings.compression.custom_presets }
  }
})

const selectedPreset = ref('Default')
const showAddPresetPopup = ref(false)
const editingPreset = ref<string | null>(null)
const presetForm = reactive({
  name: '',
  format: ''
})

const systemInfo = ref({
  username: 'john_doe',
  hostname: 'DESKTOP-PC'
})

// Tag definitions
const projectTags: Tag[] = [
  { code: '[Project]', label: 'Project name', description: 'Name of the project' },
  { code: '[Type]', label: 'Cpp or Bp', description: 'Project type (C++ or Blueprint)' },
  { code: '[Engine]', label: 'Engine version', description: 'Unreal Engine version' },
  { code: '[SizeMB]', label: 'Size in MB', description: 'Project size in megabytes' },
  { code: '[SizeGB]', label: 'Size in GB', description: 'Project size in gigabytes' },
  { code: '[PluginCount]', label: 'Plugin count', description: 'Number of plugins in the project' }
]

const dateTags: Tag[] = [
  { code: '[YYYY]', label: 'Full year', description: 'Full year (e.g., 2024)' },
  { code: '[YY]', label: 'Short year', description: 'Two-digit year (e.g., 24)' },
  { code: '[MM]', label: 'Month', description: 'Month with leading zero (01-12)' },
  { code: '[DD]', label: 'Day', description: 'Day with leading zero (01-31)' },
  { code: '[HH]', label: 'Hour', description: 'Hour in 24-hour format (00-23)' },
  { code: '[mm]', label: 'Minute', description: 'Minute with leading zero (00-59)' },
  { code: '[ss]', label: 'Second', description: 'Second with leading zero (00-59)' },
  { code: '[Month]', label: 'Full month', description: 'Full month name (e.g., January)' },
  { code: '[Mon]', label: 'Short month', description: 'Short month name (e.g., Jan)' },
  { code: '[Day]', label: 'Full day', description: 'Full day name (e.g., Monday)' },
  { code: '[Weekday]', label: 'Short day', description: 'Short day name (e.g., Mon)' }
]

const systemTags: Tag[] = [
  { code: '[User]', label: 'Username', description: 'Current system username' },
  { code: '[Computer]', label: 'Computer name', description: 'Computer hostname' },
  { code: '[Timestamp]', label: 'Unix timestamp', description: 'Unix timestamp in seconds' },
  { code: '[Algorithm]', label: 'Compression type', description: 'Compression algorithm used' }
]

const previewFilename = computed(() => {
  return getPreviewForFormat(localSettings.compression.filename_format)
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

const emitUpdate = () => {
  emit('update', { compression: localSettings.compression })
}

const applyPreset = () => {
  if (selectedPreset.value && localSettings.compression.custom_presets[selectedPreset.value]) {
    localSettings.compression.filename_format = localSettings.compression.custom_presets[selectedPreset.value]
    emitUpdate()
  }
}

const editPreset = (name: string, format: string) => {
  editingPreset.value = name
  presetForm.name = name
  presetForm.format = format
  showAddPresetPopup.value = true
}

const removePreset = (name: string) => {
  if (name === 'Default') return // Prevent removing the default preset
  
  if (confirm(`Are you sure you want to remove the preset "${name}"?`)) {
    delete localSettings.compression.custom_presets[name]
    
    // If the removed preset was selected, switch to Default
    if (selectedPreset.value === name) {
      selectedPreset.value = 'Default'
      applyPreset()
    }
    
    emitUpdate()
  }
}

const addTagToFormat = (tagCode: string) => {
  // Add the tag at the cursor position or at the end
  const input = document.querySelector('.form-input') as HTMLInputElement
  if (input && input === document.activeElement) {
    const start = input.selectionStart || 0
    const end = input.selectionEnd || 0
    const before = presetForm.format.substring(0, start)
    const after = presetForm.format.substring(end)
    presetForm.format = before + tagCode + after
    
    // Set cursor position after the inserted tag
    setTimeout(() => {
      input.setSelectionRange(start + tagCode.length, start + tagCode.length)
      input.focus()
    }, 0)
  } else {
    // Just append to the end if no cursor position
    presetForm.format += tagCode
  }
}

const savePreset = () => {
  if (!presetForm.name.trim() || !presetForm.format.trim()) return

  if (editingPreset.value) {
    // Remove old entry if name changed
    if (editingPreset.value !== presetForm.name) {
      delete localSettings.compression.custom_presets[editingPreset.value]
    }
  }

  localSettings.compression.custom_presets[presetForm.name] = presetForm.format
  
  // If this is a new preset or the name changed, select it
  selectedPreset.value = presetForm.name
  applyPreset()
  
  emitUpdate()
  cancelPresetForm()
}

const cancelPresetForm = () => {
  showAddPresetPopup.value = false
  editingPreset.value = null
  presetForm.name = ''
  presetForm.format = ''
}

const loadSystemInfo = async () => {
  try {
    systemInfo.value.username = await invoke('get_system_username') as string
    systemInfo.value.hostname = await invoke('get_system_hostname') as string
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

// Watch for external changes to props
watch(() => props.settings.compression, (newCompression) => {
  localSettings.compression.filename_format = newCompression.filename_format
  localSettings.compression.custom_presets = { ...newCompression.custom_presets }
  
  // Ensure Default preset is selected if it exists
  if (localSettings.compression.custom_presets['Default']) {
    selectedPreset.value = 'Default'
  }
}, { deep: true })

onMounted(() => {
  loadSystemInfo()
  
  // Set Default as selected preset if it exists
  if (localSettings.compression.custom_presets['Default']) {
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

/* Popup Styles */
.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(2px);
  z-index: 10000;
}

.preset-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 42rem;
  max-height: 85vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
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
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
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

.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-input {
  width: 100%;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  transition: border-color var(--transition-fast);
  font-family: var(--font-mono);
}

.form-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.form-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tags-section {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.tags-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
}

.tags-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0 0 var(--spacing-md) 0;
  line-height: var(--line-height-normal);
}

.tags-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-md);
}

.tag-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.tag-group-title {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.tag-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.tag-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-xs);
  transition: all var(--transition-fast);
  text-align: left;
}

.tag-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.tag-button:active {
  background-color: var(--active-color);
}

.tag-button code {
  background-color: var(--surface-color);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  font-family: var(--font-mono);
  color: var(--accent-color);
  font-weight: var(--font-weight-medium);
  flex-shrink: 0;
  font-size: var(--font-size-xs);
}

.tag-button span {
  color: var(--text-secondary);
  flex-grow: 1;
  min-width: 0;
}

.popup-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  flex-shrink: 0;
}

.form-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  border: var(--border-width) solid;
}

.cancel-btn {
  background-color: transparent;
  border-color: var(--border-color);
  color: var(--text-secondary);
}

.cancel-btn:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.save-btn {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
  color: white;
}

.save-btn:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Transitions */
.popup-overlay-enter-active,
.popup-overlay-leave-active {
  transition: opacity var(--transition-normal);
}

.popup-overlay-enter-from,
.popup-overlay-leave-to {
  opacity: 0;
}

.popup-content-enter-active,
.popup-content-leave-active {
  transition: all var(--transition-normal);
}

.popup-content-enter-from,
.popup-content-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .tags-grid {
    grid-template-columns: 1fr;
  }
  
  .format-input-group {
    flex-direction: column;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm);
  }
  
  .preset-popup {
    max-width: 90vw;
    margin: var(--spacing-md);
  }
}
</style>