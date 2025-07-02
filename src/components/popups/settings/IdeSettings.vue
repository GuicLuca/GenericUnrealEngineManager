<template>
  <div class="ide-settings">
    <div class="settings-section">
      <h4 class="section-title">Custom IDE Programs</h4>
      <div class="section-description">
        Add custom IDE programs that can be used to open C++ projects. These will appear in the project launch options.
      </div>
      
      <div class="programs-list">
        <div 
          v-for="(path, name) in localPrograms"
          :key="name"
          class="program-item"
        >
          <div class="program-info">
            <div class="program-name">{{ name }}</div>
            <div class="program-path">{{ path }}</div>
          </div>
          <div class="program-actions">
            <button
              class="action-btn edit-btn"
              @click="editProgram(name as string, path)"
              title="Edit program"
            >
              ✏️
            </button>
            <button
              class="action-btn remove-btn"
              @click="handleRemoveProgram(name as string)"
              title="Remove program"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="Object.keys(localPrograms).length === 0" class="no-programs">
          <div class="no-programs-icon">💻</div>
          <div class="no-programs-text">No custom IDE programs configured</div>
          <div class="no-programs-subtext">Add IDE programs to launch C++ projects</div>
        </div>
      </div>

      <button class="add-program-btn" @click="showAddProgram = true">
        <span class="button-icon">➕</span>
        Add IDE Program
      </button>
    </div>

    <!-- Add/Edit Program Form -->
    <div v-if="showAddProgram || editingProgram" class="program-form">
      <h4 class="form-title">{{ editingProgram ? 'Edit' : 'Add' }} IDE Program</h4>
      
      <div class="form-group">
        <label class="form-label">Program Name</label>
        <input
          v-model="programForm.name"
          type="text"
          class="form-input"
          placeholder="e.g., Visual Studio 2022"
          :disabled="!!editingProgram"
        />
      </div>

      <div class="form-group">
        <label class="form-label">Executable Path</label>
        <div class="path-input-group">
          <input
            v-model="programForm.path"
            type="text"
            class="form-input"
            placeholder="Path to IDE executable..."
          />
          <button
            type="button"
            class="browse-button"
            @click="browseForExecutable"
            title="Browse for executable"
          >
            📂
          </button>
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
          @click="saveProgram"
          :disabled="!programForm.name.trim() || !programForm.path.trim()"
        >
          {{ editingProgram ? 'Update' : 'Add' }} Program
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useSettingsStore } from '../../../stores/settingsStore'

const { 
  getSettings, 
  addIdeProgram, 
  removeIdeProgram, 
  updateIdeProgram 
} = useSettingsStore()

const localPrograms = reactive({ ...getSettings('ide_programs').custom_programs })

const showAddProgram = ref(false)
const editingProgram = ref<string | null>(null)
const programForm = reactive({
  name: '',
  path: ''
})

const editProgram = (name: string, path: string) => {
  editingProgram.value = name
  programForm.name = name
  programForm.path = path
  showAddProgram.value = false
}

const handleRemoveProgram = (name: string) => {
  if (confirm(`Are you sure you want to remove "${name}"?`)) {
    removeIdeProgram(name)
    delete localPrograms[name]
  }
}

const browseForExecutable = async () => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: 'Select IDE Executable',
      filters: [
        {
          name: 'Executable Files',
          extensions: ['exe', 'app', 'AppImage']
        }
      ]
    })
    
    if (selected && typeof selected === 'string') {
      programForm.path = selected
    }
  } catch (error) {
    console.error('Failed to open file dialog:', error)
  }
}

const saveProgram = () => {
  if (!programForm.name.trim() || !programForm.path.trim()) return

  if (editingProgram.value) {
    updateIdeProgram(editingProgram.value, programForm.name, programForm.path)
    if (editingProgram.value !== programForm.name) {
      delete localPrograms[editingProgram.value]
    }
  } else {
    addIdeProgram(programForm.name, programForm.path)
  }

  localPrograms[programForm.name] = programForm.path
  cancelForm()
}

const cancelForm = () => {
  showAddProgram.value = false
  editingProgram.value = null
  programForm.name = ''
  programForm.path = ''
}

// Watch for external changes to settings
watch(() => getSettings('ide_programs').custom_programs, (newPrograms) => {
  Object.assign(localPrograms, newPrograms)
}, { deep: true })
</script>

<style scoped>
.ide-settings {
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

.programs-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.program-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.program-info {
  flex-grow: 1;
  min-width: 0;
}

.program-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.program-path {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  word-break: break-all;
}

.program-actions {
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

.no-programs {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-xl);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.no-programs-icon {
  font-size: var(--icon-size-lg);
  margin-bottom: var(--spacing-sm);
  opacity: 0.5;
}

.no-programs-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-programs-subtext {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  opacity: 0.7;
}

.add-program-btn {
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

.add-program-btn:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}

.program-form {
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