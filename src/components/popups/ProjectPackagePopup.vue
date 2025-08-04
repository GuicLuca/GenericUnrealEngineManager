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
              {{ getBuildTypeDescription(packageConfig.buildType) }}
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
              Target platform for the packaged build
            </div>
          </div>
        </div>
      </div>

      <!-- Output Directory Section -->
      <div class="package-section">
        <h3 class="section-title">Output Directory</h3>
        <div class="directory-input-group">
          <input
            v-model="packageConfig.outputDirectory"
            type="text"
            class="directory-input"
            placeholder="Select output directory for packaged build..."
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
        <div class="config-hint">
          Directory where the packaged build will be saved
        </div>
      </div>

      <!-- Advanced Options Section -->
      <div class="package-section">
        <h3 class="section-title">Advanced Options</h3>
        
        <div class="checkbox-group">
          <div class="checkbox-item">
            <input
              id="package-for-distribution"
              v-model="packageConfig.forDistribution"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="package-for-distribution" class="checkbox-label">
              Package for distribution
              <InfoTooltip 
                content="Creates a build optimized for distribution. Enables additional optimizations and removes development-only content."
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
                content="Includes the Visual C++ Redistributable and other required components with the build."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="include-app-local-prerequisites"
              v-model="packageConfig.includeAppLocalPrerequisites"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="include-app-local-prerequisites" class="checkbox-label">
              Include app-local prerequisites
              <InfoTooltip 
                content="Includes prerequisites in the application directory instead of system-wide installation."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="include-crash-reporter"
              v-model="packageConfig.includeCrashReporter"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="include-crash-reporter" class="checkbox-label">
              Include crash reporter
              <InfoTooltip 
                content="Includes the Unreal Engine crash reporter for collecting crash data."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="use-pak-file"
              v-model="packageConfig.usePakFile"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="use-pak-file" class="checkbox-label">
              Use PAK file
              <InfoTooltip 
                content="Packages game content into PAK files for better loading performance and content protection."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="compress-content"
              v-model="packageConfig.compressContent"
              type="checkbox"
              class="checkbox-input"
              :disabled="isPackaging"
            />
            <label for="compress-content" class="checkbox-label">
              Compress content
              <InfoTooltip 
                content="Compresses packaged content to reduce file size. May increase loading times on slower devices."
              />
            </label>
          </div>
        </div>
      </div>

      <!-- Archive Section -->
      <div class="package-section" :class="{ 'section-collapsed': !packageConfig.createArchive }">
        <div class="section-header">
          <input
            id="create-archive"
            v-model="packageConfig.createArchive"
            type="checkbox"
            class="checkbox-input"
            :disabled="isPackaging"
          />
          <label for="create-archive" class="section-title-label">
            Create archive after packaging
            <InfoTooltip 
              content="Automatically create a compressed archive of the packaged build for easy distribution."
            />
          </label>
        </div>

        <!-- Archive Options (shown when create archive is enabled) -->
        <div v-if="packageConfig.createArchive" class="archive-options">
          <div class="config-group">
            <label class="config-label">Archive Format</label>
            <select
              v-model="packageConfig.archiveFormat"
              class="config-select"
              :disabled="isPackaging"
            >
              <option value="Zip">ZIP</option>
              <option value="SevenZip">7-Zip</option>
              <option value="Tar">TAR</option>
              <option value="TarGz">TAR.GZ</option>
            </select>
          </div>

          <div class="config-group">
            <label class="config-label">Archive Filename Format</label>
            <select
              v-model="packageConfig.archiveFilenameFormat"
              class="config-select"
              :disabled="isPackaging"
            >
              <option 
                v-for="(format, name) in availableFormats"
                :key="name"
                :value="format"
              >
                {{ name }}
              </option>
            </select>
            <div class="format-preview">
              <span class="preview-label">Preview:</span>
              <span class="preview-filename">{{ archivePreview }}</span>
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
import { useProjectStore } from '../../stores/projectStore'

interface Props {
  projectName: string
  projectPath: string
}

interface PackageConfig {
  buildType: string
  targetPlatform: string
  outputDirectory: string
  forDistribution: boolean
  includePrerequisites: boolean
  includeAppLocalPrerequisites: boolean
  includeCrashReporter: boolean
  usePakFile: boolean
  compressContent: boolean
  createArchive: boolean
  archiveFormat: string
  archiveFilenameFormat: string
}

type CompressionAlgorithm = 'Zip' | 'SevenZip' | 'Tar' | 'TarGz'

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()
const { findProjectByPath } = useProjectStore()

const isPackaging = ref(false)
const availableFormats = ref<Record<string, string>>({})

const packageConfig = reactive<PackageConfig>({
  buildType: 'Development',
  targetPlatform: 'Win64',
  outputDirectory: '',
  forDistribution: false,
  includePrerequisites: true,
  includeAppLocalPrerequisites: false,
  includeCrashReporter: true,
  usePakFile: true,
  compressContent: false,
  createArchive: false,
  archiveFormat: 'Zip',
  archiveFilenameFormat: '[Project]_[Platform]_[BuildType]_[YYYY][MM][DD][HH][mm]'
})

const canPackage = computed(() => {
  return packageConfig.outputDirectory.trim() !== ''
})

