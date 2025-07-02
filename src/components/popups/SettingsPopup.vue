<template>
  <div class="settings-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">⚙️</span>
        Settings
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <div class="settings-layout">
        <!-- Sidebar Navigation -->
        <div class="settings-sidebar">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            class="tab-button"
            :class="{ active: activeTab === tab.id }"
            @click="activeTab = tab.id"
          >
            <span class="tab-icon">{{ tab.icon }}</span>
            <span class="tab-label">{{ tab.label }}</span>
          </button>
        </div>

        <!-- Content Area -->
        <div class="settings-content">
          <!-- General Tab -->
          <div v-if="activeTab === 'general'" class="tab-content">
            <h3 class="section-title">General</h3>
            
            <div class="general-section">
              <div class="setting-item">
                <div class="setting-info">
                  <div class="setting-label">Autostart</div>
                  <div class="setting-description">Start UE Project Manager automatically when you log in</div>
                </div>
                <div class="setting-control">
                  <input
                    v-model="localSettings.general.autostart_enabled"
                    type="checkbox"
                    class="checkbox-input"
                  />
                </div>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <div class="setting-label">Show Welcome Popup</div>
                  <div class="setting-description">Show the welcome message when the application starts</div>
                </div>
                <div class="setting-control">
                  <input
                    v-model="localSettings.general.show_welcome_popup"
                    type="checkbox"
                    class="checkbox-input"
                  />
                </div>
              </div>
            </div>
          </div>

          <!-- Programs Tab -->
          <div v-if="activeTab === 'programs'" class="tab-content">
            <h3 class="section-title">Programs</h3>
            
            <!-- IDE Programs Section -->
            <div class="programs-section">
              <div class="section-header">
                <h4 class="subsection-title">IDE Programs</h4>
                <button class="add-button" @click="addIdeProgram">
                  <span class="button-icon">➕</span>
                  Add IDE
                </button>
              </div>
              
              <div v-if="Object.keys(localSettings.ide_programs.custom_programs).length === 0" class="empty-state">
                <div class="empty-icon">💻</div>
                <div class="empty-text">No IDE programs configured</div>
                <div class="empty-subtext">Add your preferred IDE programs for opening C++ projects</div>
              </div>
              
              <div v-else class="programs-list">
                <div 
                  v-for="(_path, name) in sortedIdePrograms"
                  :key="name"
                  class="program-item"
                >
                  <div class="program-info">
                    <input
                      v-model="localSettings.ide_programs.custom_programs[name]"
                      type="text"
                      class="program-path"
                      :placeholder="'Path to ' + name"
                    />
                  </div>
                  <button 
                    class="remove-button"
                    @click="removeIdeProgram(name as string)"
                    title="Remove IDE program"
                  >
                    🗑️
                  </button>
                </div>
              </div>
            </div>

            <div class="section-divider"></div>

            <!-- Engine Programs Section -->
            <div class="programs-section">
              <div class="section-header">
                <h4 class="subsection-title">Engines</h4>
                <div class="engine-actions">
                  <button class="detect-button" @click="openEngineDetection">
                    <span class="button-icon">🔍</span>
                    Auto-Detect
                  </button>
                  <button class="add-button" @click="addEngineProgram">
                    <span class="button-icon">➕</span>
                    Add Engine
                  </button>
                </div>
              </div>
              
              <div v-if="Object.keys(localSettings.engine_programs.custom_engines).length === 0" class="empty-state">
                <div class="empty-icon">⚙️</div>
                <div class="empty-text">No custom engines configured</div>
                <div class="empty-subtext">Add custom Unreal Engine installations or use auto-detect to find them automatically</div>
              </div>
              
              <div v-else class="programs-list">
                <div 
                  v-for="(_path, name) in sortedEnginePrograms"
                  :key="name"
                  class="program-item"
                >
                  <div class="program-info">
                    <input
                      v-model="localSettings.engine_programs.custom_engines[name]"
                      type="text"
                      class="program-path"
                      :placeholder="'Path to ' + name"
                    />
                  </div>
                  <button 
                    class="remove-button"
                    @click="removeEngineProgram(name as string)"
                    title="Remove engine"
                  >
                    🗑️
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Cleaning Tab -->
          <div v-if="activeTab === 'cleaning'" class="tab-content">
            <h3 class="section-title">Cleaning Defaults</h3>
            
            <div class="cleaning-section">
              <div class="cleaning-subsection">
                <h4 class="subsection-title">Project Scanning</h4>
                <div class="checkbox-group">
                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.ide_files"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      IDE files (.vs and .idea)
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.binaries"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Binaries
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.build"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Build
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.intermediate"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Intermediate
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.derived_data_cache"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      DerivedDataCache
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.saved"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Saved
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.analyze_plugins"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Analyze plugins
                    </label>
                  </div>
                </div>
              </div>

              <div v-if="localSettings.cleaning_defaults.analyze_plugins" class="cleaning-subsection">
                <h4 class="subsection-title">Plugins Scanning</h4>
                <div class="checkbox-group">
                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.plugin_binaries"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Binaries
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.plugin_intermediate"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      Intermediate
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      v-model="localSettings.cleaning_defaults.plugin_node_size_cache"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label class="checkbox-label">
                      NodeSizeCache
                    </label>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Compression Tab -->
          <div v-if="activeTab === 'compression'" class="tab-content">
            <h3 class="section-title">Compression</h3>
            
            <div class="compression-section">
              <div class="format-section">
                <h4 class="subsection-title">Filename Format</h4>
                
                <div class="format-input-group">
                  <input
                    v-model="localSettings.compression.filename_format"
                    type="text"
                    class="format-input"
                    placeholder="Enter filename format..."
                    @input="validateFormat"
                  />
                  <div class="format-preview-small">
                    <span class="preview-label">Preview:</span>
                    <span class="preview-filename">{{ formatPreview }}</span>
                  </div>
                </div>
                
                <div v-if="formatWarning" class="format-warning">
                  ⚠️ {{ formatWarning }}
                </div>
                
                <div class="format-tags">
                  <div class="tags-header">
                    <span class="tags-title">Available Tags:</span>
                  </div>
                  <div class="tags-grid">
                    <button
                      v-for="tag in availableTags"
                      :key="tag.name"
                      class="tag-button"
                      @click="insertTag(tag.name)"
                      :title="tag.description"
                      @mouseenter="showTagTooltip($event, tag.description)"
                      @mouseleave="hideTagTooltip"
                    >
                      [{{ tag.name }}]
                    </button>
                  </div>
                </div>
              </div>

              <div class="section-divider"></div>

              <div class="presets-section">
                <div class="presets-header">
                  <h4 class="subsection-title">Saved Presets</h4>
                  <button class="save-preset-button" @click="saveCurrentAsPreset">
                    <span class="button-icon">💾</span>
                    Save Current
                  </button>
                </div>
                
                <div v-if="Object.keys(localSettings.compression.custom_presets).length === 0" class="empty-state">
                  <div class="empty-icon">📋</div>
                  <div class="empty-text">No saved presets</div>
                  <div class="empty-subtext">Save your current format as a preset for quick access</div>
                </div>
                
                <div v-else class="presets-list">
                  <div 
                    v-for="(format, name) in sortedPresets"
                    :key="name"
                    class="preset-item"
                  >
                    <div class="preset-info">
                      <div class="preset-name">{{ name }}</div>
                      <div class="preset-format">{{ format }}</div>
                    </div>
                    <div class="preset-actions">
                      <button 
                        class="use-button"
                        @click="usePreset(format)"
                        title="Use this preset"
                      >
                        ↰
                      </button>
                      <button 
                        class="remove-button"
                        @click="removePreset(name as string)"
                        title="Delete preset"
                      >
                        🗑️
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="popup-actions">
      <button class="cancel-button" @click="$emit('close')">
        Cancel
      </button>
      <button class="save-button" @click="saveSettings" :disabled="isSaving">
        <span class="button-icon">{{ isSaving ? '⏳' : '💾' }}</span>
        {{ isSaving ? 'Saving...' : 'Save Settings' }}
      </button>
    </div>

    <!-- Tag Tooltip -->
    <Teleport to="body">
      <div 
        v-if="tagTooltip.visible" 
        class="tag-tooltip" 
        :style="tagTooltip.style"
      >
        {{ tagTooltip.content }}
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {ref, reactive, computed, onMounted, nextTick, onUnmounted} from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogStore } from '../../stores/logStore'
import { usePopup } from '../../composables/usePopup'

