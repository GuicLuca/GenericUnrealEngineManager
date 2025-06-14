import { ref, computed } from 'vue'

export interface Project {
  id: string
  name: string
  path: string
  engineVersion: string
  description: string
  lastScan: string
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