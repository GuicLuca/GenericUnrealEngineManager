<template>
  <div class="main-area">
    <div class="main-area-content">
      <div class="placeholder-content">
        <div class="placeholder-icon">🎯</div>
        <div class="placeholder-text">Main workspace area</div>
        <div class="placeholder-subtext">Project content will appear here</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// This component will be expanded with actual project content later
</script>

<style scoped>
.main-area {
  flex-grow: 1;
  background-color: var(--background-color);
  position: relative;
  overflow: hidden;
}

.main-area-content {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-image: 
    linear-gradient(45deg, transparent 48%, var(--grid-line-color) 49%, var(--grid-line-color) 51%, transparent 52%),
    linear-gradient(-45deg, transparent 48%, var(--grid-line-color) 49%, var(--grid-line-color) 51%, transparent 52%);
  background-size: var(--grid-size) var(--grid-size);
  background-position: 0 0, calc(var(--grid-size) / 2) calc(var(--grid-size) / 2);
}

.placeholder-content {
  background-color: var(--surface-color);
  padding: var(--spacing-xl);
  border-radius: var(--border-radius-lg);
  text-align: center;
  box-shadow: var(--shadow-sm);
  border: var(--border-width) solid var(--border-color);
}

.placeholder-icon {
  font-size: var(--icon-size-xl);
  margin-bottom: var(--spacing-md);
}

.placeholder-text {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.placeholder-subtext {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}
</style>