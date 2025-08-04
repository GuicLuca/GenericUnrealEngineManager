<template>
  <div class="project-package-popup">
    <div class="popup-header">
      <div class="header-content">
        <h2 class="popup-title">
          <span class="title-icon">📦</span>
          Package {{ projectName }}
        </h2>
        <div class="project-note">Configure build settings and package your project</div>
      </div>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <!-- Build Configuration Section -->
      <div class="package-section">
        <h3 class="section-title">Build Configuration</h3>
        
        <div class="config-grid">
          <!-- Build Type -->
          <div class="config-group">
            <label class="config-label">Build Type</label>
            <select
              v-model="packageConfig.buildType"
              class="config-select"
              :disabled="isPackaging"
            >
              <option value="Debug">Debug</option>
              <option value="DebugGame">DebugGame</option>
              <option value="Development">Development</option>
              <option value="Shipping">Shipping</option>
              <option value="Test">Test</option>
            </select>
            <div class="config-hint">
              Debug: Full debugging info. DebugGame: Game debugging only. Development: Default for testing. Shipping: Optimized for release. Test: For automated testing.
            </div>
          </div>

          <!-- Target Platform -->
          <div class="config-group">
            <label class="config-label">Target Platform</label>
            <select
              v-model="packageConfig.targetPlatform"
              class="config-select"
              :disabled="isPackaging"
            >
              <option value="Win64">Windows (64-bit)</option>
              <option value="Mac">macOS</option>
              <option value="Linux">Linux</option>
              <option value="Android">Android</option>
              <option value="iOS">iOS</option>
              <option value="HoloLens">HoloLens</option>
              <option value="TVOS">tvOS</option>
            </select>
            <div class="config-hint">
              Select the target platform for your packaged build.
            </div>
          </div>
        </div>
      </div>

      <!-- Advanced Options Section -->
      <div class="package-section">
        <h3 class="section-title">Advanced Options</h3>
        
        <div class="advanced-options">
          <div class="checkbox-item">
            <input
              id="cook-content"
              v-model="packageConfig.cookContent"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="cook-content" class="checkbox-label">
              Cook content for target platform
              <InfoTooltip 
                content="Cook (optimize) game content for the target platform. Recommended for most builds."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="pak-files"
              v-model="packageConfig.pakFiles"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="pak-files" class="checkbox-label">
              Package files into .pak files
              <InfoTooltip 
                content="Bundle game content into compressed .pak files for better performance and security."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="include-prerequisites"
              v-model="packageConfig.includePrerequisites"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="include-prerequisites" class="checkbox-label">
              Include prerequisites installer
              <InfoTooltip 
                content="Include Visual C++ redistributables and other required components."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="include-debug-files"
              v-model="packageConfig.includeDebugFiles"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="include-debug-files" class="checkbox-label">
              Include debug files
              <InfoTooltip 
                content="Include .pdb files and other debugging information. Increases build size."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="create-release-version"
              v-model="packageConfig.createReleaseVersion"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="create-release-version" class="checkbox-label">
              Create release version
              <InfoTooltip 
                content="Create a release version with version information embedded."
              />
            </label>
          </div>
        </div>
      </div>

      <!-- Build Destination Section -->
      <div class="package-section">
        <h3 class="section-title">Build Destination</h3>
        <div class="destination-input-group">
          <input
            v-model="packageConfig.outputDirectory"
            type="text"
            class="destination-input"
            placeholder="Select output directory..."
            readonly
            :disabled="isPackaging"
          />
          <button
            type="button"
            class="browse-button"
            @click="selectOutputDirectory"
            title="Browse for output directory"
            :disabled="isPackaging"
          >
            📂
          </button>
        </div>
      </div>

      <!-- Post-Build Compression Section -->
      <div class="package-section" :class="{ 'section-collapsed': !packageConfig.compressAfterBuild }">
        <div class="section-header">
          <input
            id="compress-after-build"
            v-model="packageConfig.compressAfterBuild"
            type="checkbox"
            class="checkbox-input"
            :disabled="isPackaging"
          />
          <label for="compress-after-build" class="section-title-label">
            Compress build after packaging
            <InfoTooltip 
              content="Create a compressed archive of the packaged build for easier distribution."
            />
          </label>
        </div>

        <!-- Compression Options (shown when compress is enabled) -->
        <div v-if="packageConfig.compressAfterBuild" class="compression-options">
          <div class="compression-group">
            <label class="config-label">Compression Format</label>
            <div class="algorithm-selection">
              <div 
                v-for="algorithm in availableAlgorithms"
                :key="algorithm"
                class="algorithm-item"
              >
                <input
                  :id="`package-algorithm-${algorithm}`"
                  v-model="packageConfig.compressionAlgorithm"
                  :value="algorithm"
                  type="radio"
                  class="radio-input"
                  :disabled="isPackaging"
                />
                <label :for="`package-algorithm-${algorithm}`" class="algorithm-label">
                  <span class="algorithm-name">{{ getAlgorithmDisplayName(algorithm) }}</span>
                  <span class="algorithm-description">{{ getAlgorithmDescription(algorithm) }}</span>
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <button class="cancel-button" @click="$emit('close')" :disabled="isPackaging">
        Cancel
      </button>
      <button 
        class="package-button" 
        @click="startPackaging" 
        :disabled="isPackaging || !canPackage"
      >
        <span class="button-icon">{{ isPackaging ? '⏳' : '📦' }}</span>
        {{ isPackaging ? 'Packaging...' : 'Start Packaging' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import InfoTooltip from '../InfoTooltip.vue'
import { useLogStore } from '../../stores/logStore'

interface Props {
  projectName: string
  projectPath: string
}

interface PackageConfig {
  buildType: string
  targetPlatform: string
  outputDirectory: string
  cookContent: boolean
  pakFiles: boolean
  includePrerequisites: boolean
  includeDebugFiles: boolean
  createReleaseVersion: boolean
  compressAfterBuild: boolean
  compressionAlgorithm: string
}

type CompressionAlgorithm = 'Zip' | 'SevenZip' | 'Tar' | 'TarGz'

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const isPackaging = ref(false)
const availableAlgorithms = ref<CompressionAlgorithm[]>(['Zip', 'SevenZip', 'Tar', 'TarGz'])

const packageConfig = reactive<PackageConfig>({
  buildType: 'Development',
  targetPlatform: 'Win64',
  outputDirectory: '',
  cookContent: true,
  pakFiles: true,
  includePrerequisites: true,
  includeDebugFiles: false,
  createReleaseVersion: false,
  compressAfterBuild: false,
  compressionAlgorithm: 'Zip'
})

const canPackage = computed(() => {
  return packageConfig.outputDirectory.trim() !== ''
})

const getAlgorithmDisplayName = (algorithm: CompressionAlgorithm): string => {
  switch (algorithm) {
    case 'Zip': return 'ZIP'
    case 'SevenZip': return '7-Zip'
    case 'Tar': return 'TAR'
    case 'TarGz': return 'TAR.GZ'
    default: return algorithm
  }
}

const getAlgorithmDescription = (algorithm: CompressionAlgorithm): string => {
  switch (algorithm) {
    case 'Zip': return 'Standard ZIP compression, widely supported'
    case 'SevenZip': return 'High compression ratio, requires 7-Zip'
    case 'Tar': return 'Archive format, no compression'
    case 'TarGz': return 'TAR with GZIP compression'
    default: return ''
  }
}

const selectOutputDirectory = async () => {
  if (isPackaging.value) return
  
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select output directory for packaged build'
    })
    
    if (selected) {
      packageConfig.outputDirectory = selected
    }
  } catch (error) {
    console.error('Failed to open output directory dialog:', error)
    addLog('Failed to open output directory dialog', 'error')
  }
}

