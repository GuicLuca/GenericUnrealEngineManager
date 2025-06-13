<template>
  <div class="development-panel">
    <div class="panel-header">
      <h4 class="panel-title">Development Tools</h4>
    </div>
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

const { showPopup } = usePopup()

const platformInfo = ref('Unknown')
const appVersion = ref('0.1.0')

const openProjectDiscoveryPopup = () => {
  showPopup({
    id: 'project-discovery',
    component: 'ProjectDiscovery',
    props: {}
  })
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

.panel-header {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
}

.panel-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
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

.dev-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.dev-button:active {
  background-color: var(--active-color);
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