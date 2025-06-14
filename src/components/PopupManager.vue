<template>
  <Teleport to="body">
    <Transition name="popup-overlay">
      <div 
        v-if="popupState.isVisible" 
        class="popup-overlay"
        @click="handleOverlayClick"
      >
        <Transition name="popup-content">
          <div 
            v-if="popupState.isVisible"
            class="popup-container"
            @click.stop
          >
            <!-- Project Discovery Popup -->
            <ProjectDiscoveryPopup
              v-if="popupState.config?.component === 'ProjectDiscovery'"
              v-bind="popupState.config.props"
              @close="hidePopup"
              @submit="handleProjectDiscoverySubmit"
            />
            
            <!-- Project Manager Popup -->
            <ProjectManagerPopup
              v-if="popupState.config?.component === 'ProjectManager'"
              v-bind="popupState.config.props"
              @close="hidePopup"
            />
            
            <!-- Add more popup components here as needed -->
          </div>
        </Transition>
      </div>
    </Transition>
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

const handleOverlayClick = () => {
  if (!popupState.config?.persistent) {
    hidePopup()
  }
}

const handleProjectDiscoverySubmit = (data: any) => {
  addLog('Starting project discovery...')
  console.log('Project discovery submitted:', data)
  // Here you would typically call a Tauri command to start the discovery
  hidePopup()
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
  z-index: 1000;
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