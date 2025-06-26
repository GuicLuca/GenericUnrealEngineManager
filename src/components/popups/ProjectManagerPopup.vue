<template>
  <div class="project-manager-popup">
    <div class="popup-header">
      <h2 class="popup-title">
        <span class="title-icon">📁</span>
        Manage Tracked Projects
      </h2>
      <button class="close-button" @click="$emit('close')" title="Close">
        ✕
      </button>
    </div>

    <div class="popup-content">
      <!-- Search and Sort Controls -->
      <div class="controls-section">
        <div class="search-container">
          <div class="search-input-wrapper">
            <input
              v-model="searchQuery"
              type="text"
              class="search-input"
              placeholder="Search projects by name..."
              :disabled="isLoading"
            />
            <button
              v-if="searchQuery"
              class="clear-search-btn"
              @click="clearSearch"
              title="Clear search"
            >
              ✕
            </button>
          </div>
        </div>
        
        <div class="sort-container">
          <button
            class="sort-button"
            @click="toggleSortDropdown"
            :disabled="isLoading"
            title="Sort projects"
          >
            <span class="sort-icon">{{ getSortIcon() }}</span>
            <span class="sort-text">{{ getSortText() }}</span>
            <span class="dropdown-arrow" :class="{ 'open': showSortDropdown }">▼</span>
          </button>
          
          <div v-if="showSortDropdown" class="sort-dropdown" @click.stop>
            <div class="sort-group">
              <div class="sort-group-title">Name</div>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'name' && sortOrder === 'asc' }"
                @click="setSorting('name', 'asc')"
              >
                <span class="sort-option-icon">📝</span>
                Name (A-Z)
              </button>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'name' && sortOrder === 'desc' }"
                @click="setSorting('name', 'desc')"
              >
                <span class="sort-option-icon">📝</span>
                Name (Z-A)
              </button>
            </div>
            
            <div class="sort-group">
              <div class="sort-group-title">Type</div>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'type' && sortOrder === 'asc' }"
                @click="setSorting('type', 'asc')"
              >
                <span class="sort-option-icon">💻</span>
                Type (C++ first)
              </button>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'type' && sortOrder === 'desc' }"
                @click="setSorting('type', 'desc')"
              >
                <span class="sort-option-icon">🎨</span>
                Type (Blueprint first)
              </button>
            </div>
            
            <div class="sort-group">
              <div class="sort-group-title">Size</div>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'size' && sortOrder === 'asc' }"
                @click="setSorting('size', 'asc')"
              >
                <span class="sort-option-icon">📦</span>
                Size (Smallest first)
              </button>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'size' && sortOrder === 'desc' }"
                @click="setSorting('size', 'desc')"
              >
                <span class="sort-option-icon">📦</span>
                Size (Largest first)
              </button>
            </div>
            
            <div class="sort-group">
              <div class="sort-group-title">Last Scan</div>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'lastScan' && sortOrder === 'asc' }"
                @click="setSorting('lastScan', 'asc')"
              >
                <span class="sort-option-icon">🕒</span>
                Last Scan (Oldest first)
              </button>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'lastScan' && sortOrder === 'desc' }"
                @click="setSorting('lastScan', 'desc')"
              >
                <span class="sort-option-icon">🕒</span>
                Last Scan (Newest first)
              </button>
            </div>
            
            <div class="sort-group">
              <div class="sort-group-title">Engine Version</div>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'version' && sortOrder === 'asc' }"
                @click="setSorting('version', 'asc')"
              >
                <span class="sort-option-icon">⚙️</span>
                Version (Custom first)
              </button>
              <button
                class="sort-option"
                :class="{ active: sortBy === 'version' && sortOrder === 'desc' }"
                @click="setSorting('version', 'desc')"
              >
                <span class="sort-option-icon">⚙️</span>
                Version (Newest first)
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="projects-section">
        <div class="section-header">
          <h3 class="section-title">
            {{ filteredAndSortedProjects.length }} of {{ projectCount }} project(s)
            <span v-if="searchQuery" class="search-indicator">matching "{{ searchQuery }}"</span>
          </h3>
          <div class="header-actions">
            <button 
              class="discover-btn"
              @click="openProjectDiscovery"
              :disabled="isLoading"
            >
              <span class="button-icon">➕</span>
              Add Projects
            </button>
            <button 
              class="refresh-btn"
              @click="handleRefresh"
              :disabled="isLoading"
              title="Refresh project list"
            >
              {{ isLoading ? '⏳' : '🔄' }}
            </button>
          </div>
        </div>

        <div v-if="filteredAndSortedProjects.length === 0 && !searchQuery" class="no-projects">
          <div class="no-projects-icon">📂</div>
          <div class="no-projects-text">No projects tracked yet</div>
          <div class="no-projects-subtext">Use the Add Projects button to find projects</div>
        </div>

        <div v-else-if="filteredAndSortedProjects.length === 0 && searchQuery" class="no-results">
          <div class="no-results-icon">🔍</div>
          <div class="no-results-text">No projects found</div>
          <div class="no-results-subtext">Try adjusting your search terms</div>
          <button class="clear-search-button" @click="clearSearch">
            Clear Search
          </button>
        </div>

        <div v-else class="projects-list">
          <div 
            v-for="project in filteredAndSortedProjects"
            :key="project.path"
            class="project-item"
            :class="{ selected: selectedProject?.path === project.path }"
            @click="selectProject(project)"
          >
            <div class="project-info">
              <div class="project-name">{{ project.name }}</div>
              <div class="project-path">{{ project.path }}</div>
              <div class="project-meta">
                <span class="engine-version">{{ getEngineVersionString(project.engine_association) }}</span>
                <span class="has-cpp">{{ project.has_cpp ? 'C++' : 'Blueprint' }}</span>
                <span class="plugin-count">{{ project.plugins.length }} plugin(s)</span>
                <span class="size-on-disk">{{ formatSize(project.size_on_disk) }}</span>
                <span class="last-modified">{{ getTimeSince(project.last_scan_date) }}</span>
              </div>
            </div>
            <div class="project-actions">
              <FileExplorerButton
                :project-path="project.path"
                :project-name="project.name"
                size="small"
                :disabled="isLoading"
              />
              <button 
                class="action-btn remove-btn"
                @click.stop="confirmRemoveProject(project)"
                title="Remove project from tracking"
                :disabled="isLoading"
              >
                🗑️
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, computed, onMounted, onUnmounted, onClickOutside} from 'vue'
import {useProjectStore, type Project} from '../../stores/projectStore'
import { useLogStore } from '../../stores/logStore'
import { usePopup } from '../../composables/usePopup'
import FileExplorerButton from '../FileExplorerButton.vue'
import {formatSize, timeSince} from "../../utils.ts";

