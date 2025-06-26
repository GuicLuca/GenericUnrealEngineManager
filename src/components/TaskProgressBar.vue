<template>
  <div class="task-progress-bar" :class="{ 'expanded': showAllTasks }">
    <!-- Main Progress Bar -->
    <div v-if="activeTasks.length > 0" class="progress-container">
      <!-- Single Task Display -->
      <div v-if="activeTasks.length === 1" class="single-task">
        <div class="task-info">
          <span class="task-name">{{ activeTasks[0].task_name }}</span>
          <span v-if="activeTasks[0].message" class="task-message">{{ activeTasks[0].message }}</span>
        </div>
        <div class="progress-wrapper">
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ width: (activeTasks[0].progress * 100) + '%' }"
              :class="getProgressClass(activeTasks[0].status)"
            ></div>
          </div>
          <span class="progress-percentage">{{ Math.round(activeTasks[0].progress * 100) }}%</span>
        </div>
      </div>

      <!-- Multiple Tasks Display -->
      <div v-else class="multiple-tasks">
        <div class="task-summary" @click="toggleExpanded">
          <span class="task-count">{{ activeTasks.length }} tasks running</span>
          <button class="expand-button" :class="{ 'expanded': showAllTasks }">
            {{ showAllTasks ? '▼' : '▲' }}
          </button>
        </div>
        <div class="overall-progress">
          <div class="progress-bar">
            <div 
              class="progress-fill overall" 
              :style="{ width: overallProgress + '%' }"
            ></div>
          </div>
          <span class="progress-percentage">{{ Math.round(overallProgress) }}%</span>
        </div>
      </div>
    </div>

    <!-- Expanded Task List -->
    <Transition name="task-list">
      <div v-if="showAllTasks && activeTasks.length > 1" class="expanded-tasks">
        <div 
          v-for="task in activeTasks" 
          :key="task.task_id"
          class="task-item"
        >
          <div class="task-info">
            <span class="task-name">{{ task.task_name }}</span>
            <span v-if="task.message" class="task-message">{{ task.message }}</span>
          </div>
          <div class="progress-wrapper">
            <div class="progress-bar small">
              <div 
                class="progress-fill" 
                :style="{ width: (task.progress * 100) + '%' }"
                :class="getProgressClass(task.status)"
              ></div>
            </div>
            <span class="progress-percentage small">{{ Math.round(task.progress * 100) }}%</span>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'

interface TaskProgress {
  task_id: string
  task_name: string
  progress: number // 0.0 to 1.0
  status: 'Started' | 'InProgress' | 'Completed' | 'Failed'
  message?: string
}

const tasks = ref<Map<string, TaskProgress>>(new Map())
const showAllTasks = ref(false)

// Computed properties
const activeTasks = computed(() => {
  return Array.from(tasks.value.values()).filter(task => 
    task.status === 'Started' || task.status === 'InProgress'
  )
})

const overallProgress = computed(() => {
  if (activeTasks.value.length === 0) return 0
  const totalProgress = activeTasks.value.reduce((sum, task) => sum + task.progress, 0)
  return (totalProgress / activeTasks.value.length) * 100
})

// Methods
const getProgressClass = (status: string) => {
  switch (status) {
    case 'Started':
    case 'InProgress':
      return 'in-progress'
    case 'Completed':
      return 'completed'
    case 'Failed':
      return 'failed'
    default:
      return 'in-progress'
  }
}

const toggleExpanded = () => {
  showAllTasks.value = !showAllTasks.value
}

const handleTaskProgress = (event: any) => {
  const taskData: TaskProgress = event.payload
  
  if (taskData.status === 'Completed' || taskData.status === 'Failed') {
    // Remove completed/failed tasks after a short delay
    setTimeout(() => {
      tasks.value.delete(taskData.task_id)
      
      // Auto-collapse if no tasks remain
      if (activeTasks.value.length <= 1) {
        showAllTasks.value = false
      }
    }, 1000)
  } else {
    // Update or add task
    tasks.value.set(taskData.task_id, taskData)
  }
}

// Lifecycle
let unlistenTaskProgress: (() => void) | null = null

onMounted(async () => {
  try {
    unlistenTaskProgress = await listen('task_progress', handleTaskProgress)
  } catch (error) {
    console.error('Failed to listen for task progress events:', error)
  }
})

onUnmounted(() => {
  if (unlistenTaskProgress) {
    unlistenTaskProgress()
  }
})
</script>

<style scoped>
.task-progress-bar {
  background-color: var(--surface-color);
  border-top: var(--border-width) solid var(--border-color);
  position: relative;
  z-index: 100;
  transition: all var(--transition-normal);
}

.progress-container {
  padding: var(--spacing-sm) var(--spacing-md);
}

.single-task,
.multiple-tasks {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.task-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.task-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-message {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.progress-wrapper {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  min-width: 8rem;
}

.progress-bar {
  flex: 1;
  height: 0.5rem;
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
  overflow: hidden;
  position: relative;
}

.progress-bar.small {
  height: 0.375rem;
}

.progress-fill {
  height: 100%;
  border-radius: var(--border-radius-sm);
  transition: width var(--transition-fast), background-color var(--transition-fast);
  position: relative;
}

.progress-fill.in-progress {
  background-color: var(--accent-color);
}

.progress-fill.completed {
  background-color: #38a169;
}

.progress-fill.failed {
  background-color: #e53e3e;
}

.progress-fill.overall {
  background: linear-gradient(90deg, var(--accent-color) 0%, #38a169 100%);
}

.progress-percentage {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  min-width: 2.5rem;
  text-align: right;
  font-family: var(--font-mono);
}

.progress-percentage.small {
  font-size: 0.625rem;
  min-width: 2rem;
}

.task-summary {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  transition: background-color var(--transition-fast);
  flex: 1;
  min-width: 0;
}

.task-summary:hover {
  background-color: var(--hover-color);
}

.task-count {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.expand-button {
  background: none;
  border: none;
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  transition: all var(--transition-fast);
  transform: rotate(0deg);
}

.expand-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.expand-button.expanded {
  transform: rotate(180deg);
}

.overall-progress {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  min-width: 8rem;
}

.expanded-tasks {
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  max-height: 12rem;
  overflow-y: auto;
}

.task-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: var(--border-width) solid var(--border-color);
}

.task-item:last-child {
  border-bottom: none;
}

/* Transitions */
.task-list-enter-active,
.task-list-leave-active {
  transition: all var(--transition-normal);
  overflow: hidden;
}

.task-list-enter-from,
.task-list-leave-to {
  max-height: 0;
  opacity: 0;
}

.task-list-enter-to,
.task-list-leave-from {
  max-height: 12rem;
  opacity: 1;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .single-task,
  .multiple-tasks,
  .task-item {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-sm);
  }
  
  .progress-wrapper,
  .overall-progress {
    min-width: auto;
  }
  
  .task-summary {
    flex-direction: row;
    align-items: center;
  }
}
</style>