<template>
  <div class="settings-popup">
    <div class="popup-header">
      <div class="header-content">
        <h2 class="popup-title">
          <span class="title-icon">⚙️</span>
          Settings
        </h2>
      </div>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <!-- Tabs Navigation -->
      <div class="tabs-nav">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          class="tab-button"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >
          <span class="tab-icon">{{ tab.icon }}</span>
          <span class="tab-title">{{ tab.title }}</span>
        </button>
      </div>

      <!-- Tab Content Container with Fixed Height -->
      <div class="tab-content-container">
        <!-- General Tab -->
        <div v-show="activeTab === 'general'" class="tab-panel">
          <div class="settings-section">
            <h3 class="section-title">Application Startup</h3>
            <div class="section-description">
              Configure how the application behaves when starting up.
            </div>

            <div class="setting-item">
              <div class="setting-header">
                <label class="setting-label">
                  <input
                    v-model="localSettings.general.autostart_enabled"
                    type="checkbox"
                    class="setting-checkbox"
                    @change="handleAutostartChange"
                  />
                  <span class="setting-text">Start automatically when I log in</span>
                </label>
              </div>
              <div class="setting-description">
                The application will start automatically when you log into your computer.
              </div>
            </div>

            <div class="setting-item">
              <div class="setting-header">
                <label class="setting-label">
                  <input
                    v-model="localSettings.general.show_welcome_popup"
                    type="checkbox"
                    class="setting-checkbox"
                    @change="handleWelcomePopupChange"
                  />
                  <span class="setting-text">Show welcome message on startup</span>
                </label>
              </div>
              <div class="setting-description">
                Display the welcome popup when the application starts for the first time.
              </div>
            </div>
          </div>
        </div>

        <!-- Programs Tab -->
        <div v-show="activeTab === 'programs'" class="tab-panel">
          <!-- IDE Programs Subsection -->
          <div class="settings-section">
            <div class="section-header">
              <h3 class="section-title">IDE Programs</h3>
              <button class="add-button" @click="addCustomIdeProgram">
                <span class="button-icon">➕</span>
                Add IDE Program
              </button>
            </div>
            <div class="section-description">
              Configure your preferred IDEs for opening C++ projects. Add any IDE or code editor you want to use.
            </div>

            <div class="custom-programs">
              <div class="custom-program-list">
                <div 
                  v-for="(_path, name) in localSettings.ide_programs.custom_programs"
                  :key="name"
                  class="custom-program-item"
                >
                  <div class="program-icon">
                    <img 
                      v-if="programIcons[name]" 
                      :src="programIcons[name]" 
                      :alt="name"
                      class="icon-image"
                      @error="handleIconError(name)"
                    />
                    <span v-else class="fallback-icon">⚙️</span>
                  </div>
                  <input
                    v-model="customProgramNames[name]"
                    type="text"
                    class="custom-name-input"
                    placeholder="Program name..."
                    @blur="updateCustomProgramName(name, customProgramNames[name])"
                  />
                  <input
                    v-model="localSettings.ide_programs.custom_programs[name]"
                    type="text"
                    class="custom-path-input"
                    placeholder="Path to executable..."
                    @change="extractIcon(name, localSettings.ide_programs.custom_programs[name])"
                  />
                  <button
                    class="browse-button"
                    @click="browseForCustomIde(name)"
                    title="Browse for executable"
                  >
                    📂
                  </button>
                  <button
                    class="remove-button"
                    @click="removeCustomProgram(name)"
                    title="Remove program"
                  >
                    🗑️
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Engines Subsection -->
          <div class="settings-section">
            <div class="section-header">
              <h3 class="section-title">Engines</h3>
              <button class="add-button" @click="addCustomEngine">
                <span class="button-icon">➕</span>
                Add Engine
              </button>
            </div>
            <div class="section-description">
              Configure custom Unreal Engine installations. The path must point to the engine folder containing Engine, Samples, and Templates directories.
            </div>

            <div class="custom-programs">
              <div class="custom-program-list">
                <div 
                  v-for="(_path, name) in localSettings.engine_programs.custom_engines"
                  :key="name"
                  class="custom-program-item"
                >
                  <div class="program-icon">
                    <span class="fallback-icon">🎮</span>
                  </div>
                  <input
                    v-model="customEngineNames[name]"
                    type="text"
                    class="custom-name-input"
                    placeholder="Engine name..."
                    @blur="updateCustomEngineName(name, customEngineNames[name])"
                  />
                  <input
                    v-model="localSettings.engine_programs.custom_engines[name]"
                    type="text"
                    class="custom-path-input"
                    placeholder="Path to engine folder..."
                  />
                  <button
                    class="browse-button"
                    @click="browseForCustomEngine(name)"
                    title="Browse for engine folder"
                  >
                    📂
                  </button>
                  <button
                    class="remove-button"
                    @click="removeCustomEngine(name)"
                    title="Remove engine"
                  >
                    🗑️
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Compression Tab -->
        <div v-show="activeTab === 'compression'" class="tab-panel">
          <div class="settings-section">
            <h3 class="section-title">Filename Format</h3>
            <div class="section-description">
              Customize how compressed archive filenames are generated. Use formatting tags in square brackets to insert dynamic values.
            </div>

            <div class="format-editor">
              <div class="format-input-section">
                <label class="format-label">Filename Format Template:</label>
                <div class="format-input-wrapper">
                  <input
                    v-model="localSettings.compression.filename_format"
                    type="text"
                    class="format-input"
                    placeholder="[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]"
                    @input="updatePreview"
                  />
                  <button
                    class="reset-format-button"
                    @click="resetToDefault"
                    title="Reset to default format"
                  >
                    🔄
                  </button>
                </div>
              </div>

              <div class="format-preview">
                <div class="preview-label">Preview:</div>
                <div class="preview-output">{{ formatPreview }}</div>
              </div>

              <div class="format-tags">
                <h4 class="tags-title">Available Format Tags</h4>
                <div class="tags-grid">
                  <div class="tag-category">
                    <h5 class="category-title">Project Information</h5>
                    <div class="tag-list">
                      <button 
                        v-for="tag in projectTags" 
                        :key="tag.name"
                        class="tag-button"
                        @click="insertTag(tag.name)"
                        :title="tag.description"
                      >
                        <span class="tag-name">[{{ tag.name }}]</span>
                        <span class="tag-desc">{{ tag.description }}</span>
                      </button>
                    </div>
                  </div>

                  <div class="tag-category">
                    <h5 class="category-title">Date & Time</h5>
                    <div class="tag-list">
                      <button 
                        v-for="tag in dateTags" 
                        :key="tag.name"
                        class="tag-button"
                        @click="insertTag(tag.name)"
                        :title="tag.description"
                      >
                        <span class="tag-name">[{{ tag.name }}]</span>
                        <span class="tag-desc">{{ tag.description }}</span>
                      </button>
                    </div>
                  </div>

                  <div class="tag-category">
                    <h5 class="category-title">System Information</h5>
                    <div class="tag-list">
                      <button 
                        v-for="tag in systemTags" 
                        :key="tag.name"
                        class="tag-button"
                        @click="insertTag(tag.name)"
                        :title="tag.description"
                      >
                        <span class="tag-name">[{{ tag.name }}]</span>
                        <span class="tag-desc">{{ tag.description }}</span>
                      </button>
                    </div>
                  </div>
                </div>
              </div>

              <div class="format-presets">
                <h4 class="presets-title">Quick Presets</h4>
                <div class="preset-buttons">
                  <button 
                    v-for="preset in formatPresets" 
                    :key="preset.name"
                    class="preset-button"
                    @click="applyPreset(preset.format)"
                    :title="preset.description"
                  >
                    <span class="preset-name">{{ preset.name }}</span>
                    <span class="preset-format">{{ preset.format }}</span>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Cleaning Defaults Tab -->
        <div v-show="activeTab === 'cleaning'" class="tab-panel">
          <div class="settings-section">
            <h3 class="section-title">Cleaning Default Selection</h3>
            <div class="section-description">
              Set the default selection for the clean project popup.
            </div>

            <div class="cleaning-defaults-grid">
              <!-- Project Scanning Column -->
              <div class="defaults-column">
                <h4 class="subsection-title">Project Scanning</h4>
                <div class="checkbox-group">
                  <div class="checkbox-item">
                    <input
                      id="default-ide-files"
                      v-model="localSettings.cleaning_defaults.ide_files"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-ide-files" class="checkbox-label">
                      IDE files (.vs and .idea)
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-binaries"
                      v-model="localSettings.cleaning_defaults.binaries"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-binaries" class="checkbox-label">
                      Binaries
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-build"
                      v-model="localSettings.cleaning_defaults.build"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-build" class="checkbox-label">
                      Build
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-intermediate"
                      v-model="localSettings.cleaning_defaults.intermediate"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-intermediate" class="checkbox-label">
                      Intermediate
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-derived-data-cache"
                      v-model="localSettings.cleaning_defaults.derived_data_cache"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-derived-data-cache" class="checkbox-label">
                      DerivedDataCache
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-saved"
                      v-model="localSettings.cleaning_defaults.saved"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-saved" class="checkbox-label">
                      Saved
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-analyze-plugins"
                      v-model="localSettings.cleaning_defaults.analyze_plugins"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-analyze-plugins" class="checkbox-label">
                      Analyze plugins
                    </label>
                  </div>
                </div>
              </div>

              <!-- Plugins Scanning Column -->
              <div class="defaults-column">
                <h4 class="subsection-title">Plugins Scanning</h4>
                <div class="checkbox-group">
                  <div class="checkbox-item">
                    <input
                      id="default-plugin-binaries"
                      v-model="localSettings.cleaning_defaults.plugin_binaries"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-plugin-binaries" class="checkbox-label">
                      Binaries
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-plugin-intermediate"
                      v-model="localSettings.cleaning_defaults.plugin_intermediate"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-plugin-intermediate" class="checkbox-label">
                      Intermediate
                    </label>
                  </div>

                  <div class="checkbox-item">
                    <input
                      id="default-plugin-node-size-cache"
                      v-model="localSettings.cleaning_defaults.plugin_node_size_cache"
                      type="checkbox"
                      class="checkbox-input"
                    />
                    <label for="default-plugin-node-size-cache" class="checkbox-label">
                      NodeSizeCache
                    </label>
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
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
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
  }
}

