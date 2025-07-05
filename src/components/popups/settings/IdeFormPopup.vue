<template>
  <div class="ide-form-popup">
    <div class="popup-header">
      <h2 class="popup-title">{{ editingProgram ? 'Edit' : 'Add' }} IDE Program</h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="form-group">
        <label class="form-label">Program Name</label>
        <input
          v-model="programForm.name"
          type="text"
          class="form-input"
          placeholder="e.g., Visual Studio 2022"
          :disabled="!!editingProgram"
          ref="nameInput"
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
        <div class="form-hint">
          Select the main executable file of your IDE (e.g., devenv.exe for Visual Studio)
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
        @click="saveProgram"
        :disabled="!programForm.name.trim() || !programForm.path.trim()"
      >
        <span class="button-icon">{{ editingProgram ? '💾' : '➕' }}</span>
        {{ editingProgram ? 'Update Program' : 'Add Program' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted, nextTick, ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

interface Props {
  editingProgram?: string | null
  initialName?: string
  initialPath?: string
  onSave?: (data: { name: string; path: string; isEdit: boolean; originalName?: string }) => void
}

interface Emits {
  (e: 'close'): void
}

const props = withDefaults(defineProps<Props>(), {
  editingProgram: null,
  initialName: '',
  initialPath: ''
})

const emit = defineEmits<Emits>()

const nameInput = ref<HTMLInputElement>()

const programForm = reactive({
  name: props.initialName || '',
  path: props.initialPath || ''
})

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

  const data = {
    name: programForm.name,
    path: programForm.path,
    isEdit: !!props.editingProgram,
    originalName: props.editingProgram || undefined
  }

  // Call the onSave callback if provided
  if (props.onSave) {
    props.onSave(data)
  }

  // Close the popup
  emit('close')
}

onMounted(async () => {
  // Focus the appropriate input
  await nextTick()
  if (!props.editingProgram && nameInput.value) {
    nameInput.value.focus()
  }
})
</script>

<style scoped>
.ide-form-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 32rem;
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
}

.form-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.form-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: var(--surface-color);
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

.popup-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
}

.form-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  border: var(--border-width) solid;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
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

.button-icon {
  font-size: var(--font-size-sm);
}
</style>