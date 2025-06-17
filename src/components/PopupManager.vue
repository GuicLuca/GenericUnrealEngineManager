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
import { usePopup } from '../composables/usePopup'
import { useLogStore } from '../stores/logStore'
import ProjectDiscoveryPopup from './popups/ProjectDiscoveryPopup.vue'
import ProjectManagerPopup from './popups/ProjectManagerPopup.vue'

const { popupState, hidePopup, initPopupListener } = usePopup()
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

onMounted(() => {
  initPopupListener()
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
  overflow: auto;
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