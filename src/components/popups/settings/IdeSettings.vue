<template>
  <div class="ide-settings">
    <div class="settings-section">
      <h4 class="section-title">Custom IDE Programs</h4>
      <div class="section-description">
        Add custom IDE programs that can be used to open C++ projects. These will appear in the project launch options.
      </div>
      
      <div class="programs-list">
        <div 
          v-for="(path, name) in localPrograms"
          :key="name"
          class="program-item"
        >
          <div class="program-info">
            <div class="program-name">{{ name }}</div>
            <div class="program-path">{{ path }}</div>
          </div>
          <div class="program-actions">
            <button
              class="action-btn edit-btn"
              @click="editProgram(name as string, path)"
              title="Edit program"
            >
              ✏️
            </button>
            <button
              class="action-btn remove-btn"
              @click="handleRemoveProgram(name as string)"
              title="Remove program"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="Object.keys(localPrograms).length === 0" class="no-programs">
          <div class="no-programs-icon">💻</div>
          <div class="no-programs-text">No custom IDE programs configured</div>
          <div class="no-programs-subtext">Add IDE programs to launch C++ projects</div>
        </div>
      </div>
      
      <button class="add-program-btn" @click="openAddProgramPopup">
        <span class="button-icon">➕</span>
        Add IDE Program
      </button>
    </div>

  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import { usePopup } from '../../../composables/usePopup'
import { useSettingsStore } from '../../../stores/settingsStore'

const { 
  getSettings, 
  addIdeProgram, 
  removeIdeProgram, 
  updateIdeProgram 
} = useSettingsStore()

const { showPopup } = usePopup()

const localPrograms = reactive({ ...getSettings('ide_programs').custom_programs })

const editProgram = (name: string, path: string) => {
  showPopup({
    id: 'ide-form',
    component: 'IdeForm',
    props: {
      editingProgram: name,
      initialName: name,
      initialPath: path,
      onSave: handleProgramSave
    }
  })
}

const handleRemoveProgram = (name: string) => {
  if (confirm(`Are you sure you want to remove "${name}"?`)) {
    removeIdeProgram(name)
    delete localPrograms[name]
  }
}

const openAddProgramPopup = () => {
  showPopup({
    id: 'ide-form',
    component: 'IdeForm',
    props: {
      onSave: handleProgramSave
    }
  })
}

const handleProgramSave = (data: { name: string; path: string; isEdit: boolean; originalName?: string }) => {
  if (data.isEdit && data.originalName) {
    updateIdeProgram(data.originalName, data.name, data.path)
    if (data.originalName !== data.name) {
      delete localPrograms[data.originalName]
    }
  } else {
    addIdeProgram(data.name, data.path)
  }

  localPrograms[data.name] = data.path
}

// Watch for external changes to settings
watch(() => getSettings('ide_programs').custom_programs, (newPrograms) => {
  Object.assign(localPrograms, newPrograms)
}, { deep: true })
</script>

<style scoped>
.ide-settings {
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

.programs-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.program-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm);
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
}

.program-info {
  flex-grow: 1;
  min-width: 0;
}

.program-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.program-path {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  word-break: break-all;
}

.program-actions {
  display: flex;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.action-btn {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  width: 1.75rem;
  height: 1.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn:hover {
  background-color: var(--hover-color);
}

.remove-btn:hover {
  border-color: #e53e3e;
  background-color: #fed7d7;
}

.no-programs {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-xl);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.no-programs-icon {
  font-size: var(--icon-size-lg);
  margin-bottom: var(--spacing-sm);
  opacity: 0.5;
}

.no-programs-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-programs-subtext {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  opacity: 0.7;
}

.add-program-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-fast);
  align-self: flex-start;
}

.add-program-btn:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}
</style>