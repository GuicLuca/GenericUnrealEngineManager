<template>
  <div class="main-area">
    <div class="main-area-content">
      <PluginsView />
    </div>
  </div>
</template>

<script setup lang="ts">
import PluginsView from './PluginsView.vue'
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
  flex-direction: column;
}
</style>