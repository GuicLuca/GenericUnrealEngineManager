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
    addLog('Application started successfully')

    // Initialize project store and listen for backend events
    await initializeStore()
    addLog('Project store initialized with '+ useProjectStore().projectCount.value +' tracked project(s).')

} catch (error) {
    console.error('Failed to initialize application:', error)
    addLog('Error: Failed to initialize application. Check console for details.', 'error')
}