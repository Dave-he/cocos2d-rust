use crate::base::{Ref, Node, RefPtr};
use crate::base::types::Color3B;
use crate::math::Vec2;
use crate::renderer::Texture2D;
use super::{TextHAlignment, TextVAlignment};

/// LabelTTF renders text using TrueType fonts
#[derive(Debug)]
pub struct LabelTTF {
    node: Node,
    text: String,
    font_name: String,
    font_size: f32,
    dimensions: Vec2,
    h_alignment: TextHAlignment,
    v_alignment: TextVAlignment,
    color: Color3B,
    texture: Option<RefPtr<Texture2D>>,
}

impl LabelTTF {
    /// Creates a new TTF label
    pub fn new() -> LabelTTF {
        LabelTTF {
            node: Node::new(),
            text: String::new(),
            font_name: String::from("Arial"),
            font_size: 12.0,
            dimensions: Vec2::ZERO,
            h_alignment: TextHAlignment::LEFT,
            v_alignment: TextVAlignment::TOP,
            color: Color3B::WHITE,
            texture: None,
        }
    }

    /// Creates a TTF label with text, font and font size
    pub fn create(text: &str, font_name: &str, font_size: f32) -> LabelTTF {
        let mut label = LabelTTF::new();
        label.init_with_string(text, font_name, font_size);
        label
    }

    /// Creates a TTF label with dimensions
    pub fn create_with_dimensions(
        text: &str,
        font_name: &str,
        font_size: f32,
        dimensions: Vec2,
        h_alignment: TextHAlignment,
        v_alignment: TextVAlignment,
    ) -> LabelTTF {
        let mut label = LabelTTF::new();
        label.init_with_string_and_dimensions(text, font_name, font_size, dimensions, h_alignment, v_alignment);
        label
    }

    /// Initializes the label with a string
    pub fn init_with_string(&mut self, text: &str, font_name: &str, font_size: f32) -> bool {
        self.text = text.to_string();
        self.font_name = font_name.to_string();
        self.font_size = font_size;
        self.update_texture();
        true
    }

    /// Initializes the label with dimensions
    pub fn init_with_string_and_dimensions(
        &mut self,
        text: &str,
        font_name: &str,
        font_size: f32,
        dimensions: Vec2,
        h_alignment: TextHAlignment,
        v_alignment: TextVAlignment,
    ) -> bool {
        self.text = text.to_string();
        self.font_name = font_name.to_string();
        self.font_size = font_size;
        self.dimensions = dimensions;
        self.h_alignment = h_alignment;
        self.v_alignment = v_alignment;
        self.update_texture();
        true
    }

    /// Sets the string
    pub fn set_string(&mut self, text: &str) {
        self.text = text.to_string();
        self.update_texture();
    }

    /// Gets the string
    pub fn get_string(&self) -> &str {
        &self.text
    }

    /// Sets the font name
    pub fn set_font_name(&mut self, font_name: &str) {
        self.font_name = font_name.to_string();
        self.update_texture();
    }

    /// Gets the font name
    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }

    /// Sets the font size
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.update_texture();
    }

    /// Gets the font size
    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    /// Sets the dimensions
    pub fn set_dimensions(&mut self, dimensions: Vec2) {
        self.dimensions = dimensions;
        self.update_texture();
    }

    /// Gets the dimensions
    pub fn get_dimensions(&self) -> Vec2 {
        self.dimensions
    }

    /// Sets horizontal alignment
    pub fn set_horizontal_alignment(&mut self, alignment: TextHAlignment) {
        self.h_alignment = alignment;
        self.update_texture();
    }

    /// Gets horizontal alignment
    pub fn get_horizontal_alignment(&self) -> TextHAlignment {
        self.h_alignment
    }

    /// Sets vertical alignment
    pub fn set_vertical_alignment(&mut self, alignment: TextVAlignment) {
        self.v_alignment = alignment;
        self.update_texture();
    }

    /// Gets vertical alignment
    pub fn get_vertical_alignment(&self) -> TextVAlignment {
        self.v_alignment
    }

    /// Sets the text color
    pub fn set_text_color(&mut self, color: Color3B) {
        self.color = color;
    }

    /// Gets the text color
    pub fn get_text_color(&self) -> Color3B {
        self.color
    }

    /// Updates the texture
    fn update_texture(&mut self) {
        // Render text to texture
        // This would use a TTF rendering library
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

impl Default for LabelTTF {
    fn default() -> Self {
        Self::new()
    }
}
