import { ref, reactive } from 'vue'
import { listen } from '@tauri-apps/api/event'

export interface PopupConfig {
  id: string
  component: string
  props?: Record<string, any>
  persistent?: boolean
}

interface PopupState {
  stack: PopupConfig[]
}

const popupState = reactive<PopupState>({
  stack: []
})

export const usePopup = () => {
  const showPopup = (config: PopupConfig) => {
    // Check if popup with same ID already exists in stack
    const existingIndex = popupState.stack.findIndex(p => p.id === config.id)
    if (existingIndex !== -1) {
      // Remove existing popup and add new one to top
      popupState.stack.splice(existingIndex, 1)
    }
    
    // Add new popup to top of stack
    popupState.stack.push(config)
  }

  const hidePopup = (popupId?: string) => {
    if (popupId) {
      // Remove specific popup by ID
      const index = popupState.stack.findIndex(p => p.id === popupId)
      if (index !== -1) {
        popupState.stack.splice(index, 1)
      }
    } else {
      // Remove top popup (most recent)
      popupState.stack.pop()
    }
  }

  const hideAllPopups = () => {
    popupState.stack.length = 0
  }

  const getCurrentPopup = () => {
    return popupState.stack.length > 0 ? popupState.stack[popupState.stack.length - 1] : null
  }

  const isPopupVisible = ref(() => popupState.stack.length > 0)
  const currentPopup = ref(() => getCurrentPopup())

  // Listen for backend events to show popups
  const initPopupListener = async () => {
    try {
      await listen('show-popup', (event: any) => {
        const config = event.payload as PopupConfig
        showPopup(config)
      })
    } catch (error) {
      console.error('Failed to initialize popup listener:', error)
    }
  }

  return {
    popupState,
    showPopup,
    hidePopup,
    hideAllPopups,
    getCurrentPopup,
    isPopupVisible,
    currentPopup,
    initPopupListener
  }
}