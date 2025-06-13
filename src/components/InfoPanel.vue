<template>
  <div class="info-panel" :style="{ width: width + 'px' }">
    <div 
      class="resize-handle" 
      @mousedown="startResize"
    ></div>
    
    <div class="info-content">
      <h3 class="info-title">Project Information</h3>
      
      <div class="info-section">
        <InfoItem 
          label="Last scan" 
          :value="lastScan" 
          icon="🔍"
        />
        <InfoItem 
          label="Engine version" 
          :value="engineVersion" 
          icon="⚙️"
        />
        <InfoItem 
          label="Description" 
          :value="description" 
          icon="📝"
          multiline
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import InfoItem from './InfoItem.vue'

interface Props {
  lastScan: string
  engineVersion: string
  description: string
  width: number
  minWidth: number
  maxWidth: number
}

interface Emits {
  (e: 'resize', width: number): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const isResizing = ref(false)

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
  
  const newWidth = containerRect.right - event.clientX
  const clampedWidth = Math.min(Math.max(newWidth, props.minWidth), props.maxWidth)
  emit('resize', clampedWidth)
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
}
</script>

<style scoped>
.info-panel {
  background-color: var(--surface-color);
  border-left: var(--border-width) solid var(--border-color);
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.resize-handle {
  position: absolute;
  top: 0;
  left: -3px;
  bottom: 0;
  width: 6px;
  background-color: transparent;
  cursor: ew-resize;
  z-index: 10;
  transition: background-color var(--transition-fast);
}

.resize-handle:hover {
  background-color: var(--accent-color-alpha);
}

.resize-handle:active {
  background-color: var(--accent-color);
}

.info-content {
  padding: var(--spacing-md);
  overflow-y: auto;
  flex-grow: 1;
}

.info-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
  padding-bottom: var(--spacing-sm);
  border-bottom: var(--border-width) solid var(--border-color);
}

.info-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}
</style>