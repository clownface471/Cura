# CuraUI - The Face

**Layer: User Interface (Ring 3)**

## ⚠️ CRITICAL: Don't Start Here!

> "Jangan Mulai dari UI. UI itu gampang. Kernel itu neraka."

This is Phase 3-4 work. Focus on the kernel driver and service first!

## Overview

CuraUI is the user-facing interface for CURA. It's built with Flutter for native Windows performance and beautiful, data-driven visualizations.

```
┌─────────────────────────────────┐
│      CuraUI.exe (Flutter)       │
├─────────────────────────────────┤
│                                 │
│  ┌───────────────────────┐      │
│  │   System Health       │      │
│  │   ┌─────────────┐     │      │
│  │   │ ██████░░░░░ │ 87% │      │
│  │   └─────────────┘     │      │
│  └───────────────────────┘      │
│                                 │
│  ┌───────────────────────┐      │
│  │   Recent Threats      │      │
│  │   🛡️ Blocked: 3       │      │
│  │   🔍 Vault: 1         │      │
│  │   ✅ Clean: 127       │      │
│  └───────────────────────┘      │
│                                 │
│  ┌───────────────────────┐      │
│  │   Active Mode         │      │
│  │   ⚡ Turbo (RTX)      │      │
│  └───────────────────────┘      │
│                                 │
└─────────────────────────────────┘
         ↕ Named Pipe
   ┌─────────────────┐
   │  CuraCore.exe   │
   └─────────────────┘
```

## Design Philosophy

### Minimalist & Data-Driven
- NO skeuomorphic "virus scanner" animations
- NO green/red flashy warnings
- YES clean, modern, information-dense UI
- YES subtle animations that convey system state

### Visual Metaphors
- **System Health**: Organic, living system visualization
- **Threat Timeline**: Clean event stream, not scary virus icons
- **Vault**: Glass container showing isolated threats

### Inspiration
- **Windows 11 Settings**: Clean, modern, familiar
- **Task Manager (Win11)**: Performance graphs
- **Notion**: Minimalist data presentation

## Phase 3 Goals (Months 17-24)

### Month 17-18: Flutter Setup
- [ ] Install Flutter SDK for Windows
- [ ] Create Flutter desktop project
- [ ] Set up platform channels (Dart ↔ C++)

### Month 19-20: Core UI
- [ ] Dashboard screen (system health)
- [ ] Threat history screen
- [ ] Settings screen
- [ ] Vault viewer

### Month 21-22: Integration
- [ ] Named pipe communication with CuraCore
- [ ] Real-time status updates
- [ ] Notification system (Windows Toast)

### Month 23-24: Polish
- [ ] Animations and transitions
- [ ] Dark/Light theme
- [ ] Accessibility (screen reader support)
- [ ] **MILESTONE**: CURA Alpha with full UI

## Screen Layouts

### 1. Dashboard (Home)
```
╔═══════════════════════════════════╗
║  CURA                    ⚙️  👤   ║
╠═══════════════════════════════════╣
║                                   ║
║   System Health: Excellent        ║
║   ┌──────────────────────┐        ║
║   │  ████████████░░  92% │        ║
║   └──────────────────────┘        ║
║                                   ║
║   Mode: ⚡ Turbo                  ║
║   GPU: NVIDIA RTX 5060            ║
║   Inference: 47ms avg             ║
║                                   ║
║   ┌─────────────────────────────┐ ║
║   │ Recent Activity             │ ║
║   ├─────────────────────────────┤ ║
║   │ 2m ago  chrome.exe  ✅      │ ║
║   │ 5m ago  discord.exe ✅      │ ║
║   │ 8m ago  unknown.exe 🛡️      │ ║
║   └─────────────────────────────┘ ║
║                                   ║
╚═══════════════════════════════════╝
```

### 2. Vault (Quarantine)
```
╔═══════════════════════════════════╗
║  🔬 The Vault                     ║
╠═══════════════════════════════════╣
║                                   ║
║  Active Containment: 2 items      ║
║                                   ║
║  ┌───────────────────────────┐    ║
║  │ 🧪 suspicious.exe         │    ║
║  │ ├─ Threat: 68%            │    ║
║  │ ├─ Captured: 12m ago      │    ║
║  │ └─ Behavior: Network scan │    ║
║  │    [Analyze] [Delete]     │    ║
║  └───────────────────────────┘    ║
║                                   ║
║  ┌───────────────────────────┐    ║
║  │ 🧪 crack.exe              │    ║
║  │ ├─ Threat: 54%            │    ║
║  │ ├─ Captured: 1h ago       │    ║
║  │ └─ Behavior: Registry mod │    ║
║  │    [Analyze] [Delete]     │    ║
║  └───────────────────────────┘    ║
║                                   ║
╚═══════════════════════════════════╝
```