interface AppSettings {
  ide_programs: {
    custom_programs: Record<string, string>
  }
  engine_programs: {
    custom_engines: Record<string, string>
  }
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
  general: {
    autostart_enabled: boolean
    show_welcome_popup: boolean
  }
  compression: {
    filename_format: string
    custom_presets: Record<string, string>
  }
}

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()
const { showPopup } = usePopup()

const activeTab = ref('general') // The default active tab is the General tab
const isSaving = ref(false)
const formatWarning = ref('')
const systemUsername = ref('john_doe') // Fallback username
const systemHostname = ref('DESKTOP-PC') // Fallback hostname

const tabs = [
  { id: 'general', label: 'General', icon: '⚙️' },
  { id: 'programs', label: 'Programs', icon: '💻' },
  { id: 'cleaning', label: 'Cleaning', icon: '🧹' },
  { id: 'compression', label: 'Compression', icon: '🗜️' }
]

const availableTags = [
  { name: 'Project', description: 'Project name' },
  { name: 'Type', description: 'Project type (Cpp or Bp)' },
  { name: 'Engine', description: 'Engine version (e.g., 5-4-2)' },
  { name: 'YYYY', description: 'Full year (e.g., 2024)' },
  { name: 'YY', description: '2-digit year (e.g., 24)' },
  { name: 'MM', description: 'Month (01-12)' },
  { name: 'DD', description: 'Day (01-31)' },
  { name: 'HH', description: 'Hour (00-23)' },
  { name: 'mm', description: 'Minutes (00-59)' },
  { name: 'ss', description: 'Seconds (00-59)' },
  { name: 'Month', description: 'Full month name (e.g., January)' },
  { name: 'Mon', description: 'Short month name (e.g., Jan)' },
  { name: 'Day', description: 'Full day name (e.g., Monday)' },
  { name: 'Weekday', description: 'Short day name (e.g., Mon)' },
  { name: 'User', description: 'System username' },
  { name: 'Computer', description: 'Computer hostname' },
  { name: 'SizeMB', description: 'Project size in MB' },
  { name: 'SizeGB', description: 'Project size in GB' },
  { name: 'PluginCount', description: 'Number of plugins' },
  { name: 'Algorithm', description: 'Compression algorithm' },
  { name: 'Timestamp', description: 'Unix timestamp' }
]

