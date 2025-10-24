import {invoke} from "@tauri-apps/api/core";
import {AppSettings} from "../stores/settingsStore.ts";
import { useLogStore } from '../stores/logStore'
const { addLog } = useLogStore()

export const useCompression = () => {
    const loadAvailableFormats = async () => {
        let availableFormats: Record<string, string> = {}
        let selectedFormat: string = ""
        
        try {
            const settings = await invoke('get_settings') as AppSettings
            availableFormats = settings.compression.custom_presets
            
            // Set default format
            if (settings.compression.filename_format) {
                selectedFormat = settings.compression.filename_format
            }
        } catch (error) {
            console.error('Failed to load compression settings:', error)
            addLog('Failed to load compression settings', 'error')
            // Fallback formats
            availableFormats = {
                'Default': '[Project]_[YYYY][MM][DD][HH][mm]',
                'Default Extended': '[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]',
                'Simple': '[Project]_[Type]'
            }
        }
        
        return {
            selectedFormat,
            availableFormats,
        }
            
    }
    
    return {
        loadAvailableFormats
    }
}