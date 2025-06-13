<template>
  <div class="info-item">
    <div class="info-header">
      <span class="info-icon" v-if="icon">{{ icon }}</span>
      <span class="info-label">{{ label }}</span>
    </div>
    <div class="info-value" :class="{ multiline }">
      {{ value }}
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  label: string
  value: string
  icon?: string
  multiline?: boolean
}

defineProps<Props>()
</script>

<style scoped>
.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.info-icon {
  font-size: var(--font-size-sm);
}

.info-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-secondary);
}

.info-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.4;
  padding: var(--spacing-xs);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.info-value.multiline {
  white-space: pre-wrap;
  word-wrap: break-word;
  max-height: 6rem;
  overflow-y: auto;
}
</style>