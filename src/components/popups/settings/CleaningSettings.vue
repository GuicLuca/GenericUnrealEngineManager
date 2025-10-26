<script setup lang="ts">
import { reactive } from 'vue'
import { useSettingsStore } from '../../../stores/settingsStore'
import InfoTooltip from '../../InfoTooltip.vue'

const { getSettings, updateCleaningDefaults } = useSettingsStore()

const localCleaning = reactive({ ...getSettings('cleaning_defaults') })

const handleUpdate = () => {
  updateCleaningDefaults(localCleaning)
}
</script>

<template>
  <div class="cleaning-settings">
    <div class="section-header">
      <h3 class="section-title">Cleaning Defaults</h3>
      <p class="section-description">Configure default options for project cleaning operations</p>
    </div>

    <div class="settings-section">
      <h4 class="subsection-title">Project Files</h4>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.ide_files"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              IDE Files (.idea, .vscode, .vs)
              <InfoTooltip content="Remove IDE-specific configuration files" />
            </span>
            <span class="setting-hint">Clean up IDE metadata and configuration</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.binaries"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              Binaries
              <InfoTooltip content="Remove compiled binaries from the Binaries folder" />
            </span>
            <span class="setting-hint">Compiled executables and libraries</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.build"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              Build
              <InfoTooltip content="Remove build receipts and metadata" />
            </span>
            <span class="setting-hint">Build system files and receipts</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.intermediate"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              Intermediate
              <InfoTooltip content="Remove intermediate build files (object files, etc.)" />
            </span>
            <span class="setting-hint">Temporary build artifacts</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.derived_data_cache"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              Derived Data Cache (DDC)
              <InfoTooltip content="Remove cached derived data (shaders, etc.)" />
            </span>
            <span class="setting-hint">Cached compiled assets</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.saved"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              Saved (Logs, Config, Crashes)
              <InfoTooltip content="Remove logs, local config, and crash reports" />
            </span>
            <span class="setting-hint">Runtime data and logs</span>
          </span>
        </label>
      </div>
    </div>

    <div class="settings-section">
      <h4 class="subsection-title">Plugin Cleaning</h4>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.analyze_plugins"
            @change="handleUpdate"
            class="checkbox-input"
          />
          <span class="setting-text">
            <span class="setting-name">
              Analyze Plugins
              <InfoTooltip content="Include plugin directories in cleaning analysis" />
            </span>
            <span class="setting-hint">Scan and clean plugin folders</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.plugin_binaries"
            @change="handleUpdate"
            class="checkbox-input"
            :disabled="!localCleaning.analyze_plugins"
          />
          <span class="setting-text">
            <span class="setting-name">
              Plugin Binaries
              <InfoTooltip content="Remove plugin binaries" />
            </span>
            <span class="setting-hint">Compiled plugin files</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.plugin_intermediate"
            @change="handleUpdate"
            class="checkbox-input"
            :disabled="!localCleaning.analyze_plugins"
          />
          <span class="setting-text">
            <span class="setting-name">
              Plugin Intermediate
              <InfoTooltip content="Remove plugin intermediate files" />
            </span>
            <span class="setting-hint">Plugin build artifacts</span>
          </span>
        </label>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input
            type="checkbox"
            v-model="localCleaning.plugin_node_size_cache"
            @change="handleUpdate"
            class="checkbox-input"
            :disabled="!localCleaning.analyze_plugins"
          />
          <span class="setting-text">
            <span class="setting-name">
              Plugin Node Size Cache
              <InfoTooltip content="Remove plugin node size cache files" />
            </span>
            <span class="setting-hint">Blueprint node cache</span>
          </span>
        </label>
      </div>
    </div>
  </div>
</template>

<style scoped>
.cleaning-settings {
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

.checkbox-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.setting-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}
</style>
