<template>
  <div class="development-panel">
    <div class="panel-content">
      <div class="tool-section">
        <h5 class="section-title">Popup System Testing</h5>
        <div class="button-group">
          <button 
            class="dev-button"
            @click="openProjectDiscoveryPopup"
          >
            <span class="button-icon">🔍</span>
            Test Project Discovery Popup
          </button>
        </div>
      </div>
      
      <div class="tool-section">
        <h5 class="section-title">Project Testing</h5>
        <div class="button-group">
          <button 
            class="dev-button"
            @click="addMockProjects"
            :disabled="isAddingProjects"
          >
            <span class="button-icon">{{ isAddingProjects ? '⏳' : '📁' }}</span>
            {{ isAddingProjects ? 'Adding Projects...' : 'Add 5 Mock Projects' }}
          </button>
        </div>
      </div>
      
      <div class="tool-section">
        <h5 class="section-title">System Information</h5>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Platform:</span>
            <span class="info-value">{{ platformInfo }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">App Version:</span>
            <span class="info-value">{{ appVersion }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { usePopup } from '../composables/usePopup'
import { useProjectStore, type Project, type EngineAssociation } from '../stores/projectStore'
import { useLogStore } from '../stores/logStore'

const { showPopup } = usePopup()
const { projects } = useProjectStore()
const { addLog } = useLogStore()

const platformInfo = ref('Unknown')
const appVersion = ref('0.1.0')
const isAddingProjects = ref(false)

const openProjectDiscoveryPopup = () => {
  showPopup({
    id: 'project-discovery',
    component: 'ProjectDiscovery',
    props: {}
  })
}

const addMockProjects = async () => {
  try {
    isAddingProjects.value = true
    addLog('Adding 5 mock projects for testing...')
    
    const mockProjects: Project[] = [
      {
        name: 'ActionRPG_Blueprint',
        description: 'A blueprint-only action RPG game with magic and combat systems',
        engine_association: { Standard: '5.3' },
        path: 'C:/UnrealProjects/ActionRPG_Blueprint/ActionRPG_Blueprint.uproject',
        has_cpp: false,
        plugins: [
          {
            name: 'EnhancedInput',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: null,
            target_allow_list: ['Editor', 'Game']
          }
        ],
        size_on_disk: 2147483648, // 2GB
        last_scan_date: Math.floor(Date.now() / 1000) - 300 // 5 minutes ago
      },
      {
        name: 'ShooterGame_CPP',
        description: 'A C++ first-person shooter with advanced AI and networking',
        engine_association: { Standard: '5.4' },
        path: 'C:/UnrealProjects/ShooterGame_CPP/ShooterGame_CPP.uproject',
        has_cpp: true,
        plugins: [
          {
            name: 'OnlineSubsystem',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: null,
            target_allow_list: ['Editor', 'Game', 'Server']
          },
          {
            name: 'Niagara',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: null,
            target_allow_list: ['Editor', 'Game']
          }
        ],
        size_on_disk: 5368709120, // 5GB
        last_scan_date: Math.floor(Date.now() / 1000) - 1800 // 30 minutes ago
      },
      {
        name: 'CustomEngine_Project',
        description: 'A project using a custom Unreal Engine build with specialized features',
        engine_association: 'Custom',
        path: 'D:/CustomEngine/MyCustomProject/MyCustomProject.uproject',
        has_cpp: true,
        plugins: [
          {
            name: 'CustomRenderer',
            is_enabled: true,
            is_in_project: true,
            marketplace_url: null,
            target_allow_list: ['Editor', 'Game']
          }
        ],
        size_on_disk: 8589934592, // 8GB
        last_scan_date: Math.floor(Date.now() / 1000) - 7200 // 2 hours ago
      },
      {
        name: 'MobileGame_UE4',
        description: 'A mobile-optimized puzzle game built for iOS and Android',
        engine_association: { Standard: '4.27' },
        path: 'C:/UnrealProjects/MobileGame_UE4/MobileGame_UE4.uproject',
        has_cpp: false,
        plugins: [
          {
            name: 'GooglePlayGames',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: 'https://www.unrealengine.com/marketplace/en-US/product/google-play-games',
            target_allow_list: ['Game']
          },
          {
            name: 'AppleGameCenter',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: null,
            target_allow_list: ['Game']
          }
        ],
        size_on_disk: 1073741824, // 1GB
        last_scan_date: Math.floor(Date.now() / 1000) - 86400 // 1 day ago
      },
      {
        name: 'VR_Experience',
        description: 'An immersive VR experience with hand tracking and spatial audio',
        engine_association: { Standard: '5.2' },
        path: 'C:/UnrealProjects/VR_Experience/VR_Experience.uproject',
        has_cpp: true,
        plugins: [
          {
            name: 'OculusVR',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: null,
            target_allow_list: ['Editor', 'Game']
          },
          {
            name: 'SteamVR',
            is_enabled: true,
            is_in_project: false,
            marketplace_url: null,
            target_allow_list: ['Editor', 'Game']
          },
          {
            name: 'HandTracking',
            is_enabled: true,
            is_in_project: true,
            marketplace_url: 'https://www.unrealengine.com/marketplace/en-US/product/hand-tracking',
            target_allow_list: ['Game']
          }
        ],
        size_on_disk: 3221225472, // 3GB
        last_scan_date: Math.floor(Date.now() / 1000) - 30 // 30 seconds ago
      }
    ]
    
    // Add mock projects to the existing projects array
    // Filter out any existing mock projects first to avoid duplicates
    const existingPaths = projects.value.map(p => p.path)
    const newProjects = mockProjects.filter(p => !existingPaths.includes(p.path))
    
    if (newProjects.length > 0) {
      // Simulate the backend adding projects by directly updating the store
      // In a real scenario, this would go through the backend
      projects.value.push(...newProjects)
      addLog(`Added ${newProjects.length} mock projects successfully`)
    } else {
      addLog('Mock projects already exist, skipping addition')
    }
    
  } catch (error) {
    console.error('Failed to add mock projects:', error)
    addLog('Failed to add mock projects', 'error')
  } finally {
    isAddingProjects.value = false
  }
}

onMounted(async () => {
  try {
    // Get platform info if available
    platformInfo.value = navigator.platform || 'Unknown'
  } catch (error) {
    console.error('Failed to get platform info:', error)
  }
})
</script>

<style scoped>
.development-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.panel-content {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.tool-section {
  margin-bottom: var(--spacing-lg);
}

.tool-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
  padding-bottom: var(--spacing-xs);
  border-bottom: var(--border-width) solid var(--border-color);
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.dev-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
  text-align: left;
}

.dev-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.dev-button:active:not(:disabled) {
  background-color: var(--active-color);
}

.dev-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-md);
}

.info-grid {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
}

.info-label {
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.info-value {
  color: var(--text-primary);
  font-family: var(--font-mono);
}
</style>