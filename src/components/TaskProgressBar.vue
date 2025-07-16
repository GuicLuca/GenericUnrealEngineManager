<template>
  <div class="task-progress-bar">
    <!-- Always visible compact progress bar -->
    <div class="progress-container" :class="{ 'has-multiple-tasks': activeTasks.length > 1 }">
      <!-- Single Task Display -->
      <div v-if="activeTasks.length === 1" class="single-task">
        <div class="task-info">
          <span class="task-name">{{ activeTasks[0].task_name }}</span>
          <span v-if="activeTasks[0].message" class="task-message">{{ activeTasks[0].message }}</span>
        </div>
        <div class="progress-wrapper">
          <div class="progress-bar-container">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: (activeTasks[0].progress * 100) + '%' }"
                :class="getProgressClass(activeTasks[0].status)"
              ></div>
            </div>
          </div>
          <span class="progress-percentage">{{ Math.round(activeTasks[0].progress * 100) }}%</span>
        </div>
      </div>

      <!-- Multiple Tasks Display -->
      <div v-else-if="activeTasks.length > 1" class="multiple-tasks">
        <div class="task-summary" @click="toggleExpanded" :class="{ 'expanded': showAllTasks }">
          <span class="task-count">{{ activeTasks.length }} tasks running</span>
          <button class="expand-button" :class="{ 'expanded': showAllTasks }">
            {{ showAllTasks ? '▼' : '▲' }}
          </button>
        </div>
        <div class="progress-wrapper">
          <div class="progress-bar-container">
            <div class="progress-bar">
              <div 
                class="progress-fill overall" 
                :style="{ width: overallProgress + '%' }"
              ></div>
            </div>
          </div>
          <span class="progress-percentage">{{ Math.round(overallProgress) }}%</span>
        </div>
      </div>

      <!-- No Tasks Display -->
      <div v-else class="no-tasks">
        <span class="status-text">No task running</span>
      </div>
    </div>

    <!-- Expanded Task List -->
    <Transition name="task-list">
      <div v-if="showAllTasks && activeTasks.length > 1" class="expanded-tasks">
        <div 
          v-for="task in activeTasks" 
          :key="task.task_id"
          class="task-item"
          @click="handleTaskClick(task)"
          :class="{ 'clickable': isTaskClickable(task) }"
        >
          <div class="task-info">
            <span class="task-name">{{ task.task_name }}</span>
            <span v-if="task.message" class="task-message">{{ task.message }}</span>
          </div>
          <div class="progress-wrapper">
            <div class="progress-bar-container">
              <div class="progress-bar small">
                <div 
                  class="progress-fill" 
                  :style="{ width: (task.progress * 100) + '%' }"
                  :class="getProgressClass(task.status)"
                ></div>
              </div>
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
import { useTaskStore } from '../stores/taskStore'
import { usePopup } from '../composables/usePopup'

const { activeTasks } = useTaskStore()
const { showPopup } = usePopup()
const showAllTasks = ref(false)

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

// Handle task completion for engine detection
const handleTaskClick = (task: any) => {
  // If it's an engine detection task that completed, show results
  if (task.task_name.toLowerCase().includes('auto-detecting unreal engine') ||
      task.task_name.toLowerCase().includes('engine detection')) {
    if (task.status === 'Completed') {
      showPopup({
        id: 'engine-detection',
        component: 'EngineDetection',
        props: {
          showResults: true
        }
      })
    }
  }
}

const isTaskClickable = (task: any) => {
  return (task.task_name.toLowerCase().includes('auto-detecting unreal engine') ||
          task.task_name.toLowerCase().includes('engine detection')) &&
         task.status === 'Completed'
}

// Auto-collapse when no tasks remain
const checkAutoCollapse = () => {
  if (activeTasks.value.length <= 1) {
    showAllTasks.value = false
  }
}
</script>

