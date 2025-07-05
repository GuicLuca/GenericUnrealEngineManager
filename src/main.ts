import { createApp } from "vue";
import App from "./App.vue";
import "./styles/variables.css";
import { useLogStore } from './stores/logStore'
import { useProjectStore } from './stores/projectStore'
import { useTaskStore } from './stores/taskStore'

createApp(App).mount("#app");

// Initialize stores
const { addLog, initLogListener } = useLogStore()
const { initializeStore } = useProjectStore()
const { initTaskListener } = useTaskStore()

try {
    // Initialize log system
    await initLogListener()
    // Initialize task system
    await initTaskListener()
    // Initialize the project store and listen for backend events
    await initializeStore()
    
    // Frontend initialization complete
    addLog('Application started successfully')
} catch (error) {
    console.error('Failed to initialize application:', error)
    addLog('Error: Failed to initialize application. Check console for details.', 'error')
}