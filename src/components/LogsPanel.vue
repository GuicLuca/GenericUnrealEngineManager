<template>
  <div class="logs-panel">
    <div class="logs-header">
      <h4 class="logs-title">Application Logs</h4>
      <button class="clear-logs-btn" @click="$emit('clear-logs')" title="Clear logs">
        🗑️
      </button>
    </div>
    <div class="logs-content">
      <div 
        v-for="log in logs" 
        :key="log.id"
        class="log-entry"
      >
        <span class="log-timestamp">{{ formatTimestamp(log.timestamp) }}</span>
        <span class="log-message">{{ log.message }}</span>
      </div>
      <div v-if="logs.length === 0" class="no-logs">
        No logs available
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LogEntry } from './BottomPanel.vue'

interface Props {
  logs: LogEntry[]
}

interface Emits {
  (e: 'clear-logs'): void
}

defineProps<Props>()
defineEmits<Emits>()

const formatTimestamp = (timestamp: string): string => {
  // Simple timestamp formatting - can be enhanced later
  return timestamp
}
</script>

<style scoped>
.logs-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.logs-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
}

.logs-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.clear-logs-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  transition: background-color var(--transition-fast);
}

.clear-logs-btn:hover {
  background-color: var(--hover-color);
}

.logs-content {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-sm);
  font-family: var(--font-mono);
  font-size: var(--font-size-xs);
  line-height: 1.4;
}

.log-entry {
  display: flex;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-xs);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  transition: background-color var(--transition-fast);
}

.log-entry:hover {
  background-color: var(--hover-color);
}

.log-timestamp {
  color: var(--text-secondary);
  white-space: nowrap;
  font-weight: var(--font-weight-medium);
  min-width: 8rem;
}

.log-message {
  color: var(--text-primary);
  word-wrap: break-word;
  flex-grow: 1;
}

.no-logs {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
  font-style: italic;
}
</style>