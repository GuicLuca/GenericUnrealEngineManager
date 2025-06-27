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
            <h3 class="section-title">Application Settings</h3>
            <div class="section-description">
              Configure general application behavior and startup options.
            </div>

            <div class="settings-grid">
              <div class="setting-item">
                <div class="setting-header">
                  <label class="setting-label">Autostart</label>
                  <div class="setting-control">
                    <input
                      id="autostart-enabled"
                      v-model="localSettings.general.autostart_enabled"
                      type="checkbox"
                      class="checkbox-input"
                      @change="handleAutostartChange"
                    />
                    <label for="autostart-enabled" class="checkbox-label">
                      Start automatically when you log in
                    </label>
                  </div>
                </div>
                <div class="setting-description">
                  When enabled, the application will start automatically when you log into your computer.
                </div>
              </div>

              <div class="setting-item">
                <div class="setting-header">
                  <label class="setting-label">Welcome Popup</label>
                  <div class="setting-control">
                    <input
                      id="show-welcome-popup"
                      v-model="localSettings.general.show_welcome_popup"
                      type="checkbox"
                      class="checkbox-input"
                      @change="handleWelcomePopupChange"
                    />
                    <label for="show-welcome-popup" class="checkbox-label">
                      Show welcome message on startup
                    </label>
                  </div>
                </div>
                <div class="setting-description">
                  When enabled, a welcome message will be shown when the application starts.
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Programs Tab -->
        <div v-show="activeTab === 'programs'" class="tab-panel">
          <!-- IDE Programs Section -->
          <div class="settings-section">
            <div class="section-header">
              <h3 class="section-title">IDE Programs</h3>
              <button class="add-button" @click="addCustomIdeProgram">
                <span class="button-icon">➕</span>
                Add IDE
              </button>
            </div>
            <div class="section-description">
              Configure your preferred IDEs for opening C++ projects.
            </div>

            <div class="program-list">
              <div 
                v-for="(path, name) in localSettings.ide_programs.custom_programs"
                :key="name"
                class="program-item"
              >
                <div class="program-icon">
                  <img 
                    v-if="programIcons[name]" 
                    :src="programIcons[name]" 
                    :alt="name"
                    class="icon-image"
                    @error="handleIconError(name)"
                  />
                  <span v-else class="fallback-icon">💻</span>
                </div>
                <input
                  v-model="customProgramNames[name]"
                  type="text"
                  class="program-name-input"
                  placeholder="Program name..."
                  @blur="updateCustomProgramName(name, customProgramNames[name])"
                />
                <input
                  v-model="localSettings.ide_programs.custom_programs[name]"
                  type="text"
                  class="program-path-input"
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
                  @click="removeCustomIdeProgram(name)"
                  title="Remove program"
                >
                  🗑️
                </button>
              </div>
            </div>
          </div>

          <!-- Engine Programs Section -->
          <div class="settings-section">
            <div class="section-header">
              <h3 class="section-title">Engine Programs</h3>
              <button class="add-button" @click="addCustomEngineProgram">
                <span class="button-icon">➕</span>
                Add Engine
              </button>
            </div>
            <div class="section-description">
              Configure paths to your Unreal Engine installations.
            </div>

            <div class="program-list">
              <div 
                v-for="(path, name) in localSettings.engine_programs.custom_engines"
                :key="name"
                class="program-item"
              >
                <div class="program-icon">
                  <span class="fallback-icon">⚙️</span>
                </div>
                <input
                  v-model="customEngineNames[name]"
                  type="text"
                  class="program-name-input"
                  placeholder="Engine name..."
                  @blur="updateCustomEngineName(name, customEngineNames[name])"
                />
                <input
                  v-model="localSettings.engine_programs.custom_engines[name]"
                  type="text"
                  class="program-path-input"
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
                  @click="removeCustomEngineProgram(name)"
                  title="Remove engine"
                >
                  🗑️
                </button>
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

        <!-- Compression Tab -->
        <div v-show="activeTab === 'compression'" class="tab-panel">
          <div class="settings-section">
            <h3 class="section-title">Filename Format</h3>
            <div class="section-description">
              Customize the output filename format for compressed projects.
            </div>

            <div class="format-settings">
              <div class="format-input-section">
                <label class="format-label">Filename Format Template:</label>
                <input
                  v-model="localSettings.compression.filename_format"
                  type="text"
                  class="format-input"
                  placeholder="Enter filename format..."
                  @input="updatePreview"
                />
                <div v-if="formatWarning" class="format-warning">
                  {{ formatWarning }}
                </div>
                <div class="format-preview-small">
                  <span class="preview-label">Preview:</span>
                  <span class="preview-text">{{ formatPreview }}</span>
                </div>
              </div>

              <div class="format-tags-section">
                <h4 class="tags-title">Available Format Tags</h4>
                <div class="format-tags-grid">
                  <button
                    v-for="tag in formatTags"
                    :key="tag.name"
                    class="format-tag-button"
                    @click="insertTag(tag.name)"
                    @mouseenter="showTagTooltip($event, tag)"
                    @mouseleave="hideTagTooltip"
                  >
                    [{{ tag.name }}]
                  </button>
                </div>
              </div>

              <div class="presets-section">
                <div class="presets-header">
                  <h4 class="presets-title">Saved Presets</h4>
                  <button class="save-preset-button" @click="saveCurrentAsPreset">
                    <span class="button-icon">💾</span>
                    Save Current
                  </button>
                </div>
                <div class="presets-list">
                  <div 
                    v-for="(format, name) in sortedCustomPresets"
                    :key="name"
                    class="preset-item"
                  >
                    <button
                      class="preset-button"
                      @click="loadPreset(format)"
                      :title="`Load preset: ${name}`"
                    >
                      <span class="preset-name">{{ name }}</span>
                      <span class="preset-format">{{ format }}</span>
                    </button>
                    <button
                      v-if="!isDefaultPreset(name)"
                      class="delete-preset-button"
                      @click="deletePreset(name)"
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

    <div class="popup-actions">
      <button class="cancel-button" @click="$emit('close')">
        Cancel
      </button>
      <button class="save-button" @click="saveSettings" :disabled="isSaving">
        <span class="button-icon">{{ isSaving ? '⏳' : '💾' }}</span>
        {{ isSaving ? 'Saving...' : 'Save Settings' }}
      </button>
    </div>

    <!-- Custom Tooltip -->
    <Teleport to="body">
      <div
        v-if="tooltipVisible"
        class="custom-tooltip"
        :style="tooltipStyle"
      >
        <div class="tooltip-title">{{ tooltipContent.title }}</div>
        <div class="tooltip-description">{{ tooltipContent.description }}</div>
        <div class="tooltip-example">Example: {{ tooltipContent.example }}</div>
      </div>
    </Teleport>
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
    custom_presets: Record<string, string>
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
  example: string
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

