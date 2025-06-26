<template>
  <div class="plugins-view">
    <div class="plugins-header">
      <div class="header-left">
        <h2 class="plugins-title">
          <span class="title-icon">🔌</span>
          Project Plugins
        </h2>
        <div class="plugins-count">{{ pluginCount }} plugin(s)</div>
      </div>
    </div>

    <div v-if="!selectedProject" class="no-project">
      <div class="no-project-icon">📂</div>
      <div class="no-project-text">No project selected</div>
      <div class="no-project-subtext">Select a project to view its plugins</div>
    </div>

    <div v-else-if="plugins.length === 0" class="no-plugins">
      <div class="no-plugins-icon">🔌</div>
      <div class="no-plugins-text">No plugins found</div>
      <div class="no-plugins-subtext">This project doesn't have any plugins</div>
    </div>

    <div v-else class="plugins-content">
      <!-- Filter Tabs -->
      <div class="filter-tabs">
        <button 
          v-for="filter in filterOptions"
          :key="filter.id"
          class="filter-tab"
          :class="{ active: activeFilter === filter.id }"
          @click="activeFilter = filter.id"
        >
          <span class="filter-icon">{{ filter.icon }}</span>
          <span class="filter-label">{{ filter.label }}</span>
          <span class="filter-count">({{ getFilteredPlugins(filter.id).length }})</span>
        </button>
      </div>

      <!-- Plugins List -->
      <div class="plugins-list">
        <div 
          v-for="plugin in filteredPlugins"
          :key="plugin.name"
          class="plugin-item"
          :class="{ 
            'plugin-disabled': !plugin.is_enabled,
            'plugin-external': !plugin.is_in_project 
          }"
        >
          <div class="plugin-header">
            <div class="plugin-name-section">
              <div class="plugin-name">{{ plugin.name }}</div>
              <div class="plugin-badges">
                <span 
                  v-if="!plugin.is_enabled" 
                  class="plugin-badge disabled"
                  title="Plugin is disabled"
                >
                  Disabled
                </span>
                <span 
                  v-if="!plugin.is_in_project" 
                  class="plugin-badge external"
                  title="Engine or external plugin"
                >
                  External
                </span>
                <span 
                  v-if="plugin.is_in_project" 
                  class="plugin-badge project"
                  title="Project plugin"
                >
                  Project
                </span>
                <span 
                  v-if="plugin.marketplace_url" 
                  class="plugin-badge marketplace"
                  title="Available on Marketplace"
                >
                  Marketplace
                </span>
              </div>
            </div>
            <div class="plugin-actions">
              <button
                v-if="plugin.marketplace_url"
                class="action-btn marketplace-btn"
                @click="openMarketplaceUrl(plugin.marketplace_url)"
                title="Open in Marketplace"
              >
                🛒
              </button>
              <button
                v-if="plugin.docs_url"
                class="action-btn docs-btn"
                @click="openDocsUrl(plugin.docs_url)"
                title="Open Documentation"
              >
                📖
              </button>
              <FileExplorerButton
                v-if="plugin.is_in_project"
                :project-path="getPluginPath(plugin.name)"
                :project-name="`${plugin.name} Plugin`"
                size="small"
                title="Open plugin folder"
              />
            </div>
          </div>

          <div class="plugin-details">
            <div class="plugin-meta">
              <div class="meta-item">
                <span class="meta-label">Status:</span>
                <span class="meta-value" :class="{ 'status-enabled': plugin.is_enabled, 'status-disabled': !plugin.is_enabled }">
                  {{ plugin.is_enabled ? 'Enabled' : 'Disabled' }}
                </span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Location:</span>
                <span class="meta-value">{{ plugin.is_in_project ? 'Project' : 'Engine/External' }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Size:</span>
                <span class="meta-value">{{ plugin.size_on_disk !== null ? formatSize(plugin.size_on_disk) : 'N/A' }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useProjectStore, type ProjectPlugin } from '../stores/projectStore'
import { useLogStore } from '../stores/logStore'
import FileExplorerButton from './FileExplorerButton.vue'
import { formatSize, timeSince } from '../utils'

const { selectedProject, isLoading } = useProjectStore()
const { addLog } = useLogStore()

const activeFilter = ref('all')

const filterOptions = [
  { id: 'all', label: 'All', icon: '🔌' },
  { id: 'enabled', label: 'Enabled', icon: '✅' },
  { id: 'disabled', label: 'Disabled', icon: '❌' },
  { id: 'project', label: 'Project', icon: '📁' },
  { id: 'external', label: 'External', icon: '🌐' },
  { id: 'marketplace', label: 'Marketplace', icon: '🛒' }
]

const plugins = computed(() => selectedProject.value?.plugins || [])
const pluginCount = computed(() => plugins.value.length)

const getFilteredPlugins = (filterId: string): ProjectPlugin[] => {
  switch (filterId) {
    case 'enabled':
      return plugins.value.filter(p => p.is_enabled)
    case 'disabled':
      return plugins.value.filter(p => !p.is_enabled)
    case 'project':
      return plugins.value.filter(p => p.is_in_project)
    case 'external':
      return plugins.value.filter(p => !p.is_in_project)
    case 'marketplace':
      return plugins.value.filter(p => p.marketplace_url !== null && p.marketplace_url !== undefined)
    default:
      return plugins.value
  }
}

const filteredPlugins = computed(() => getFilteredPlugins(activeFilter.value))

const getPluginPath = (pluginName: string): string => {
  if (!selectedProject.value) return ''
  
  // Extract the project directory and construct the plugin path
  const projectDir = selectedProject.value.path.replace(/[^/\\]*\.uproject$/, '')
  return `${projectDir}Plugins/${pluginName}`
}

const openMarketplaceUrl = (url: string) => {
  // Open the marketplace URL in the default browser
  window.open(url, '_blank')
  addLog(`Opened marketplace URL: ${url}`)
}

const openDocsUrl = (url: string) => {
  // Open the documentation URL in the default browser
  window.open(url, '_blank')
  addLog(`Opened documentation URL: ${url}`)
}
</script>

<style scoped>
.plugins-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--background-color);
}

