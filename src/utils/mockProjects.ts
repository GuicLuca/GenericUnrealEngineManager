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
        name: "EnhancedInput",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Editor", "Game"]
      },
      {
        name: "Niagara",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Editor", "Game"]
      },
      {
        name: "GameplayAbilities",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game"]
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
        name: "OnlineSubsystem",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game", "Server"]
      },
      {
        name: "ReplicationGraph",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game", "Server"]
      },
      {
        name: "NetworkPrediction",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game"]
      },
      {
        name: "WeaponSystem",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        target_allow_list: ["Game"]
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
        name: "ProceduralMeshComponent",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Editor", "Game"]
      },
      {
        name: "CustomRenderer",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        target_allow_list: ["Editor", "Game"]
      },
      {
        name: "ProprietaryTools",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        target_allow_list: ["Editor"]
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
        name: "MobileUtils",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game"]
      },
      {
        name: "InAppPurchase",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game"]
      },
      {
        name: "PuzzleFramework",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "https://www.unrealengine.com/marketplace/puzzle-framework",
        target_allow_list: ["Game"]
      },
      {
        name: "SocialPlugin",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "https://www.unrealengine.com/marketplace/social-integration",
        target_allow_list: ["Game"]
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
        name: "VRTemplate",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game"]
      },
      {
        name: "OpenXR",
        is_enabled: true,
        is_in_project: false,
        marketplace_url: null,
        target_allow_list: ["Game"]
      },
      {
        name: "HandTracking",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: null,
        target_allow_list: ["Game"]
      },
      {
        name: "HapticFeedback",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "https://www.unrealengine.com/marketplace/haptic-suite",
        target_allow_list: ["Game"]
      },
      {
        name: "ArchViz",
        is_enabled: true,
        is_in_project: true,
        marketplace_url: "https://www.unrealengine.com/marketplace/archviz-tools",
        target_allow_list: ["Editor", "Game"]
      }
    ],
    size_on_disk: 4294967296, // 4GB
    last_scan_date: Math.floor(Date.now() / 1000) - 300 // 5 minutes ago
  }
]