const localSettings = reactive<AppSettings>({
  ide_programs: { custom_programs: {} },
  engine_programs: { custom_engines: {} },
  cleaning_defaults: {
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
  },
  general: {
    autostart_enabled: false,
    show_welcome_popup: true
  },
  compression: {
    filename_format: '[Project]_[YYYY][MM][DD][HH][mm]',
    custom_presets: {}
  }
})

// Tag tooltip state
const tagTooltip = reactive({
  visible: false,
  content: '',
  style: {}
})

// Computed properties for sorted lists
const sortedIdePrograms = computed(() => {
  const entries = Object.entries(localSettings.ide_programs.custom_programs)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

const sortedEnginePrograms = computed(() => {
  const entries = Object.entries(localSettings.engine_programs.custom_engines)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

const sortedPresets = computed(() => {
  const entries = Object.entries(localSettings.compression.custom_presets)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

const loadSystemInfo = async () => {
  try {
    systemUsername.value = await invoke('get_system_username') as string
    systemHostname.value = await invoke('get_system_hostname') as string
  } catch (error) {
    console.error('Failed to load system info:', error)
    // Keep fallback values
  }
}

// Generate the preview of the current format
const formatPreview = computed(() => {
  const now = new Date()
  let preview = localSettings.compression.filename_format
  
  const replacements: Record<string, string> = {
    'Project': 'MyProject',
    'Type': 'Cpp',
    'Engine': '5-4-2',
    'YYYY': now.getFullYear().toString(),
    'YY': now.getFullYear().toString().slice(-2),
    'MM': (now.getMonth() + 1).toString().padStart(2, '0'),
    'DD': now.getDate().toString().padStart(2, '0'),
    'HH': now.getHours().toString().padStart(2, '0'),
    'mm': now.getMinutes().toString().padStart(2, '0'),
    'ss': now.getSeconds().toString().padStart(2, '0'),
    'Month': now.toLocaleDateString('en-US', {month: 'long'}),
    'Mon': now.toLocaleDateString('en-US', {month: 'short'}),
    'Day': now.toLocaleDateString('en-US', {weekday: 'long'}),
    'Weekday': now.toLocaleDateString('en-US', {weekday: 'short'}),
    'User': systemUsername.value,
    'Computer': systemHostname.value,
    'SizeMB': '1024',
    'SizeGB': '1',
    'PluginCount': '5',
    'Algorithm': 'ZIP',
    'Timestamp': Math.floor(now.getTime() / 1000).toString()
  }
  
  for (const [key, value] of Object.entries(replacements)) {
    preview = preview.replace(new RegExp(`\\[${key}\\]`, 'g'), value)
  }
  
  return preview + '.zip'
})

const loadSettings = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    Object.assign(localSettings, settings)
  } catch (error) {
    console.error('Failed to load settings:', error)
    addLog('Failed to load settings', 'error')
  }
}

const saveSettings = async () => {
  try {
    isSaving.value = true
    await invoke('save_settings', { settings: localSettings })
    addLog('Settings saved successfully')
    emit('close')
  } catch (error) {
    console.error('Failed to save settings:', error)
    addLog('Failed to save settings', 'error')
  } finally {
    isSaving.value = false
  }
}

const addIdeProgram = () => {
  const name = prompt('Enter IDE program name:')
  if (name && name.trim()) {
    localSettings.ide_programs.custom_programs[name.trim()] = ''
  }
}

const removeIdeProgram = (name: string) => {
  delete localSettings.ide_programs.custom_programs[name]
}

const addEngineProgram = () => {
  const name = prompt('Enter engine name:')
  if (name && name.trim()) {
    localSettings.engine_programs.custom_engines[name.trim()] = ''
  }
}

const removeEngineProgram = (name: string) => {
  delete localSettings.engine_programs.custom_engines[name]
}

const openEngineDetection = () => {
  showPopup({
    id: 'engine-detection',
    component: 'EngineDetection',
    props: {}
  })
}

const insertTag = (tagName: string) => {
  const tag = `[${tagName}]`
  localSettings.compression.filename_format += tag
  validateFormat()
}

const validateFormat = () => {
  const format = localSettings.compression.filename_format
  const tagPattern = /\[([^\]]+)\]/g
  const validTags = availableTags.map(tag => tag.name)
  const matches = [...format.matchAll(tagPattern)]
  
  const invalidTags = matches
    .map(match => match[1])
    .filter(tag => !validTags.includes(tag))
  
  if (invalidTags.length > 0) {
    formatWarning.value = `Unknown tags: ${invalidTags.map(tag => `[${tag}]`).join(', ')}`
  } else {
    formatWarning.value = ''
  }
}

const saveCurrentAsPreset = () => {
  const name = prompt('Enter preset name:')
  if (name && name.trim()) {
    localSettings.compression.custom_presets[name.trim()] = localSettings.compression.filename_format
  }
}

const usePreset = (format: string) => {
  localSettings.compression.filename_format = format
  validateFormat()
}

const removePreset = (name: string) => {
  delete localSettings.compression.custom_presets[name]
}

const showTagTooltip = async (event: MouseEvent, content: string) => {
  tagTooltip.content = content
  tagTooltip.visible = true
  
  await nextTick()
  
  const rect = (event.target as HTMLElement).getBoundingClientRect()
  tagTooltip.style = {
    position: 'fixed',
    left: `${rect.left + rect.width / 2}px`,
    top: `${rect.top - 8}px`,
    transform: 'translate(-50%, -100%)',
    zIndex: 10001
  }
}

const hideTagTooltip = () => {
  tagTooltip.visible = false
}

onMounted(() => {
  loadSystemInfo()
  loadSettings()
})

onUnmounted(() => {
  // Save settings on unmounting
  saveSettings().catch(error => {
    console.error('Error saving settings on unmount:', error)
    addLog('Error saving settings on unmount', 'error')
  })
})
</script>

<style scoped>
.settings-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 56rem;
  min-width: 56rem;
  height: 42rem;
  max-height: 42rem;
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
  flex-shrink: 0;
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
  overflow: hidden;
}

.settings-layout {
  display: flex;
  height: 100%;
}

.settings-sidebar {
  width: 12rem;
  background-color: var(--surface-color);
  border-right: var(--border-width) solid var(--border-color);
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border: none;
  background: none;
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  text-align: left;
}

.tab-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.tab-button.active {
  background-color: var(--accent-color-alpha);
  color: var(--accent-color);
  font-weight: var(--font-weight-medium);
}

.tab-icon {
  font-size: var(--font-size-md);
}

.settings-content {
  flex-grow: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.tab-content {
  max-width: 100%;
}

.section-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-lg) 0;
}

.programs-section,
.compression-section,
.general-section {
  margin-bottom: var(--spacing-lg);
}

.cleaning-section {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-lg);
}

