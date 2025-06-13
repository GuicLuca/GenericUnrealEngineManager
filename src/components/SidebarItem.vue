<template>
  <div class="sidebar-item" @click="$emit('click')">
    <div class="sidebar-icon">{{ icon }}</div>
    <div class="sidebar-label">{{ label }}</div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon: string
  label: string
}

interface Emits {
  (e: 'click'): void
}

defineProps<Props>()
defineEmits<Emits>()
</script>

<style scoped>
.sidebar-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-xs);
  cursor: pointer;
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  transition: background-color var(--transition-fast);
  user-select: none;
}

.sidebar-item:hover {
  background-color: var(--hover-color);
}

.sidebar-item:active {
  background-color: var(--active-color);
}

.sidebar-icon {
  font-size: var(--icon-size-md);
  color: var(--text-secondary);
  transition: color var(--transition-fast);
}

.sidebar-item:hover .sidebar-icon {
  color: var(--text-primary);
}

.sidebar-label {
  font-size: var(--font-size-xs);
  text-align: center;
  color: var(--text-primary);
  font-weight: var(--font-weight-medium);
  line-height: 1.2;
}
</style>