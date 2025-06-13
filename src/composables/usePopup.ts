import { ref, reactive } from 'vue'
import { listen } from '@tauri-apps/api/event'

export interface PopupConfig {
  id: string
  component: string
  props?: Record<string, any>
  persistent?: boolean
}

interface PopupState {
  isVisible: boolean
  config: PopupConfig | null
}

const popupState = reactive<PopupState>({
  isVisible: false,
  config: null
})

export const usePopup = () => {
  const showPopup = (config: PopupConfig) => {
    popupState.config = config
    popupState.isVisible = true
  }

  const hidePopup = () => {
    popupState.isVisible = false
    popupState.config = null
  }

  const isPopupVisible = ref(() => popupState.isVisible)
  const currentPopup = ref(() => popupState.config)

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
    isPopupVisible,
    currentPopup,
    initPopupListener
  }
}