// Tooltip state
const tooltipVisible = ref(false)
const tooltipPosition = ref({ x: 0, y: 0 })
const tooltipContent = ref({ title: '', description: '', example: '' })

const tabs: Tab[] = [
  { id: 'general', title: 'General', icon: '🏠' },
  { id: 'programs', title: 'Programs', icon: '💻' },
  { id: 'cleaning', title: 'Cleaning Defaults', icon: '🧹' },
  { id: 'compression', title: 'Compression', icon: '🗜️' }
]

const formatTags: FormatTag[] = [
  { name: 'Project', description: 'Project name', example: 'MyProject' },
  { name: 'Type', description: 'Project type (Cpp or Bp)', example: 'Cpp' },
  { name: 'Engine', description: 'Engine version (dots replaced with dashes)', example: '5-4-2' },
  { name: 'Algorithm', description: 'Compression algorithm', example: 'ZIP' },
  { name: 'YYYY', description: 'Full year', example: '2024' },
  { name: 'YY', description: 'Two-digit year', example: '24' },
  { name: 'MM', description: 'Month (01-12)', example: '03' },
  { name: 'DD', description: 'Day (01-31)', example: '15' },
  { name: 'HH', description: 'Hour (00-23)', example: '14' },
  { name: 'mm', description: 'Minutes (00-59)', example: '30' },
  { name: 'ss', description: 'Seconds (00-59)', example: '45' },
  { name: 'Month', description: 'Full month name', example: 'March' },
  { name: 'Mon', description: 'Short month name', example: 'Mar' },
  { name: 'Day', description: 'Full day name', example: 'Friday' },
  { name: 'Weekday', description: 'Short day name', example: 'Fri' },
  { name: 'User', description: 'Current username', example: 'john_doe' },
  { name: 'Computer', description: 'Computer hostname', example: 'DESKTOP-PC' },
  { name: 'Timestamp', description: 'Unix timestamp', example: '1710504645' },
  { name: 'SizeMB', description: 'Project size in MB', example: '1024' },
  { name: 'SizeGB', description: 'Project size in GB', example: '1' },
  { name: 'PluginCount', description: 'Number of plugins', example: '5' }
]

