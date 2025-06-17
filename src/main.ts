import { createApp } from "vue";
import App from "./App.vue";
import "./styles/variables.css";
import { useLogStore } from './stores/logStore'
import { useProjectStore } from './stores/projectStore'

createApp(App).mount("#app");

// Initialize stores
const { addLog, initLogListener } = useLogStore()
const { initializeStore } = useProjectStore()

try {
    // Initialize log system
    await initLogListener()
    // Initialize the project store and listen for backend events
    await initializeStore()
    
    // Frontend initialization complete
    addLog('Application started successfully')
} catch (error) {
    console.error('Failed to initialize application:', error)
    addLog('Error: Failed to initialize application. Check console for details.', 'error')
}