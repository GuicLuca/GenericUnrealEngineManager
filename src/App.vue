<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core"
import ProjectBar from './components/ProjectBar.vue'
import Sidebar, { type SidebarItem } from './components/Sidebar.vue'
import MainArea from './components/MainArea.vue'
import InfoPanel from './components/InfoPanel.vue'
import BottomPanel, { type BottomTab, type LogEntry } from './components/BottomPanel.vue'

// Reactive data
const projectPath = ref<string>('D:\\Dev\\P4v\\TN_BoardGame\\BoardGame')
const lastScan = ref<string>('04/08/2025 00:40')
const engineVersion = ref<string>('5.4.3')
const description = ref<string>('hgkjh kjhws ekjhlk xksghx gs ixu hxhj iohuxosi osixgqwo pzswuiouyxgo ieuygdwsh qwiodugho jedk eiouocp riuhg exsgie uehjexh dh h jdoj weqo olekdmoed ieuh qakmjowekjd iuwhd hed wojs diwkj qsuiahsjd f feko fepo.')

const sidebarItems = ref<SidebarItem[]>([
  { name: 'Refresh', icon: '🔄', action: 'refresh' },
  { name: 'Open', icon: '↰', action: 'open' },
  { name: 'Build', icon: '🔨', action: 'build' },
  { name: 'Package', icon: '📦', action: 'package' },
  { name: 'Clean', icon: '🧹', action: 'clean' },
  { name: 'Compress', icon: '🗜️', action: 'compress' },
  { name: 'Delete', icon: '🗑️', action: 'delete' }
])

const logs = ref<LogEntry[]>([
  {
    id: 1,
    timestamp: '20250804 00:37',
    message: 'Application started successfully. Scanning for Unreal Engine projects...'
  },
  {
    id: 2,
    timestamp: '20250804 00:42',
    message: 'Found 3 projects in the specified directory.'
  },
  {
    id: 3,
    timestamp: '20250804 01:07',
    message: 'Project analysis completed. Ready for operations.'
  }
])

const bottomTabs = ref<BottomTab[]>([
  { id: 'logs', title: 'Logs', icon: '📄' }
])

// Panel dimensions
const logsHeight = ref<number>(120)
const infoPanelWidth = ref<number>(300)

// Constants for panel constraints
const minLogsHeight = 80
const maxLogsHeight = 400
const minInfoPanelWidth = 200
const maxInfoPanelWidth = 600

// Event handlers
const handleOpenExplorer = async (path: string): Promise<void> => {
  try {
    await invoke('open_file_explorer', { path })
  } catch (error) {
    console.error('Failed to open file explorer:', error)
    addLog('Error: Failed to open file explorer. Check console for details.')
  }
}

const handleSidebarItemClick = (item: SidebarItem): void => {
  addLog(`Action triggered: ${item.name}`)
  console.log('Sidebar item clicked:', item)
  
  // Handle specific actions
  switch (item.action) {
    case 'refresh':
      handleRefresh()
      break
    case 'open':
      handleOpenExplorer(projectPath.value)
      break
    // Add more action handlers as needed
    default:
      console.log(`Action ${item.action} not implemented yet`)
  }
}

const handleRefresh = (): void => {
  addLog('Refreshing project data...')
  // Implement refresh logic here
}

const handleLogsResize = (height: number): void => {
  logsHeight.value = height
}

const handleInfoPanelResize = (width: number): void => {
  infoPanelWidth.value = width
}

const handleTabChange = (tabId: string): void => {
  console.log('Tab changed to:', tabId)
}

const handleClearLogs = (): void => {
  logs.value = []
  addLog('Logs cleared')
}

// Utility functions
const addLog = (message: string): void => {
  const now = new Date()
  const timestamp = now.toISOString().slice(0, 19).replace('T', ' ')
  
  logs.value.push({
    id: Date.now(),
    timestamp,
    message
  })
  
  // Keep only last 100 logs to prevent memory issues
  if (logs.value.length > 100) {
    logs.value = logs.value.slice(-100)
  }
}
</script>

<template>
  <div class="app-container">
    <ProjectBar 
      :project-path="projectPath"
      @open-explorer="handleOpenExplorer"
    />

    <div class="main-content">
      <Sidebar 
        :items="sidebarItems"
        @item-click="handleSidebarItemClick"
      />

      <MainArea />

      <InfoPanel
        :last-scan="lastScan"
        :engine-version="engineVersion"
        :description="description"
        :width="infoPanelWidth"
        :min-width="minInfoPanelWidth"
        :max-width="maxInfoPanelWidth"
        @resize="handleInfoPanelResize"
      />
    </div>

    <BottomPanel
      :tabs="bottomTabs"
      :logs="logs"
      :height="logsHeight"
      :min-height="minLogsHeight"
      :max-height="maxLogsHeight"
      @resize="handleLogsResize"
      @tab-change="handleTabChange"
      @clear-logs="handleClearLogs"
    />
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