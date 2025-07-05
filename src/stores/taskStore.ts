import { ref, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'

interface TaskProgress {
  task_id: string
  task_name: string
  progress: number // 0.0 to 1.0
  status: 'Started' | 'InProgress' | 'Completed' | 'Failed'
  message?: string
}

// Global task state
const tasks = ref<Map<string, TaskProgress>>(new Map())

export const useTaskStore = () => {
  // Initialize task listener
  const initTaskListener = async () => {
    try {
      await listen('task_progress', (event: any) => {
        const taskData: TaskProgress = event.payload
        
        if (taskData.status === 'Completed' || taskData.status === 'Failed') {
          // Keep completed/failed tasks for a short time for UI feedback
          tasks.value.set(taskData.task_id, taskData)
          
          // Remove after delay
          setTimeout(() => {
            tasks.value.delete(taskData.task_id)
          }, 2000)
        } else {
          // Update or add the active task
          tasks.value.set(taskData.task_id, taskData)
        }
      })
    } catch (error) {
      console.error('Failed to initialize task listener:', error)
    }
  }

  // Check if a specific task type is running
  const isTaskTypeRunning = (taskNamePattern: string): boolean => {
    return Array.from(tasks.value.values()).some(task => 
      (task.status === 'Started' || task.status === 'InProgress') &&
      task.task_name.toLowerCase().includes(taskNamePattern.toLowerCase())
    )
  }

  // Get running the task by its name pattern
  const getRunningTask = (taskNamePattern: string): TaskProgress | null => {
    return Array.from(tasks.value.values()).find(task => 
      (task.status === 'Started' || task.status === 'InProgress') &&
      task.task_name.toLowerCase().includes(taskNamePattern.toLowerCase())
    ) || null
  }

  // Get all active tasks
  const activeTasks = computed(() => {
    return Array.from(tasks.value.values()).filter(task => 
      task.status === 'Started' || task.status === 'InProgress'
    )
  })

  // Check if engine detection is running
  const isEngineDetectionRunning = computed(() => {
    return isTaskTypeRunning('auto-detecting unreal engine') || 
           isTaskTypeRunning('engine detection') ||
           isTaskTypeRunning('auto-detection')
  })

  // Get current engine detection task
  const currentEngineDetectionTask = computed(() => {
    return getRunningTask('auto-detecting unreal engine') || 
           getRunningTask('engine detection') ||
           getRunningTask('auto-detection')
  })

  return {
    // State
    tasks,
    activeTasks,
    
    // Engine detection specific
    isEngineDetectionRunning,
    currentEngineDetectionTask,
    
    // General methods
    initTaskListener,
    isTaskTypeRunning,
    getRunningTask
  }
}