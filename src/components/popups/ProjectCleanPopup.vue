<template>
  <div class="project-clean-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">🧹</span>
        Clean Project
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="project-info">
        <div class="project-name">{{ projectName }}</div>
        <div class="project-note">Select temporary and generated files to remove:</div>
      </div>

      <!-- Project Scanning Section -->
      <div class="cleaning-section">
        <h3 class="section-title">Project Scanning</h3>
        <div class="checkbox-group">
          <div class="checkbox-item">
            <input
              id="ide-files"
              v-model="selection.ide_files"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="ide-files" class="checkbox-label">
              IDE files (.vs and .idea)
              <InfoTooltip 
                content="Visual Studio and JetBrains IDE cache and configuration files. Generated when opening the project."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="binaries"
              v-model="selection.binaries"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="binaries" class="checkbox-label">
              Binaries
              <InfoTooltip 
                content="Contains executable files or other files created during compiling."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="build"
              v-model="selection.build"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="build" class="checkbox-label">
              Build
              <InfoTooltip 
                content="Holds files needed for building the engine or game, including files necessary for creating platform-specific builds."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="intermediate"
              v-model="selection.intermediate"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="intermediate" class="checkbox-label">
              Intermediate
              <InfoTooltip 
                content="Contains temporary files generated during building the engine or game. In game directories, Shaders are stored in the Intermediate directory."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="derived-data-cache"
              v-model="selection.derived_data_cache"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="derived-data-cache" class="checkbox-label">
              DerivedDataCache
              <InfoTooltip 
                content="Contains derived data files generated on-load for referenced content. Not having cache files present for referenced content can increase load times dramatically."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="saved"
              v-model="selection.saved"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="saved" class="checkbox-label">
              Saved
              <InfoTooltip 
                content="Contains autosaves, configuration (.ini) files, and log files. Additionally, the Engine>Saved directory contains crash logs, hardware information, and Swarm options and data."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="analyze-plugins"
              v-model="selection.analyze_plugins"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="analyze-plugins" class="checkbox-label">
              Analyze plugins
              <InfoTooltip 
                content="Enable cleaning of temporary files within plugin directories. This will scan all plugins for cleanable files."
              />
            </label>
          </div>
        </div>
      </div>

      <!-- Plugins Scanning Section -->
      <div v-if="selection.analyze_plugins" class="cleaning-section">
        <h3 class="section-title">Plugins Scanning</h3>
        <div class="checkbox-group">
          <div class="checkbox-item">
            <input
              id="plugin-binaries"
              v-model="selection.plugin_binaries"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="plugin-binaries" class="checkbox-label">
              Binaries
              <InfoTooltip 
                content="Contains executable files or other files created during compiling."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="plugin-intermediate"
              v-model="selection.plugin_intermediate"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="plugin-intermediate" class="checkbox-label">
              Intermediate
              <InfoTooltip 
                content="Contains temporary files generated during building the engine or game. In game directories, Shaders are stored in the Intermediate directory."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="plugin-node-size-cache"
              v-model="selection.plugin_node_size_cache"
              type="checkbox"
              class="checkbox-input"
              :disabled="isCleaning"
            />
            <label for="plugin-node-size-cache" class="checkbox-label">
              NodeSizeCache
              <InfoTooltip 
                content="Cache files that store Blueprint node information for Blueprint graphs in plugins."
              />
            </label>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <div class="left-actions">
        <div class="checkbox-item">
          <input
            id="save-as-default"
            v-model="selection.save_as_default"
            type="checkbox"
            class="checkbox-input"
            :disabled="isCleaning"
          />
          <label for="save-as-default" class="checkbox-label">
            Set selection as default selection
          </label>
        </div>
      </div>
      
      <div class="right-actions">
        <button class="cancel-button" @click="$emit('close')" :disabled="isCleaning">
          Cancel
        </button>
        <button 
          class="clean-button" 
          @click="startCleaning" 
          :disabled="isCleaning || !hasSelection"
        >
          <span class="button-icon">{{ isCleaning ? '⏳' : '🧹' }}</span>
          {{ isCleaning ? 'Cleaning...' : 'Start Cleaning' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import InfoTooltip from '../InfoTooltip.vue'
import { useLogStore } from '../../stores/logStore'

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
  save_as_default: boolean
}

interface AppSettings {
  cleaning_defaults: {
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
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()

const isCleaning = ref(false)

const selection = reactive<CleaningSelection>({
  ide_files: true,
  binaries: true,
  build: true,
  intermediate: true,
  derived_data_cache: false,
  saved: false,
  analyze_plugins: false,
  plugin_binaries: false,
  plugin_intermediate: false,
  plugin_node_size_cache: false,
  save_as_default: false
})

const hasSelection = computed(() => {
  return selection.ide_files || 
         selection.binaries || 
         selection.build || 
         selection.intermediate || 
         selection.derived_data_cache || 
         selection.saved ||
         (selection.analyze_plugins && (
           selection.plugin_binaries || 
           selection.plugin_intermediate || 
           selection.plugin_node_size_cache
         ))
})

const loadDefaults = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    const defaults = settings.cleaning_defaults
    
    selection.ide_files = defaults.ide_files
    selection.binaries = defaults.binaries
    selection.build = defaults.build
    selection.intermediate = defaults.intermediate
    selection.derived_data_cache = defaults.derived_data_cache
    selection.saved = defaults.saved
    selection.analyze_plugins = defaults.analyze_plugins
    selection.plugin_binaries = defaults.plugin_binaries
    selection.plugin_intermediate = defaults.plugin_intermediate
    selection.plugin_node_size_cache = defaults.plugin_node_size_cache
    
  } catch (error) {
    console.error('Failed to load cleaning defaults:', error)
    addLog('Failed to load cleaning defaults', 'error')
  }
}

const startCleaning = async () => {
  if (!hasSelection.value || isCleaning.value) return
  
  try {
    isCleaning.value = true
    addLog(`Starting cleaning process for ${props.projectName}...`)
    
    const result = await invoke('clean_project', {
      projectPath: props.projectPath,
      selection: {
        ide_files: selection.ide_files,
        binaries: selection.binaries,
        build: selection.build,
        intermediate: selection.intermediate,
        derived_data_cache: selection.derived_data_cache,
        saved: selection.saved,
        analyze_plugins: selection.analyze_plugins,
        plugin_binaries: selection.plugin_binaries,
        plugin_intermediate: selection.plugin_intermediate,
        plugin_node_size_cache: selection.plugin_node_size_cache,
        save_as_default: selection.save_as_default
      }
    })
    
    addLog(`Cleaning completed successfully for ${props.projectName}`)
    emit('close')
    
  } catch (error) {
    console.error('Failed to clean project:', error)
    addLog('Failed to clean project. Check console for details.', 'error')
  } finally {
    isCleaning.value = false
  }
}

onMounted(() => {
  loadDefaults()
})
</script>

<style scoped>
.project-clean-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 36rem;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.popup-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
}

.popup-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.title-icon {
  font-size: var(--icon-size-lg);
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
}

.close-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.popup-content {
  flex-grow: 1;
  padding: var(--spacing-lg);
  overflow: hidden;
}

.project-info {
  margin-bottom: var(--spacing-lg);
  text-align: center;
}

.project-name {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
}

.project-note {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.cleaning-section {
  margin-bottom: var(--spacing-lg);
}

.cleaning-section:last-of-type {
  margin-bottom: 0;
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
  padding-bottom: var(--spacing-sm);
  border-bottom: var(--border-width) solid var(--border-color);
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

.popup-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
}

.left-actions {
  display: flex;
  align-items: center;
}

.right-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.cancel-button,
.clean-button {
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

.clean-button {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.clean-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.clean-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>