const loadAvailableAlgorithms = async () => {
  try {
    const algorithms = await invoke('get_available_compression_algorithms') as CompressionAlgorithm[]
    availableAlgorithms.value = algorithms
    
    // Set the default algorithm to the first available one
    if (algorithms.length > 0) {
      packageConfig.compressionAlgorithm = algorithms[0]
    }
  } catch (error) {
    console.error('Failed to load available compression algorithms:', error)
    addLog('Failed to load compression algorithms', 'error')
    // Fallback to ZIP
    availableAlgorithms.value = ['Zip']
    packageConfig.compressionAlgorithm = 'Zip'
  }
}

const detectCurrentPlatform = async () => {
  try {
    const platform = await invoke('get_current_platform') as string
    
    // Set default platform based on current system
    switch (platform.toLowerCase()) {
      case 'windows':
        packageConfig.targetPlatform = 'Win64'
        break
      case 'macos':
      case 'darwin':
        packageConfig.targetPlatform = 'Mac'
        break
      case 'linux':
        packageConfig.targetPlatform = 'Linux'
        break
      default:
        packageConfig.targetPlatform = 'Win64'
    }
  } catch (error) {
    console.error('Failed to detect current platform:', error)
    // Default to Windows
    packageConfig.targetPlatform = 'Win64'
  }
}