interface Tab {
  id: string
  title: string
  icon: string
}

interface FormatTag {
  name: string
  description: string
}

interface FormatPreset {
  name: string
  format: string
  description: string
}

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { addLog } = useLogStore()
const { showPopup } = usePopup()

const activeTab = ref('general')
const isSaving = ref(false)
const customProgramNames = ref<Record<string, string>>({})
const customEngineNames = ref<Record<string, string>>({})
const programIcons = ref<Record<string, string>>({})

const tabs: Tab[] = [
  { id: 'general', title: 'General', icon: '🏠' },
  { id: 'programs', title: 'Programs', icon: '💻' },
  { id: 'compression', title: 'Compression', icon: '🗜️' },
  { id: 'cleaning', title: 'Cleaning Defaults', icon: '🧹' }
]

const localSettings = reactive<AppSettings>({
  ide_programs: {
    custom_programs: {}
  },
  engine_programs: {
    custom_engines: {}
  },
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
    filename_format: '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]'
  }
})

// Format tags organized by category
const projectTags: FormatTag[] = [
  { name: 'Project', description: 'Project name' },
  { name: 'Type', description: 'Cpp or Bp' },
  { name: 'Engine', description: 'Engine version (5-4-2)' },
  { name: 'SizeMB', description: 'Project size in MB' },
  { name: 'SizeGB', description: 'Project size in GB' },
  { name: 'PluginCount', description: 'Number of plugins' },
  { name: 'Algorithm', description: 'Compression algorithm' }
]

