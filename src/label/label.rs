use crate::base::{Ref, Node, RefPtr};
use crate::base::types::Color3B;
use crate::math::Vec2;
use crate::renderer::Texture2D;

/// Text horizontal alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextHAlignment {
    LEFT,
    CENTER,
    RIGHT,
}

/// Text vertical alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextVAlignment {
    TOP,
    CENTER,
    BOTTOM,
}

/// Overflow type for labels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelOverflow {
    NONE,
    CLAMP,
    SHRINK,
    RESIZE_HEIGHT,
}

/// Label is a text display component
#[derive(Debug)]
pub struct Label {
    node: Node,
    text: String,
    font_name: String,
    font_size: f32,
    dimensions: Vec2,
    h_alignment: TextHAlignment,
    v_alignment: TextVAlignment,
    color: Color3B,
    overflow_type: LabelOverflow,
    line_height: f32,
    line_spacing: f32,
    enable_wrap: bool,
    max_line_width: f32,
    use_shadow: bool,
    shadow_offset: Vec2,
    shadow_blur: f32,
    shadow_color: Color3B,
    use_outline: bool,
    outline_size: f32,
    outline_color: Color3B,
    texture: Option<RefPtr<Texture2D>>,
}

impl Label {
    /// Creates a new label
    pub fn new() -> Label {
        Label {
            node: Node::new(),
            text: String::new(),
            font_name: String::from("Arial"),
            font_size: 12.0,
            dimensions: Vec2::ZERO,
            h_alignment: TextHAlignment::LEFT,
            v_alignment: TextVAlignment::TOP,
            color: Color3B::WHITE,
            overflow_type: LabelOverflow::NONE,
            line_height: 0.0,
            line_spacing: 0.0,
            enable_wrap: false,
            max_line_width: 0.0,
            use_shadow: false,
            shadow_offset: Vec2::ZERO,
            shadow_blur: 0.0,
            shadow_color: Color3B::BLACK,
            use_outline: false,
            outline_size: 0.0,
            outline_color: Color3B::BLACK,
            texture: None,
        }
    }

    /// Creates a label with text, font name and font size
    pub fn create_with_ttf(text: &str, font_name: &str, font_size: f32) -> Label {
        let mut label = Label::new();
        label.set_string(text);
        label.set_font_name(font_name);
        label.set_font_size(font_size);
        label
    }

    /// Creates a label with system font
    pub fn create_with_system_font(text: &str, font_name: &str, font_size: f32) -> Label {
        Self::create_with_ttf(text, font_name, font_size)
    }

    /// Creates a label with bitmap font
    pub fn create_with_bmfont(text: &str, bmfont_path: &str) -> Label {
        let mut label = Label::new();
        label.set_string(text);
        label.set_font_name(bmfont_path);
        label
    }

    /// Creates a label with char map
    pub fn create_with_char_map(text: &str, char_map_file: &str, item_width: i32, item_height: i32, start_char: char) -> Label {
        let mut label = Label::new();
        label.set_string(text);
        label
    }

    /// Sets the string content
    pub fn set_string(&mut self, text: &str) {
        self.text = text.to_string();
        self.update_content();
    }

    /// Gets the string content
    pub fn get_string(&self) -> &str {
        &self.text
    }

    /// Sets the font name
    pub fn set_font_name(&mut self, font_name: &str) {
        self.font_name = font_name.to_string();
        self.update_content();
    }

    /// Gets the font name
    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }

    /// Sets the font size
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.update_content();
    }

    /// Gets the font size
    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    /// Sets the dimensions
    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions = Vec2::new(width, height);
        self.update_content();
    }

    /// Gets the dimensions
    pub fn get_dimensions(&self) -> Vec2 {
        self.dimensions
    }

    /// Sets horizontal alignment
    pub fn set_horizontal_alignment(&mut self, alignment: TextHAlignment) {
        self.h_alignment = alignment;
        self.update_content();
    }

    /// Gets horizontal alignment
    pub fn get_horizontal_alignment(&self) -> TextHAlignment {
        self.h_alignment
    }

    /// Sets vertical alignment
    pub fn set_vertical_alignment(&mut self, alignment: TextVAlignment) {
        self.v_alignment = alignment;
        self.update_content();
    }

    /// Gets vertical alignment
    pub fn get_vertical_alignment(&self) -> TextVAlignment {
        self.v_alignment
    }

    /// Sets text alignment
    pub fn set_alignment(&mut self, h_alignment: TextHAlignment, v_alignment: TextVAlignment) {
        self.h_alignment = h_alignment;
        self.v_alignment = v_alignment;
        self.update_content();
    }

    /// Sets the text color
    pub fn set_text_color(&mut self, color: Color3B) {
        self.color = color;
    }

    /// Gets the text color
    pub fn get_text_color(&self) -> Color3B {
        self.color
    }

    /// Sets the line height
    pub fn set_line_height(&mut self, line_height: f32) {
        self.line_height = line_height;
        self.update_content();
    }

    /// Gets the line height
    pub fn get_line_height(&self) -> f32 {
        self.line_height
    }

    /// Sets line spacing
    pub fn set_line_spacing(&mut self, spacing: f32) {
        self.line_spacing = spacing;
        self.update_content();
    }

    /// Gets line spacing
    pub fn get_line_spacing(&self) -> f32 {
        self.line_spacing
    }

    /// Enables or disables word wrap
    pub fn enable_wrap(&mut self, enabled: bool) {
        self.enable_wrap = enabled;
        self.update_content();
    }

    /// Checks if word wrap is enabled
    pub fn is_wrap_enabled(&self) -> bool {
        self.enable_wrap
    }

    /// Sets the overflow type
    pub fn set_overflow(&mut self, overflow: LabelOverflow) {
        self.overflow_type = overflow;
        self.update_content();
    }

    /// Gets the overflow type
    pub fn get_overflow(&self) -> LabelOverflow {
        self.overflow_type
    }

    /// Enables shadow effect
    pub fn enable_shadow(&mut self, shadow_color: Color3B, offset: Vec2, blur_radius: f32) {
        self.use_shadow = true;
        self.shadow_color = shadow_color;
        self.shadow_offset = offset;
        self.shadow_blur = blur_radius;
        self.update_content();
    }

    /// Disables shadow effect
    pub fn disable_shadow(&mut self) {
        self.use_shadow = false;
        self.update_content();
    }

    /// Enables outline effect
    pub fn enable_outline(&mut self, outline_color: Color3B, outline_size: f32) {
        self.use_outline = true;
        self.outline_color = outline_color;
        self.outline_size = outline_size;
        self.update_content();
    }

    /// Disables outline effect
    pub fn disable_outline(&mut self) {
        self.use_outline = false;
        self.update_content();
    }

    /// Gets the content size
    pub fn get_content_size(&self) -> Vec2 {
        self.node.get_content_size()
    }

    /// Gets the string length
    pub fn get_string_length(&self) -> usize {
        self.text.len()
    }

    /// Gets the string number of lines
    pub fn get_string_num_lines(&self) -> usize {
        self.text.lines().count()
    }

    /// Sets max line width
    pub fn set_max_line_width(&mut self, width: f32) {
        self.max_line_width = width;
        self.update_content();
    }

    /// Gets max line width
    pub fn get_max_line_width(&self) -> f32 {
        self.max_line_width
    }

    /// Updates the label content
    fn update_content(&mut self) {
        // This would normally update the texture based on text rendering
        // For now, this is a placeholder
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

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}