.cleaning-subsection {
  display: flex;
  flex-direction: column;
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
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

.checkbox-label {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  line-height: var(--line-height-normal);
  flex-grow: 1;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md);
}

.subsection-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.engine-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.add-button,
.detect-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
}

.add-button:hover,
.detect-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.detect-button {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
  color: white;
}

.detect-button:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.button-icon {
  font-size: var(--font-size-sm);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-xl);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
}

.empty-icon {
  font-size: var(--icon-size-xl);
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.empty-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.empty-subtext {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  opacity: 0.7;
}

.programs-list,
.presets-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.program-item,
.preset-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
}

.program-info,
.preset-info {
  flex-grow: 1;
  min-width: 0;
}

.program-path {
  width: 100%;
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
}

.program-path:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.preset-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.preset-format {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.preset-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.use-button,
.remove-button {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
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

.use-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.remove-button:hover {
  border-color: #e53e3e;
  background-color: #fed7d7;
}

.section-divider {
  height: var(--border-width);
  background-color: var(--border-color);
  margin: var(--spacing-md) 0;
}

.format-section {
  margin-bottom: var(--spacing-lg);
}

.format-input-group {
  margin-bottom: var(--spacing-sm);
}

.format-input {
  width: 100%;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  margin-bottom: var(--spacing-xs);
}

.format-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.format-preview-small {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
  font-size: var(--font-size-xs);
}

.preview-label {
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
  flex-shrink: 0;
}

.preview-filename {
  color: var(--text-primary);
  font-family: var(--font-mono);
  word-break: break-all;
  flex-grow: 1;
}

.format-warning {
  color: #d69e2e;
  font-size: var(--font-size-sm);
  font-style: italic;
  margin-bottom: var(--spacing-sm);
}

.format-tags {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
}

.tags-header {
  margin-bottom: var(--spacing-sm);
}

.tags-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
  gap: var(--spacing-xs);
}

.tag-button {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-xs);
  color: var(--text-primary);
  transition: all var(--transition-fast);
  font-family: var(--font-mono);
  text-align: center;
}

