<template>
  <div class="bottom-panel" :style="{ height: height + 'px' }">
    <div 
      class="resize-handle" 
      @mousedown="startResize"
    ></div>
    
    <div class="tabs-header">
      <div class="tabs-nav">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          class="tab-button"
          :class="{ active: activeTab === tab.id }"
          @click="setActiveTab(tab.id)"
        >
          <span class="tab-icon">{{ tab.icon }}</span>
          <span class="tab-title">{{ tab.title }}</span>
        </button>
      </div>
    </div>
    
    <div class="tabs-content">
      <div v-if="activeTab === 'logs'" class="tab-panel">
        <LogsPanel />
      </div>
      <div v-else-if="activeTab === 'development'" class="tab-panel">
        <DevelopmentPanel />
      </div>
      <!-- Add more tab panels here as needed -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import LogsPanel from './LogsPanel.vue'
import DevelopmentPanel from './DevelopmentPanel.vue'

export interface BottomTab {
  id: string
  title: string
  icon: string
}

interface Props {
  tabs: BottomTab[]
  height: number
  minHeight: number
  maxHeight: number
}

interface Emits {
  (e: 'resize', height: number): void
  (e: 'tab-change', tabId: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const activeTab = ref(props.tabs[0]?.id || 'logs')
const isResizing = ref(false)

const setActiveTab = (tabId: string) => {
  activeTab.value = tabId
  emit('tab-change', tabId)
}

const startResize = (event: MouseEvent) => {
  isResizing.value = true
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  event.preventDefault()
}

const handleResize = (event: MouseEvent) => {
  if (!isResizing.value) return
  
  const containerRect = document.querySelector('.app-container')?.getBoundingClientRect()
  if (!containerRect) return
  
  const newHeight = containerRect.bottom - event.clientY
  const clampedHeight = Math.min(Math.max(newHeight, props.minHeight), props.maxHeight)
  emit('resize', clampedHeight)
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
}
</script>

<style scoped>
.bottom-panel {
  background-color: var(--background-color);
  border-top: var(--border-width) solid var(--border-color);
  display: flex;
  flex-direction: column;
  position: relative;
}

.resize-handle {
  position: absolute;
  top: -3px;
  left: 0;
  right: 0;
  height: 6px;
  background-color: transparent;
  cursor: ns-resize;
  z-index: 10;
  transition: background-color var(--transition-fast);
}

.resize-handle:hover {
  background-color: var(--accent-color-alpha);
}

.resize-handle:active {
  background-color: var(--accent-color);
}

.tabs-header {
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
}

.tabs-nav {
  display: flex;
  gap: 0;
}

.tab-button {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  background-color: transparent;
  border-right: var(--border-width) solid var(--border-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  font-weight: var(--font-weight-medium);
}

.tab-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.tab-button.active {
  background-color: var(--background-color);
  color: var(--text-primary);
  border-bottom: 2px solid var(--accent-color);
  font-weight: var(--font-weight-semibold);
}

.tab-icon {
  font-size: var(--font-size-md);
}

.tabs-content {
  flex-grow: 1;
  overflow: hidden;
}

.tab-panel {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>