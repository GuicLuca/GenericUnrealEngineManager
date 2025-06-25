<script setup lang="ts">
import { ref } from 'vue'
import ProjectBar from './components/ProjectBar.vue'
import Sidebar, { type SidebarItem } from './components/Sidebar.vue'
import MainArea from './components/MainArea.vue'
import InfoPanel from './components/InfoPanel.vue'
import BottomPanel, { type BottomTab } from './components/BottomPanel.vue'
import PopupManager from './components/PopupManager.vue'

const sidebarItems = ref<SidebarItem[]>([
  { name: 'Rescan', icon: '🔄', action: 'rescan', requiresProject: true },
  { name: 'Open', icon: '↰', action: 'open', requiresProject: true },
  { name: 'Build', icon: '🔨', action: 'build', requiresProject: true },
  { name: 'Package', icon: '📦', action: 'package', requiresProject: true },
  { name: 'Clean', icon: '🧹', action: 'clean', requiresProject: true },
  { name: 'Compress', icon: '🗜️', action: 'compress', requiresProject: true },
  { name: 'Untrack', icon: '🖇️', action: 'untrack', requiresProject: true }
])

const bottomTabs = ref<BottomTab[]>([
  { id: 'logs', title: 'Logs', icon: '📄' },
  { id: 'development', title: 'Development', icon: '🛠️' }
])

// Panel dimensions
const logsHeight = ref<number>(170)
const infoPanelWidth = ref<number>(250)

// Constants for panel constraints
const minLogsHeight = 66
const maxLogsHeight = 600
const minInfoPanelWidth = 220
const maxInfoPanelWidth = 600

// Event handlers
const handleLogsResize = (height: number): void => {
  logsHeight.value = height
}

const handleInfoPanelResize = (width: number): void => {
  infoPanelWidth.value = width
}

const handleTabChange = (tabId: string): void => {
  console.log('Tab changed to:', tabId)
}
</script>

<template>
  <div class="app-container">
    <ProjectBar />

    <div class="main-content">
      <Sidebar :items="sidebarItems" />

      <MainArea />

      <InfoPanel
        :width="infoPanelWidth"
        :min-width="minInfoPanelWidth"
        :max-width="maxInfoPanelWidth"
        @resize="handleInfoPanelResize"
      />
    </div>

    <BottomPanel
      :tabs="bottomTabs"
      :height="logsHeight"
      :min-height="minLogsHeight"
      :max-height="maxLogsHeight"
      @resize="handleLogsResize"
      @tab-change="handleTabChange"
    />

    <!-- Popup Manager -->
    <PopupManager />
  </div>
</template>

<style>
/* Import CSS variables */
@import './styles/variables.css';

/* Global styles */
html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden;
  font-family: var(--font-family);
  font-size: var(--font-size-sm);
  line-height: var(--line-height-normal);
  color: var(--text-primary);
  background-color: var(--background-color);
}

#app {
  height: 100vh;
  width: 100vw;
}

/* Scrollbar styling for webkit browsers */
* {
  scrollbar-width: thin;
  scrollbar-color: var(--scrollbar-thumb) var(--scrollbar-track);
}

*::-webkit-scrollbar {
  width: var(--scrollbar-width);
  height: var(--scrollbar-width);
}

*::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
}

*::-webkit-scrollbar-thumb {
  background-color: var(--scrollbar-thumb);
  border-radius: var(--border-radius-sm);
}

*::-webkit-scrollbar-thumb:hover {
  background-color: var(--scrollbar-thumb-hover);
}

/* Focus styles */
*:focus {
  outline: 2px solid var(--accent-color);
  outline-offset: 2px;
}

*:focus:not(:focus-visible) {
  outline: none;
}

/* Selection styles */
::selection {
  background-color: var(--accent-color-alpha);
  color: var(--text-primary);
}
</style>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-content {
  display: flex;
  flex-grow: 1;
  overflow: hidden;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .main-content {
    flex-direction: column;
  }
}
</style>