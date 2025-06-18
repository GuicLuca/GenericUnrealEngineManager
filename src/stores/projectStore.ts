import {ref, computed} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen} from "@tauri-apps/api/event";

// Match the backend Project structure
export interface Project {
    name: string
    description: string
    engine_association: EngineAssociation
    path: string
    has_cpp: boolean
    plugins: ProjectPlugin[]
    size_on_disk: number // Size in bytes
    last_scan_date: number // Duration since UNIX epoch in seconds
}

// Match the backend EngineAssociation enum
export type EngineAssociation =
    | { Standard: string }
    | "Custom"

// Match the backend ProjectPlugin structure
export interface ProjectPlugin {
    name: string
    is_enabled: boolean
    is_in_project: boolean
    marketplace_url: string | null
    target_allow_list: string[]
}

// Global project state
const selectedProject = ref<Project | null>(null)
const projects = ref<Project[]>([])
const isLoading = ref(false)

export const useProjectStore = () => {
    // Initialize the store by listening to backend events
    const initializeStore = async () => {
        try {
            // Listen for the app initialization event
            await listen('app_initialized', (event: any) => {
                const payload = event.payload
                if (payload && payload.projects) {
                    setProjects(payload.projects)

                }
            })

            // Listen for project updates from the backend
            await listen('projects_updated', (event: any) => {
                const payload = event.payload
                if (payload && payload.projects) {
                    setProjects(payload.projects)
                }
            })

            // request an initial project list from the backend
            await refreshProjects();
        } catch (error) {
            console.error('Failed to initialize project store listeners:', error)
        }
    }

    // Set projects from the backend (internal function)
    const setProjects = (backendProjects: Project[]) => {
        projects.value = backendProjects

        // If the current selected project is no longer in the list, clear selection
        if (selectedProject.value) {
            const stillExists = backendProjects.some(p =>
                p.path === selectedProject.value?.path
            )
            if (!stillExists) {
                selectedProject.value = null
            }
        }
    }

    // Set the selected project (UI only)
    const setSelectedProject = (project: Project | null) => {
        selectedProject.value = project
    }

    // Discover projects by calling backend
    const discoverProjects = async (request: {
        base_folder: string
        ignore_engine: boolean
        ignore_templates: boolean
        ignore_samples: boolean
    }) => {
        try {
            isLoading.value = true
            // Backend will emit events to update the store
            return await invoke('discover_projects', {request})
        } catch (error) {
            console.error('Failed to discover projects:', error)
            throw error
        } finally {
            isLoading.value = false
        }
    }

    // Remove projects by calling backend
    const removeProjects = async (projectPaths: string[]) => {
        try {
            isLoading.value = true
            await invoke('remove_projects', {projectPaths})
            // Backend will emit events to update the store
        } catch (error) {
            console.error('Failed to remove projects:', error)
            throw error
        } finally {
            isLoading.value = false
        }
    }

    // Get projects from the backend (refresh the project list)
    const refreshProjects = async () => {
        try {
            isLoading.value = true
            const backendProjects = await invoke('get_projects')
            setProjects(backendProjects as Project[])
        } catch (error) {
            console.error('Failed to refresh projects:', error)
            throw error
        } finally {
            isLoading.value = false
        }
    }

    // Helper to get engine version string
    const getEngineVersionString = (engineAssociation: EngineAssociation): string => {
        if (typeof engineAssociation === 'string' && engineAssociation === 'Custom') {
            return 'Custom'
        }
        if (typeof engineAssociation === 'object' && engineAssociation.Standard) {
            return engineAssociation.Standard
        }
        return 'Unknown'
    }

    // Helper to find a project by path
    const findProjectByPath = (path: string): Project | undefined => {
        return projects.value.find(p => p.path === path)
    }

    const hasSelectedProject = computed(() => selectedProject.value !== null)
    const projectCount = computed(() => projects.value.length)

    return {
        // State
        selectedProject,
        projects,
        isLoading,

        // Computed
        hasSelectedProject,
        projectCount,

        // Actions
        initializeStore,
        setSelectedProject,
        discoverProjects,
        removeProjects,
        refreshProjects,

        // Helpers
        getEngineVersionString,
        findProjectByPath
    }
}