const archivePreview = computed(() => {
  if (!packageConfig.createArchive) return ''
  
  const project = findProjectByPath(props.projectPath)
  const now = new Date()
  let preview = packageConfig.archiveFilenameFormat
  
  const replacements: Record<string, string> = {
    'Project': props.projectName,
    'Platform': packageConfig.targetPlatform,
    'BuildType': packageConfig.buildType,
    'Type': project?.has_cpp ? 'Cpp' : 'Bp',
    'Engine': project ? getEngineVersionFormatted(project.engine_association) : 'Unknown',
    'YYYY': now.getFullYear().toString(),
    'YY': now.getFullYear().toString().slice(-2),
    'MM': (now.getMonth() + 1).toString().padStart(2, '0'),
    'DD': now.getDate().toString().padStart(2, '0'),
    'HH': now.getHours().toString().padStart(2, '0'),
    'mm': now.getMinutes().toString().padStart(2, '0'),
    'ss': now.getSeconds().toString().padStart(2, '0')
  }
  
  for (const [key, value] of Object.entries(replacements)) {
    preview = preview.replace(new RegExp(`\\[${key}\\]`, 'g'), value)
  }
  
  const extension = getExtensionForAlgorithm(packageConfig.archiveFormat as CompressionAlgorithm)
  if (!preview.includes('.')) {
    preview += `.${extension}`
  }
  
  return preview
})

const getBuildTypeDescription = (buildType: string): string => {
  switch (buildType) {
    case 'Debug':
      return 'Full debugging information, no optimizations. Largest and slowest build.'
    case 'DebugGame':
      return 'Debugging for game code only, engine is optimized. Good for gameplay debugging.'
    case 'Development':
      return 'Balanced build with some optimizations and debugging capabilities.'
    case 'Shipping':
      return 'Fully optimized build for release. No debugging information, smallest size.'
    case 'Test':
      return 'Similar to Shipping but with some console commands and stats enabled.'
    default:
      return ''
  }
}

const getEngineVersionFormatted = (engineAssociation: any): string => {
  if (typeof engineAssociation === 'string' && engineAssociation === 'Custom') {
    return 'Custom'
  }
  if (typeof engineAssociation === 'object' && engineAssociation.Standard) {
    return engineAssociation.Standard.replace(/\./g, '-')
  }
  return 'Unknown'
}

const getExtensionForAlgorithm = (algorithm: CompressionAlgorithm): string => {
  switch (algorithm) {
    case 'Zip': return 'zip'
    case 'SevenZip': return '7z'
    case 'Tar': return 'tar'
    case 'TarGz': return 'tar.gz'
    default: return 'zip'
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
    console.error('Failed to open directory dialog:', error)
    addLog('Failed to open directory dialog', 'error')
  }
}

const loadAvailableFormats = async () => {
  try {
    const settings = await invoke('get_settings') as any
    availableFormats.value = {
      'Default': '[Project]_[Platform]_[BuildType]_[YYYY][MM][DD][HH][mm]',
      'Extended': '[Project]_[Platform]_[BuildType]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
      'Simple': '[Project]_[Platform]_[BuildType]',
      ...settings.compression.custom_presets
    }
  } catch (error) {
    console.error('Failed to load compression settings:', error)
    availableFormats.value = {
      'Default': '[Project]_[Platform]_[BuildType]_[YYYY][MM][DD][HH][mm]',
      'Extended': '[Project]_[Platform]_[BuildType]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
      'Simple': '[Project]_[Platform]_[BuildType]'
    }
  }
}

const detectPlatform = async () => {
  try {
    const platform = await invoke('get_current_platform') as string
    switch (platform.toLowerCase()) {
      case 'windows':
        packageConfig.targetPlatform = 'Win64'
        break
      case 'macos':
        packageConfig.targetPlatform = 'Mac'
        break
      case 'linux':
        packageConfig.targetPlatform = 'Linux'
        break
    }
  } catch (error) {
    console.error('Failed to detect platform:', error)
    // Keep default Win64
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
      for_distribution: packageConfig.forDistribution,
      include_prerequisites: packageConfig.includePrerequisites,
      include_app_local_prerequisites: packageConfig.includeAppLocalPrerequisites,
      include_crash_reporter: packageConfig.includeCrashReporter,
      use_pak_file: packageConfig.usePakFile,
      compress_content: packageConfig.compressContent,
      create_archive: packageConfig.createArchive,
      archive_format: packageConfig.createArchive ? packageConfig.archiveFormat : null,
      archive_filename_format: packageConfig.createArchive ? packageConfig.archiveFilenameFormat : null
    }
    
    await invoke('package_project', { request })
    
    emit('close')
    
  } catch (error) {
    // Backend will handle error logging
  } finally {
    isPackaging.value = false
  }
}

onMounted(() => {
  loadAvailableFormats()
  detectPlatform()
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
  background-color: var(--surface-color);
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

.directory-input-group {
  display: flex;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-xs);
}

.directory-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--surface-color);
  cursor: pointer;
}

.directory-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.browse-button {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
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

.checkbox-group {
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

.archive-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
}

.format-preview {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  margin-top: var(--spacing-xs);
}

.preview-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
  flex-shrink: 0;
}

.preview-filename {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
  word-break: break-all;
  flex-grow: 1;
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
  .config-grid,
  .archive-options {
    grid-template-columns: 1fr;
  }
  
  .project-package-popup {
    min-width: 90vw;
    max-width: 90vw;
  }
}
</style>