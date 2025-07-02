<template>
  <div class="engine-detection-popup">
    <div class="popup-header">
      <div class="header-content">
        <h2 class="popup-title">
          <span class="title-icon">🔍</span>
          Auto-Detect Unreal Engine Installations
        </h2>
        <div class="warning-subtitle">This process may take several minutes</div>
      </div>
      <button class="close-button" @click="$emit('close')" title="Close" :disabled="isDetecting">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div v-if="!isDetecting && !detectionComplete" class="detection-info">
        <div class="info-section">
          <div class="info-icon">⚠️</div>
          <div class="info-content">
            <h3 class="info-title">Important Information</h3>
            <div class="info-text">
              This feature will scan your entire computer to find installed Unreal Engine versions. 
              The process may take several minutes depending on your system and the number of drives.
            </div>
          </div>
        </div>

        <div class="detection-details">
          <h4 class="details-title">What will be detected:</h4>
          <ul class="detection-list">
            <li>Official Unreal Engine installations (Epic Games Launcher)</li>
            <li>Custom engine builds from source code</li>
            <li>Engine installations in any location on your system</li>
          </ul>
        </div>

        <div class="scan-scope">
          <h4 class="scope-title">Scan scope:</h4>
          <div class="scope-description">
            All available drives and common installation directories will be scanned for engine installations.
          </div>
        </div>
      </div>

      <div v-else-if="isDetecting" class="detection-progress">
        <div class="progress-info">
          <div class="progress-icon">🔄</div>
          <div class="progress-text">
            <div class="progress-title">Scanning for Unreal Engine installations...</div>
            <div class="progress-subtitle">This may take several minutes. Please wait.</div>
          </div>
        </div>
        
        <div class="progress-note">
          <div class="note-icon">💡</div>
          <div class="note-text">
            The scan is running in the background. You can monitor the progress in the task progress bar at the bottom of the application.
          </div>
        </div>
      </div>

      <div v-else-if="detectionComplete" class="detection-results">
        <div class="results-header">
          <div class="results-icon">{{ detectionResult.total_found > 0 ? '✅' : '❌' }}</div>
          <div class="results-text">
            <div class="results-title">
              {{ detectionResult.total_found > 0 ? 'Detection Complete!' : 'No Engines Found' }}
            </div>
            <div class="results-subtitle">
              {{ detectionResult.total_found > 0 
                ? `Found ${detectionResult.total_found} engine installation(s)` 
                : 'No Unreal Engine installations were detected on your system' }}
            </div>
          </div>
        </div>

        <div v-if="detectionResult.total_found > 0" class="detected-engines">
          <h4 class="engines-title">Detected Engines:</h4>
          <div class="engines-list">
            <div 
              v-for="engine in detectionResult.engines" 
              :key="engine.path"
              class="engine-item"
            >
              <div class="engine-info">
                <div class="engine-name">{{ engine.name }}</div>
                <div class="engine-path">{{ engine.path }}</div>
              </div>
              <div class="engine-badges">
                <span class="engine-badge" :class="{ 'custom': engine.is_custom, 'official': !engine.is_custom }">
                  {{ engine.is_custom ? 'Custom' : 'Official' }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="scan-stats">
          <div class="stat-item">
            <span class="stat-label">Scan Duration:</span>
            <span class="stat-value">{{ formatDuration(detectionResult.scan_duration_ms) }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <button 
        v-if="!isDetecting && !detectionComplete"
        class="cancel-button" 
        @click="$emit('close')"
      >
        Cancel
      </button>
      <button 
        v-if="!isDetecting && !detectionComplete"
        class="detect-button" 
        @click="startDetection"
      >
        <span class="button-icon">🔍</span>
        Start Detection
      </button>

      <button 
        v-if="isDetecting"
        class="cancel-button" 
        @click="$emit('close')"
        disabled
      >
        Please Wait...
      </button>

      <button 
        v-if="detectionComplete"
        class="close-button-action" 
        @click="$emit('close')"
      >
        <span class="button-icon">✅</span>
        Close
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogStore } from '../../stores/logStore'

interface DetectedEngine {
  name: string
  path: string
  version: string
  is_custom: boolean
}

interface EngineDetectionResult {
  engines: DetectedEngine[]
  total_found: number
  scan_duration_ms: number
}

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const isDetecting = ref(false)
const detectionComplete = ref(false)
const detectionResult = ref<EngineDetectionResult>({
  engines: [],
  total_found: 0,
  scan_duration_ms: 0
})

const startDetection = async () => {
  try {
    isDetecting.value = true
    detectionComplete.value = false
    
    addLog('Starting auto-detection of Unreal Engine installations...')
    
    const result = await invoke('auto_detect_engines') as EngineDetectionResult
    
    detectionResult.value = result
    detectionComplete.value = true
    
    if (result.total_found > 0) {
      addLog(`Auto-detection completed successfully. Found ${result.total_found} engine(s).`)
    } else {
      addLog('Auto-detection completed. No engines were found.')
    }
    
  } catch (error) {
    console.error('Engine auto-detection failed:', error)
    addLog('Engine auto-detection failed. Check console for details.', 'error')
    detectionComplete.value = true
    detectionResult.value = {
      engines: [],
      total_found: 0,
      scan_duration_ms: 0
    }
  } finally {
    isDetecting.value = false
  }
}

const formatDuration = (ms: number): string => {
  const seconds = Math.floor(ms / 1000)
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  
  if (minutes > 0) {
    return `${minutes}m ${remainingSeconds}s`
  } else {
    return `${remainingSeconds}s`
  }
}
</script>

<style scoped>
.engine-detection-popup {
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
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
}

.header-content {
  flex-grow: 1;
}

.popup-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.title-icon {
  font-size: var(--icon-size-lg);
}

.warning-subtitle {
  font-size: var(--font-size-sm);
  color: #d69e2e;
  margin: 0;
  font-weight: var(--font-weight-medium);
}

.close-button {
  background: none;
  border: none;
  font-size: var(--font-size-lg);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.close-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.close-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.popup-content {
  padding: var(--spacing-lg);
  max-height: 60vh;
  overflow-y: auto;
}

.detection-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.info-section {
  display: flex;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background-color: #fef5e7;
  border: var(--border-width) solid #f6e05e;
  border-radius: var(--border-radius-md);
}

.info-icon {
  font-size: var(--icon-size-lg);
  flex-shrink: 0;
}

.info-content {
  flex-grow: 1;
}

.info-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs) 0;
}

