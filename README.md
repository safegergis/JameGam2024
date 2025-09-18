# JameGam 2024 - Christmas Survival Game 🎄

A festive survival game built for JameGam 2024 using Rust and the Bevy game engine. Battle waves of enemies as a snowman wielding candy cane shurikens and snowballs in this action-packed Christmas adventure.

## 🎮 Game Features

- **Survival Gameplay**: Fight endless waves of increasingly difficult enemies
- **Character Progression**: XP-based leveling system with stat upgrades
- **Combat System**: Multiple weapon types with unique mechanics:
  - Snowballs with knockback and piercing
  - Candy cane shurikens
  - Shield rotation system
- **Status Effects**: Advanced elemental system with freeze and fire mechanics
  - Flash Freeze: Freezing burning enemies deals percentage damage
  - Freezer Burn: Burning frozen enemies become vulnerable
- **Dynamic Enemy Spawning**: Adaptive wave system with increasing difficulty
- **Audio Integration**: Background music and sound effects
- **UI Systems**: Health bars, XP tracking, upgrade menus

## 🛠 Technical Implementation

### Architecture & Design Patterns
- **Plugin-based Architecture**: Modular system using Bevy's plugin pattern
- **Entity Component System (ECS)**: Leveraging Bevy's high-performance ECS architecture
- **State Management**: Multiple game states (MainMenu, InGame, GameOver, etc.)
- **Resource Management**: Centralized game stats and configuration

### Key Systems
- **Collision Detection**: Custom collision system with invincibility frames
- **Animation System**: Sprite-based animations with timing controls
- **Particle Effects**: Visual effects using bevy_hanabi
- **Camera System**: Multi-camera setup with resolution scaling
- **Audio Engine**: Sound management with volume controls

### Code Organization
```
src/
├── main.rs          # Application entry point and plugin registration
├── player.rs        # Player mechanics, stats, and controls
├── enemy.rs         # Enemy AI, spawning, and behavior
├── collision.rs     # Collision detection and damage systems
├── pickup.rs        # Item collection and power-ups
├── ui.rs           # User interface and HUD elements
├── audio.rs        # Sound and music management
├── camera.rs       # Camera controls and rendering
├── background.rs   # Background rendering and effects
├── mainmenu.rs     # Menu systems and navigation
└── utils.rs        # Shared utilities and helper functions
```

## 🚀 Technologies Used

- **Rust**: Systems programming language for performance and memory safety
- **Bevy Engine**: Modern ECS-based game engine
- **bevy_hanabi**: Particle effects system
- **bevy_egui**: Immediate mode GUI integration
- **Audio**: OGG format sound effects and background music

## 🔧 Performance Optimizations

- **Optimized Build Profile**: Custom compiler optimizations for development
- **Dynamic Linking**: Faster compilation during development
- **Y-Sort Rendering**: Efficient sprite layering system
- **Resource Pooling**: Efficient memory management for game objects

## 🎯 Game Mechanics Highlights

### Combat System
- Multi-layered damage calculation
- Projectile physics with bouncing and piercing
- Shield mechanics with rotation and effect application
- Elemental interaction system (fire + freeze combinations)

### Progression System
- XP-based character advancement
- Multiple upgrade paths for different playstyles
- Stat scaling for damage, speed, and special abilities

### Enemy AI
- Wave-based spawning with escalating difficulty
- Different enemy types with unique behaviors
- Adaptive spawn timing based on player performance

## 🛠 Build & Run

```bash
# Clone the repository
git clone <repository-url>
cd JameGam2024

# Run in development mode
cargo run

# Build optimized release
cargo build --release
```

## 📊 Project Stats

- **Language**: Rust
- **Framework**: Bevy 0.15.0
- **Lines of Code**: ~2000+ (estimated across all modules)
- **Development Time**: Game jam project (limited timeframe)
- **Architecture**: Component-based with plugin system

## 🎨 Assets & Art

- Custom Christmas-themed sprite art
- Particle effects for combat feedback
- Background music and sound effects
- UI elements and icons

## 🏆 Technical Achievements

- **Real-time Performance**: 60 FPS gameplay with complex systems
- **Memory Safety**: Zero-cost abstractions using Rust
- **Modular Design**: Plugin-based architecture for easy feature addition
- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Advanced Graphics**: Particle systems and visual effects

---

*This project demonstrates proficiency in Rust programming, game development, systems architecture, and performance optimization. The codebase showcases clean separation of concerns, efficient algorithms, and modern software engineering practices.*