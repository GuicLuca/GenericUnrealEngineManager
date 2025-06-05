<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

interface SidebarItem {
  name: string
  icon: string
}

interface LogEntry {
  id: number
  timestamp: string
  message: string
}

interface BottomTab {
  id: string
  title: string
  icon: string
}

// Reactive data
const projectPath = ref<string>('D:\\Dev\\P4v\\TN_BoardGame\\BoardGame')
const lastScan = ref<string>('04/08/2025 00:40')
const engineVersion = ref<string>('5.4.3')
const description = ref<string>('hgkjh kjhws ekjhlk xksghx gs ixu hxhj iohuxosi osixgqwo pzswuiouyxgo ieuygdwsh qwiodugho jedk eiouocp riuhg exsgie uehjexh dh h jdoj weqo olekdmoed ieuh qakmjowekjd iuwhd hed wojs diwkj qsuiahsjd f feko fepo.')

const sidebarItems = ref<SidebarItem[]>([
  { name: 'Refresh', icon: '🔄' },
  { name: 'Open', icon: '↰' },
  { name: 'jjj', icon: '🔧' },
  { name: 'jjj', icon: '🔧' },
  { name: 'Clean', icon: '🔧' },
  { name: 'Compress', icon: '🔧' },
  { name: 'Delete', icon: '🔧' }
])

const logs = ref<LogEntry[]>([
  {
    id: 1,
    timestamp: '20250804 00:37',
    message: 'jlkrj elkdjwe oiuedoi oweiud eofo fioruye oiufrholf ui ewiuyoieupo wiu eiwuyd ieuy.'
  },
  {
    id: 2,
    timestamp: '20250804 00:42',
    message: 'h ojik h :hujyhgu guy.'
  },
  {
    id: 3,
    timestamp: '20250804 01:07',
    message: 'jlkrj elkdjhkj jkl;kkg jf ytruhjtg'
  }
])

// Bottom tabs management
const bottomTabs = ref<BottomTab[]>([
  { id: 'logs', title: 'Logs', icon: '📄' }
])

const activeBottomTab = ref<string>('logs')

// Resizable logs section
const logsHeight = ref<number>(120)
const isResizing = ref<boolean>(false)
const minLogsHeight = 80
const maxLogsHeight = 400

// Resizable info panel
const infoPanelWidth = ref<number>(300)
const isResizingInfoPanel = ref<boolean>(false)
const minInfoPanelWidth = 200
const maxInfoPanelWidth = 600

const setActiveBottomTab = (tabId: string): void => {
  activeBottomTab.value = tabId
}

const startResize = (event: MouseEvent): void => {
  isResizing.value = true
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  event.preventDefault()
}

const handleResize = (event: MouseEvent): void => {
  if (!isResizing.value) return
  
  const containerRect = document.querySelector('.app-container')?.getBoundingClientRect()
  if (!containerRect) return
  
  const newHeight = containerRect.bottom - event.clientY
  const clampedHeight = Math.min(Math.max(newHeight, minLogsHeight), maxLogsHeight)
  logsHeight.value = clampedHeight
}

const stopResize = (): void => {
  isResizing.value = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
}

const startResizeInfoPanel = (event: MouseEvent): void => {
  isResizingInfoPanel.value = true
  document.addEventListener('mousemove', handleResizeInfoPanel)
  document.addEventListener('mouseup', stopResizeInfoPanel)
  event.preventDefault()
}

const handleResizeInfoPanel = (event: MouseEvent): void => {
  if (!isResizingInfoPanel.value) return
  
  const containerRect = document.querySelector('.app-container')?.getBoundingClientRect()
  if (!containerRect) return
  
  const newWidth = containerRect.right - event.clientX
  const clampedWidth = Math.min(Math.max(newWidth, minInfoPanelWidth), maxInfoPanelWidth)
  infoPanelWidth.value = clampedWidth
}

const stopResizeInfoPanel = (): void => {
  isResizingInfoPanel.value = false
  document.removeEventListener('mousemove', handleResizeInfoPanel)
  document.removeEventListener('mouseup', stopResizeInfoPanel)
}

