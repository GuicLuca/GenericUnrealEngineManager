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
    <Transition name="tooltip">
      <div v-if="showTooltip" class="tooltip">
        {{ content }}
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  content: string
}

defineProps<Props>()

const showTooltip = ref(false)
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
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  background-color: var(--text-primary);
  color: var(--background-color);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  white-space: nowrap;
  max-width: 20rem;
  white-space: normal;
  z-index: 1001;
  margin-bottom: var(--spacing-xs);
  box-shadow: var(--shadow-md);
  line-height: var(--line-height-normal);
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
  transform: translateX(-50%) translateY(4px);
}
</style>