const defaultPresets = [
  'Default',
  'Default Extended',
  'Simple',
  'Detailed',
  'Archive Style',
  'User Specific',
  'Timestamp',
  'Size Aware',
  'Engine Specific',
  'Professional',
  'Minimal',
  'Verbose'
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
    filename_format: '[Project]_[YYYY][MM][DD][HH][mm]',
    custom_presets: {}
  }
})

// Sort custom presets alphabetically
const sortedCustomPresets = computed(() => {
  const entries = Object.entries(localSettings.compression.custom_presets)
  entries.sort(([nameA], [nameB]) => nameA.localeCompare(nameB))
  return Object.fromEntries(entries)
})

const formatPreview = computed(() => {
  const now = new Date()
  let preview = localSettings.compression.filename_format
  
  const replacements: Record<string, string> = {
    'Project': 'MyProject',
    'Type': 'Cpp',
    'Engine': '5-4-2',
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
    'Timestamp': Math.floor(now.getTime() / 1000).toString(),
    'SizeMB': '1024',
    'SizeGB': '1',
    'PluginCount': '5'
  }
  
  for (const [key, value] of Object.entries(replacements)) {
    preview = preview.replace(new RegExp(`\\[${key}\\]`, 'g'), value)
  }
  
  return preview + '.zip'
})

const formatWarning = computed(() => {
  const format = localSettings.compression.filename_format
  const tagPattern = /\[([^\]]+)\]/g
  const matches = format.match(tagPattern)
  
  if (!matches) return ''
  
  const validTags = formatTags.map(tag => `[${tag.name}]`)
  const invalidTags = matches.filter(match => !validTags.includes(match))
  
  if (invalidTags.length > 0) {
    return `Warning: Unknown format tags: ${invalidTags.join(', ')}`
  }
  
  return ''
})

const tooltipStyle = computed(() => ({
  position: 'fixed' as const,
  left: `${tooltipPosition.value.x}px`,
  top: `${tooltipPosition.value.y}px`,
  zIndex: 10001,
  transform: 'translateY(-100%)'
}))

const isDefaultPreset = (name: string): boolean => {
  return defaultPresets.includes(name)
}

const loadSettings = async () => {
  try {
    const settings = await invoke('get_settings') as AppSettings
    
    // Update local settings
    localSettings.ide_programs.custom_programs = { ...settings.ide_programs.custom_programs }
    localSettings.engine_programs.custom_engines = { ...settings.engine_programs.custom_engines }
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
    // Revert the change
    localSettings.general.autostart_enabled = !localSettings.general.autostart_enabled
  }
}

