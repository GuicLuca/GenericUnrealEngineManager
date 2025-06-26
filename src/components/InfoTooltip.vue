<template>
  <div class="info-tooltip-container">
    <div 
      class="info-icon"
      @mouseenter="showTooltip = true"
      @mouseleave="showTooltip = false"
      @focus="showTooltip = true"
      @blur="showTooltip = false"
      tabindex="0"
      ref="iconRef"
    >
      ℹ️
    </div>
    <Teleport to="body">
      <Transition name="tooltip">
        <div 
          v-if="showTooltip" 
          class="tooltip" 
          :style="tooltipStyle"
          ref="tooltipRef"
        >
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
const iconRef = ref<HTMLElement>()
const tooltipRef = ref<HTMLElement>()
const tooltipPosition = ref({ left: 0, top: 0 })

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
    maxWidth: '90vw',
    position: 'fixed' as const,
    zIndex: 10001,
    left: `${tooltipPosition.value.left}px`,
    top: `${tooltipPosition.value.top}px`,
    transform: 'translateY(-100%)'
  }
})

// Position tooltip relative to icon when shown
watch(showTooltip, async (isVisible) => {
  if (isVisible && iconRef.value) {
    await nextTick()
    
    // Wait a bit more for the tooltip to be fully rendered
    setTimeout(() => {
      if (!iconRef.value || !tooltipRef.value) return
      
      const iconRect = iconRef.value.getBoundingClientRect()
      const tooltipElement = tooltipRef.value
      const tooltipRect = tooltipElement.getBoundingClientRect()
      
      // Calculate initial position (centered above icon)
      let left = iconRect.left + (iconRect.width / 2) - (tooltipRect.width / 2)
      const top = iconRect.top - 8 // 8px gap above icon
      
      // Ensure tooltip doesn't go beyond left edge
      if (left < 8) {
        left = 8
      }
      
      // Ensure tooltip doesn't go beyond right edge
      const viewportWidth = window.innerWidth
      if (left + tooltipRect.width > viewportWidth - 8) {
        left = viewportWidth - tooltipRect.width - 8
      }
      
      // Update position
      tooltipPosition.value = { left, top }
    }, 10)
  }
})
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
  background-color: var(--text-primary);
  color: var(--background-color);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  white-space: normal;
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

/* Only fade in/out animation */
.tooltip-enter-active,
.tooltip-leave-active {
  transition: opacity var(--transition-fast);
}

.tooltip-enter-from,
.tooltip-leave-to {
  opacity: 0;
}
</style>