const startPackaging = async () => {
  if (!canPackage.value || isPackaging.value) return
  
  try {
    isPackaging.value = true
    
    const request = {
      project_path: props.projectPath,
      build_type: packageConfig.buildType,
      target_platform: packageConfig.targetPlatform,
      output_directory: packageConfig.outputDirectory,
      cook_content: packageConfig.cookContent,
      pak_files: packageConfig.pakFiles,
      include_prerequisites: packageConfig.includePrerequisites,
      include_debug_files: packageConfig.includeDebugFiles,
      create_release_version: packageConfig.createReleaseVersion,
      compress_after_build: packageConfig.compressAfterBuild,
      compression_algorithm: packageConfig.compressAfterBuild ? packageConfig.compressionAlgorithm : null
    }
    
    await invoke('package_project', { request })
    
    emit('close')
    
  } catch (error) {
    // Do nothing, the backend will handle the error
  } finally {
    isPackaging.value = false
  }
}

onMounted(() => {
  detectCurrentPlatform()
  loadAvailableAlgorithms()
})
</script>

<style scoped>
.project-package-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 48rem;
  min-width: 48rem;
  max-height: 85vh;
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
  flex-shrink: 0;
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

.project-note {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
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

.close-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.popup-content {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-lg);
}

.package-section {
  margin-bottom: var(--spacing-lg);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
}

.package-section:last-of-type {
  margin-bottom: 0;
}

.package-section.section-collapsed {
  padding-bottom: var(--spacing-md);
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.section-title-label {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.config-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-lg);
}

.config-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.config-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.config-select {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  cursor: pointer;
}

.config-select:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.config-select:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.config-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

.advanced-options {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.checkbox-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
}

.checkbox-input {
  margin-top: 2px;
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
  flex-shrink: 0;
}

.checkbox-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.checkbox-label {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  line-height: var(--line-height-normal);
  flex-grow: 1;
}

.destination-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.destination-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  cursor: pointer;
}

.destination-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.browse-button {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--background-color);
  cursor: pointer;
  font-size: var(--font-size-md);
  transition: all var(--transition-fast);
  min-width: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.browse-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.browse-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.compression-options {
  padding: var(--spacing-md);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
}

.compression-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.algorithm-selection {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.algorithm-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  transition: all var(--transition-fast);
}

.algorithm-item:has(.radio-input:checked) {
  border-color: var(--accent-color);
  background-color: var(--accent-color-alpha);
}

.radio-input {
  margin-top: 2px;
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
  flex-shrink: 0;
}

.radio-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.algorithm-label {
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  flex-grow: 1;
}

.algorithm-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.algorithm-description {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.popup-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  flex-shrink: 0;
}

.cancel-button,
.package-button {
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

.package-button {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.package-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.package-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .config-grid {
    grid-template-columns: 1fr;
  }
  
  .project-package-popup {
    min-width: 90vw;
    max-width: 90vw;
  }
}
</style>