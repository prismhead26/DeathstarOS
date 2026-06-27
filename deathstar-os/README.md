# DeathstarOS

A macOS system control application built with Tauri, Svelte, and TypeScript.

## Features

### Implemented
- **Audio Control**: Mute/unmute system audio and adjust volume

### Coming Soon
- Microphone Control
- Camera Status
- Network Controls (WiFi & Bluetooth)
- Display Brightness Control
- Do Not Disturb / Focus Modes

## Prerequisites

- Node.js (v22+)
- Rust (1.92+)
- macOS 10.15+

## Development

### Install Dependencies

```bash
npm install
```

### Run Development Server

```bash
npm run tauri:dev
```

This will start both the Vite dev server and the Tauri app.

### Build for Production

```bash
npm run tauri:build
```

The built app will be in `src-tauri/target/release/bundle/`.

## Permissions

DeathstarOS requires the following macOS permissions:

- **Microphone**: To control microphone settings
- **Camera**: To monitor camera status
- **Accessibility**: For system-level controls (brightness, DND)
- **Full Disk Access** (optional): For advanced Do Not Disturb controls

These permissions will be requested when you first use the relevant features.

## Architecture

- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2
- **System Integration**: CoreAudio, IOKit, CoreWLAN, AppleScript

## Project Structure

```
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   └── api/            # Tauri command wrappers
│   └── App.svelte         # Main app component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── commands/      # Tauri command handlers
│   │   ├── lib.rs         # App initialization
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── Info.plist         # macOS permissions
```

## License

MIT
