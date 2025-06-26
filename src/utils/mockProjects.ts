import type { Project } from '../stores/projectStore'

export const mockProjects: Project[] = [
  {
    name: "ActionRPG_Blueprint",
    description: "A third-person action RPG with magic system and inventory management. Features include character progression, quest system, and dynamic weather.",
    engine_association: { Standard: "5.3" },
    path: "C:/UnrealProjects/ActionRPG_Blueprint/ActionRPG_Blueprint.uproject",
    has_cpp: false,
    plugins: [
      {
        name: "Enhanced Input",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: null,
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 3600
      },
      {
        name: "Niagara",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.3/en-US/overview-of-niagara-effects-for-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 3600
      },
      {
        name: "Gameplay Abilities",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.3/en-US/gameplay-ability-system-for-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 3600
      },
      {
        name: "Magic System",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/magic-system-pro",
        docs_url: "https://docs.magicsystem.com/getting-started",
        size_on_disk: 45678912, // ~43.5 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 3600
      },
      {
        name: "Inventory Framework",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/inventory-framework",
        docs_url: null,
        size_on_disk: 23456789, // ~22.4 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 3600
      }
    ],
    size_on_disk: 2147483648, // 2GB
    last_scan_date: Math.floor(Date.now() / 1000) - 3600 // 1 hour ago
  },
  {
    name: "ShooterGame_CPP",
    description: "Multiplayer first-person shooter with advanced networking features. Includes weapon customization, team-based gameplay, and anti-cheat integration.",
    engine_association: { Standard: "5.4" },
    path: "D:/GameDev/ShooterGame_CPP/ShooterGame_CPP.uproject",
    has_cpp: true,
    plugins: [
      {
        name: "Online Subsystem",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.4/en-US/online-subsystem-in-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 1800
      },
      {
        name: "Replication Graph",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.4/en-US/replication-graph-in-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 1800
      },
      {
        name: "Network Prediction",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: null,
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 1800
      },
      {
        name: "Weapon System Pro",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/weapon-system-pro",
        docs_url: "https://weaponsystempro.com/documentation",
        size_on_disk: 156789123, // ~149.5 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 1800
      },
      {
        name: "Anti-Cheat Integration",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        docs_url: "https://github.com/anticheat/ue-integration/wiki",
        size_on_disk: 34567890, // ~33 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 1800
      },
      {
        name: "Advanced Locomotion",
        is_enabled: false,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/advanced-locomotion-system",
        docs_url: "https://als.community/documentation",
        size_on_disk: 78901234, // ~75.3 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 1800
      }
    ],
    size_on_disk: 5368709120, // 5GB
    last_scan_date: Math.floor(Date.now() / 1000) - 1800 // 30 minutes ago
  },
  {
    name: "CustomEngine_Project",
    description: "Experimental project using a custom Unreal Engine build with proprietary rendering features and custom tools for procedural content generation.",
    engine_association: "Custom",
    path: "E:/CustomEngine/Projects/CustomEngine_Project/CustomEngine_Project.uproject",
    has_cpp: true,
    plugins: [
      {
        name: "Procedural Mesh Component",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.2/en-US/procedural-mesh-component-in-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 7200
      },
      {
        name: "Custom Renderer",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        docs_url: null,
        size_on_disk: 234567890, // ~223.7 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 7200
      },
      {
        name: "Proprietary Tools",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        docs_url: "https://internal.company.com/tools/documentation",
        size_on_disk: 123456789, // ~117.7 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 7200
      },
      {
        name: "Procedural Generation Suite",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        docs_url: null,
        size_on_disk: 345678901, // ~329.6 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 7200
      }
    ],
    size_on_disk: 8589934592, // 8GB
    last_scan_date: Math.floor(Date.now() / 1000) - 7200 // 2 hours ago
  },
  {
    name: "MobileGame_UE4",
    description: "Cross-platform mobile puzzle game optimized for iOS and Android. Features touch controls, in-app purchases, and social integration.",
    engine_association: { Standard: "4.27" },
    path: "C:/MobileProjects/MobileGame_UE4/MobileGame_UE4.uproject",
    has_cpp: false,
    plugins: [
      {
        name: "Mobile Utils",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/4.27/en-US/SharingAndReleasing/Mobile/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 86400
      },
      {
        name: "In App Purchase",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: null,
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 86400
      },
      {
        name: "Puzzle Framework",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/puzzle-framework",
        docs_url: "https://puzzleframework.dev/docs",
        size_on_disk: 67890123, // ~64.7 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 86400
      },
      {
        name: "Social Integration",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/social-integration",
        docs_url: null,
        size_on_disk: 12345678, // ~11.8 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 86400
      },
      {
        name: "Touch Controls Pro",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/touch-controls-pro",
        docs_url: "https://touchcontrolspro.com/documentation",
        size_on_disk: 8901234, // ~8.5 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 86400
      }
    ],
    size_on_disk: 1073741824, // 1GB
    last_scan_date: Math.floor(Date.now() / 1000) - 86400 // 1 day ago
  },
  {
    name: "VR_Experience",
    description: "Immersive virtual reality experience with hand tracking and haptic feedback. Designed for architectural visualization and training simulations.",
    engine_association: { Standard: "5.2" },
    path: "F:/VRProjects/VR_Experience/VR_Experience.uproject",
    has_cpp: true,
    plugins: [
      {
        name: "VR Template",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.2/en-US/vr-template-in-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 300
      },
      {
        name: "OpenXR",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        docs_url: "https://docs.unrealengine.com/5.2/en-US/openxr-in-unreal-engine/",
        size_on_disk: null,
        last_scan_date: Math.floor(Date.now() / 1000) - 300
      },
      {
        name: "Hand Tracking Pro",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/hand-tracking-pro",
        docs_url: "https://handtrackingpro.com/docs",
        size_on_disk: 89012345, // ~84.9 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 300
      },
      {
        name: "Haptic Feedback Suite",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/haptic-suite",
        docs_url: null,
        size_on_disk: 45678901, // ~43.5 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 300
      },
      {
        name: "ArchViz Tools",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "com.epicgames.launcher://ue/marketplace/content/archviz-tools",
        docs_url: "https://archviztools.com/documentation",
        size_on_disk: 123456789, // ~117.7 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 300
      },
      {
        name: "VR Interaction Framework",
        is_enabled: false,
        is_in_project: true,
        marketplace_url: null,
        docs_url: "https://github.com/vrinteraction/framework/wiki",
        size_on_disk: 56789012, // ~54.1 MB
        last_scan_date: Math.floor(Date.now() / 1000) - 300
      }
    ],
    size_on_disk: 4294967296, // 4GB
    last_scan_date: Math.floor(Date.now() / 1000) - 300 // 5 minutes ago
  }
]