.tag-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.presets-section {
  margin-bottom: var(--spacing-lg);
}

.presets-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md);
}

.save-preset-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
}

.save-preset-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  margin-bottom: var(--spacing-sm);
}

.setting-info {
  flex-grow: 1;
}

.setting-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.setting-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.setting-control {
  flex-shrink: 0;
}

.checkbox-input {
  width: 1.25rem;
  height: 1.25rem;
  accent-color: var(--accent-color);
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
.save-button {
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

.save-button {
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
}

.save-button:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.save-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tag-tooltip {
  background-color: var(--text-primary);
  color: var(--background-color);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  white-space: nowrap;
  box-shadow: var(--shadow-md);
  pointer-events: none;
  max-width: 20rem;
}

.tag-tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 4px solid transparent;
  border-top-color: var(--text-primary);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .settings-popup {
    min-width: 90vw;
    max-width: 90vw;
    height: 90vh;
    max-height: 90vh;
  }
  
  .settings-layout {
    flex-direction: column;
  }
  
  .settings-sidebar {
    width: 100%;
    flex-direction: row;
    overflow-x: auto;
  }
  
  .tags-grid {
    grid-template-columns: repeat(auto-fill, minmax(4rem, 1fr));
  }
  
  .cleaning-section {
    grid-template-columns: 1fr;
  }
}
</style>