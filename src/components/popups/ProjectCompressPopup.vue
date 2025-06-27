<template>
  <div class="project-compress-popup">
    <div class="popup-header">
      <div class="header-content">
        <h2 class="popup-title">
          <span class="title-icon">🗜️</span>
          Compress {{ projectName }}
        </h2>
        <div class="project-note">Create a compressed archive of your project</div>
      </div>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <!-- Clean Before Compress Section -->
      <div class="compress-section" :class="{ 'section-collapsed': !cleanBeforeCompress }">
        <div class="section-header">
          <input
            id="clean-before-compress"
            v-model="cleanBeforeCompress"
            type="checkbox"
            class="checkbox-input"
            :disabled="isCompressing"
          />
          <label for="clean-before-compress" class="section-title-label">
            Clean project before compressing
            <InfoTooltip 
              content="Remove temporary and generated files before creating the archive to reduce the final file size."
            />
          </label>
        </div>

        <!-- Cleaning Options (shown when clean is enabled) -->
        <div v-if="cleanBeforeCompress" class="cleaning-options">
          <!-- Project Scanning Section -->
          <div class="cleaning-subsection">
            <h4 class="subsection-title">Project Scanning</h4>
            <div class="checkbox-group">
              <div class="checkbox-item">
                <input
                  id="compress-ide-files"
                  v-model="cleaningSelection.ide_files"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-ide-files" class="checkbox-label">
                  IDE files (.vs and .idea)
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-binaries"
                  v-model="cleaningSelection.binaries"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-binaries" class="checkbox-label">
                  Binaries
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-build"
                  v-model="cleaningSelection.build"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-build" class="checkbox-label">
                  Build
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-intermediate"
                  v-model="cleaningSelection.intermediate"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-intermediate" class="checkbox-label">
                  Intermediate
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-derived-data-cache"
                  v-model="cleaningSelection.derived_data_cache"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-derived-data-cache" class="checkbox-label">
                  DerivedDataCache
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-saved"
                  v-model="cleaningSelection.saved"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-saved" class="checkbox-label">
                  Saved
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-analyze-plugins"
                  v-model="cleaningSelection.analyze_plugins"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-analyze-plugins" class="checkbox-label">
                  Analyze plugins
                </label>
              </div>
            </div>
          </div>

          <!-- Plugins Scanning Section -->
          <div v-if="cleaningSelection.analyze_plugins" class="cleaning-subsection">
            <h4 class="subsection-title">Plugins Scanning</h4>
            <div class="checkbox-group">
              <div class="checkbox-item">
                <input
                  id="compress-plugin-binaries"
                  v-model="cleaningSelection.plugin_binaries"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-plugin-binaries" class="checkbox-label">
                  Binaries
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-plugin-intermediate"
                  v-model="cleaningSelection.plugin_intermediate"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-plugin-intermediate" class="checkbox-label">
                  Intermediate
                </label>
              </div>

              <div class="checkbox-item">
                <input
                  id="compress-plugin-node-size-cache"
                  v-model="cleaningSelection.plugin_node_size_cache"
                  type="checkbox"
                  class="checkbox-input"
                  :disabled="isCompressing"
                />
                <label for="compress-plugin-node-size-cache" class="checkbox-label">
                  NodeSizeCache
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Compression Algorithm Section -->
      <div class="compress-section">
        <h3 class="section-title">Compression Algorithm</h3>
        <div class="algorithm-selection">
          <div 
            v-for="algorithm in availableAlgorithms"
            :key="algorithm"
            class="algorithm-item"
          >
            <input
              :id="`algorithm-${algorithm}`"
              v-model="selectedAlgorithm"
              :value="algorithm"
              type="radio"
              class="radio-input"
              :disabled="isCompressing"
            />
            <label :for="`algorithm-${algorithm}`" class="algorithm-label">
              <span class="algorithm-name">{{ getAlgorithmDisplayName(algorithm) }}</span>
              <span class="algorithm-description">{{ getAlgorithmDescription(algorithm) }}</span>
            </label>
          </div>
        </div>
      </div>

      <!-- Filename Format Section -->
      <div class="compress-section">
        <h3 class="section-title">Filename Format</h3>
        <div class="format-selection">
          <label class="format-label">Output filename format:</label>
          <select
            v-model="selectedFormat"
            class="format-dropdown"
            :disabled="isCompressing"
            @change="updatePreview"
          >
            <option 
              v-for="(format, name) in sortedAvailableFormats"
              :key="name"
              :value="format"
            >
              {{ name }}
            </option>
          </select>
          <div class="format-preview">
            <span class="preview-label">Preview:</span>
            <span class="preview-filename">{{ outputFilename }}</span>
          </div>
        </div>
      </div>

      <!-- Destination Section -->
      <div class="compress-section">
        <h3 class="section-title">Destination</h3>
        <div class="destination-input-group">
          <input
            v-model="destinationPath"
            type="text"
            class="destination-input"
            placeholder="Select destination folder..."
            readonly
            :disabled="isCompressing"
          />
          <button
            type="button"
            class="browse-button"
            @click="selectDestination"
            title="Browse for destination folder"
            :disabled="isCompressing"
          >
            📂
          </button>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <button class="cancel-button" @click="$emit('close')" :disabled="isCompressing">
        Cancel
      </button>
      <button 
        class="compress-button" 
        @click="startCompression" 
        :disabled="isCompressing || !canCompress"
      >
        <span class="button-icon">{{ isCompressing ? '⏳' : '🗜️' }}</span>
        {{ isCompressing ? 'Compressing...' : 'Start Compression' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import InfoTooltip from '../InfoTooltip.vue'
import { useLogStore } from '../../stores/logStore'
import { useProjectStore } from '../../stores/projectStore'

interface Props {
  projectName: string
  projectPath: string
}

interface CleaningSelection {
  ide_files: boolean
  binaries: boolean
  build: boolean
  intermediate: boolean
  derived_data_cache: boolean
  saved: boolean
  analyze_plugins: boolean
  plugin_binaries: boolean
  plugin_intermediate: boolean
  plugin_node_size_cache: boolean
}

type CompressionAlgorithm = 'Zip' | 'SevenZip' | 'Tar' | 'TarGz'

interface AppSettings {
  compression: {
    filename_format: string
    custom_presets: Record<string, string>
  }
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()
const { findProjectByPath } = useProjectStore()

const isCompressing = ref(false)
const cleanBeforeCompress = ref(false)
const selectedAlgorithm = ref<CompressionAlgorithm>('Zip')
const selectedFormat = ref('[Project]_[YYYY][MM][DD][HH][mm]')
const destinationPath = ref('')
const availableAlgorithms = ref<CompressionAlgorithm[]>([])
const availableFormats = ref<Record<string, string>>({})
const systemUsername = ref('john_doe')
const systemHostname = ref('DESKTOP-PC')

const cleaningSelection = reactive<CleaningSelection>({
  ide_files: true,
  binaries: true,
  build: true,
  intermediate: true,
  derived_data_cache: false,
  saved: false,
  analyze_plugins: false,
  plugin_binaries: false,
  plugin_intermediate: false,
  plugin_node_size_cache: false
})

const canCompress = computed(() => {
  return destinationPath.value.trim() !== '' && selectedAlgorithm.value !== null
})

// Sort available formats alphabetically by name
const sortedAvailableFormats = computed(() => {
  const entries = Object.entries(availableFormats.value)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

const outputFilename = computed(() => {
  // Get the project details for better filename generation
  const project = findProjectByPath(props.projectPath)
  
  // Generate the preview using the selected format
  const now = new Date()
  let preview = selectedFormat.value
  
  // Replace common tags with example values
  const replacements: Record<string, string> = {
    'Project': props.projectName,
    'Type': project?.has_cpp ? 'Cpp' : 'Bp',
    'Engine': project ? getEngineVersionFormatted(project.engine_association) : 'Unknown',
    'SizeMB': project ? Math.floor(project.size_on_disk / (1024 * 1024)).toString() : '0',
    'SizeGB': project ? Math.floor(project.size_on_disk / (1024 * 1024 * 1024)).toString() : '0',
    'PluginCount': project ? project.plugins.length.toString() : '0',
    'Algorithm': getAlgorithmDisplayName(selectedAlgorithm.value),
    'YYYY': now.getFullYear().toString(),
    'YY': now.getFullYear().toString().slice(-2),
    'MM': (now.getMonth() + 1).toString().padStart(2, '0'),
    'DD': now.getDate().toString().padStart(2, '0'),
    'HH': now.getHours().toString().padStart(2, '0'),
    'mm': now.getMinutes().toString().padStart(2, '0'),
    'ss': now.getSeconds().toString().padStart(2, '0'),
    'Month': now.toLocaleDateString('en-US', { month: 'long' }),
    'Mon': now.toLocaleDateString('en-US', { month: 'short' }),
    'Day': now.toLocaleDateString('en-US', { weekday: 'long' }),
    'Weekday': now.toLocaleDateString('en-US', { weekday: 'short' }),
    'User': systemUsername.value,
    'Computer': systemHostname.value,
    'Timestamp': Math.floor(now.getTime() / 1000).toString()
  }
  
  for (const [key, value] of Object.entries(replacements)) {
    preview = preview.replace(new RegExp(`\\[${key}\\]`, 'g'), value)
  }
  
  const extension = getExtensionForAlgorithm(selectedAlgorithm.value)
  if (!preview.includes('.')) {
    preview += `.${extension}`
  }
  
  return preview
})

// Watch for changes in selectedFormat to update preview
watch(selectedFormat, () => {
  // The computed property will automatically update
})

const getEngineVersionFormatted = (engineAssociation: any): string => {
  if (typeof engineAssociation === 'string' && engineAssociation === 'Custom') {
    return 'Custom'
  }
  if (typeof engineAssociation === 'object' && engineAssociation.Standard) {
    return engineAssociation.Standard.replace(/\./g, '-')
  }
  return 'Unknown'
}

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

const getExtensionForAlgorithm = (algorithm: CompressionAlgorithm): string => {
  switch (algorithm) {
    case 'Zip': return 'zip'
    case 'SevenZip': return '7z'
    case 'Tar': return 'tar'
    case 'TarGz': return 'tar.gz'
    default: return 'zip'
  }
}

const loadSystemInfo = async () => {
  try {
    systemUsername.value = await invoke('get_system_username') as string
    systemHostname.value = await invoke('get_system_hostname') as string
  } catch (error) {
    console.error('Failed to load system info:', error)
    // Keep fallback values
  }
}

const loadAvailableAlgorithms = async () => {
  try {
    const algorithms = await invoke('get_available_compression_algorithms') as CompressionAlgorithm[]
    availableAlgorithms.value = algorithms
    
    // Set the default algorithm to the first available one
    if (algorithms.length > 0) {
      selectedAlgorithm.value = algorithms[0]
    }
  } catch (error) {
    console.error('Failed to load available compression algorithms:', error)
    addLog('Failed to load compression algorithms', 'error')
    // Fallback to ZIP
    availableAlgorithms.value = ['Zip']
    selectedAlgorithm.value = 'Zip'
  }
}

const loadAvailableFormats = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    availableFormats.value = settings.compression.custom_presets
    
    // Set default format
    if (settings.compression.filename_format) {
      selectedFormat.value = settings.compression.filename_format
    }
  } catch (error) {
    console.error('Failed to load compression settings:', error)
    addLog('Failed to load compression settings', 'error')
    // Fallback formats
    availableFormats.value = {
      'Default': '[Project]_[YYYY][MM][DD][HH][mm]',
      'Default Extended': '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
      'Simple': '[Project]_[Type]'
    }
  }
}

const updatePreview = () => {
  // Force reactivity update for the computed property
  // The computed property will automatically recalculate
}

const selectDestination = async () => {
  if (isCompressing.value) return
  
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select destination folder for compressed archive'
    })
    
    if (selected) {
      destinationPath.value = selected
    }
  } catch (error) {
    console.error('Failed to open destination dialog:', error)
    addLog('Failed to open destination dialog', 'error')
  }
}

