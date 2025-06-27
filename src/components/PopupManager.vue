<template>
  <Teleport to="body">
    <!-- Render all popups in the stack -->
    <div v-for="(popup, index) in popupState.stack" :key="popup.id">
      <Transition name="popup-overlay">
        <div 
          v-if="popup"
          class="popup-overlay"
          :style="{ zIndex: 1000 + index }"
          @click="handleOverlayClick(popup)"
        >
          <Transition name="popup-content">
            <div 
              v-if="popup"
              class="popup-container"
              @click.stop
            >
              <!-- Welcome Popup -->
              <WelcomePopup
                v-if="popup.component === 'Welcome'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
              />
              
              <!-- Project Discovery Popup -->
              <ProjectDiscoveryPopup
                v-if="popup.component === 'ProjectDiscovery'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
                @submit="handleProjectDiscoverySubmit"
              />
              
              <!-- Project Manager Popup -->
              <ProjectManagerPopup
                v-if="popup.component === 'ProjectManager'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
              />
              
              <!-- Project Launch Choice Popup -->
              <ProjectLaunchChoicePopup
                v-if="popup.component === 'ProjectLaunchChoice'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
              />
              
              <!-- Project Clean Popup -->
              <ProjectCleanPopup
                v-if="popup.component === 'ProjectClean'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
              />
              
              <!-- Project Compress Popup -->
              <ProjectCompressPopup
                v-if="popup.component === 'ProjectCompress'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
              />
              
              <!-- Settings Popup -->
              <SettingsPopup
                v-if="popup.component === 'Settings'"
                v-bind="popup.props"
                @close="hidePopup(popup.id)"
              />
              
              <!-- Add more popup components here as needed -->
            </div>
          </Transition>
        </div>
      </Transition>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { usePopup } from '../composables/usePopup'
import { useLogStore } from '../stores/logStore'
import WelcomePopup from './popups/WelcomePopup.vue'
import ProjectDiscoveryPopup from './popups/ProjectDiscoveryPopup.vue'
import ProjectManagerPopup from './popups/ProjectManagerPopup.vue'
import ProjectLaunchChoicePopup from './popups/ProjectLaunchChoicePopup.vue'
import ProjectCleanPopup from './popups/ProjectCleanPopup.vue'
import ProjectCompressPopup from './popups/ProjectCompressPopup.vue'
import SettingsPopup from './popups/SettingsPopup.vue'

const { popupState, hidePopup, initPopupListener, showPopup } = usePopup()
const { addLog } = useLogStore()

const handleOverlayClick = (popup: any) => {
  if (!popup.persistent) {
    hidePopup(popup.id)
  }
}

const handleProjectDiscoverySubmit = (data: any) => {
  addLog('Starting project discovery...')
  console.log('Project discovery submitted:', data)
  // Here you would typically call a Tauri command to start the discovery
  hidePopup() // Close the top popup
}

onMounted(async () => {
  initPopupListener()
  
  // Listen for welcome popup event from backend
  try {
    await listen('show_welcome_popup', () => {
      showPopup({
        id: 'welcome',
        component: 'Welcome',
        props: {}
      })
    })
  } catch (error) {
    console.error('Failed to listen for welcome popup event:', error)
  }
})
</script>

<style scoped>
.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(2px);
}

.popup-container {
  max-width: 90vw;
  max-height: 90vh;
  border-radius: var(--border-radius-lg);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

/* Transitions */
.popup-overlay-enter-active,
.popup-overlay-leave-active {
  transition: opacity var(--transition-normal);
}

.popup-overlay-enter-from,
.popup-overlay-leave-to {
  opacity: 0;
}

.popup-content-enter-active,
.popup-content-leave-active {
  transition: all var(--transition-normal);
}

.popup-content-enter-from,
.popup-content-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
</style>