.info-text {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: var(--line-height-normal);
}

.detection-details,
.scan-scope {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.details-title,
.scope-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.detection-list {
  margin: 0;
  padding-left: var(--spacing-lg);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.detection-list li {
  margin-bottom: var(--spacing-xs);
}

.scope-description {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: var(--line-height-normal);
}

.detection-progress {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
  text-align: center;
}

.progress-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-md);
}

.progress-icon {
  font-size: var(--icon-size-xl);
  animation: spin 2s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.progress-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.progress-subtitle {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.progress-note {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.note-icon {
  font-size: var(--font-size-md);
  flex-shrink: 0;
}

.note-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

.detection-results {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.results-header {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
}

.results-icon {
  font-size: var(--icon-size-xl);
  flex-shrink: 0;
}

.results-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.results-subtitle {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.detected-engines {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.engines-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.engines-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.engine-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.engine-info {
  flex-grow: 1;
  min-width: 0;
}

.engine-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.engine-path {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  word-break: break-all;
}

.engine-badges {
  display: flex;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.engine-badge {
  font-size: var(--font-size-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-weight: var(--font-weight-medium);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.engine-badge.official {
  background-color: #e6fffa;
  color: #319795;
}

.engine-badge.custom {
  background-color: #fef5e7;
  color: #d69e2e;
}

.scan-stats {
  display: flex;
  justify-content: center;
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
}

.stat-item {
  display: flex;
  gap: var(--spacing-sm);
  font-size: var(--font-size-sm);
}

.stat-label {
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.stat-value {
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.popup-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
}

.cancel-button,
.detect-button,
.close-button-action {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.cancel-button {
  background-color: transparent;
  border: var(--border-width) solid var(--border-color);
  color: var(--text-secondary);
}

.cancel-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.cancel-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.detect-button,
.close-button-action {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.detect-button:hover,
.close-button-action:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>