use cocos2d_rust::{Director, Scene, Layer, Color4F, Vec2};

#[derive(Debug)]
struct GameLayer {
    layer: Layer,
    player_position: Vec2,
    score: i32,
}

impl GameLayer {
    pub fn new() -> GameLayer {
        GameLayer {
            layer: Layer::new(),
            player_position: Vec2::new(400.0, 300.0),
            score: 0,
        }
    }

    pub fn create() -> GameLayer {
        let mut game = GameLayer::new();
        game.init();
        game
    }

    fn init(&mut self) {
        self.layer.set_touch_enabled(true);
    }

    pub fn update(&mut self, delta: f32) {
        self.score += 1;
    }

    pub fn on_touch_began(&mut self, location: &Vec2) -> bool {
        self.player_position = *location;
        true
    }

    pub fn on_touch_moved(&mut self, location: &Vec2) {
        self.player_position = *location;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}

fn main() {
    println!("=== Cocos2d-Rust Game Demo ===\n");

    // Initialize the director
    let mut director = Director::get_instance();
    println!("✓ Director initialized");

    // Create the main scene
    let mut scene = Scene::new();
    println!("✓ Scene created");

    // Create game layer
    let mut game_layer = GameLayer::new();
    scene.add_child(game_layer.get_node_mut().clone());
    println!("✓ Game layer added");

    // Run the scene
    director.run_scene(scene);
    println!("✓ Scene running\n");

    println!("Game demo structure:");
    println!("  - Director: Main game loop controller");
    println!("  - Scene: Container for game layers");
    println!("  - Layer: UI and game elements container");
    println!("  - Touch handling: Player movement control");
    println!("  - Score system: Tracks player progress\n");

    println!("To run the full game:");
    println!("  1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
    println!("  2. Build: cd cocos2d-rust && cargo build");
    println!("  3. Run: cargo run --example game_demo\n");

    println!("Demo completed successfully! 🎮");
}