const dateTags: FormatTag[] = [
  { name: 'YYYY', description: 'Full year (2024)' },
  { name: 'YY', description: 'Short year (24)' },
  { name: 'MM', description: 'Month number (01-12)' },
  { name: 'DD', description: 'Day number (01-31)' },
  { name: 'HH', description: 'Hour (00-23)' },
  { name: 'mm', description: 'Minutes (00-59)' },
  { name: 'ss', description: 'Seconds (00-59)' },
  { name: 'Month', description: 'Full month name' },
  { name: 'Mon', description: 'Short month name' },
  { name: 'Day', description: 'Full day name' },
  { name: 'Weekday', description: 'Short day name' }
]

const systemTags: FormatTag[] = [
  { name: 'User', description: 'Current username' },
  { name: 'Computer', description: 'Computer hostname' },
  { name: 'Timestamp', description: 'Unix timestamp' }
]

const formatPresets: FormatPreset[] = [
  {
    name: 'Default',
    format: '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
    description: 'Project name with date and time'
  },
  {
    name: 'Simple',
    format: '[Project]_[Type]',
    description: 'Just project name and type'
  },
  {
    name: 'Detailed',
    format: '[Project]_[Engine]_[Type]_[YYYY][MM][DD]_[HH][mm]',
    description: 'Project with engine version and compact timestamp'
  },
  {
    name: 'Archive Style',
    format: '[YYYY]-[MM]-[DD]_[Project]_[Algorithm]',
    description: 'Date first, then project and compression type'
  },
  {
    name: 'User Specific',
    format: '[User]_[Project]_[Mon][DD]_[HH][mm]',
    description: 'Include username with short date format'
  }
]

