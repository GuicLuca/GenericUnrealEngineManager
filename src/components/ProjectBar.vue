<template>
  <div class="project-bar">
    <div class="project-section">
      <label class="project-label">Project:</label>
      <select class="project-dropdown">
        <option>Select Project</option>
      </select>
    </div>
    <div class="project-path">
      <span class="path-text">{{ projectPath }}</span>
      <button 
        class="open-folder-btn" 
        @click="handleOpenExplorer" 
        title="Open in file explorer"
        :disabled="!projectPath"
      >
        📁
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  projectPath: string
}

interface Emits {
  (e: 'open-explorer', path: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleOpenExplorer = () => {
  if (props.projectPath) {
    emit('open-explorer', props.projectPath)
  }
}
</script>

<style scoped>
.project-bar {
  background-color: var(--surface-color);
  padding: var(--spacing-md);
  display: flex;
  align-items: center;
  border-bottom: var(--border-width) solid var(--border-color);
  gap: var(--spacing-lg);
  min-height: var(--project-bar-height);
}

.project-section {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.project-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
}

.project-dropdown {
  padding: var(--spacing-xs) var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  min-width: 12rem;
}

.project-dropdown:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.project-path {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  padding: var(--spacing-xs) var(--spacing-sm);
  flex-grow: 1;
  border-radius: var(--border-radius-md);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-sm);
  min-height: 2rem;
}

.path-text {
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.open-folder-btn {
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

.open-folder-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
}

.open-folder-btn:active:not(:disabled) {
  background-color: var(--active-color);
}

.open-folder-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>