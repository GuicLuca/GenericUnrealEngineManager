import {ref} from 'vue'
import {listen} from "@tauri-apps/api/event";

export interface LogEntry {
    id: number
    timestamp: string
    message: string
    level?: 'info' | 'warn' | 'error' | 'debug' | 'trace'
}

const MAX_MESSAGE_SAVED = 1600;
const logs = ref<LogEntry[]>([])

export const useLogStore = () => {
    const addLog = (message: string, level: 'info' | 'warn' | 'error' | 'debug' | 'trace' = 'info') => {
        const now = new Date()
        const timestamp = now.toISOString().slice(0, 19).replace('T', ' ')

        logs.value.push({
            id: Date.now(),
            timestamp,
            message,
            level
        })

        // Keep only the last 200 logs to prevent memory issues
        if (logs.value.length > MAX_MESSAGE_SAVED) {
            logs.value = logs.value.slice(-MAX_MESSAGE_SAVED)
        }
    }

    const clearLogs = () => {
        logs.value = []
        addLog('Logs cleared')
    }

    // Listen for backend log events
    const initLogListener = async () => {
        try {
            await listen('add_log', (event: any) => {
                const {message, level} = event.payload
                addLog(message, level || 'info')
            })

            // Listen for streaming build output
            await listen('log-message', (event: any) => {
                const {message, level} = event.payload
                addLog(message, level || 'info')
            })
        } catch (error) {
            console.error('Failed to initialize log listener:', error)
        }
    }

    return {
        logs,
        addLog,
        clearLogs,
        initLogListener
    }
}