const startCompression = async () => {
  if (!canCompress.value || isCompressing.value) return
  
  try {
    isCompressing.value = true
    
    const request = {
      project_path: props.projectPath,
      destination_path: destinationPath.value,
      compression_algorithm: selectedAlgorithm.value,
      clean_before_compress: cleanBeforeCompress.value,
      cleaning_selection: cleanBeforeCompress.value ? {
        ide_files: cleaningSelection.ide_files,
        binaries: cleaningSelection.binaries,
        build: cleaningSelection.build,
        intermediate: cleaningSelection.intermediate,
        derived_data_cache: cleaningSelection.derived_data_cache,
        saved: cleaningSelection.saved,
        analyze_plugins: cleaningSelection.analyze_plugins,
        plugin_binaries: cleaningSelection.plugin_binaries,
        plugin_intermediate: cleaningSelection.plugin_intermediate,
        plugin_node_size_cache: cleaningSelection.plugin_node_size_cache
      } : null
    }
    
    await invoke('compress_project', { request })
    
    emit('close')
    
  } catch (error) {
    // Do nothing, the backend will handle the error
  } finally {
    isCompressing.value = false
  }
}

onMounted(() => {
  loadSystemInfo()
  loadAvailableAlgorithms()
  loadAvailableFormats()
})
</script>

<style scoped>
.project-compress-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 42rem;
  min-width: 42rem;
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

.compress-section {
  margin-bottom: var(--spacing-lg);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
}

.compress-section:last-of-type {
  margin-bottom: 0;
}

.compress-section.section-collapsed {
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
  margin: 0;
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

.cleaning-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
}

.cleaning-subsection {
  display: flex;
  flex-direction: column;
}

.subsection-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
  padding-bottom: var(--spacing-xs);
  border-bottom: var(--border-width) solid var(--border-color);
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
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
  line-height: var(--line-height-normal);
  flex-grow: 1;
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

.format-selection {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.format-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.format-dropdown {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--surface-color);
  cursor: pointer;
}

.format-dropdown:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.format-dropdown:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.format-preview {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
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
  background-color: var(--surface-color);
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
.compress-button {
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

.compress-button {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.compress-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.compress-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .cleaning-options {
    grid-template-columns: 1fr;
  }
}
</style>