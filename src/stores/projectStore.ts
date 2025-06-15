import { ref, computed } from 'vue'


export interface Project {
  name: string,        // Name of the project (from .uproject file)
  description: string, // Description of the project (from .uproject file)
  engine_association: string, // Engine version or "Custom" for Unreal Source
  path: string,       // Path to the project (.uproject file)
  has_cpp: boolean,       // Indicates if the project has C++ code
  plugins: ProjectPlugin[], // List of plugins associated with the project
}

export interface ProjectPlugin {
  name: string, // Name of the plugin (from .uplugin file)
  is_enabled: boolean, // Indicates if the plugin is enabled
  is_in_project: boolean, // Indicates if the plugin is part of the project (in ./Plugins directory) or in the engine (in ENGINE/Plugins/... directory)
  marketplace_url: string | null, // URL to the plugin on the Unreal Marketplace, if available
  target_allow_list: string[], // List of target the plugin is embedded in (e.g., "Editor", "Game", etc.)
}

// Global project state
const selectedProject = ref<Project | null>(null)
const projects = ref<Project[]>([])

export const useProjectStore = () => {
  const setSelectedProject = (project: Project | null) => {
    selectedProject.value = project
  }

  const addProject = (project: Project) => {
    const existingIndex = projects.value.findIndex(p => p.id === project.id)
    if (existingIndex >= 0) {
      projects.value[existingIndex] = project
    } else {
      projects.value.push(project)
    }
  }

  const removeProject = (projectId: string) => {
    projects.value = projects.value.filter(p => p.id !== projectId)
    if (selectedProject.value?.id === projectId) {
      selectedProject.value = null
    }
  }

  const hasSelectedProject = computed(() => selectedProject.value !== null)

  return {
    selectedProject,
    projects,
    hasSelectedProject,
    setSelectedProject,
    addProject,
    removeProject
  }
}