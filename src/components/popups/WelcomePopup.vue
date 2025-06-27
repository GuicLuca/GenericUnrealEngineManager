<template>
  <div class="welcome-popup">
    <div class="popup-header">
      <div class="header-content">
        <h2 class="popup-title">
          <span class="title-icon">👋</span>
          Welcome to UE Project Manager
        </h2>
        <div class="welcome-subtitle">Get started with managing your Unreal Engine projects</div>
      </div>
    </div>

    <div class="popup-content">
      <div class="welcome-message">
        <div class="feature-list">
          <div class="feature-item">
            <span class="feature-icon">🔍</span>
            <span class="feature-text">Discover and organize your UE projects</span>
          </div>
          <div class="feature-item">
            <span class="feature-icon">🧹</span>
            <span class="feature-text">Clean temporary files to save disk space</span>
          </div>
          <div class="feature-item">
            <span class="feature-icon">🚀</span>
            <span class="feature-text">Launch projects with your preferred IDE</span>
          </div>
          <div class="feature-item">
            <span class="feature-icon">🔌</span>
            <span class="feature-text">Manage plugins and project settings</span>
          </div>
        </div>

        <div class="autostart-section">
          <h3 class="section-title">Quick Setup</h3>
          <div class="autostart-question">
            <div class="question-text">
              <span class="question-icon">🚀</span>
              Would you like UE Project Manager to start automatically when you log in?
            </div>
            <div class="question-description">
              This will allow you to quickly access your projects without manually starting the application.
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <div class="left-actions">
        <div class="checkbox-item">
          <input
            id="dont-show-again"
            v-model="dontShowAgain"
            type="checkbox"
            class="checkbox-input"
          />
          <label for="dont-show-again" class="checkbox-label">
            Don't show this welcome message again
          </label>
        </div>
      </div>
      
      <div class="right-actions">
        <button 
          class="action-button secondary-button" 
          @click="handleDecline"
          :disabled="isProcessing"
        >
          <span class="button-icon">❌</span>
          No, Thanks
        </button>
        <button 
          class="action-button primary-button" 
          @click="handleAccept"
          :disabled="isProcessing"
        >
          <span class="button-icon">{{ isProcessing ? '⏳' : '✅' }}</span>
          {{ isProcessing ? 'Setting up...' : 'Yes, Enable Autostart' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogStore } from '../../stores/logStore'

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const dontShowAgain = ref(false)
const isProcessing = ref(false)

const handleAccept = async () => {
  try {
    isProcessing.value = true
    
    // Enable autostart
    await invoke('enable_autostart')
    addLog('Autostart enabled successfully')
    
    // Update settings to reflect autostart is enabled
    await updateSettings(true)
    
    // Close the popup
    await handleClose()
    
  } catch (error) {
    console.error('Failed to enable autostart:', error)
    addLog('Failed to enable autostart', 'error')
  } finally {
    isProcessing.value = false
  }
}

const handleDecline = async () => {
  try {
    isProcessing.value = true
    
    // Update settings to reflect autostart is disabled
    await updateSettings(false)
    
    // Close the popup
    await handleClose()
    
  } catch (error) {
    console.error('Failed to update settings:', error)
    addLog('Failed to update settings', 'error')
  } finally {
    isProcessing.value = false
  }
}

const updateSettings = async (autostartEnabled: boolean) => {
  try {
    // Get current settings
    const settings = await invoke('get_settings') as any
    
    // Update autostart setting
    settings.general.autostart_enabled = autostartEnabled
    
    // If "don't show again" is checked, disable welcome popup
    if (dontShowAgain.value) {
      settings.general.show_welcome_popup = false
    }
    
    // Save updated settings
    await invoke('save_settings', { settings })
    
  } catch (error) {
    console.error('Failed to update settings:', error)
    throw error
  }
}

const handleClose = async () => {
  // If "don't show again" is checked but we haven't updated settings yet, do it now
  if (dontShowAgain.value) {
    try {
      const settings = await invoke('get_settings') as any
      settings.general.show_welcome_popup = false
      await invoke('save_settings', { settings })
    } catch (error) {
      console.error('Failed to disable welcome popup:', error)
    }
  }
  
  emit('close')
}
</script>

<style scoped>
.welcome-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 36rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.popup-header {
  padding: var(--spacing-lg);
  background: linear-gradient(135deg, var(--accent-color) 0%, #2c5aa0 100%);
  color: white;
}

.header-content {
  text-align: center;
}

.popup-title {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  margin: 0 0 var(--spacing-sm) 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
}

.title-icon {
  font-size: var(--icon-size-xl);
}

.welcome-subtitle {
  font-size: var(--font-size-sm);
  opacity: 0.9;
  margin: 0;
}

.popup-content {
  padding: var(--spacing-lg);
}

.welcome-message {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.feature-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.feature-icon {
  font-size: var(--font-size-lg);
  flex-shrink: 0;
}

.feature-text {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.autostart-section {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.autostart-question {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.question-text {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.question-icon {
  font-size: var(--font-size-md);
}

.question-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

.popup-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
}

.left-actions {
  display: flex;
  align-items: center;
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.checkbox-input {
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
}

.checkbox-label {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
}

.right-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.action-button {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  border: var(--border-width) solid;
}

.secondary-button {
  background-color: transparent;
  border-color: var(--border-color);
  color: var(--text-secondary);
}

.secondary-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.primary-button {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
  color: white;
}

.primary-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>