### 3. Settings
```
╔═══════════════════════════════════╗
║  ⚙️ Settings                      ║
╠═══════════════════════════════════╣
║                                   ║
║  Performance                      ║
║  ○ Eco (NPU)   ● Turbo (RTX)      ║
║  □ Auto-switch based on power     ║
║                                   ║
║  Protection                       ║
║  ☑ Real-time scanning             ║
║  ☑ Snapshot protection            ║
║  ☑ Vault (sandbox unknown files)  ║
║                                   ║
║  Updates                          ║
║  Last check: 2 days ago           ║
║  [Check for updates]              ║
║                                   ║
║  License                          ║
║  Status: ✅ Active                ║
║  Next validation: 5 days          ║
║                                   ║
╚═══════════════════════════════════╝
```

## Technical Stack

### Framework
- **Flutter 3.x** (Stable channel)
- **Dart 3.x**

### Windows Integration
- **win32** package (Windows API access)
- **ffi** package (C++ interop)

### UI Libraries
- **fl_chart** (Performance graphs)
- **animations** (Smooth transitions)
- **provider** (State management)

## Communication Protocol

### Named Pipe (CuraCore → UI)
```dart
// Connect to CuraCore service
final pipe = await NamedPipe.connect(r'\\.\pipe\CuraUI');

// Receive status updates
pipe.listen((data) {
  final status = StatusUpdate.fromBytes(data);
  setState(() {
    systemHealth = status.health;
    currentGear = status.gear;
  });
});
```

### Status Update Format
```dart
class StatusUpdate {
  final int health;          // 0-100
  final GearMode gear;       // Eco or Turbo
  final int threatsBlocked;
  final int vaultCount;
  final double avgLatency;   // ms
}
```

## Development Workflow

### Setup
```powershell
# Install Flutter
# https://docs.flutter.dev/get-started/install/windows

# Verify installation
flutter doctor

# Create project
cd ui/CuraUI
flutter create --platforms=windows .

# Run
flutter run -d windows
```

### Hot Reload
```powershell
# Flutter's killer feature: instant UI updates
# Press 'r' to hot reload
# Press 'R' to hot restart
```

## File Structure

```
CuraUI/
├── lib/
│   ├── main.dart              # Entry point
│   ├── screens/
│   │   ├── dashboard.dart     # Home screen
│   │   ├── vault.dart         # Quarantine viewer
│   │   ├── history.dart       # Event timeline
│   │   └── settings.dart      # Configuration
│   ├── widgets/
│   │   ├── health_indicator.dart
│   │   ├── threat_card.dart
│   │   └── gear_selector.dart
│   ├── services/
│   │   └── cura_service.dart  # Named pipe comm
│   └── models/
│       ├── status_update.dart
│       └── threat_event.dart
├── windows/
│   └── runner/
│       └── main.cpp           # Windows entry point
├── assets/
│   ├── icons/
│   └── images/
├── pubspec.yaml               # Dependencies
└── README.md (this file)
```

## Windows-Specific Features

### System Tray Icon
```dart
// Add to system tray
SystemTray tray = SystemTray();
await tray.initSystemTray(
  title: "CURA",
  iconPath: "assets/icons/cura.ico",
);
```

### Windows Toast Notifications
```dart
// Show threat notification
await showNotification(
  title: "Threat Blocked",
  body: "Malicious file prevented from executing",
);
```

### Auto-start on Boot
```dart
// Add to Windows registry (HKCU\Software\Microsoft\Windows\CurrentVersion\Run)
// This will be done by installer
```

## Accessibility

- **High Contrast Mode**: Support Windows high contrast themes
- **Screen Reader**: All UI elements have semantic labels
- **Keyboard Navigation**: Full keyboard support (Tab, Enter, Space)

## Performance Targets

- **Startup Time**: <2 seconds (cold start)
- **UI FPS**: 60fps smooth animations
- **Memory**: <200MB RAM usage
- **CPU**: <5% when idle

## Future Enhancements (Post-Launch)

- **Multi-language support** (English, Indonesian)
- **Export reports** (PDF threat analysis)
- **Scheduled scans** (daily deep scan)
- **Remote management** (web dashboard)

---

**Remember**: UI is the last piece. Make sure the kernel and service work perfectly first. A beautiful UI that doesn't work is useless. An ugly UI that works is shipped.
