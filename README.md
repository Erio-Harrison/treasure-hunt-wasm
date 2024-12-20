# Treasure Hunt WebAssembly Game

A simple treasure hunting game built with Rust and WebAssembly, demonstrating the integration of Rust's performance with web technologies.

![example](assets/example.png)

## 🎮 Game Features

- **Time Challenge**: Complete the hunt within 60 seconds
- **Treasure Collection**: Find and collect all treasures to win
- **Obstacle Navigation**: Avoid walls and obstacles
- **Score Tracking**: Keep track of your best times
- **Responsive Controls**: Smooth keyboard-based movement
- **Audio System**: Immersive sound effects and background music

## 🛠️ Technology Stack

- **Backend**: Rust + WebAssembly
- **Frontend**: JavaScript + HTML5 Canvas
- **Audio**: Web Audio API
- **Build Tool**: wasm-pack
- **Package Manager**: Cargo

## 🚀 Getting Started

### Prerequisites

Make sure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- A local web server (e.g., Python's `http.server` or `live-server`)

### Installation

1. Clone the repository
```bash
git clone https://github.com/Erio-Harrison/treasure-hunt-wasm.git
cd treasure-hunt-wasm
```

2. Build the WebAssembly module
```bash
wasm-pack build --target web
```

3. Start a local server
```bash
# Using Python
python -m http.server
```

4. Open your browser and navigate to `http://localhost:8000`. Then go to "www/" content.

## 🎯 How to Play

- Use arrow keys to move the player (blue square)
- Collect all treasures (gold circles) before time runs out
- Avoid walls (dark gray) and obstacles (light gray)
- Complete the level as quickly as possible to set a new best time
- Enjoy background music and sound effects while playing

## 🏗️ Project Structure

```
treasure-hunt-wasm/
├── Cargo.toml                # Rust project configuration
├── src/                      # Rust source code
│   ├── lib.rs               # Main entry point
│   ├── game.rs              # Core game logic
│   ├── player.rs            # Player system
│   ├── map.rs               # Map system
│   ├── treasure.rs          # Treasure system
│   ├── audio.rs             # Audio system
│   └── renderer.rs          # Rendering system
└── www/                      # Web frontend
    ├── index.html           # Main page
    ├── index.js             # JavaScript interface
    └── sounds/              # Audio assets
        ├── collect.mp3      # Collection sound
        ├── win.mp3          # Victory sound
        ├── timeup.mp3       # Time's up sound
        └── background.mp3   # Background music
```

## 🔧 Development

### Building

```bash
wasm-pack build --target web
```

### Testing

```bash
cargo test
wasm-pack test --headless --firefox
```

### Local Development

After building, serve the `www` directory with your preferred local server:
```bash
cd www
python -m http.server
```

## 🎨 Features Implemented

- [x] Core Game Systems
  - Game state management
  - 60-second countdown timer
  - Best time recording
  - Score system
  - Game reset functionality

- [x] Map System
  - Grid-based map generation
  - Automatic wall generation
  - Random obstacle placement
  - Safe spawn area in top-left corner

- [x] Player System
  - Arrow key movement
  - Collision detection
  - Fixed spawn point

- [x] Treasure System
  - Random treasure generation
  - Collection detection
  - Safe treasure placement
  - Victory condition

- [x] Audio System
  - Background music
  - Collection sound effects
  - Victory/Game Over sounds
  - Async audio loading
  - Audio command queue

- [x] Rendering System
  - Canvas 2D rendering
  - Visual differentiation of elements
  - UI rendering
  - Victory/Game Over screens

## 🔜 Planned Features

- [ ] Multiple levels
- [ ] Moving enemies
- [ ] Special items and abilities
- [ ] Save system
- [ ] Complex map generation
- [ ] Volume control
- [ ] More sound effects

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Rust WebAssembly Working Group
- wasm-bindgen contributors
- Web Audio API team
- The Rust community

---

Made with ❤️ by Harrison