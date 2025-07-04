import {ref, reactive} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {useLogStore} from '../stores/logStore'

export interface AppSettings {
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

// Default settings structure
const defaultSettings: AppSettings = {
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
        custom_presets: {
            'Default': '[Project]_[YYYY][MM][DD][HH][mm]',
            'Default Extended': '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
            'Simple': '[Project]_[Type]_[Timestamp]',
            'User Specific': '[User]_[Computer]_[Project]_[Mon][DD]_[HH][mm]'
        }
    }
}

// Global settings state
const settings = reactive<AppSettings>({...defaultSettings})
const isLoading = ref(false)
const hasUnsavedChanges = ref(false)
const lastSaveTime = ref<Date | null>(null)
const {addLog} = useLogStore()

export const useSettingsStore = () => {
    // Load settings from backend
    const loadSettings = async (): Promise<void> => {
        try {
            isLoading.value = true
            const backendSettings = await invoke('get_settings') as AppSettings

            Object.assign(settings, {...backendSettings})

            hasUnsavedChanges.value = false
        } catch (error) {
            addLog('Failed to load settings:', "error")
            throw error
        } finally {
            isLoading.value = false
        }
    }

    // Save settings to the backend
    const saveSettings = async (): Promise<void> => {
        try {
            isLoading.value = true
            await invoke('save_settings', {settings})
            hasUnsavedChanges.value = false
            lastSaveTime.value = new Date()
        } catch (error) {
            addLog('Failed to save settings:', "error")
            throw error
        } finally {
            isLoading.value = false
        }
    }

    // Update a specific section of settings
    const updateSettings = <K extends keyof AppSettings>(
        section: K,
        updates: Partial<AppSettings[K]>,
        autosave: boolean = false
    ): void => {
        Object.assign(settings[section], updates)
        hasUnsavedChanges.value = true

        if (autosave) {
            saveSettings().then()
        }
    }

    // Get a specific section of settings
    const getSettings = <K extends keyof AppSettings>(section: K): AppSettings[K] => {
        return settings[section]
    }

    // Get all settings
    const getAllSettings = (): AppSettings => {
        return settings
    }

    // Reset settings to defaults
    const resetToDefaults = async () => {
        const new_settings = await invoke("reset_settings") as AppSettings

        Object.assign(settings, {...new_settings})
        // No need to save immediately, as this is a reset operation and the backend has already handled it
    }

    // Reset a specific section to defaults
    const resetSectionToDefaults = <K extends keyof AppSettings>(section: K): void => {
        Object.assign(settings[section], JSON.parse(JSON.stringify(defaultSettings[section])))
        hasUnsavedChanges.value = true

        saveSettings().then()
    }

    // IDE Programs specific methods
    const addIdeProgram = (name: string, path: string): void => {
        settings.ide_programs.custom_programs[name] = path
        hasUnsavedChanges.value = true
    }

    const removeIdeProgram = (name: string): void => {
        delete settings.ide_programs.custom_programs[name]
        hasUnsavedChanges.value = true
    }

    const updateIdeProgram = (oldName: string, newName: string, path: string): void => {
        if (oldName !== newName) {
            delete settings.ide_programs.custom_programs[oldName]
        }
        settings.ide_programs.custom_programs[newName] = path
        hasUnsavedChanges.value = true
    }

    // Engine Programs specific methods
    const addEngineProgram = (name: string, path: string): void => {
        settings.engine_programs.custom_engines[name] = path
        hasUnsavedChanges.value = true
    }

    const removeEngineProgram = (name: string): void => {
        delete settings.engine_programs.custom_engines[name]
        hasUnsavedChanges.value = true
    }

    const updateEngineProgram = (oldName: string, newName: string, path: string): void => {
        if (oldName !== newName) {
            delete settings.engine_programs.custom_engines[oldName]
        }
        settings.engine_programs.custom_engines[newName] = path
        hasUnsavedChanges.value = true
    }

    // Compression-specific methods
    const addCompressionPreset = (name: string, format: string): void => {
        settings.compression.custom_presets[name] = format
        hasUnsavedChanges.value = true
    }

    const removeCompressionPreset = (name: string): void => {
        delete settings.compression.custom_presets[name]
        hasUnsavedChanges.value = true
    }

    const updateCompressionPreset = (oldName: string, newName: string, format: string): void => {
        if (oldName !== newName) {
            delete settings.compression.custom_presets[oldName]
        }
        settings.compression.custom_presets[newName] = format
        hasUnsavedChanges.value = true
    }

    const setCompressionFormat = (format: string): void => {
        settings.compression.filename_format = format
        hasUnsavedChanges.value = true
    }

    // Cleaning defaults specific methods
    const updateCleaningDefaults = (updates: Partial<AppSettings['cleaning_defaults']>): void => {
        Object.assign(settings.cleaning_defaults, updates)
        hasUnsavedChanges.value = true
    }

    // General settings specific methods
    const updateGeneralSettings = (updates: Partial<AppSettings['general']>): void => {
        Object.assign(settings.general, updates)
        hasUnsavedChanges.value = true
    }

    return {
        // State
        settings,
        isLoading,
        hasUnsavedChanges,
        lastSaveTime,

        // Core methods
        loadSettings,
        saveSettings,
        updateSettings,
        getSettings,
        getAllSettings,
        resetToDefaults,
        resetSectionToDefaults,

        // IDE Programs
        addIdeProgram,
        removeIdeProgram,
        updateIdeProgram,

        // Engine Programs
        addEngineProgram,
        removeEngineProgram,
        updateEngineProgram,

        // Compression
        addCompressionPreset,
        removeCompressionPreset,
        updateCompressionPreset,
        setCompressionFormat,

        // Cleaning
        updateCleaningDefaults,

        // General
        updateGeneralSettings
    }
}