// Generate preview of the current format
const formatPreview = computed(() => {
  const now = new Date()
  const mockProject = 'MyAwesomeGame'
  
  let preview = localSettings.compression.filename_format
  
  // Replace common tags with example values
  const replacements: Record<string, string> = {
    'Project': mockProject,
    'Type': 'Cpp',
    'Engine': '5-4-2',
    'SizeMB': '1024',
    'SizeGB': '1',
    'PluginCount': '5',
    'Algorithm': 'ZIP',
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
    'User': 'john_doe',
    'Computer': 'DESKTOP-PC',
    'Timestamp': Math.floor(now.getTime() / 1000).toString()
  }
  
  for (const [key, value] of Object.entries(replacements)) {
    preview = preview.replace(new RegExp(`\\[${key}\\]`, 'g'), value)
  }
  
  // Add extension if not present
  if (!preview.includes('.')) {
    preview += '.zip'
  }
  
  return preview
})

const loadSettings = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    
    // Update local settings
    localSettings.ide_programs.custom_programs = { ...settings.ide_programs.custom_programs }
    localSettings.engine_programs.custom_engines = { ...(settings.engine_programs?.custom_engines || {}) }
    localSettings.cleaning_defaults = { ...settings.cleaning_defaults }
    localSettings.general = { ...settings.general }
    localSettings.compression = { ...settings.compression }
    
    // Initialize custom program names
    Object.keys(localSettings.ide_programs.custom_programs).forEach(name => {
      customProgramNames.value[name] = name
      extractIcon(name, localSettings.ide_programs.custom_programs[name])
    })
    
    // Initialize custom engine names
    Object.keys(localSettings.engine_programs.custom_engines).forEach(name => {
      customEngineNames.value[name] = name
    })
    
  } catch (error) {
    console.error('Failed to load settings:', error)
    addLog('Failed to load settings', 'error')
  }
}

const saveSettings = async () => {
  try {
    isSaving.value = true
    
    // Clean up empty values
    const settingsToSave: AppSettings = {
      ide_programs: {
        custom_programs: { ...localSettings.ide_programs.custom_programs }
      },
      engine_programs: {
        custom_engines: { ...localSettings.engine_programs.custom_engines }
      },
      cleaning_defaults: { ...localSettings.cleaning_defaults },
      general: { ...localSettings.general },
      compression: { ...localSettings.compression }
    }
    
    await invoke('save_settings', { settings: settingsToSave })
    addLog('Settings saved successfully')
    emit('close')
    
  } catch (error) {
    console.error('Failed to save settings:', error)
    addLog('Failed to save settings', 'error')
  } finally {
    isSaving.value = false
  }
}

