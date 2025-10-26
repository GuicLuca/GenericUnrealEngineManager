<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useSettingsStore } from '../../../stores/settingsStore'

const { getSettings, updateSettings } = useSettingsStore()

const localCompression = reactive({ ...getSettings('compression') })
const newPresetName = ref('')
const newPresetFormat = ref('')
const isAddingPreset = ref(false)

const customPresetEntries = computed(() => {
  return Object.entries(localCompression.custom_presets || {})
})

const handleUpdate = () => {
  updateSettings('compression', localCompression)
}

const addPreset = () => {
  if (newPresetName.value.trim() && newPresetFormat.value.trim()) {
    localCompression.custom_presets = {
      ...localCompression.custom_presets,
      [newPresetName.value]: newPresetFormat.value
    }
    handleUpdate()
    newPresetName.value = ''
    newPresetFormat.value = ''
    isAddingPreset.value = false
  }
}

const removePreset = (presetName: string) => {
  if (confirm(`Are you sure you want to remove the preset "${presetName}"?`)) {
    const presets = { ...localCompression.custom_presets }
    delete presets[presetName]
    localCompression.custom_presets = presets
    handleUpdate()
  }
}

const availableTags = [
  { tag: '[Project]', description: 'Project name' },
  { tag: '[Type]', description: 'Project type (Cpp/Bp)' },
  { tag: '[Engine]', description: 'Engine version' },
  { tag: '[SizeMB]', description: 'Project size in MB' },
  { tag: '[SizeGB]', description: 'Project size in GB' },
  { tag: '[PluginCount]', description: 'Number of plugins' },
  { tag: '[Algorithm]', description: 'Compression algorithm' },
  { tag: '[YYYY]', description: 'Year (4 digits)' },
  { tag: '[YY]', description: 'Year (2 digits)' },
  { tag: '[MM]', description: 'Month (01-12)' },
  { tag: '[DD]', description: 'Day (01-31)' },
  { tag: '[HH]', description: 'Hour (00-23)' },
  { tag: '[mm]', description: 'Minute (00-59)' },
  { tag: '[ss]', description: 'Second (00-59)' },
  { tag: '[Month]', description: 'Month name' },
  { tag: '[Mon]', description: 'Month short name' },
  { tag: '[Day]', description: 'Day name' },
  { tag: '[Weekday]', description: 'Day short name' },
  { tag: '[User]', description: 'Username' },
  { tag: '[Computer]', description: 'Computer name' },
  { tag: '[Timestamp]', description: 'Unix timestamp' }
]
</script>

<template>
  <div class="compression-settings">
    <div class="section-header">
      <h3 class="section-title">Compression Settings</h3>
      <p class="section-description">Configure archive filename formats and custom presets</p>
    </div>

    <div class="settings-section">
      <h4 class="subsection-title">Default Filename Format</h4>
      <div class="input-group">
        <input
          v-model="localCompression.filename_format"
          @input="handleUpdate"
          type="text"
          class="text-input"
          placeholder="Enter filename format..."
        />
      </div>
      <div class="format-hint">
        This format will be used as the default for compression operations
      </div>
    </div>

    <div class="settings-section">
      <div class="subsection-header">
        <h4 class="subsection-title">Custom Presets</h4>
        <button
          class="add-preset-button"
          @click="isAddingPreset = !isAddingPreset"
        >
          <span class="button-icon">{{ isAddingPreset ? '✕' : '➕' }}</span>
          {{ isAddingPreset ? 'Cancel' : 'Add Preset' }}
        </button>
      </div>

      <div v-if="isAddingPreset" class="add-preset-form">
        <div class="form-group">
          <label class="form-label">Preset Name</label>
          <input
            v-model="newPresetName"
            type="text"
            class="text-input"
            placeholder="e.g., 'Detailed Format'"
          />
        </div>
        <div class="form-group">
          <label class="form-label">Filename Format</label>
          <input
            v-model="newPresetFormat"
            type="text"
            class="text-input"
            placeholder="e.g., '[Project]_[Engine]_[YYYY][MM][DD]'"
          />
        </div>
        <button class="save-preset-button" @click="addPreset">
          Save Preset
        </button>
      </div>

      <div v-if="customPresetEntries.length === 0 && !isAddingPreset" class="empty-state">
        <div class="empty-icon">📋</div>
        <div class="empty-title">No custom presets</div>
        <div class="empty-description">Create custom filename format presets for quick access</div>
      </div>

      <div v-else-if="!isAddingPreset" class="preset-list">
        <div
          v-for="[name, format] in customPresetEntries"
          :key="name"
          class="preset-item"
        >
          <div class="preset-info">
            <div class="preset-name">{{ name }}</div>
            <div class="preset-format">{{ format }}</div>
          </div>
          <button
            class="remove-button"
            @click="removePreset(name)"
            title="Remove preset"
          >
            ✕
          </button>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h4 class="subsection-title">Available Tags</h4>
      <div class="tags-grid">
        <div
          v-for="tag in availableTags"
          :key="tag.tag"
          class="tag-item"
        >
          <code class="tag-code">{{ tag.tag }}</code>
          <span class="tag-description">{{ tag.description }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.compression-settings {
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

.subsection-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.subsection-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.text-input {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  font-family: var(--font-mono);
}

.text-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.format-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

.add-preset-button {
  padding: var(--spacing-xs) var(--spacing-md);
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

.add-preset-button:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}

.add-preset-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.save-preset-button {
  padding: var(--spacing-sm) var(--spacing-md);
  border: var(--border-width) solid var(--accent-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--accent-color);
  color: white;
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-fast);
  align-self: flex-start;
}

.save-preset-button:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-lg);
  text-align: center;
}

.empty-icon {
  font-size: 3rem;
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

.preset-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.preset-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  gap: var(--spacing-md);
}

.preset-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  min-width: 0;
}

.preset-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.preset-format {
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

.tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: var(--spacing-sm);
}

.tag-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: var(--spacing-xs);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
}

.tag-code {
  font-size: var(--font-size-xs);
  color: var(--accent-color);
  font-family: var(--font-mono);
  font-weight: var(--font-weight-medium);
}

.tag-description {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}
</style>
