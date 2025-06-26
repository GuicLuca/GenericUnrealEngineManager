<template>
  <div class="development-panel">
    <div class="panel-content">
      <div class="tool-section">
        <h5 class="section-title">Popup System Testing</h5>
        <div class="button-group">
          <button 
            class="dev-button"
            @click="openProjectDiscoveryPopup"
          >
            <span class="button-icon">🔍</span>
            Test Project Discovery Popup
          </button>
        </div>
      </div>
      
      <div class="tool-section">
        <h5 class="section-title">Project Testing</h5>
        <div class="button-group">
          <button 
            class="dev-button"
            @click="addMockProjects"
            :disabled="isAddingMockProjects"
          >
            <span class="button-icon">{{ isAddingMockProjects ? '⏳' : '📁' }}</span>
            {{ isAddingMockProjects ? 'Adding Projects...' : 'Add 5 Mock Projects' }}
          </button>
        </div>
      </div>

      <div class="tool-section">
        <h5 class="section-title">Task Progress Testing</h5>
        <div class="button-group">
          <button 
            class="dev-button"
            @click="addSingleFakeTask"
          >
            <span class="button-icon">🔄</span>
            Add Single Random Task
          </button>
          <button 
            class="dev-button"
            @click="addMultipleFakeTasks"
          >
            <span class="button-icon">⚡</span>
            Add 3-5 Random Tasks
          </button>
          <button 
            class="dev-button"
            @click="addStressTestTasks"
          >
            <span class="button-icon">🚀</span>
            Stress Test (10 Tasks)
          </button>
        </div>
      </div>
      
      <div class="tool-section">
        <h5 class="section-title">System Information</h5>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Platform:</span>
            <span class="info-value">{{ platformInfo }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">App Version:</span>
            <span class="info-value">{{ appVersion }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Active Tasks:</span>
            <span class="info-value">{{ activeTaskCount }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { usePopup } from '../composables/usePopup'
import { useProjectStore } from '../stores/projectStore'
import { useLogStore } from '../stores/logStore'
import { mockProjects } from '../utils/mockProjects'

const { showPopup } = usePopup()
const { projects } = useProjectStore()
const { addLog } = useLogStore()

const platformInfo = ref('Unknown')
const appVersion = ref('0.1.0')
const isAddingMockProjects = ref(false)
const activeTaskCount = ref(0)

// Track running tasks to update counter
const runningTasks = new Set<string>()

const openProjectDiscoveryPopup = () => {
  showPopup({
    id: 'project-discovery',
    component: 'ProjectDiscovery',
    props: {}
  })
}

const addMockProjects = async () => {
  if (isAddingMockProjects.value) return
  
  try {
    isAddingMockProjects.value = true
    addLog('Adding mock projects for testing...')
    
    // Filter out projects that already exist (by path)
    const existingPaths = new Set(projects.value.map(p => p.path))
    const newProjects = mockProjects.filter(project => !existingPaths.has(project.path))
    
    if (newProjects.length === 0) {
      addLog('All mock projects already exist', 'warn')
      return
    }
    
    // Add projects to the store (simulate backend behavior)
    projects.value.push(...newProjects)
    
    addLog(`Added ${newProjects.length} mock projects successfully`)
    
  } catch (error) {
    console.error('Failed to add mock projects:', error)
    addLog('Failed to add mock projects', 'error')
  } finally {
    isAddingMockProjects.value = false
  }
}

const addSingleFakeTask = () => {
  createFakeTask()
}

const addMultipleFakeTasks = () => {
  const taskCount = Math.floor(Math.random() * 3) + 3 // 3-5 tasks
  addLog(`Creating ${taskCount} concurrent tasks...`)
  
  for (let i = 0; i < taskCount; i++) {
    // Add small delay between task starts to make it more realistic
    setTimeout(() => {
      createFakeTask()
    }, i * 200)
  }
}

const addStressTestTasks = () => {
  const taskCount = 10
  addLog(`Starting stress test with ${taskCount} concurrent tasks...`)
  
  for (let i = 0; i < taskCount; i++) {
    // Stagger task starts over 2 seconds
    setTimeout(() => {
      createFakeTask()
    }, i * 200)
  }
}

const createFakeTask = () => {
  // Generate random task parameters
  const taskNames = [
    'Processing large dataset',
    'Compiling shaders',
    'Building lighting',
    'Optimizing textures',
    'Generating navigation mesh',
    'Baking audio',
    'Analyzing code complexity',
    'Synchronizing assets',
    'Validating project structure',
    'Cleaning temporary files',
    'Importing 3D models',
    'Converting video files',
    'Calculating physics',
    'Rendering preview',
    'Compressing assets',
    'Updating dependencies',
    'Running unit tests',
    'Deploying to server',
    'Scanning for errors',
    'Optimizing performance'
  ]
  
  const taskName = taskNames[Math.floor(Math.random() * taskNames.length)]
  const duration = Math.floor(Math.random() * 20000) + 10000 // 10-30 seconds
  const taskId = `fake_task_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  
  addLog(`Starting task: ${taskName} (${Math.round(duration/1000)}s)`)
  
  // Start the task simulation (non-blocking)
  simulateTaskProgress(taskId, taskName, duration)
}

const simulateTaskProgress = async (taskId: string, taskName: string, duration: number) => {
  // Add to running tasks
  runningTasks.add(taskId)
  activeTaskCount.value = runningTasks.size
  
  const steps = [
    'Initializing...',
    'Loading resources...',
    'Processing data...',
    'Applying transformations...',
    'Optimizing results...',
    'Finalizing...'
  ]
  
  try {
    // Emit task started event
    window.__TAURI__.event.emit('task_progress', {
      task_id: taskId,
      task_name: taskName,
      progress: 0.0,
      status: 'Started',
      message: steps[0]
    })
    
    const stepDuration = duration / steps.length
    
    for (let i = 0; i < steps.length; i++) {
      await new Promise(resolve => setTimeout(resolve, stepDuration))
      
      // Check if task was cancelled (for cleanup)
      if (!runningTasks.has(taskId)) {
        return
      }
      
      const progress = (i + 1) / steps.length
      const message = i < steps.length - 1 ? steps[i + 1] : 'Completed'
      
      // Add some randomness to make it more realistic
      const actualProgress = Math.min(progress + (Math.random() - 0.5) * 0.1, 1.0)
      
      window.__TAURI__.event.emit('task_progress', {
        task_id: taskId,
        task_name: taskName,
        progress: Math.max(0, actualProgress),
        status: progress >= 1.0 ? 'Completed' : 'InProgress',
        message: message
      })
    }
    
    addLog(`Task completed: ${taskName}`)
    
  } catch (error) {
    console.error('Task simulation error:', error)
    
    // Emit failure event
    window.__TAURI__.event.emit('task_progress', {
      task_id: taskId,
      task_name: taskName,
      progress: 0.0,
      status: 'Failed',
      message: 'Task failed unexpectedly'
    })
    
    addLog(`Task failed: ${taskName}`, 'error')
    
  } finally {
    // Remove from running tasks
    runningTasks.delete(taskId)
    activeTaskCount.value = runningTasks.size
  }
}

onMounted(async () => {
  try {
    // Get platform info if available
    platformInfo.value = navigator.platform || 'Unknown'
  } catch (error) {
    console.error('Failed to get platform info:', error)
  }
})
</script>

<style scoped>
.development-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.panel-content {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.tool-section {
  margin-bottom: var(--spacing-lg);
}

.tool-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
  padding-bottom: var(--spacing-xs);
  border-bottom: var(--border-width) solid var(--border-color);
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.dev-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
  text-align: left;
}

.dev-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.dev-button:active:not(:disabled) {
  background-color: var(--active-color);
}

.dev-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-md);
}

.info-grid {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
}

.info-label {
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.info-value {
  color: var(--text-primary);
  font-family: var(--font-mono);
}
</style>