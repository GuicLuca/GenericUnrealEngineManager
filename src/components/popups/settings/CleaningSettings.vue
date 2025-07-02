<template>
  <div class="cleaning-settings">
    <div class="settings-section">
      <h4 class="section-title">Default Cleaning Options</h4>
      <div class="section-description">
        Configure the default selections for project cleaning operations. These settings will be pre-selected when you clean a project.
      </div>
      
      <div class="cleaning-layout">
        <!-- Project Scanning Options -->
        <div class="cleaning-column">
          <h5 class="column-title">Project Scanning</h5>
          <div class="checkbox-group">
            <div class="checkbox-item">
              <input
                v-model="localCleaning.ide_files"
                type="checkbox"
                class="checkbox-input"
                id="default-ide-files"
                @change="handleUpdate"
              />
              <label for="default-ide-files" class="checkbox-label">
                IDE files (.vs and .idea)
                <InfoTooltip 
                  content="Visual Studio and JetBrains IDE cache and configuration files. Generated when opening the project."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.binaries"
                type="checkbox"
                class="checkbox-input"
                id="default-binaries"
                @change="handleUpdate"
              />
              <label for="default-binaries" class="checkbox-label">
                Binaries
                <InfoTooltip 
                  content="Contains executable files or other files created during compiling."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.build"
                type="checkbox"
                class="checkbox-input"
                id="default-build"
                @change="handleUpdate"
              />
              <label for="default-build" class="checkbox-label">
                Build
                <InfoTooltip 
                  content="Holds files needed for building the engine or game, including files necessary for creating platform-specific builds."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.intermediate"
                type="checkbox"
                class="checkbox-input"
                id="default-intermediate"
                @change="handleUpdate"
              />
              <label for="default-intermediate" class="checkbox-label">
                Intermediate
                <InfoTooltip 
                  content="Contains temporary files generated during building the engine or game. In game directories, Shaders are stored in the Intermediate directory."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.derived_data_cache"
                type="checkbox"
                class="checkbox-input"
                id="default-derived-data-cache"
                @change="handleUpdate"
              />
              <label for="default-derived-data-cache" class="checkbox-label">
                DerivedDataCache
                <InfoTooltip 
                  content="Contains derived data files generated on-load for referenced content. Not having cache files present for referenced content can increase load times dramatically."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.saved"
                type="checkbox"
                class="checkbox-input"
                id="default-saved"
                @change="handleUpdate"
              />
              <label for="default-saved" class="checkbox-label">
                Saved
                <InfoTooltip 
                  content="Contains autosaves, configuration (.ini) files, and log files. Additionally, the Engine>Saved directory contains crash logs, hardware information, and Swarm options and data."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.analyze_plugins"
                type="checkbox"
                class="checkbox-input"
                id="default-analyze-plugins"
                @change="handleUpdate"
              />
              <label for="default-analyze-plugins" class="checkbox-label">
                Analyze plugins
                <InfoTooltip 
                  content="Enable cleaning of temporary files within plugin directories. This will scan all plugins for cleanable files."
                />
              </label>
            </div>
          </div>
        </div>

        <!-- Plugin Scanning Options -->
        <div class="cleaning-column">
          <h5 class="column-title">Plugin Scanning</h5>
          <div class="checkbox-group">
            <div class="checkbox-item">
              <input
                v-model="localCleaning.plugin_binaries"
                type="checkbox"
                class="checkbox-input"
                id="default-plugin-binaries"
                :disabled="!localCleaning.analyze_plugins"
                @change="handleUpdate"
              />
              <label for="default-plugin-binaries" class="checkbox-label">
                Binaries
                <InfoTooltip 
                  content="Contains executable files or other files created during compiling."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.plugin_intermediate"
                type="checkbox"
                class="checkbox-input"
                id="default-plugin-intermediate"
                :disabled="!localCleaning.analyze_plugins"
                @change="handleUpdate"
              />
              <label for="default-plugin-intermediate" class="checkbox-label">
                Intermediate
                <InfoTooltip 
                  content="Contains temporary files generated during building the engine or game. In game directories, Shaders are stored in the Intermediate directory."
                />
              </label>
            </div>

            <div class="checkbox-item">
              <input
                v-model="localCleaning.plugin_node_size_cache"
                type="checkbox"
                class="checkbox-input"
                id="default-plugin-node-size-cache"
                :disabled="!localCleaning.analyze_plugins"
                @change="handleUpdate"
              />
              <label for="default-plugin-node-size-cache" class="checkbox-label">
                NodeSizeCache
                <InfoTooltip 
                  content="Cache files that store Blueprint node information for Blueprint graphs in plugins."
                />
              </label>
            </div>
          </div>

          <div v-if="!localCleaning.analyze_plugins" class="plugin-disabled-notice">
            <div class="notice-icon">ℹ️</div>
            <div class="notice-text">
              Enable "Analyze plugins" to configure plugin cleaning options.
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import InfoTooltip from '../../InfoTooltip.vue'
import { useSettingsStore } from '../../../stores/settingsStore'

const { getSettings, updateCleaningDefaults } = useSettingsStore()

const localCleaning = reactive({ ...getSettings('cleaning_defaults') })

const handleUpdate = () => {
  // If analyze_plugins is disabled, also disable plugin options
  if (!localCleaning.analyze_plugins) {
    localCleaning.plugin_binaries = false
    localCleaning.plugin_intermediate = false
    localCleaning.plugin_node_size_cache = false
  }
  
  updateCleaningDefaults(localCleaning)
}

// Watch for external changes to settings
watch(() => getSettings('cleaning_defaults'), (newCleaning) => {
  Object.assign(localCleaning, newCleaning)
}, { deep: true })
</script>

<style scoped>
.cleaning-settings {
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
  margin: 0 0 var(--spacing-xs) 0;
}

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0 0 var(--spacing-md) 0;
  line-height: var(--line-height-normal);
}

.cleaning-layout {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-lg);
}

.cleaning-column {
  display: flex;
  flex-direction: column;
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  padding: var(--spacing-md);
  background-color: var(--background-color);
}

.column-title {
  font-size: var(--font-size-sm);
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

.checkbox-input:disabled + .checkbox-label {
  opacity: 0.5;
  cursor: not-allowed;
}

.plugin-disabled-notice {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  margin-top: var(--spacing-md);
}

.notice-icon {
  font-size: var(--font-size-md);
  flex-shrink: 0;
}

.notice-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .cleaning-layout {
    grid-template-columns: 1fr;
  }
}
</style>