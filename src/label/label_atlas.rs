use crate::base::{Ref, Node, RefPtr};
use crate::base::types::Color3B;
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
    pub fn create(text: &str, char_map_file: &str, item_width: i32, item_height: i32, start_char: char) -> LabelAtlas {
        let mut label = LabelAtlas::new();
        label.init_with_string(text, char_map_file, item_width, item_height, start_char);
        label
    }

    /// Initializes the label atlas
    pub fn init_with_string(&mut self, text: &str, char_map_file: &str, item_width: i32, item_height: i32, start_char: char) -> bool {
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