const handleAutostartChange = async () => {
  try {
    if (localSettings.general.autostart_enabled) {
      await invoke('enable_autostart')
      addLog('Autostart enabled')
    } else {
      await invoke('disable_autostart')
      addLog('Autostart disabled')
    }
  } catch (error) {
    console.error('Failed to update autostart:', error)
    addLog('Failed to update autostart setting', 'error')
    // Revert the checkbox state
    localSettings.general.autostart_enabled = !localSettings.general.autostart_enabled
  }
}

const handleWelcomePopupChange = () => {
  // If re-enabling the welcome popup, show it immediately
  if (localSettings.general.show_welcome_popup) {
    showPopup({
      id: 'welcome',
      component: 'Welcome',
      props: {}
    })
  }
}

const insertTag = (tagName: string) => {
  const tag = `[${tagName}]`
  localSettings.compression.filename_format += tag
}

const resetToDefault = () => {
  localSettings.compression.filename_format = '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]'
}

const applyPreset = (format: string) => {
  localSettings.compression.filename_format = format
}

const updatePreview = () => {
  // Trigger reactivity for preview
}

const browseForCustomIde = async (programName: string) => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: `Select ${programName} executable`,
      filters: [{
        name: 'Executable',
        extensions: ['exe', 'app', 'AppImage']
      }]
    })
    
    if (selected && typeof selected === 'string') {
      localSettings.ide_programs.custom_programs[programName] = selected
      extractIcon(programName, selected)
    }
  } catch (error) {
    console.error('Failed to browse for custom IDE:', error)
    addLog('Failed to browse for custom IDE', 'error')
  }
}

const browseForCustomEngine = async (engineName: string) => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: `Select ${engineName} engine folder`
    })
    
    if (selected && typeof selected === 'string') {
      localSettings.engine_programs.custom_engines[engineName] = selected
    }
  } catch (error) {
    console.error('Failed to browse for custom engine:', error)
    addLog('Failed to browse for custom engine', 'error')
  }
}

const addCustomIdeProgram = () => {
  const newName = `IDE Program ${Object.keys(localSettings.ide_programs.custom_programs).length + 1}`
  localSettings.ide_programs.custom_programs[newName] = ''
  customProgramNames.value[newName] = newName
}

const addCustomEngine = () => {
  const newName = `Custom Engine ${Object.keys(localSettings.engine_programs.custom_engines).length + 1}`
  localSettings.engine_programs.custom_engines[newName] = ''
  customEngineNames.value[newName] = newName
}

const removeCustomProgram = (programName: string) => {
  delete localSettings.ide_programs.custom_programs[programName]
  delete customProgramNames.value[programName]
  delete programIcons.value[programName]
}

const removeCustomEngine = (engineName: string) => {
  delete localSettings.engine_programs.custom_engines[engineName]
  delete customEngineNames.value[engineName]
}

const updateCustomProgramName = (oldName: string, newName: string) => {
  if (oldName === newName || !newName.trim()) return
  
  const path = localSettings.ide_programs.custom_programs[oldName]
  const icon = programIcons.value[oldName]
  
  delete localSettings.ide_programs.custom_programs[oldName]
  delete customProgramNames.value[oldName]
  delete programIcons.value[oldName]
  
  localSettings.ide_programs.custom_programs[newName] = path
  customProgramNames.value[newName] = newName
  if (icon) {
    programIcons.value[newName] = icon
  }
}

const updateCustomEngineName = (oldName: string, newName: string) => {
  if (oldName === newName || !newName.trim()) return
  
  const path = localSettings.engine_programs.custom_engines[oldName]
  
  delete localSettings.engine_programs.custom_engines[oldName]
  delete customEngineNames.value[oldName]
  
  localSettings.engine_programs.custom_engines[newName] = path
  customEngineNames.value[newName] = newName
}

