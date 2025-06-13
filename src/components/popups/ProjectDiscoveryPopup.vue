<template>
  <div class="project-discovery-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">🔍</span>
        Discover Unreal Engine Projects
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <form @submit.prevent="handleSubmit" class="popup-form">
      <div class="form-section">
        <label class="form-label" for="base-folder">
          <span class="label-icon">📁</span>
          Base Folder to Search
        </label>
        <div class="folder-input-group">
          <input
            id="base-folder"
            v-model="formData.baseFolder"
            type="text"
            class="folder-input"
            placeholder="Select or enter folder path..."
            required
          />
          <button
            type="button"
            class="browse-button"
            @click="selectFolder"
            title="Browse for folder"
          >
            📂
          </button>
        </div>
      </div>

      <div class="form-section">
        <h3 class="section-title">Project Types to Ignore</h3>
        <div class="checkbox-group">
          <div class="checkbox-item">
            <input
              id="ignore-engine"
              v-model="formData.ignoreEngine"
              type="checkbox"
              class="checkbox-input"
            />
            <label for="ignore-engine" class="checkbox-label">
              Ignore Engine Projects
              <InfoTooltip 
                content="Engine projects are part of the Unreal Engine installation itself, typically found in the Engine/Samples directory. These are usually not user projects."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="ignore-templates"
              v-model="formData.ignoreTemplates"
              type="checkbox"
              class="checkbox-input"
            />
            <label for="ignore-templates" class="checkbox-label">
              Ignore Template Projects
              <InfoTooltip 
                content="Template projects are starter projects provided by Epic Games or third parties, usually found in Templates directories. These serve as starting points for new projects."
              />
            </label>
          </div>

          <div class="checkbox-item">
            <input
              id="ignore-samples"
              v-model="formData.ignoreSamples"
              type="checkbox"
              class="checkbox-input"
            />
            <label for="ignore-samples" class="checkbox-label">
              Ignore Sample Projects
              <InfoTooltip 
                content="Sample projects are demonstration projects that showcase specific features or techniques. They're typically found in Samples directories and are meant for learning purposes."
              />
            </label>
          </div>
        </div>
      </div>

      <div class="form-actions">
        <button
          type="button"
          class="cancel-button"
          @click="$emit('close')"
        >
          Cancel
        </button>
        <button
          type="submit"
          class="search-button"
          :disabled="!formData.baseFolder.trim()"
        >
          <span class="button-icon">🔍</span>
          Start Discovery
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import InfoTooltip from '../InfoTooltip.vue'

interface ProjectDiscoveryData {
  baseFolder: string
  ignoreEngine: boolean
  ignoreTemplates: boolean
  ignoreSamples: boolean
}

interface Emits {
  (e: 'close'): void
  (e: 'submit', data: ProjectDiscoveryData): void
}

const emit = defineEmits<Emits>()

const formData = reactive<ProjectDiscoveryData>({
  baseFolder: '',
  ignoreEngine: true,
  ignoreTemplates: true,
  ignoreSamples: true
})

const selectFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Base Folder for Project Discovery'
    })
    
    if (selected && typeof selected === 'string') {
      formData.baseFolder = selected
    }
  } catch (error) {
    console.error('Failed to open folder dialog:', error)
  }
}

const handleSubmit = () => {
  if (!formData.baseFolder.trim()) {
    return
  }
  
  emit('submit', { ...formData })
}
</script>

<style scoped>
.project-discovery-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 32rem;
  overflow: hidden;
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

.popup-form {
  padding: var(--spacing-lg);
}

.form-section {
  margin-bottom: var(--spacing-lg);
}

.form-section:last-of-type {
  margin-bottom: 0;
}

.form-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
}

.label-icon {
  font-size: var(--font-size-md);
}

.folder-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.folder-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  transition: border-color var(--transition-fast);
}

.folder-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
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

.browse-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
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
}

.checkbox-label {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  line-height: var(--line-height-normal);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-xl);
  padding-top: var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
}

.cancel-button,
.search-button {
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

.cancel-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.search-button {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.search-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.search-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>