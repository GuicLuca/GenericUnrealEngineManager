<script setup lang="ts">
import {ref, reactive, onMounted, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {useSettingsStore} from '../../../stores/settingsStore'
import {usePopup} from "../../../composables/usePopup.ts";
import {emit} from "@tauri-apps/api/event";

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
        // hide self popup after showing welcome
        hidePopup('settings')
      }
      break
  }
  updateGeneralSettings(localGeneral)
}

const loadSystemInfo = async () => {
  try {
    systemInfo.value.platform = navigator.platform || 'Unknown'
    systemInfo.value.username = await invoke('get_system_username') as string
    systemInfo.value.hostname = await invoke('get_system_hostname') as string
  } catch (error) {
    console.error('Failed to load system info:', error)
  }
}

// Watch for external changes to settings
watch( () => getSettings('general'), (newGeneral) => {
  Object.assign(localGeneral, newGeneral)
}, {deep: true})

onMounted(() => {
  loadSystemInfo()
})
</script>

<template>
  <div class="general-settings">
    <div class="settings-section">
      <h4 class="section-title">Application Behavior</h4>

      <div class="setting-item">
        <div class="setting-header">
          <label class="setting-label">
            <input
                v-model="localGeneral.autostart_enabled"
                type="checkbox"
                class="setting-checkbox"
                @change="handleUpdate('autostart_enabled')"
            />
            <span class="setting-title">Start with the system</span>
          </label>
        </div>
        <div class="setting-description">
          Automatically start this tool when you start your computer.
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-header">
          <label class="setting-label">
            <input
                v-model="localGeneral.show_welcome_popup"
                type="checkbox"
                class="setting-checkbox"
                @change="handleUpdate('show_welcome_popup')"
            />
            <span class="setting-title">Show welcome popup</span>
          </label>
        </div>
        <div class="setting-description">
          Display the welcome popup when the application starts for the first time.
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h4 class="section-title">System Information</h4>

      <div class="info-grid">
        <div class="info-item">
          <span class="info-label">Platform:</span>
          <span class="info-value">{{ systemInfo.platform }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">App Version:</span>
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

.settings-section {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
  padding-bottom: var(--spacing-sm);
  border-bottom: var(--border-width) solid var(--border-color);
}

.setting-item {
  margin-bottom: var(--spacing-md);
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-header {
  margin-bottom: var(--spacing-xs);
}

.setting-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
}

.setting-checkbox {
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
}

.setting-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.setting-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
  margin-left: 1.5rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-sm);
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.info-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.info-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>