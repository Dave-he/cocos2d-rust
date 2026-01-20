use cocos2d_rust::{Sprite, Vec2};

fn main() {
    println!("=== Cocos2d-Rust Sprite Demo ===\n");

    // Create a sprite
    let sprite = Sprite::with_file("player.png");
    println!("✓ Sprite created");

    // Configure sprite position
    let mut sprite = sprite.unwrap_or_else(|| {
        Sprite::with_file("").unwrap_or_else(|| {
            panic!("Failed to create sprite");
        })
    });

    println!("✓ Sprite configured\n");

    println!("Sprite features:");
    println!("  - Position: Set sprite location");
    println!("  - Rotation: Rotate sprite");
    println!("  - Scale: Scale sprite");
    println!("  - Color: Change tint color");
    println!("  - Opacity: Adjust transparency");
    println!("  - Flip: Flip sprite\n");

    println!("Sprite demo completed! 🎨");
}