const { 
  projects, 
  selectedProject, 
  projectCount,
  isLoading,
  setSelectedProject, 
  removeProjects,
  refreshProjects,
  getEngineVersionString 
} = useProjectStore()

const { addLog } = useLogStore()
const { showPopup } = usePopup()

// Search and sorting state
const searchQuery = ref('')
const sortBy = ref<'name' | 'type' | 'size' | 'lastScan' | 'version'>('name')
const sortOrder = ref<'asc' | 'desc'>('asc')
const showSortDropdown = ref(false)

// Timer for updating time-based fields
let timeUpdateInterval: number | null = null
let forceUpdate = ref(0)

// Function to get time since with forced reactivity
const getTimeSince = (date: number) => {
  forceUpdate.value // Add forceUpdate to ensure reactivity
  return timeSince(date)
}

// Fuzzy search function with typo tolerance
const fuzzyMatch = (searchTerm: string, target: string): number => {
  if (!searchTerm) return 1
  
  const search = searchTerm.toLowerCase()
  const text = target.toLowerCase()
  
  // Exact match gets highest score
  if (text.includes(search)) {
    return 1
  }
  
  // Calculate Levenshtein distance for fuzzy matching
  const matrix = Array(search.length + 1).fill(null).map(() => Array(text.length + 1).fill(null))
  
  for (let i = 0; i <= search.length; i++) {
    matrix[i][0] = i
  }
  
  for (let j = 0; j <= text.length; j++) {
    matrix[0][j] = j
  }
  
  for (let i = 1; i <= search.length; i++) {
    for (let j = 1; j <= text.length; j++) {
      const cost = search[i - 1] === text[j - 1] ? 0 : 1
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1,      // deletion
        matrix[i][j - 1] + 1,      // insertion
        matrix[i - 1][j - 1] + cost // substitution
      )
    }
  }
  
  const distance = matrix[search.length][text.length]
  const maxLength = Math.max(search.length, text.length)
  
  // Convert distance to similarity score (0-1)
  const similarity = 1 - (distance / maxLength)
  
  // Only consider matches with similarity > 0.6
  return similarity > 0.6 ? similarity : 0
}

