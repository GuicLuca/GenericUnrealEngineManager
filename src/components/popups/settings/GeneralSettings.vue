<script setup lang="ts">
import {ref, reactive, onMounted, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {useSettingsStore} from '../../../stores/settingsStore'
import {usePopup} from "../../../composables/usePopup.ts";

const {getSettings, updateGeneralSettings} = useSettingsStore()
const {showPopup, hidePopup} = usePopup()

const localGeneral = reactive({... getSettings('general')})

const systemInfo = ref({
  platform: 'Unknown',
  version: '0.1.0',
  username: 'Unknown',
  hostname: 'Unknown'
})

const handleUpdate = (source: string) => {
  switch (source) {
    case 'autostart_enabled':
      break
    case 'show_welcome_popup':
      if (localGeneral.show_welcome_popup) {
        showPopup({
          id: 'welcome',
          component: 'Welcome',
          props: {}
        })
      } else {
        hidePopup('welcome')
      }
      break
  }
  updateGeneralSettings(localGeneral)
}

const loadSystemInfo = async () => {
  try {
    const info = await invoke('get_system_info') as any
    systemInfo.value = info
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

watch(() => localGeneral.autostart_enabled, async (newValue) => {
  try {
    if (newValue) {
      await invoke('enable_autostart')
    } else {
      await invoke('disable_autostart')
    }
  } catch (error) {
    console.error('Failed to update autostart:', error)
    localGeneral.autostart_enabled = !newValue
  }
})

onMounted(() => {
  loadSystemInfo()
})
</script>

<template>
  <div class="general-settings">
    <div class="section-header">
      <h3 class="section-title">General Settings</h3>
      <p class="section-description">Configure general application behavior</p>
    </div>

    <div class="settings-section">
      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localGeneral.autostart_enabled"
            @change="handleUpdate('autostart_enabled')"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">Start automatically on system startup</span>
            <span class="setting-hint">Launch the application when you log in to your computer</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localGeneral.show_welcome_popup"
            @change="handleUpdate('show_welcome_popup')"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">Show welcome popup on startup</span>
            <span class="setting-hint">Display a welcome message when the application starts</span>
          </span>
        </label>
      </div>
    </div>

    <div class="settings-section">
      <h4 class="subsection-title">System Information</h4>
      <div class="info-grid">
        <div class="info-item">
          <span class="info-label">Platform:</span>
          <span class="info-value">{{ systemInfo.platform }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Version:</span>
          <span class="info-value">{{ systemInfo.version }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Username:</span>
          <span class="info-value">{{ systemInfo.username }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Hostname:</span>
          <span class="info-value">{{ systemInfo.hostname }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.general-settings {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.section-header {
  margin-bottom: var(--spacing-md);
}

.section-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
}

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
}

.settings-section {
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
}

.subsection-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.setting-group {
  padding: var(--spacing-sm) 0;
}

.setting-group:not(:last-child) {
  border-bottom: var(--border-width) solid var(--border-color);
}

.setting-label {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  cursor: pointer;
}

.checkbox-input {
  margin-top: 2px;
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
  flex-shrink: 0;
}

.setting-text {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.setting-name {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-weight: var(--font-weight-medium);
}

.setting-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-md);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-label {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.info-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
}
</style>
