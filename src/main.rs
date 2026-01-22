use cocos2d_rust::{Director, Scene, Sprite, Color3B};

fn main() {
    // Initialize the director
    let mut director = Director::get_instance();

    // Create a scene
    let mut scene = Scene::new();

    // Create a sprite
    let sprite = Sprite::with_file("test.png").unwrap_or_else(|| {
        // If loading fails, create a simple sprite
        Sprite::with_file("").unwrap_or_else(|| {
            // This will fail, but demonstrates error handling
            panic!("Could not load sprite");
        })
    });

    // Add sprite to scene
    scene.add_child(sprite);

    // Run the scene
    director.run_scene(scene);

    println!("cocos2d-rust game engine initialized successfully!");
    println!("This is a Rust port of the cocos2d-x game engine.");
}