const openFileExplorer = async (): Promise<void> => {
  try {
    
    // For Tauri apps - using the invoke API with custom command
    await invoke('open_file_explorer', { path: projectPath.value })
    
  } catch (error) {
    console.error('Failed to open file explorer:', error)
    alert('Failed to open file explorer. Please check the console for details.\n-> "ctrl + shift + i" to open console.')
  }
}
</script>

<template>
  <div class="app-container">
    <!-- Project Bar -->
    <div class="project-bar">
      <div class="project-section">
        <label>Project:</label>
        <select class="project-dropdown">
          <option>Select Project</option>
        </select>
      </div>
      <div class="project-path">
        <span class="path-text">{{ projectPath }}</span>
        <button class="open-folder-btn" @click="openFileExplorer" title="Open in file explorer">
          📁
        </button>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="main-content">
      <!-- Sidebar -->
      <div class="sidebar">
        <div class="sidebar-item" v-for="item in sidebarItems" :key="item.name">
          <div class="sidebar-icon">{{ item.icon }}</div>
          <div class="sidebar-label">{{ item.name }}</div>
        </div>
      </div>

      <!-- Main Area -->
      <div class="main-area">
        <div class="main-area-content">
          <div class="main-area-label">Main Area window</div>
        </div>
      </div>

      <!-- Info Panel -->
      <div class="info-panel" :style="{ width: infoPanelWidth + 'px' }">
        <!-- Resize handle for info panel -->
        <div class="info-panel-resize-handle" @mousedown="startResizeInfoPanel"></div>
        
        <div class="info-item">
          <strong>Last scan:</strong> {{ lastScan }}
        </div>
        <div class="info-item">
          <strong>Engine version:</strong> {{ engineVersion }}
        </div>
        <div class="info-item">
          <strong>Description:</strong> {{ description }}
        </div>
      </div>
    </div>

    <!-- Bottom Tabs Section -->
    <div class="bottom-tabs-section" :style="{ height: logsHeight + 'px' }">
      <!-- Resize handle -->
      <div class="resize-handle" @mousedown="startResize"></div>
      
      <div class="tabs-header">
        <div class="tabs-nav">
          <button 
            v-for="tab in bottomTabs" 
            :key="tab.id"
            class="tab-button"
            :class="{ active: activeBottomTab === tab.id }"
            @click="setActiveBottomTab(tab.id)"
          >
            <span class="tab-icon">{{ tab.icon }}</span>
            <span class="tab-title">{{ tab.title }}</span>
          </button>
        </div>
      </div>
      <div class="tabs-content">
        <!-- Logs Tab Content -->
        <div v-if="activeBottomTab === 'logs'" class="tab-panel">
          <div class="logs-content">
            <div class="log-entry" v-for="log in logs" :key="log.id">
              <span class="log-timestamp">{{ log.timestamp }}</span>
              <span class="log-message">{{ log.message }}</span>
            </div>
          </div>
        </div>
        <!-- Add more tab panels here as needed -->
      </div>
    </div>
  </div>
</template>

<style>
/* Global styles to remove body margins and prevent scrolling */
html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden;
}

#app {
  height: 100vh;
  width: 100vw;
}
</style>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  background-color: #f5f5f5;
  display: flex;
  flex-direction: column;
  font-family: Arial, sans-serif;
  font-size: 14px;
  overflow: hidden;
}

/* Project Bar */
.project-bar {
  background-color: #f8f8f8;
  padding: 16px;
  display: flex;
  align-items: center;
  border-bottom: 1px solid #ccc;
  gap: 20px;
}

.project-section {
  display: flex;
  align-items: center;
  gap: 8px;
}

.project-dropdown {
  padding: 4px 8px;
  border: 1px solid #ccc;
  background-color: white;
}

