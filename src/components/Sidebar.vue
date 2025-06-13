<template>
  <div class="sidebar">
    <SidebarItem
      v-for="item in items"
      :key="item.name"
      :icon="item.icon"
      :label="item.name"
      @click="handleItemClick(item)"
    />
  </div>
</template>

<script setup lang="ts">
import SidebarItem from './SidebarItem.vue'

export interface SidebarItem {
  name: string
  icon: string
  action?: string
}

interface Props {
  items: SidebarItem[]
}

interface Emits {
  (e: 'item-click', item: SidebarItem): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleItemClick = (item: SidebarItem) => {
  emit('item-click', item)
}
</script>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  min-width: var(--sidebar-width);
  background-color: var(--surface-color);
  border-right: var(--border-width) solid var(--border-color);
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--scrollbar-thumb) var(--scrollbar-track);
}

.sidebar::-webkit-scrollbar {
  width: var(--scrollbar-width);
}

.sidebar::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
}

.sidebar::-webkit-scrollbar-thumb {
  background-color: var(--scrollbar-thumb);
  border-radius: var(--border-radius-sm);
}

.sidebar::-webkit-scrollbar-thumb:hover {
  background-color: var(--scrollbar-thumb-hover);
}
</style>