<style scoped>
.task-progress-bar {
  background-color: var(--surface-color);
  border-top: var(--border-width) solid var(--border-color);
  position: relative;
  z-index: 100;
  min-height: 2.5rem;
  display: flex;
  flex-direction: column;
  max-height: 50vh;
  overflow: hidden;
}

.progress-container {
  padding: var(--spacing-xs) var(--spacing-md);
  display: flex;
  align-items: center;
  min-height: 2rem;
  flex-shrink: 0;
}

.progress-container.has-multiple-tasks {
  min-height: 2.5rem;
}

.single-task,
.multiple-tasks {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  width: 100%;
}

.no-tasks {
  display: flex;
  align-items: center;
  width: 100%;
}

.status-text {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.task-info {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.task-name {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 12rem;
}

.task-message {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  opacity: 0.8;
}

.progress-wrapper {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  min-width: 12rem;
}

.progress-bar-container {
  flex: 1;
  position: relative;
}

.progress-bar {
  height: 0.375rem;
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
  overflow: hidden;
  position: relative;
}

.progress-bar.small {
  height: 0.25rem;
}

.progress-fill {
  height: 100%;
  border-radius: var(--border-radius-sm);
  transition: width var(--transition-fast), background-color var(--transition-fast);
  position: relative;
  animation: glow 2s ease-in-out infinite alternate;
}

.progress-fill.in-progress {
  background-color: var(--accent-color);
  box-shadow: 0 0 8px rgba(49, 130, 206, 0.6);
}

.progress-fill.completed {
  background-color: #38a169;
  box-shadow: 0 0 8px rgba(56, 161, 105, 0.6);
  animation: none;
}

.progress-fill.failed {
  background-color: #e53e3e;
  box-shadow: 0 0 8px rgba(229, 62, 62, 0.6);
  animation: none;
}

.progress-fill.overall {
  background: linear-gradient(90deg, var(--accent-color) 0%, #38a169 100%);
  box-shadow: 0 0 8px rgba(49, 130, 206, 0.4);
}

@keyframes glow {
  from {
    box-shadow: 0 0 4px rgba(49, 130, 206, 0.4);
  }
  to {
    box-shadow: 0 0 12px rgba(49, 130, 206, 0.8);
  }
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
  min-height: 1.5rem;
}

.task-summary.expanded {
  background-color: var(--accent-color-alpha);
}

.task-summary:hover {
  background-color: var(--hover-color);
}

.task-count {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.expand-button {
  background: none;
  border: none;
  font-size: 0.625rem;
  color: var(--text-secondary);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  transition: all var(--transition-fast);
  transform: rotate(0deg);
  width: 1rem;
  height: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.expand-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.expand-button.expanded {
  transform: rotate(180deg);
}

.expanded-tasks {
  border-top: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  max-height: calc(50vh - 4rem);
  overflow-y: auto;
  flex-grow: 1;
}

.task-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: var(--border-width) solid var(--border-color);
  min-height: 2.5rem;
}

.task-item:last-child {
  border-bottom: none;
}

.task-item.clickable {
  cursor: pointer;
}

.task-item.clickable:hover {
  background-color: var(--hover-color);
}

/* Keep task items in single line even in expanded view */
.task-item .task-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  flex: 1;
  min-width: 0;
}

.task-item .task-name {
  max-width: 10rem;
  flex-shrink: 0;
}

.task-item .task-message {
  flex: 1;
  min-width: 0;
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
  max-height: calc(50vh - 4rem);
  opacity: 1;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .task-progress-bar {
    max-height: 40vh;
  }
  
  .expanded-tasks {
    max-height: calc(40vh - 3rem);
  }
  
  .task-name {
    max-width: 8rem;
  }
  
  .progress-wrapper {
    min-width: 8rem;
  }
  
  .task-item .task-name {
    max-width: 6rem;
  }
  
  .task-message {
    display: none; /* Hide messages on mobile to save space */
  }
  
  .task-list-enter-to,
  .task-list-leave-from {
    max-height: calc(40vh - 3rem);
  }
}
</style>