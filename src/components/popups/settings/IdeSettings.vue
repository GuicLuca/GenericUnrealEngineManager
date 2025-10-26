<script setup lang="ts">
import { reactive, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useSettingsStore } from '../../../stores/settingsStore'

const { getSettings, updateSettings } = useSettingsStore()

const localIde = reactive({ ...getSettings('ide_programs') })

const selectProgram = async (programType: 'rider' | 'visual_studio' | 'visual_studio_code' | 'clion') => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: `Select ${getProgramName(programType)} executable`
    })

    if (selected) {
      localIde.custom_programs[programType] = selected
      updateSettings('ide_programs', localIde)
    }
  } catch (error) {
    console.error(`Failed to select ${programType}:`, error)
  }
}

const clearProgram = (programType: 'rider' | 'visual_studio' | 'visual_studio_code' | 'clion') => {
  localIde.custom_programs[programType] = ''
  updateSettings('ide_programs', localIde)
}

const getProgramName = (programType: string): string => {
  switch (programType) {
    case 'rider': return 'JetBrains Rider'
    case 'visual_studio': return 'Visual Studio'
    case 'visual_studio_code': return 'Visual Studio Code'
    case 'clion': return 'JetBrains CLion'
    default: return programType
  }
}

onMounted(() => {
  // Component mounted
})
</script>

<template>
  <div class="ide-settings">
    <div class="section-header">
      <h3 class="section-title">IDE Programs</h3>
      <p class="section-description">Configure paths to your preferred development environments</p>
    </div>

    <div class="settings-section">
      <div class="program-group">
        <label class="program-label">
          <span class="program-name">JetBrains Rider</span>
          <span class="program-hint">Recommended for C++ and Blueprint development</span>
        </label>
        <div class="program-input-group">
          <input
            v-model="localIde.custom_programs.rider"
            type="text"
            class="program-input"
            placeholder="Path to Rider executable..."
            readonly
          />
          <button
            type="button"
            class="browse-button"
            @click="selectProgram('rider')"
            title="Browse for Rider"
          >
            📂
          </button>
          <button
            v-if="localIde.custom_programs.rider"
            type="button"
            class="clear-button"
            @click="clearProgram('rider')"
            title="Clear path"
          >
            ✕
          </button>
        </div>
      </div>

      <div class="program-group">
        <label class="program-label">
          <span class="program-name">Visual Studio</span>
          <span class="program-hint">Microsoft's IDE for Windows development</span>
        </label>
        <div class="program-input-group">
          <input
            v-model="localIde.custom_programs.visual_studio"
            type="text"
            class="program-input"
            placeholder="Path to Visual Studio executable..."
            readonly
          />
          <button
            type="button"
            class="browse-button"
            @click="selectProgram('visual_studio')"
            title="Browse for Visual Studio"
          >
            📂
          </button>
          <button
            v-if="localIde.custom_programs.visual_studio"
            type="button"
            class="clear-button"
            @click="clearProgram('visual_studio')"
            title="Clear path"
          >
            ✕
          </button>
        </div>
      </div>

      <div class="program-group">
        <label class="program-label">
          <span class="program-name">Visual Studio Code</span>
          <span class="program-hint">Lightweight editor with Unreal Engine support</span>
        </label>
        <div class="program-input-group">
          <input
            v-model="localIde.custom_programs.visual_studio_code"
            type="text"
            class="program-input"
            placeholder="Path to VS Code executable..."
            readonly
          />
          <button
            type="button"
            class="browse-button"
            @click="selectProgram('visual_studio_code')"
            title="Browse for VS Code"
          >
            📂
          </button>
          <button
            v-if="localIde.custom_programs.visual_studio_code"
            type="button"
            class="clear-button"
            @click="clearProgram('visual_studio_code')"
            title="Clear path"
          >
            ✕
          </button>
        </div>
      </div>

      <div class="program-group">
        <label class="program-label">
          <span class="program-name">JetBrains CLion</span>
          <span class="program-hint">Cross-platform C++ IDE</span>
        </label>
        <div class="program-input-group">
          <input
            v-model="localIde.custom_programs.clion"
            type="text"
            class="program-input"
            placeholder="Path to CLion executable..."
            readonly
          />
          <button
            type="button"
            class="browse-button"
            @click="selectProgram('clion')"
            title="Browse for CLion"
          >
            📂
          </button>
          <button
            v-if="localIde.custom_programs.clion"
            type="button"
            class="clear-button"
            @click="clearProgram('clion')"
            title="Clear path"
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ide-settings {
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
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.program-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.program-label {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.program-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.program-hint {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: var(--line-height-normal);
}

.program-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.program-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  cursor: pointer;
}

.program-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.browse-button,
.clear-button {
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

.browse-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.clear-button {
  color: var(--text-secondary);
}

.clear-button:hover {
  background-color: var(--hover-color);
  border-color: #dc3545;
  color: #dc3545;
}
</style>
