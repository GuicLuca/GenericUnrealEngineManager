<template>
  <div class="compression-settings">
    <div class="settings-section">
      <h4 class="section-title">Filename Format</h4>
      <div class="section-description">
        Configure the default filename format for compressed project archives. Use placeholders to create dynamic filenames.
      </div>
      
      <div class="format-group">
        <label class="form-label">Default Format</label>
        <div class="format-input-group">
          <select
            v-model="selectedPreset"
            class="preset-dropdown"
            @change="applyPreset"
          >
            <option value="">Select a preset...</option>
            <option 
              v-for="(_format, name) in localSettings.compression.custom_presets"
              :key="name"
              :value="name"
            >
              {{ name }}
            </option>
          </select>
          <input
            v-model="localSettings.compression.filename_format"
            type="text"
            class="format-input"
            placeholder="Enter filename format..."
            @input="emitUpdate"
          />
        </div>
        <div class="format-preview">
          <span class="preview-label">Preview:</span>
          <span class="preview-filename">{{ previewFilename }}</span>
        </div>
      </div>

      <div class="placeholders-info">
        <h5 class="placeholders-title">Available Placeholders</h5>
        <div class="placeholders-grid">
          <div class="placeholder-group">
            <h6 class="group-title">Project Info</h6>
            <div class="placeholder-item">
              <code>[Project]</code>
              <span>Project name</span>
            </div>
            <div class="placeholder-item">
              <code>[Type]</code>
              <span>Cpp or Bp</span>
            </div>
            <div class="placeholder-item">
              <code>[Engine]</code>
              <span>Engine version</span>
            </div>
            <div class="placeholder-item">
              <code>[SizeMB]</code>
              <span>Size in MB</span>
            </div>
            <div class="placeholder-item">
              <code>[PluginCount]</code>
              <span>Number of plugins</span>
            </div>
          </div>

          <div class="placeholder-group">
            <h6 class="group-title">Date & Time</h6>
            <div class="placeholder-item">
              <code>[YYYY]</code>
              <span>Full year (2024)</span>
            </div>
            <div class="placeholder-item">
              <code>[MM]</code>
              <span>Month (01-12)</span>
            </div>
            <div class="placeholder-item">
              <code>[DD]</code>
              <span>Day (01-31)</span>
            </div>
            <div class="placeholder-item">
              <code>[HH]</code>
              <span>Hour (00-23)</span>
            </div>
            <div class="placeholder-item">
              <code>[mm]</code>
              <span>Minute (00-59)</span>
            </div>
            <div class="placeholder-item">
              <code>[Month]</code>
              <span>Full month name</span>
            </div>
          </div>

          <div class="placeholder-group">
            <h6 class="group-title">System Info</h6>
            <div class="placeholder-item">
              <code>[User]</code>
              <span>Username</span>
            </div>
            <div class="placeholder-item">
              <code>[Computer]</code>
              <span>Computer name</span>
            </div>
            <div class="placeholder-item">
              <code>[Timestamp]</code>
              <span>Unix timestamp</span>
            </div>
            <div class="placeholder-item">
              <code>[Algorithm]</code>
              <span>Compression type</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h4 class="section-title">Custom Presets</h4>
      <div class="section-description">
        Create and manage custom filename format presets for quick access.
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

      <button class="add-preset-btn" @click="showAddPreset = true">
        <span class="button-icon">➕</span>
        Add Custom Preset
      </button>
    </div>

    <!-- Add/Edit Preset Form -->
    <div v-if="showAddPreset || editingPreset" class="preset-form">
      <h4 class="form-title">{{ editingPreset ? 'Edit' : 'Add' }} Custom Preset</h4>
      
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

      <div class="form-actions">
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

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localSettings = reactive({
  compression: {
    filename_format: props.settings.compression.filename_format,
    custom_presets: { ...props.settings.compression.custom_presets }
  }
})

const selectedPreset = ref('')
const showAddPreset = ref(false)
const editingPreset = ref<string | null>(null)
const presetForm = reactive({
  name: '',
  format: ''
})

const systemInfo = ref({
  username: 'john_doe',
  hostname: 'DESKTOP-PC'
})

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
  showAddPreset.value = false
}

const removePreset = (name: string) => {
  if (confirm(`Are you sure you want to remove the preset "${name}"?`)) {
    delete localSettings.compression.custom_presets[name]
    emitUpdate()
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
  emitUpdate()
  cancelPresetForm()
}

const cancelPresetForm = () => {
  showAddPreset.value = false
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
}, { deep: true })

onMounted(() => {
  loadSystemInfo()
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

.format-input:focus {
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

.placeholders-info {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  padding: var(--spacing-md);
  background-color: var(--background-color);
}

.placeholders-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.placeholders-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-md);
}

.placeholder-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.group-title {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.placeholder-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: var(--font-size-xs);
  gap: var(--spacing-xs);
}

.placeholder-item code {
  background-color: var(--surface-color);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  font-family: var(--font-mono);
  color: var(--accent-color);
  font-weight: var(--font-weight-medium);
  flex-shrink: 0;
}

.placeholder-item span {
  color: var(--text-secondary);
  text-align: right;
  flex-grow: 1;
}

.presets-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
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
  align-self: flex-start;
}

.add-preset-btn:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}

.preset-form {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.form-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.form-group {
  margin-bottom: var(--spacing-md);
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

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-lg);
  padding-top: var(--spacing-md);
  border-top: var(--border-width) solid var(--border-color);
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

/* Responsive adjustments */
@media (max-width: 768px) {
  .placeholders-grid {
    grid-template-columns: 1fr;
  }
  
  .format-input-group {
    flex-direction: column;
  }
}
</style>