// Filtered and sorted projects
const filteredAndSortedProjects = computed(() => {
  let filtered = projects.value
  
  // Apply search filter
  if (searchQuery.value) {
    const searchResults = projects.value
      .map(project => ({
        project,
        score: fuzzyMatch(searchQuery.value, project.name)
      }))
      .filter(result => result.score > 0)
      .sort((a, b) => b.score - a.score)
      .map(result => result.project)
    
    filtered = searchResults
  }
  
  // Apply sorting
  return filtered.sort((a, b) => {
    let comparison = 0
    
    switch (sortBy.value) {
      case 'name':
        comparison = a.name.localeCompare(b.name)
        break
      case 'type':
        // C++ projects first when ascending, Blueprint first when descending
        if (a.has_cpp === b.has_cpp) {
          comparison = a.name.localeCompare(b.name)
        } else {
          comparison = a.has_cpp ? -1 : 1
        }
        break
      case 'size':
        comparison = a.size_on_disk - b.size_on_disk
        break
      case 'lastScan':
        comparison = a.last_scan_date - b.last_scan_date
        break
      case 'version':
        // Custom engines always on top when ascending
        const aIsCustom = typeof a.engine_association === 'string' && a.engine_association === 'Custom'
        const bIsCustom = typeof b.engine_association === 'string' && b.engine_association === 'Custom'
        
        if (aIsCustom && !bIsCustom) return -1
        if (!aIsCustom && bIsCustom) return 1
        
        // Both custom or both standard - compare versions
        const aVersion = getEngineVersionString(a.engine_association)
        const bVersion = getEngineVersionString(b.engine_association)
        comparison = aVersion.localeCompare(bVersion, undefined, { numeric: true })
        break
    }
    
    return sortOrder.value === 'asc' ? comparison : -comparison
  })
})

const selectProject = (project: Project) => {
  setSelectedProject(project)
  addLog(`Selected project: ${project.name}`)
}

const confirmRemoveProject = async (project: Project) => {
  await removeProjects([project.path])
}

const handleRefresh = async () => {
  try {
    await refreshProjects()
  } catch (error) {
    // Ignore errors, the backend will handle it.
  }
}

const openProjectDiscovery = () => {
  showPopup({
    id: 'project-discovery',
    component: 'ProjectDiscovery',
    props: {}
  })
}

const clearSearch = () => {
  searchQuery.value = ''
}

const toggleSortDropdown = () => {
  showSortDropdown.value = !showSortDropdown.value
}

const setSorting = (newSortBy: typeof sortBy.value, newSortOrder: typeof sortOrder.value) => {
  sortBy.value = newSortBy
  sortOrder.value = newSortOrder
  showSortDropdown.value = false
}

const getSortIcon = () => {
  switch (sortBy.value) {
    case 'name': return '📝'
    case 'type': return sortOrder.value === 'asc' ? '💻' : '🎨'
    case 'size': return '📦'
    case 'lastScan': return '🕒'
    case 'version': return '⚙️'
    default: return '📝'
  }
}

const getSortText = () => {
  switch (sortBy.value) {
    case 'name': return `Name (${sortOrder.value === 'asc' ? 'A-Z' : 'Z-A'})`
    case 'type': return `Type (${sortOrder.value === 'asc' ? 'C++ first' : 'Blueprint first'})`
    case 'size': return `Size (${sortOrder.value === 'asc' ? 'Smallest' : 'Largest'})`
    case 'lastScan': return `Last Scan (${sortOrder.value === 'asc' ? 'Oldest' : 'Newest'})`
    case 'version': return `Version (${sortOrder.value === 'asc' ? 'Custom first' : 'Newest first'})`
    default: return 'Name (A-Z)'
  }
}

// Close dropdown when clicking outside
const sortDropdownRef = ref()
onClickOutside(sortDropdownRef, () => {
  showSortDropdown.value = false
})

// Setup timer for updating time-based fields
onMounted(() => {
  // Update every minute (60 000 ms)
  timeUpdateInterval = window.setInterval(async () => {
    forceUpdate.value = (forceUpdate.value + 1) % 60
  }, 60000)
})

onUnmounted(() => {
  if (timeUpdateInterval) {
    clearInterval(timeUpdateInterval)
    timeUpdateInterval = null
  }
})
</script>

<style scoped>
.project-manager-popup {
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-lg);
  width: 100%;
  max-width: 64rem;
  min-width: 64rem;
  max-height: 85vh;
  min-height: 36rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.popup-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
  flex-shrink: 0;
}

.popup-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.title-icon {
  font-size: var(--icon-size-lg);
}