const handleWelcomePopupChange = () => {
  if (localSettings.general.show_welcome_popup) {
    // Show the welcome popup immediately when re-enabled
    showPopup({
      id: 'welcome',
      component: 'Welcome',
      props: {}
    })
  }
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

const addCustomEngineProgram = () => {
  const newName = `Engine ${Object.keys(localSettings.engine_programs.custom_engines).length + 1}`
  localSettings.engine_programs.custom_engines[newName] = ''
  customEngineNames.value[newName] = newName
}

const removeCustomIdeProgram = (programName: string) => {
  delete localSettings.ide_programs.custom_programs[programName]
  delete customProgramNames.value[programName]
  delete programIcons.value[programName]
}

const removeCustomEngineProgram = (engineName: string) => {
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
  
  try {
    const iconUrl = `file://${executablePath}`
    programIcons.value[programName] = iconUrl
  } catch (error) {
    console.warn('Failed to extract icon for', programName, error)
    delete programIcons.value[programName]
  }
}

const handleIconError = (programName: string) => {
  delete programIcons.value[programName]
}

const insertTag = (tagName: string) => {
  const tag = `[${tagName}]`
  localSettings.compression.filename_format += tag
  updatePreview()
}

const updatePreview = () => {
  // Force reactivity update
}

const showTagTooltip = (event: MouseEvent, tag: FormatTag) => {
  tooltipContent.value = {
    title: `[${tag.name}]`,
    description: tag.description,
    example: tag.example
  }
  tooltipPosition.value = {
    x: event.clientX,
    y: event.clientY - 10
  }
  tooltipVisible.value = true
}

const hideTagTooltip = () => {
  tooltipVisible.value = false
}

const saveCurrentAsPreset = () => {
  const presetName = prompt('Enter a name for this preset:')
  if (presetName && presetName.trim()) {
    localSettings.compression.custom_presets[presetName.trim()] = localSettings.compression.filename_format
    addLog(`Preset "${presetName}" saved`)
  }
}

const loadPreset = (format: string) => {
  localSettings.compression.filename_format = format
  updatePreview()
}

const deletePreset = (presetName: string) => {
  if (confirm(`Are you sure you want to delete the preset "${presetName}"?`)) {
    delete localSettings.compression.custom_presets[presetName]
    addLog(`Preset "${presetName}" deleted`)
  }
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
  width: 52rem;
  height: 40rem;
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
  padding: var(--spacing-md);
  overflow-y: auto;
}

.settings-section {
  margin-bottom: var(--spacing-md);
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

.section-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-md);
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

.settings-grid {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-md);
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.setting-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.setting-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.setting-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-left: var(--spacing-md);
}

.program-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-md);
}

.program-item {
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

.program-name-input {
  flex: 0 0 8rem;
  padding: var(--spacing-sm);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
}

.program-path-input {
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
  gap: var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-md);
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

.format-settings {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-md);
}

.format-input-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.format-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.format-input {
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

.format-warning {
  font-size: var(--font-size-sm);
  color: #d69e2e;
  font-style: italic;
  padding: var(--spacing-xs);
  background-color: rgba(214, 158, 46, 0.1);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid #d69e2e;
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

.preview-text {
  color: var(--text-primary);
  font-family: var(--font-mono);
  word-break: break-all;
  flex-grow: 1;
}

.format-tags-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.tags-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.format-tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));
  gap: var(--spacing-xs);
}

.format-tag-button {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-xs);
  color: var(--text-primary);
  transition: all var(--transition-fast);
  font-family: var(--font-mono);
  text-align: center;
}

.format-tag-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.presets-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.presets-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.presets-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.save-preset-button {
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

.save-preset-button:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.presets-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  max-height: 8rem;
  overflow-y: auto;
}

.preset-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.preset-button {
  flex: 1;
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

.delete-preset-button {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  min-width: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.delete-preset-button:hover {
  background-color: #fed7d7;
  border-color: #e53e3e;
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

.custom-tooltip {
  background-color: var(--text-primary);
  color: var(--background-color);
  padding: var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  box-shadow: var(--shadow-md);
  max-width: 20rem;
  pointer-events: none;
}

.tooltip-title {
  font-weight: var(--font-weight-semibold);
  margin-bottom: var(--spacing-xs);
  font-family: var(--font-mono);
}

.tooltip-description {
  margin-bottom: var(--spacing-xs);
}

.tooltip-example {
  font-family: var(--font-mono);
  opacity: 0.8;
}
</style>