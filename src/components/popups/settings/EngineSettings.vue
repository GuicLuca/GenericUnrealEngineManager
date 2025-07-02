<template>
  <div class="engine-settings">
    <div class="settings-section">
      <h4 class="section-title">Custom Engine Installations</h4>
      <div class="section-description">
        Manage custom Unreal Engine installations. These can be source builds or custom engine versions.
      </div>
      
      <div class="engines-list">
        <div 
          v-for="(path, name) in localSettings.engine_programs.custom_engines"
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
              @click="editEngine(name, path)"
              title="Edit engine"
            >
              ✏️
            </button>
            <button
              class="action-btn remove-btn"
              @click="removeEngine(name)"
              title="Remove engine"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="Object.keys(localSettings.engine_programs.custom_engines).length === 0" class="no-engines">
          <div class="no-engines-icon">⚙️</div>
          <div class="no-engines-text">No custom engines configured</div>
          <div class="no-engines-subtext">Add custom engine installations or source builds</div>
        </div>
      </div>

      <div class="engine-actions-row">
        <button class="add-engine-btn" @click="showAddEngine = true">
          <span class="button-icon">➕</span>
          Add Custom Engine
        </button>
        
        <button class="auto-detect-btn" @click="autoDetectEngines">
          <span class="button-icon">🔍</span>
          Auto-Detect Engines
        </button>
      </div>
    </div>

    <!-- Add/Edit Engine Form -->
    <div v-if="showAddEngine || editingEngine" class="engine-form">
      <h4 class="form-title">{{ editingEngine ? 'Edit' : 'Add' }} Custom Engine</h4>
      
      <div class="form-group">
        <label class="form-label">Engine Name</label>
        <input
          v-model="engineForm.name"
          type="text"
          class="form-input"
          placeholder="e.g., UE5.3-Custom, MyCustomEngine"
          :disabled="!!editingEngine"
        />
      </div>

      <div class="form-group">
        <label class="form-label">Engine Directory Path</label>
        <div class="path-input-group">
          <input
            v-model="engineForm.path"
            type="text"
            class="form-input"
            placeholder="Path to engine root directory..."
          />
          <button
            type="button"
            class="browse-button"
            @click="browseForEngineDirectory"
            title="Browse for engine directory"
          >
            📂
          </button>
        </div>
        <div class="form-hint">
          Select the root directory of your Unreal Engine installation (contains Engine, Templates, etc.)
        </div>
      </div>

      <div class="form-actions">
        <button
          class="form-btn cancel-btn"
          @click="cancelForm"
        >
          Cancel
        </button>
        <button
          class="form-btn save-btn"
          @click="saveEngine"
          :disabled="!engineForm.name.trim() || !engineForm.path.trim()"
        >
          {{ editingEngine ? 'Update' : 'Add' }} Engine
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { usePopup } from '../../../composables/usePopup'

interface Props {
  settings: {
    engine_programs: {
      custom_engines: Record<string, string>
    }
  }
}

interface Emits {
  (e: 'update', settings: any): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { showPopup } = usePopup()

const localSettings = reactive({
  engine_programs: {
    custom_engines: { ...props.settings.engine_programs.custom_engines }
  }
})

const showAddEngine = ref(false)
const editingEngine = ref<string | null>(null)
const engineForm = reactive({
  name: '',
  path: ''
})

const emitUpdate = () => {
  emit('update', { engine_programs: localSettings.engine_programs })
}

const editEngine = (name: string, path: string) => {
  editingEngine.value = name
  engineForm.name = name
  engineForm.path = path
  showAddEngine.value = false
}

const removeEngine = (name: string) => {
  if (confirm(`Are you sure you want to remove "${name}"?`)) {
    delete localSettings.engine_programs.custom_engines[name]
    emitUpdate()
  }
}

const browseForEngineDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Engine Root Directory'
    })
    
    if (selected && typeof selected === 'string') {
      engineForm.path = selected
    }
  } catch (error) {
    console.error('Failed to open directory dialog:', error)
  }
}

const saveEngine = () => {
  if (!engineForm.name.trim() || !engineForm.path.trim()) return

  if (editingEngine.value) {
    // Remove old entry if name changed
    if (editingEngine.value !== engineForm.name) {
      delete localSettings.engine_programs.custom_engines[editingEngine.value]
    }
  }

  localSettings.engine_programs.custom_engines[engineForm.name] = engineForm.path
  emitUpdate()
  cancelForm()
}

const cancelForm = () => {
  showAddEngine.value = false
  editingEngine.value = null
  engineForm.name = ''
  engineForm.path = ''
}

const autoDetectEngines = () => {
  showPopup({
    id: 'engine-detection',
    component: 'EngineDetection',
    props: {}
  })
}

// Watch for external changes to props
watch(() => props.settings.engine_programs, (newEnginePrograms) => {
  localSettings.engine_programs.custom_engines = { ...newEnginePrograms.custom_engines }
}, { deep: true })
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

.engine-form {
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

.path-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.path-input-group .form-input {
  flex: 1;
}

.browse-button {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-md);
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

.form-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  margin-top: var(--spacing-xs);
  line-height: var(--line-height-normal);
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
</style>