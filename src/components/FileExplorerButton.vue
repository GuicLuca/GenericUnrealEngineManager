<template>
  <button 
    class="file-explorer-btn"
    :class="{ 'btn-small': size === 'small' }"
    @click="handleOpenExplorer" 
    :title="title"
    :disabled="disabled"
  >
    📁
  </button>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { useLogStore } from '../stores/logStore'

interface Props {
  projectPath: string
  projectName?: string
  size?: 'normal' | 'small'
  title?: string
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 'normal',
  title: 'Open in file explorer',
  disabled: false
})

const { addLog } = useLogStore()

const handleOpenExplorer = async (): Promise<void> => {
  if (!props.projectPath || props.disabled) return
  
  try {
    // Extract directory path from the .uproject file path
    const projectDir = props.projectPath.replace(/[^/\\]*\.uproject$/, '')
    await invoke('open_file_explorer', { path: projectDir })
    
    const projectDisplayName = props.projectName || 'project'
    addLog(`Opened file explorer for: ${projectDisplayName}`)
  } catch (error) {
    console.error('Failed to open file explorer:', error)
    addLog('Error: Failed to open file explorer. Check console for details.', 'error')
  }
}
</script>

<style scoped>
.file-explorer-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: var(--font-size-md);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color var(--transition-fast);
  flex-shrink: 0;
  width: 2rem;
  height: 2rem;
}

.file-explorer-btn.btn-small {
  width: 1.75rem;
  height: 1.75rem;
  font-size: var(--font-size-sm);
  padding: var(--spacing-xs);
}

.file-explorer-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
}

.file-explorer-btn:active:not(:disabled) {
  background-color: var(--active-color);
}

.file-explorer-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>