.plugins-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.plugins-title {
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

.plugins-count {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  background-color: var(--background-color);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  border: var(--border-width) solid var(--border-color);
}

.no-project,
.no-plugins {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex-grow: 1;
  text-align: center;
  padding: var(--spacing-xl);
}

.no-project-icon,
.no-plugins-icon {
  font-size: var(--icon-size-xl);
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.no-project-text,
.no-plugins-text {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-project-subtext,
.no-plugins-subtext {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  opacity: 0.7;
}

.plugins-content {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.filter-tabs {
  display: flex;
  background-color: var(--surface-color);
  border-bottom: var(--border-width) solid var(--border-color);
  overflow-x: auto;
  flex-shrink: 0;
}

.filter-tab {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  background-color: transparent;
  border-right: var(--border-width) solid var(--border-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  font-weight: var(--font-weight-medium);
  white-space: nowrap;
  position: relative;
  border-bottom: 2px solid transparent;
}

.filter-tab:hover {
  background-color: var(--hover-color);
  color: var(--text-primary);
}

.filter-tab.active {
  background-color: var(--background-color);
  color: var(--text-primary);
  border-bottom-color: var(--accent-color);
}

.filter-icon {
  font-size: var(--font-size-sm);
}

.filter-count {
  opacity: 0.7;
}

.plugins-list {
  flex-grow: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.plugin-item {
  background-color: var(--surface-color);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--border-radius-md);
  margin-bottom: var(--spacing-md);
  padding: var(--spacing-md);
  transition: all var(--transition-fast);
}

.plugin-item:last-child {
  margin-bottom: 0;
}

.plugin-item:hover {
  border-color: var(--accent-color);
  box-shadow: var(--shadow-sm);
}

.plugin-item.plugin-disabled {
  opacity: 0.7;
}

.plugin-item.plugin-external {
  border-left: 3px solid #f56500;
}

.plugin-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: var(--spacing-sm);
}

.plugin-name-section {
  flex-grow: 1;
  margin-right: var(--spacing-md);
  min-width: 0;
}

.plugin-name {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
  word-break: break-word;
}

.plugin-badges {
  display: flex;
  gap: var(--spacing-xs);
  flex-wrap: wrap;
}

.plugin-badge {
  font-size: var(--font-size-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-weight: var(--font-weight-medium);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.plugin-badge.disabled {
  background-color: #fed7d7;
  color: #c53030;
}

.plugin-badge.external {
  background-color: #fef5e7;
  color: #d69e2e;
}

.plugin-badge.project {
  background-color: #e6fffa;
  color: #319795;
}

.plugin-badge.marketplace {
  background-color: #e6f3ff;
  color: #3182ce;
}

.plugin-actions {
  display: flex;
  gap: var(--spacing-xs);
  flex-shrink: 0;
  align-items: flex-start;
}

.action-btn {
  padding: var(--spacing-xs);
  border: var(--border-width) solid var(--border-color);
  background-color: var(--background-color);
  border-radius: var(--border-radius-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
  width: 1.75rem;
  height: 1.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.action-btn:hover {
  background-color: var(--hover-color);
  border-color: var(--accent-color);
}

.marketplace-btn:hover {
  background-color: #e6f3ff;
  border-color: #3182ce;
}

.docs-btn:hover {
  background-color: #f0fff4;
  border-color: #38a169;
}

.plugin-details {
  border-top: var(--border-width) solid var(--border-color);
  padding-top: var(--spacing-sm);
}

.plugin-meta {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-sm);
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  min-width: 0;
}

.meta-label {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.meta-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
  word-break: break-word;
}

.meta-value.status-enabled {
  color: #38a169;
  font-weight: var(--font-weight-medium);
}

.meta-value.status-disabled {
  color: #e53e3e;
  font-weight: var(--font-weight-medium);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .plugin-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm);
  }
  
  .plugin-actions {
    align-self: flex-end;
  }
  
  .plugin-meta {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 480px) {
  .plugin-meta {
    grid-template-columns: 1fr;
  }
}
</style>