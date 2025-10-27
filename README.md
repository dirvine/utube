# utube - Minimal YouTube Player

A lightweight, always-on-top YouTube player built with Tauri v2 and Rust. Watch your favorite YouTube videos and playlists in a small, movable window with minimal UI clutter.

## Features

- **Frameless Window**: Clean, borderless design that keeps focus on your content
- **Always On Top**: Window stays above other applications for easy multitasking
- **Auto-hiding Controls**: Transparent overlay controls that fade away after 3 seconds
- **Window Persistence**: Remembers your window position and size across sessions
- **Minimal UI**: Video content fills the entire app window
- **Draggable**: Move the window anywhere on your screen
- **Embedded YouTube**: Full YouTube browsing experience with login support

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or later)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/dirvine/utube.git
cd utube

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. Launch the application
2. The app opens with YouTube.com loaded
3. Log in to your YouTube account (optional)
4. Browse and watch videos or playlists
5. Move your mouse to reveal the window controls
6. Use the minimize/close buttons in the top-right corner

### Window Controls

- **Drag**: Click and drag the top bar to move the window
- **Minimize**: Click the `_` button to minimize the window
- **Close**: Click the `×` button to close the app
- **Resize**: Drag the window edges to resize (aspect ratio maintained)

### Keyboard Shortcuts

The app uses YouTube's native keyboard shortcuts:
- `Space` or `K`: Play/Pause
- `J`: Rewind 10 seconds
- `L`: Fast forward 10 seconds
- `F`: Toggle fullscreen
- And more...

## Known Limitations

### Passkey Authentication
**Important:** The embedded webview has limited support for passkey/WebAuthn authentication. This is a platform limitation of WKWebView on macOS.

**Workarounds:**
1. **Use Password Authentication** - Sign in with your Google password instead of passkeys
2. **Login in Safari First** - Log into YouTube in Safari, then the app may share the session cookies
3. **Use Standard Browser** - For initial authentication, you may need to use Safari or Chrome with passkeys, then return to the app

This limitation affects all embedded webview applications and cannot be fully resolved without using a full browser engine.

## Configuration

### Window Settings

Default window size: 640x480
Default behavior: Always on top, frameless

To modify these settings, edit `src-tauri/tauri.conf.json`:

```json
{
  "windows": [{
    "width": 640,
    "height": 480,
    "decorations": false,
    "alwaysOnTop": true,
    "resizable": true,
    "transparent": true
  }]
}
```

## Development

### Project Structure

```
utube/
├── src/                    # Frontend code
│   ├── index.html         # Main HTML structure
│   ├── main.js            # Window controls logic
│   └── styles.css         # UI styling
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Window state persistence
│   │   └── main.rs        # Application entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
└── package.json           # Node.js dependencies
```

### Code Quality

This project maintains zero-defect standards:
- Zero compilation errors
- Zero compilation warnings
- Zero clippy violations
- Formatted with `rustfmt`

Run quality checks:

```bash
cd src-tauri
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Technology Stack

- **Backend**: Rust
- **Framework**: Tauri v2
- **Frontend**: HTML5, CSS3, Vanilla JavaScript
- **Window Management**: Tauri Window API
- **State Persistence**: JSON file storage

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

Built with [Tauri](https://tauri.app/) - A framework for building tiny, fast binaries for all major desktop platforms.

## Repository

GitHub: https://github.com/dirvine/utube
