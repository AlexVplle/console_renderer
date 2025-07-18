# Console Renderer

A real-time 3D renderer for the terminal that displays OBJ files using ASCII characters. Built with Rust for performance and cross-platform compatibility.

## Features

- **Real-time 3D rendering** in the terminal using ASCII characters
- **OBJ file support** with automatic model centering and normalization
- **Interactive controls** for rotation and scaling
- **Lighting system** with brightness-based ASCII shading
- **Perspective projection** with configurable camera
- **Back-face culling** for performance optimization

## Installation

### Prerequisites

- Rust (1.70 or later)
- Cargo package manager

### Build from Source

```bash
git clone <repository-url>
cd console_renderer
cargo build --release
```

## Usage

Run the renderer with an OBJ file:

```bash
cargo run -- -f <path-to-obj-file>
```

### Examples

The project includes sample models in the `resources/` directory:

```bash
# Simple cube
cargo run -- -f resources/cube.obj

# Pyramid shapes
cargo run -- -f resources/pyramids.obj

# Complex model
cargo run -- -f resources/Rafale.obj
```

## Controls

| Key | Action |
|-----|--------|
| `a` | Rotate counterclockwise around Z-axis |
| `e` | Rotate clockwise around Z-axis |
| `q` | Rotate clockwise around Y-axis |
| `d` | Rotate counterclockwise around Y-axis |
| `r` | Rotate clockwise around X-axis |
| `f` | Rotate counterclockwise around X-axis |
| `z` | Scale up |
| `s` | Scale down |
| `Esc` | Exit application |

## How It Works

### Rendering Pipeline

1. **Model Loading**: OBJ files are parsed and vertices are extracted
2. **Normalization**: Models are centered and scaled to fit the viewport
3. **Triangle Generation**: Vertices are grouped into triangles with normal calculation
4. **Culling**: Back-facing triangles are removed for performance
5. **Projection**: 3D coordinates are projected to 2D screen space
6. **Lighting**: Brightness is calculated using the dot product of surface normals and light direction
7. **Rasterization**: ASCII characters are selected based on brightness levels

### ASCII Shading

The renderer uses 6 brightness levels to create depth perception:

- `@` - Brightest (230-255)
- `#` - Very bright (200-229)
- `*` - Bright (160-199)
- `+` - Medium (120-159)
- `-` - Dim (80-119)
- `.` - Darkest (0-79)

## Technical Details

### Dependencies

- **clap** - Command-line argument parsing
- **console_engine** - Terminal rendering and input handling
- **crossterm** - Cross-platform terminal manipulation
- **nalgebra** - Linear algebra operations for 3D math

### Architecture

The codebase is organized into several key modules:

- `main.rs` - Application loop and rendering logic
- `loader.rs` - OBJ file parser
- `structures/` - Core 3D mathematics
  - `triangle.rs` - Triangle primitive with transformations
  - `camera.rs` - Perspective projection camera
  - `light.rs` - Point light source
  - `rotation_matrix.rs` - Pre-computed rotation matrices

## Performance

The renderer is optimized for real-time performance:

- Back-face culling reduces triangle count
- Pre-computed rotation matrices
- Efficient triangle rasterization
- Runs at approximately 30 FPS on modern hardware

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

This project is open source. See the LICENSE file for details.

## Author

Created by AlexVplle

## Acknowledgments

- Built with the Rust programming language
- Uses nalgebra for linear algebra operations
- Terminal rendering powered by console_engine