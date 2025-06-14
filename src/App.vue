<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ProjectBar from './components/ProjectBar.vue'
import Sidebar, { type SidebarItem } from './components/Sidebar.vue'
import MainArea from './components/MainArea.vue'
import InfoPanel from './components/InfoPanel.vue'
import BottomPanel, { type BottomTab } from './components/BottomPanel.vue'
import PopupManager from './components/PopupManager.vue'
import { useLogStore } from './stores/logStore'
import { useProjectStore } from './stores/projectStore'

// Initialize stores
const { addLog, initLogListener } = useLogStore()
const { addProject } = useProjectStore()

const sidebarItems = ref<SidebarItem[]>([
  { name: 'Refresh', icon: '🔄', action: 'refresh', requiresProject: true },
  { name: 'Open', icon: '↰', action: 'open', requiresProject: true },
  { name: 'Build', icon: '🔨', action: 'build', requiresProject: true },
  { name: 'Package', icon: '📦', action: 'package', requiresProject: true },
  { name: 'Clean', icon: '🧹', action: 'clean', requiresProject: true },
  { name: 'Compress', icon: '🗜️', action: 'compress', requiresProject: true },
  { name: 'Delete', icon: '🗑️', action: 'delete', requiresProject: true }
])

const bottomTabs = ref<BottomTab[]>([
  { id: 'logs', title: 'Logs', icon: '📄' },
  { id: 'development', title: 'Development', icon: '🛠️' }
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
const handleLogsResize = (height: number): void => {
  logsHeight.value = height
}

const handleInfoPanelResize = (width: number): void => {
  infoPanelWidth.value = width
}

const handleTabChange = (tabId: string): void => {
  console.log('Tab changed to:', tabId)
}

// TODO: fake projects for testing purpose, comment this before build
const initializeFakeProjects = () => {
  const fakeProjects = [
    {
      id: 'fake-1',
      name: 'ActionRPG',
      path: 'C:/UnrealProjects/ActionRPG/ActionRPG.uproject',
      engineVersion: '5.4.3',
      description: 'A sample action RPG game demonstrating combat mechanics, inventory systems, and character progression.',
      lastScan: '2025-01-04 14:30'
    },
    {
      id: 'fake-2',
      name: 'ArchVizInterior',
      path: 'D:/Projects/ArchViz/ArchVizInterior/ArchVizInterior.uproject',
      engineVersion: '5.3.2',
      description: 'Architectural visualization project showcasing realistic interior lighting and materials.',
      lastScan: '2025-01-03 09:15'
    },
    {
      id: 'fake-3',
      name: 'MultiplayerShooter',
      path: 'E:/GameDev/MultiplayerShooter/MultiplayerShooter.uproject',
      engineVersion: '5.4.1',
      description: 'Competitive multiplayer first-person shooter with dedicated server support and anti-cheat integration.',
      lastScan: '2025-01-04 16:45'
    },
    {
      id: 'fake-4',
      name: 'VRExperience',
      path: 'C:/VRProjects/VRExperience/VRExperience.uproject',
      engineVersion: '5.2.1',
      description: 'Immersive virtual reality experience featuring hand tracking and spatial audio.',
      lastScan: '2025-01-02 11:20'
    }
  ]

  fakeProjects.forEach(project => {
    addProject(project)
  })

  addLog('Initialized with 4 fake test projects')
}

// Initialize application
onMounted(async () => {
  await initLogListener()
  addLog('Application started successfully')
  
  // TODO: fake projects for testing purpose, comment this before build
  initializeFakeProjects()
})
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