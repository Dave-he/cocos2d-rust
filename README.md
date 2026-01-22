# cocos2d-rust

Rust implementation of the cocos2d-x game engine.

## Overview

This project is a direct port of [cocos2d-x](https://github.com/cocos2d/cocos2d-x) to Rust, providing a safe and performant game development framework.

## Modules

- **math**: Vector, Matrix, Quaternion, and Geometry types
- **base**: Core types, reference counting, events, scheduler
- **platform**: Platform abstraction layer (file system, application)
- **sprite**: 2D sprite rendering and texture management
- **action**: Action system for animations and behaviors
- **scene**: Scene management

## Getting Started

### Prerequisites

- Rust 1.70.0 or higher
- Cargo

### Building

```bash
cd cocos2d-rust
cargo build
```

### Running Examples

```bash
cargo run --example hello_world
```

## Usage

### Creating a Simple Game

```rust
use cocos2d_rust::{Director, Scene, Sprite};

fn main() {
    let mut director = Director::get_instance();
    let mut scene = Scene::new();

    // Create and configure a sprite
    let mut sprite = Sprite::with_file("player.png").unwrap();
    sprite.get_node_mut().set_position(Vec2::new(100.0, 100.0));

    scene.add_child(sprite);
    director.run_scene(scene);
}
```

## Architecture

The architecture follows the original cocos2d-x design with Rust safety guarantees:

- **Reference Counting**: Uses `Rc` and `RefCell` for automatic memory management
- **Event System**: Type-safe event handling with closures
- **Scheduler**: Time-based callback scheduling
- **Action System**: Composable animations and behaviors

## Features

- [x] Math library (Vec2, Vec3, Vec4, Mat4, Quaternion)
- [x] Reference counting system
- [x] Event dispatcher
- [x] Scheduler
- [x] Action system
- [x] Sprite and texture management
- [x] Scene management
- [x] Platform abstraction

## Roadmap

- [ ] Audio system
- [ ] Network system  
- [ ] Physics engine
- [ ] 3D rendering support
- [ ] UI system
- [ ] Particle system
- [ ] Tilemap support
- [ ] Box2D integration
- [ ] Spine integration

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