const extractIcon = (programName: string, executablePath: string) => {
  if (!executablePath) return
  
  // Try to extract icon using file:// protocol for local files
  // This is a simplified approach - in a real application you might want to use
  // a more sophisticated icon extraction method
  try {
    // For now, we'll use a simple file:// URL approach
    // Note: This might not work in all browsers due to security restrictions
    const iconUrl = `file://${executablePath}`
    programIcons.value[programName] = iconUrl
  } catch (error) {
    console.warn('Failed to extract icon for', programName, error)
    // Fallback to default icon
    delete programIcons.value[programName]
  }
}

const handleIconError = (programName: string) => {
  // Remove the failed icon URL so the fallback icon is shown
  delete programIcons.value[programName]
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.settings-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 56rem;
  height: 42rem;
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
  flex-shrink: 0;
}

.close-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.popup-content {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tabs-nav {
  display: flex;
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
  flex-shrink: 0;
}

.tab-button {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  background-color: transparent;
  border-right: var(--border-width) solid var(--border-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  font-weight: var(--font-weight-medium);
  position: relative;
  border-bottom: 2px solid transparent;
}

.tab-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.tab-button.active {
  background-color: var(--background-color);
  color: var(--text-primary);
  border-bottom-color: var(--accent-color);
}

.tab-icon {
  font-size: var(--font-size-sm);
}

.tab-content-container {
  flex-grow: 1;
  overflow: hidden;
  position: relative;
}

.tab-panel {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.settings-section {
  margin-bottom: var(--spacing-xl);
}

.settings-section:last-child {
  margin-bottom: 0;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-sm);
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.add-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-xs);
  color: var(--text-primary);
  transition: all var(--transition-fast);
}

.add-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-lg);
}

.setting-item {
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-header {
  margin-bottom: var(--spacing-sm);
}

.setting-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
}

.setting-checkbox {
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent-color);
}

.setting-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.setting-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-left: 1.5rem;
}

.format-editor {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.format-input-section {
  margin-bottom: var(--spacing-lg);
}

.format-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
}

.format-input-wrapper {
  display: flex;
  gap: var(--spacing-sm);
}

.format-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  font-family: var(--font-mono);
}

.format-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.reset-format-button {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  min-width: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.reset-format-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.format-preview {
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.preview-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.preview-output {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
  background-color: var(--background-color);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.format-tags {
  margin-bottom: var(--spacing-lg);
}

.tags-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.tags-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-md);
}

.tag-category {
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  padding: var(--spacing-sm);
  background-color: var(--surface-color);
}

.category-title {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.tag-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.tag-button {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--background-color);
  cursor: pointer;
  transition: all var(--transition-fast);
  text-align: left;
}

.tag-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.tag-name {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  color: var(--accent-color);
  font-family: var(--font-mono);
}

.tag-desc {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  line-height: 1.2;
}

.format-presets {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.presets-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.preset-buttons {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.preset-button {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  transition: all var(--transition-fast);
  text-align: left;
}

.preset-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.preset-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.preset-format {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.custom-programs {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.custom-program-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.custom-program-item {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
}

.program-icon {
  flex: 0 0 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-image {
  width: 1.5rem;
  height: 1.5rem;
  object-fit: contain;
}

.fallback-icon {
  font-size: var(--font-size-lg);
}

.custom-name-input {
  flex: 0 0 8rem;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
}

.custom-path-input {
  flex: 1;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
}

.browse-button,
.remove-button {
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
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

.remove-button:hover {
  background-color: #fed7d7;
  border-color: #e53e3e;
}

.cleaning-defaults-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-xl);
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-lg);
}

.defaults-column {
  display: flex;
  flex-direction: column;
}

.subsection-title {
  font-size: var(--font-size-sm);
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
  flex-shrink: 0;
}

.checkbox-label {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  line-height: var(--line-height-normal);
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

.button-icon {
  font-size: var(--font-size-sm);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .tags-grid {
    grid-template-columns: 1fr;
  }
  
  .cleaning-defaults-grid {
    grid-template-columns: 1fr;
  }
}
</style>