.project-path {
  background-color: white;
  border: 1px solid #ccc;
  padding: 4px 8px;
  flex-grow: 1;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.path-text {
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.open-folder-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 16px;
  padding: 2px 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
  flex-shrink: 0;
}

.open-folder-btn:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.open-folder-btn:active {
  background-color: rgba(0, 0, 0, 0.2);
}

/* Main Content */
.main-content {
  display: flex;
  flex-grow: 1;
  overflow: hidden;
}

/* Sidebar */
.sidebar {
  width: 70px;
  min-width: 70px;
  background-color: #f0f0f0;
  border-right: 1px solid #ccc;
  padding: 8px;
  padding-left: 10px; /* Extra space for left scrollbar */
  display: flex;
  flex-direction: column;
  gap: 4px;
  overflow-y: overlay; /* Overlay scrollbar that doesn't take layout space */
  scrollbar-width: thin;
  scrollbar-color: #ccc #f0f0f0;
  direction: rtl; /* Right-to-left to move scrollbar to left */
}

/* Reset direction for content */
.sidebar > * {
  direction: ltr;
}

/* Webkit browsers (Chrome, Safari, Edge) */
.sidebar::-webkit-scrollbar {
  width: 8px;
}

.sidebar::-webkit-scrollbar-track {
  background: #f0f0f0;
}

.sidebar::-webkit-scrollbar-thumb {
  background-color: #ccc;
  border-radius: 4px;
}

.sidebar::-webkit-scrollbar-thumb:hover {
  background-color: #999;
}

.sidebar-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
}

.sidebar-item:hover {
  background-color: #e0e0e0;
}

.sidebar-icon {
  font-size: 20px;
  color: #666;
}

.sidebar-label {
  font-size: 11px;
  text-align: center;
  color: #333;
}

/* Main Area */
.main-area {
  flex-grow: 1;
  background-color: white;
  position: relative;
}

.main-area-content {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-image: 
    linear-gradient(45deg, transparent 45%, #000 47%, #000 53%, transparent 55%),
    linear-gradient(-45deg, transparent 45%, #000 47%, #000 53%, transparent 55%);
  background-size: 60px 60px;
  background-position: 0 0, 30px 30px;
}

.main-area-label {
  background-color: rgba(255, 255, 255, 0.9);
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: bold;
}

/* Info Panel */
.info-panel {
  background-color: #f8f8f8;
  border-left: 1px solid #ccc;
  padding: 16px;
  overflow-y: auto;
  position: relative;
  min-width: 200px;
  max-width: 600px;
}

.info-panel-resize-handle {
  position: absolute;
  top: 0;
  left: -3px;
  bottom: 0;
  width: 6px;
  background-color: transparent;
  cursor: ew-resize;
  z-index: 10;
}

.info-panel-resize-handle:hover {
  background-color: rgba(0, 122, 204, 0.3);
}

.info-panel-resize-handle:active {
  background-color: rgba(0, 122, 204, 0.5);
}

.info-item {
  margin-bottom: 12px;
  font-size: 12px;
  line-height: 1.4;
}

/* Bottom Tabs Section */
.bottom-tabs-section {
  background-color: white;
  border-top: 1px solid #ccc;
  display: flex;
  flex-direction: column;
  position: relative;
  min-height: 80px;
  max-height: 400px;
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
}

.resize-handle:hover {
  background-color: rgba(0, 122, 204, 0.3);
}

.resize-handle:active {
  background-color: rgba(0, 122, 204, 0.5);
}

.tabs-header {
  background-color: #f0f0f0;
  border-bottom: 1px solid #ccc;
  padding: 0;
}

.tabs-nav {
  display: flex;
  gap: 0;
}

.tab-button {
  padding: 8px 16px;
  border: none;
  background-color: #f0f0f0;
  border-right: 1px solid #ccc;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #666;
  transition: background-color 0.2s;
}

.tab-button:hover {
  background-color: #e0e0e0;
}

.tab-button.active {
  background-color: white;
  color: #333;
  border-bottom: 2px solid #007acc;
}

.tab-icon {
  font-size: 16px;
}

.tabs-content {
  flex-grow: 1;
  overflow: hidden;
}

.tab-panel {
  width: 100%;
  height: 100%;
  padding: 8px;
  overflow-y: auto;
}

.logs-content {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  height: 100%;
  overflow-y: auto;
}

.log-entry {
  margin-bottom: 2px;
  display: flex;
  gap: 8px;
}

.log-timestamp {
  color: #666;
  white-space: nowrap;
}

.log-message {
  color: #333;
}
</style>