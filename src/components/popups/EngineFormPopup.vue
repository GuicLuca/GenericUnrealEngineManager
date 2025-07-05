<template>
  <div class="engine-form-popup">
    <div class="popup-header">
      <h2 class="popup-title">{{ editingEngine ? 'Edit' : 'Add' }} Custom Engine</h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="form-group">
        <label class="form-label">Engine Directory Path</label>
        <div class="path-input-group">
          <input
            v-model="engineForm.path"
            type="text"
            class="form-input"
            placeholder="Select engine root directory..."
            readonly
          />
          <button
            type="button"
            class="browse-button"
            @click="browseForEngineDirectory"
            title="Browse for engine directory"
            :disabled="isProcessing"
          >
            📂
          </button>
        </div>
        <div class="form-hint">
          Select the root directory of your Unreal Engine installation (contains Engine, Templates, etc.)
        </div>
      </div>

      <div v-if="detectedEngine" class="detected-engine">
        <h4 class="detected-title">Detected Engine Information</h4>
        <div class="engine-info">
          <div class="info-item">
            <span class="info-label">Name:</span>
            <span class="info-value">{{ detectedEngine.name }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Version:</span>
            <span class="info-value">{{ detectedEngine.version }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Type:</span>
            <span class="info-value">{{ detectedEngine.is_custom ? 'Custom' : 'Official' }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <button
        class="form-btn cancel-btn"
        @click="$emit('close')"
        :disabled="isProcessing"
      >
        Cancel
      </button>
      <button
        class="form-btn save-btn"
        @click="saveEngine"
        :disabled="!canSave || isProcessing"
      >
        <span class="button-icon">{{ isProcessing ? '⏳' : (editingEngine ? '💾' : '➕') }}</span>
        {{ isProcessing ? 'Processing...' : (editingEngine ? 'Update Engine' : 'Add Engine') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useLogStore } from '../../stores/logStore'

interface Props {
  editingEngine?: string | null
  initialPath?: string
  onSave?: (data: { name: string; path: string; isEdit: boolean; originalName?: string }) => void
}

interface Emits {
  (e: 'close'): void
}

interface DetectedEngine {
  name: string
  version: string
  path: string
  is_custom: boolean
}

const props = withDefaults(defineProps<Props>(), {
  editingEngine: null,
  initialPath: ''
})

const emit = defineEmits<Emits>()
const { addLog } = useLogStore()

const engineForm = reactive({
  path: props.initialPath || ''
})

const detectedEngine = ref<DetectedEngine | null>(null)
const isProcessing = ref(false)

const canSave = computed(() => {
  return engineForm.path.trim() !== '' && detectedEngine.value !== null && !isProcessing.value
})

const browseForEngineDirectory = async () => {
  if (isProcessing.value) return
  
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Engine Root Directory'
    })
    
    if (selected) {
      engineForm.path = selected
      await detectEngine()
    }
  } catch (error) {
    console.error('Failed to open directory dialog:', error)
    addLog('Failed to open directory dialog', 'error')
  }
}

const detectEngine = async () => {
  if (!engineForm.path.trim()) {
    detectedEngine.value = null
    return
  }
  
  try {
    isProcessing.value = true
    detectedEngine.value = null
    
    const result = await invoke('detect_engine_at_path', {
      enginePath: engineForm.path
    }) as DetectedEngine
    
    detectedEngine.value = result
    addLog(`Detected engine: ${result.name} (${result.version})`)
    
  } catch (error) {
    console.error('Failed to detect engine:', error)
    addLog('No valid Unreal Engine installation found at the specified path', 'warn')
    detectedEngine.value = null
  } finally {
    isProcessing.value = false
  }
}

const saveEngine = async () => {
  if (!canSave.value) return
  
  try {
    isProcessing.value = true
    
    const data = {
      name: detectedEngine.value!.name,
      path: engineForm.path,
      isEdit: !!props.editingEngine,
      originalName: props.editingEngine || undefined
    }
    
    // Call the onSave callback if provided
    if (props.onSave) {
      props.onSave(data)
    }
    
    addLog(`${props.editingEngine ? 'Updated' : 'Added'} engine: ${detectedEngine.value!.name}`)
    
    // Close the popup
    emit('close')
    
  } catch (error) {
    console.error('Failed to save engine:', error)
    addLog('Failed to save engine', 'error')
  } finally {
    isProcessing.value = false
  }
}

onMounted(async () => {
  if (engineForm.path) {
    await detectEngine()
  }
})
</script>

<style scoped>
.engine-form-popup {
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

.path-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.form-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--surface-color);
  cursor: pointer;
}

.form-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
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

.browse-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.browse-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.form-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  margin-top: var(--spacing-xs);
  line-height: var(--line-height-normal);
}

.detected-engine {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.detected-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.engine-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
}

.info-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.info-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
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

.cancel-btn:hover:not(:disabled) {
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

.form-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>