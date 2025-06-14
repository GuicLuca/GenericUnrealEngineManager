<template>
  <div 
    class="sidebar-item" 
    :class="{ disabled }"
    @click="handleClick"
  >
    <div class="sidebar-icon">{{ icon }}</div>
    <div class="sidebar-label">{{ label }}</div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon: string
  label: string
  disabled?: boolean
}

interface Emits {
  (e: 'click'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleClick = () => {
  if (!props.disabled) {
    emit('click')
  }
}
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

.sidebar-item:not(.disabled):hover {
  background-color: var(--hover-color);
}

.sidebar-item:not(.disabled):active {
  background-color: var(--active-color);
}

.sidebar-item.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sidebar-icon {
  font-size: var(--icon-size-md);
  color: var(--text-secondary);
  transition: color var(--transition-fast);
}

.sidebar-item:not(.disabled):hover .sidebar-icon {
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