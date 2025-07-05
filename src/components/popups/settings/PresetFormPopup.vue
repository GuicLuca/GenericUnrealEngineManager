<template>
  <div class="preset-form-popup">
    <div class="popup-header">
      <h2 class="popup-title">{{ editingPreset ? 'Edit' : 'Add' }} Custom Preset</h2>
      <button class="close-button" @click="$emit('close')" title="Close">
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
          ref="nameInput"
        />
      </div>

      <div class="form-group">
        <label class="form-label">Format Template</label>
        <input
          v-model="presetForm.format"
          type="text"
          class="form-input"
          placeholder="e.g., [Project]_[Type]_[YYYY][MM][DD]"
          ref="formatInput"
        />
        <div class="format-preview">
          <span class="preview-label">Preview:</span>
          <span class="preview-filename">{{ getPreviewForFormat(presetForm.format) }}</span>
        </div>
      </div>

      <div class="tags-section">
        <h3 class="tags-title">Available Tags</h3>
        <div class="tags-description">Click on any tag to add it to your format template</div>
        
        <div class="tags-grid">
          <div class="tag-group">
            <h4 class="tag-group-title">Project Info</h4>
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
            <h4 class="tag-group-title">Date & Time</h4>
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
            <h4 class="tag-group-title">System Info</h4>
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
        @click="$emit('close')"
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
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Props {
  editingPreset?: string | null
  initialName?: string
  initialFormat?: string
  onSave?: (data: { name: string; format: string; isEdit: boolean; originalName?: string }) => void
}

interface Emits {
  (e: 'close'): void
}

interface Tag {
  code: string
  label: string
  description: string
}

const props = withDefaults(defineProps<Props>(), {
  editingPreset: null,
  initialName: '',
  initialFormat: ''
})

const emit = defineEmits<Emits>()

const nameInput = ref<HTMLInputElement>()
const formatInput = ref<HTMLInputElement>()

const presetForm = reactive({
  name: props.initialName || '',
  format: props.initialFormat || ''
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

const addTagToFormat = (tagCode: string) => {
  // Add the tag at the cursor position or at the end
  const input = formatInput.value
  if (input && input === document.activeElement) {
    const start = input.selectionStart || 0
    const end = input.selectionEnd || 0
    const before = presetForm.format.substring(0, start)
    const after = presetForm.format.substring(end)
    presetForm.format = before + tagCode + after
    
    // Set the cursor position after the inserted tag
    nextTick(() => {
      input.setSelectionRange(start + tagCode.length, start + tagCode.length)
      input.focus()
    })
  } else {
    // Just append to the end if no cursor position
    presetForm.format += tagCode
  }
}

const savePreset = () => {
  if (!presetForm.name.trim() || !presetForm.format.trim()) return

  const data = {
    name: presetForm.name,
    format: presetForm.format,
    isEdit: !!props.editingPreset,
    originalName: props.editingPreset || undefined
  }

  // Call the onSave callback if provided
  if (props.onSave) {
    props.onSave(data)
  }

  // Close the popup
  emit('close')
}

const loadSystemInfo = async () => {
  try {
    systemInfo.value.username = await invoke('get_system_username') as string
    systemInfo.value.hostname = await invoke('get_system_hostname') as string
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

onMounted(async () => {
  await loadSystemInfo()
  
  // Focus the appropriate input
  await nextTick()
  if (!props.editingPreset && nameInput.value) {
    nameInput.value.focus()
  } else if (formatInput.value) {
    formatInput.value.focus()
  }
})
</script>

<style scoped>
.preset-form-popup {
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

.form-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
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

.format-preview {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  margin-top: var(--spacing-sm);
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
  letter-spacing: 1px;
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

/* Responsive adjustments */
@media (max-width: 768px) {
  .tags-grid {
    grid-template-columns: 1fr;
  }
  
  .preset-form-popup {
    max-width: 90vw;
    margin: var(--spacing-md);
  }
}
</style>