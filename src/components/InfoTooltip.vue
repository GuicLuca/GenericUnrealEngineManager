<template>
  <div class="info-tooltip-container">
    <div 
      class="info-icon"
      @mouseenter="showTooltip = true"
      @mouseleave="showTooltip = false"
      @focus="showTooltip = true"
      @blur="showTooltip = false"
      tabindex="0"
    >
      ℹ️
    </div>
    <Teleport to="body">
      <Transition name="tooltip">
        <div v-if="showTooltip" class="tooltip" :style="tooltipStyle" ref="tooltipRef">
          {{ content }}
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, watch } from 'vue'

interface Props {
  content: string
}

const props = defineProps<Props>()

const showTooltip = ref(false)
const tooltipRef = ref<HTMLElement | null>(null)

// Calculate dynamic width based on content length
const tooltipStyle = computed(() => {
  const contentLength = props.content.length
  let width = '20rem' // Default width
  
  if (contentLength > 100) {
    width = '30rem'
  } else if (contentLength > 200) {
    width = '40rem'
  } else if (contentLength > 300) {
    width = '50rem'
  }
  
  return {
    width,
    maxWidth: '90vw' // Ensure it doesn't exceed viewport
  }
})

// Position tooltip when it becomes visible
watch(showTooltip, async (visible) => {
  if (visible) {
    await nextTick()
    positionTooltip()
  }
})

const positionTooltip = () => {
  if (!tooltipRef.value) return
  
  const tooltip = tooltipRef.value
  const container = tooltip.closest('.info-tooltip-container') as HTMLElement
  if (!container) return
  
  const containerRect = container.getBoundingClientRect()
  const tooltipRect = tooltip.getBoundingClientRect()
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight
  
  let left = containerRect.left + containerRect.width / 2
  let top = containerRect.top - tooltipRect.height - 8
  
  // Adjust horizontal position if tooltip would overflow
  if (left + tooltipRect.width / 2 > viewportWidth - 10) {
    left = viewportWidth - tooltipRect.width - 10
  } else if (left - tooltipRect.width / 2 < 10) {
    left = 10
  } else {
    left = left - tooltipRect.width / 2
  }
  
  // Adjust vertical position if tooltip would overflow top
  if (top < 10) {
    top = containerRect.bottom + 8
  }
  
  tooltip.style.left = `${left}px`
  tooltip.style.top = `${top}px`
  tooltip.style.transform = 'none'
}
</script>

<style scoped>
.info-tooltip-container {
  position: relative;
  display: inline-block;
}

.info-icon {
  font-size: var(--font-size-xs);
  cursor: help;
  opacity: 0.7;
  transition: opacity var(--transition-fast);
  border-radius: 50%;
  width: 1rem;
  height: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.info-icon:hover,
.info-icon:focus {
  opacity: 1;
  outline: none;
}

.tooltip {
  position: fixed;
  background-color: var(--text-primary);
  color: var(--background-color);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  white-space: normal;
  z-index: 10000;
  box-shadow: var(--shadow-md);
  line-height: var(--line-height-normal);
  word-wrap: break-word;
  pointer-events: none;
}

.tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 4px solid transparent;
  border-top-color: var(--text-primary);
}

.tooltip-enter-active,
.tooltip-leave-active {
  transition: all var(--transition-fast);
}

.tooltip-enter-from,
.tooltip-leave-to {
  opacity: 0;
  transform: translateY(4px);
}
</style>