.close-button {
  background: none;
  border: none;
  font-size: var(--font-size-lg);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-button:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.popup-content {
  flex-grow: 1;
  padding: var(--spacing-lg);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.controls-section {
  display: flex;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-lg);
  flex-shrink: 0;
}

.search-container {
  flex: 1;
}

.search-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  padding-right: 2.5rem;
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background-color: var(--background-color);
  transition: border-color var(--transition-fast);
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.search-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.clear-search-btn {
  position: absolute;
  right: var(--spacing-sm);
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  transition: all var(--transition-fast);
  font-size: var(--font-size-sm);
}

.clear-search-btn:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.sort-container {
  position: relative;
}

.sort-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-sm);
  background-color: var(--surface-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
  white-space: nowrap;
}

.sort-button:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.sort-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sort-icon {
  font-size: var(--font-size-md);
}

.dropdown-arrow {
  font-size: var(--font-size-xs);
  transition: transform var(--transition-fast);
}

.dropdown-arrow.open {
  transform: rotate(180deg);
}

.sort-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  z-index: 1000;
  background-color: var(--background-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  box-shadow: var(--shadow-md);
  padding: var(--spacing-sm);
  min-width: 16rem;
  max-height: 20rem;
  overflow-y: auto;
}

.sort-group {
  margin-bottom: var(--spacing-sm);
}

.sort-group:last-child {
  margin-bottom: 0;
}

.sort-group-title {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: var(--spacing-xs);
  padding: 0 var(--spacing-sm);
}

.sort-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  width: 100%;
  padding: var(--spacing-sm);
  border: none;
  background: none;
  text-align: left;
  cursor: pointer;
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  transition: all var(--transition-fast);
}

.sort-option:hover {
  background-color: var(--hover-color);
}

.sort-option.active {
  background-color: var(--accent-color-alpha);
  color: var(--accent-color);
  font-weight: var(--font-weight-medium);
}

.sort-option-icon {
  font-size: var(--font-size-sm);
}

.projects-section {
  flex-grow: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md);
  flex-shrink: 0;
}

.section-title {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  margin: 0;
}

.search-indicator {
  font-weight: var(--font-weight-normal);
  color: var(--text-secondary);
  font-style: italic;
}

.header-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.discover-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  background: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  transition: all var(--transition-fast);
}

.discover-btn:hover:not(:disabled) {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.discover-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.refresh-btn {
  background: none;
  border: var(--border-width) solid var(--border-color);
  cursor: pointer;
  font-size: var(--font-size-md);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  width: 2rem;
  height: 2rem;
  background-color: var(--surface-color);
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button-icon {
  font-size: var(--font-size-sm);
}

.no-projects,
.no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  text-align: center;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
  flex-grow: 1;
}

.no-projects-icon,
.no-results-icon {
  font-size: var(--icon-size-xl);
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.no-projects-text,
.no-results-text {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-projects-subtext,
.no-results-subtext {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  opacity: 0.7;
  margin-bottom: var(--spacing-md);
}

.clear-search-button {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--accent-color);
  border: var(--border-width) solid var(--accent-color);
  color: white;
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
}

.clear-search-button:hover {
  background-color: #2c5aa0;
  border-color: #2c5aa0;
}

.projects-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  overflow-y: auto;
  flex-grow: 1;
}

.project-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  transition: all var(--transition-fast);
  cursor: pointer;
}

.project-item:hover {
  border-color: var(--accent-color);
  background-color: var(--hover-color);
}

.project-item.selected {
  border-color: var(--accent-color);
  background-color: var(--accent-color-alpha);
}

.project-info {
  flex-grow: 1;
  margin-right: var(--spacing-lg);
  min-width: 0;
}

.project-name {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.project-path {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-family: var(--font-mono);
  margin-bottom: var(--spacing-xs);
  word-break: break-all;
  overflow-wrap: break-word;
}

.project-meta {
  display: flex;
  gap: var(--spacing-md);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  flex-wrap: wrap;
}

.project-actions {
  display: flex;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.action-btn {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--surface-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  width: 1.75rem;
  height: 1.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn:hover:not(:disabled) {
  background-color: var(--hover-color);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.remove-btn:hover:not(:disabled) {
  border-color: #e53e3e;
  background-color: #fed7d7;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .project-manager-popup {
    min-width: 90vw;
    max-width: 90vw;
  }
  
  .controls-section {
    flex-direction: column;
  }
  
  .project-meta {
    flex-direction: column;
    gap: var(--spacing-xs);
  }
}
</style>