use std::collections::HashMap;
use crate::renderer::Texture2D;
use crate::base::{Ref, RefPtr};
use crate::math::Vec2;

/// Font letter definition
#[derive(Debug, Clone)]
pub struct FontLetterDefinition {
    pub letter_char: char,
    pub u: f32,
    pub v: f32,
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub texture_page: i32,
    pub valid: bool,
    pub x_advance: f32,
}

impl FontLetterDefinition {
    pub fn new() -> FontLetterDefinition {
        FontLetterDefinition {
            letter_char: '\0',
            u: 0.0,
            v: 0.0,
            width: 0.0,
            height: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            texture_page: 0,
            valid: false,
            x_advance: 0.0,
        }
    }
}

impl Default for FontLetterDefinition {
    fn default() -> Self {
        Self::new()
    }
}

/// Font atlas manages font textures and glyph information
#[derive(Debug)]
pub struct FontAtlas {
    font_name: String,
    font_size: f32,
    letter_definitions: HashMap<char, FontLetterDefinition>,
    textures: Vec<RefPtr<Texture2D>>,
    common_line_height: f32,
}

impl FontAtlas {
    /// Creates a new font atlas
    pub fn new(font_name: &str, font_size: f32) -> FontAtlas {
        FontAtlas {
            font_name: font_name.to_string(),
            font_size,
            letter_definitions: HashMap::new(),
            textures: Vec::new(),
            common_line_height: 0.0,
        }
    }

    /// Adds a letter definition
    pub fn add_letter_definition(&mut self, letter: char, definition: FontLetterDefinition) {
        self.letter_definitions.insert(letter, definition);
    }

    /// Gets a letter definition
    pub fn get_letter_definition(&self, letter: char) -> Option<&FontLetterDefinition> {
        self.letter_definitions.get(&letter)
    }

    /// Gets a mutable letter definition
    pub fn get_letter_definition_mut(&mut self, letter: char) -> Option<&mut FontLetterDefinition> {
        self.letter_definitions.get_mut(&letter)
    }

    /// Adds a texture
    pub fn add_texture(&mut self, texture: RefPtr<Texture2D>) {
        self.textures.push(texture);
    }

    /// Gets a texture at index
    pub fn get_texture(&self, index: usize) -> Option<&RefPtr<Texture2D>> {
        self.textures.get(index)
    }

    /// Gets the font name
    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }

    /// Gets the font size
    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    /// Sets the common line height
    pub fn set_common_line_height(&mut self, height: f32) {
        self.common_line_height = height;
    }

    /// Gets the common line height
    pub fn get_common_line_height(&self) -> f32 {
        self.common_line_height
    }

    /// Prepares letter definitions for a string
    pub fn prepare_letter_definitions(&mut self, text: &str) {
        for ch in text.chars() {
            if !self.letter_definitions.contains_key(&ch) {
                // Generate letter definition for this character
                self.generate_letter_definition(ch);
            }
        }
    }

    /// Generates a letter definition for a character
    fn generate_letter_definition(&mut self, letter: char) {
        // This would render the character using FreeType or similar
        // and create a FontLetterDefinition
        let mut definition = FontLetterDefinition::new();
        definition.letter_char = letter;
        definition.valid = true;
        self.add_letter_definition(letter, definition);
    }

    /// Measures the size of a string
    pub fn measure_string(&self, text: &str) -> Vec2 {
        let mut width = 0.0;
        let mut height = self.common_line_height;

        for ch in text.chars() {
            if let Some(def) = self.get_letter_definition(ch) {
                width += def.x_advance;
            }
        }

        Vec2::new(width, height)
    }
}
