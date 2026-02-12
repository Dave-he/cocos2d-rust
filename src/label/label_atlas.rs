use crate::base::types::Color3B;
use crate::base::{Node, RefPtr};
use crate::renderer::Texture2D;

/// LabelAtlas is a label that uses a texture atlas
#[derive(Debug)]
pub struct LabelAtlas {
    node: Node,
    text: String,
    texture: Option<RefPtr<Texture2D>>,
    item_width: i32,
    item_height: i32,
    start_char: char,
    color: Color3B,
}

impl LabelAtlas {
    /// Creates a new label atlas
    pub fn new() -> LabelAtlas {
        LabelAtlas {
            node: Node::new(),
            text: String::new(),
            texture: None,
            item_width: 0,
            item_height: 0,
            start_char: ' ',
            color: Color3B::WHITE,
        }
    }

    /// Creates a label atlas with a texture file
    pub fn create(
        text: &str,
        char_map_file: &str,
        item_width: i32,
        item_height: i32,
        start_char: char,
    ) -> LabelAtlas {
        let mut label = LabelAtlas::new();
        label.init_with_string(text, char_map_file, item_width, item_height, start_char);
        label
    }

    /// Initializes the label atlas
    pub fn init_with_string(
        &mut self,
        text: &str,
        char_map_file: &str,
        item_width: i32,
        item_height: i32,
        start_char: char,
    ) -> bool {
        self.text = text.to_string();
        self.item_width = item_width;
        self.item_height = item_height;
        self.start_char = start_char;

        // Load texture from char_map_file
        // self.texture = Some(Texture::create(char_map_file));

        self.update_atlas_values();
        true
    }

    /// Sets the string
    pub fn set_string(&mut self, text: &str) {
        self.text = text.to_string();
        self.update_atlas_values();
    }

    /// Gets the string
    pub fn get_string(&self) -> &str {
        &self.text
    }

    /// Updates the atlas values
    fn update_atlas_values(&mut self) {
        // Update texture coordinates based on text
    }

    /// Gets the node
    pub fn get_node(&self) -> &Node {
        &self.node
    }

    /// Gets the node mutably
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Default for LabelAtlas {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn test_label_atlas_new() {
        let atlas = LabelAtlas::new();
        assert!(atlas.get_string().is_empty());
        assert_eq!(atlas.get_node().get_content_size(), Vec2::ZERO);
    }

    #[test]
    fn test_label_atlas_default() {
        let atlas = LabelAtlas::default();
        assert!(atlas.get_string().is_empty());
    }

    #[test]
    fn test_label_atlas_set_string() {
        let mut atlas = LabelAtlas::new();
        atlas.set_string("Hello");
        assert_eq!(atlas.get_string(), "Hello");
    }

    #[test]
    fn test_label_atlas_init_with_string() {
        let mut atlas = LabelAtlas::new();
        let result = atlas.init_with_string("Test", "test.png", 16, 16, 'A');
        assert!(result);
        assert_eq!(atlas.get_string(), "Test");
    }

    #[test]
    fn test_label_atlas_properties() {
        let mut atlas = LabelAtlas::new();
        atlas.set_string("ABC");
        assert_eq!(atlas.get_string(), "ABC");
    }
}
