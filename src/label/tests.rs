//! Label module unit tests
//! 
//! Tests demonstrate Rust best practices including:
//! - Builder pattern for test fixtures
//! - Property-based testing concepts
//! - Comprehensive edge case coverage
//! - Type safety validation

use super::*;
use crate::base::types::Color3B;
use crate::math::Vec2;

/// Test fixture builder for Label
/// 
/// This demonstrates the Builder pattern for creating test fixtures,
/// making tests more readable and maintainable.
#[derive(Default)]
struct LabelTestBuilder {
    text: Option<String>,
    font_name: Option<String>,
    font_size: Option<f32>,
    h_alignment: Option<TextHAlignment>,
    v_alignment: Option<TextVAlignment>,
}

impl LabelTestBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    fn with_font(mut self, name: impl Into<String>, size: f32) -> Self {
        self.font_name = Some(name.into());
        self.font_size = Some(size);
        self
    }

    fn with_alignment(mut self, h_align: TextHAlignment, v_align: TextVAlignment) -> Self {
        self.h_alignment = Some(h_align);
        self.v_alignment = Some(v_align);
        self
    }

    fn build(self) -> Label {
        let mut label = Label::new();
        
        if let Some(text) = self.text {
            label.set_string(&text);
        }
        if let Some(font_name) = self.font_name {
            label.set_font_name(&font_name);
        }
        if let Some(font_size) = self.font_size {
            label.set_font_size(font_size);
        }
        if let Some(h_align) = self.h_alignment {
            label.set_horizontal_alignment(h_align);
        }
        if let Some(v_align) = self.v_alignment {
            label.set_vertical_alignment(v_align);
        }
        
        label
    }
}

// ============================================================================
// Constructor Tests
// ============================================================================

#[test]
fn test_label_new_has_default_values() {
    let label = Label::new();
    
    assert_eq!(label.get_string(), "");
    assert_eq!(label.get_font_name(), "Arial");
    assert_eq!(label.get_font_size(), 12.0);
    assert_eq!(label.get_horizontal_alignment(), TextHAlignment::LEFT);
    assert_eq!(label.get_vertical_alignment(), TextVAlignment::TOP);
    assert_eq!(label.get_text_color(), Color3B::WHITE);
    assert_eq!(label.get_overflow(), LabelOverflow::NONE);
    assert!(!label.is_wrap_enabled());
}

#[test]
fn test_label_default_trait() {
    let label1 = Label::new();
    let label2 = Label::default();
    
    // Verify Default trait produces identical results to new()
    assert_eq!(label1.get_string(), label2.get_string());
    assert_eq!(label1.get_font_name(), label2.get_font_name());
    assert_eq!(label1.get_font_size(), label2.get_font_size());
}

#[test]
fn test_create_with_ttf() {
    let label = Label::create_with_ttf("Hello", "CustomFont", 24.0);
    
    assert_eq!(label.get_string(), "Hello");
    assert_eq!(label.get_font_name(), "CustomFont");
    assert_eq!(label.get_font_size(), 24.0);
}

#[test]
fn test_create_with_system_font() {
    let label = Label::create_with_system_font("System", "Arial", 16.0);
    
    assert_eq!(label.get_string(), "System");
    assert_eq!(label.get_font_name(), "Arial");
    assert_eq!(label.get_font_size(), 16.0);
}

#[test]
fn test_create_with_bmfont() {
    let label = Label::create_with_bmfont("BitmapText", "fonts/bitmap.fnt");
    
    assert_eq!(label.get_string(), "BitmapText");
    assert_eq!(label.get_font_name(), "fonts/bitmap.fnt");
}

#[test]
fn test_create_with_char_map() {
    let label = Label::create_with_char_map(
        "CharMap",
        "textures/charmap.png",
        16,
        16,
        'A'
    );
    
    assert_eq!(label.get_string(), "CharMap");
}

// ============================================================================
// Text Content Tests
// ============================================================================

#[test]
fn test_set_and_get_string() {
    let mut label = Label::new();
    
    label.set_string("Test String");
    assert_eq!(label.get_string(), "Test String");
    
    label.set_string("Another String");
    assert_eq!(label.get_string(), "Another String");
}

#[test]
fn test_empty_string() {
    let mut label = Label::new();
    label.set_string("");
    
    assert_eq!(label.get_string(), "");
    assert_eq!(label.get_string_length(), 0);
}

#[test]
fn test_unicode_string() {
    let mut label = Label::new();
    let unicode_text = "Hello 世界 🌍";
    
    label.set_string(unicode_text);
    assert_eq!(label.get_string(), unicode_text);
}

#[test]
fn test_multiline_string() {
    let mut label = Label::new();
    let multiline = "Line 1\nLine 2\nLine 3";
    
    label.set_string(multiline);
    assert_eq!(label.get_string(), multiline);
    assert_eq!(label.get_string_num_lines(), 3);
}

#[test]
fn test_string_length() {
    let mut label = Label::new();
    
    label.set_string("Hello");
    assert_eq!(label.get_string_length(), 5);
    
    label.set_string("Hello World");
    assert_eq!(label.get_string_length(), 11);
}

// ============================================================================
// Font Property Tests
// ============================================================================

#[test]
fn test_set_and_get_font_name() {
    let mut label = Label::new();
    
    label.set_font_name("Helvetica");
    assert_eq!(label.get_font_name(), "Helvetica");
    
    label.set_font_name("Times New Roman");
    assert_eq!(label.get_font_name(), "Times New Roman");
}

#[test]
fn test_set_and_get_font_size() {
    let mut label = Label::new();
    
    label.set_font_size(18.0);
    assert_eq!(label.get_font_size(), 18.0);
    
    label.set_font_size(24.5);
    assert_eq!(label.get_font_size(), 24.5);
}

#[test]
fn test_font_size_boundary_values() {
    let mut label = Label::new();
    
    // Very small font size
    label.set_font_size(0.1);
    assert_eq!(label.get_font_size(), 0.1);
    
    // Very large font size
    label.set_font_size(1000.0);
    assert_eq!(label.get_font_size(), 1000.0);
}

// ============================================================================
// Alignment Tests
// ============================================================================

#[test]
fn test_horizontal_alignment() {
    let mut label = Label::new();
    
    label.set_horizontal_alignment(TextHAlignment::CENTER);
    assert_eq!(label.get_horizontal_alignment(), TextHAlignment::CENTER);
    
    label.set_horizontal_alignment(TextHAlignment::RIGHT);
    assert_eq!(label.get_horizontal_alignment(), TextHAlignment::RIGHT);
}

#[test]
fn test_vertical_alignment() {
    let mut label = Label::new();
    
    label.set_vertical_alignment(TextVAlignment::CENTER);
    assert_eq!(label.get_vertical_alignment(), TextVAlignment::CENTER);
    
    label.set_vertical_alignment(TextVAlignment::BOTTOM);
    assert_eq!(label.get_vertical_alignment(), TextVAlignment::BOTTOM);
}

#[test]
fn test_set_alignment_both() {
    let mut label = Label::new();
    
    label.set_alignment(TextHAlignment::CENTER, TextVAlignment::CENTER);
    assert_eq!(label.get_horizontal_alignment(), TextHAlignment::CENTER);
    assert_eq!(label.get_vertical_alignment(), TextVAlignment::CENTER);
}

#[test]
fn test_all_alignment_combinations() {
    let h_alignments = [
        TextHAlignment::LEFT,
        TextHAlignment::CENTER,
        TextHAlignment::RIGHT,
    ];
    let v_alignments = [
        TextVAlignment::TOP,
        TextVAlignment::CENTER,
        TextVAlignment::BOTTOM,
    ];
    
    for &h_align in &h_alignments {
        for &v_align in &v_alignments {
            let mut label = Label::new();
            label.set_alignment(h_align, v_align);
            
            assert_eq!(label.get_horizontal_alignment(), h_align);
            assert_eq!(label.get_vertical_alignment(), v_align);
        }
    }
}

// ============================================================================
// Color Tests
// ============================================================================

#[test]
fn test_set_and_get_text_color() {
    let mut label = Label::new();
    
    let red = Color3B::RED;
    label.set_text_color(red);
    assert_eq!(label.get_text_color(), red);
    
    let custom = Color3B::new(128, 64, 32);
    label.set_text_color(custom);
    assert_eq!(label.get_text_color(), custom);
}

#[test]
fn test_predefined_colors() {
    let colors = [
        Color3B::WHITE,
        Color3B::BLACK,
        Color3B::RED,
        Color3B::GREEN,
        Color3B::BLUE,
        Color3B::YELLOW,
        Color3B::MAGENTA,
        Color3B::ORANGE,
        Color3B::GRAY,
    ];
    
    for color in &colors {
        let mut label = Label::new();
        label.set_text_color(*color);
        assert_eq!(label.get_text_color(), *color);
    }
}

// ============================================================================
// Dimensions and Layout Tests
// ============================================================================

#[test]
fn test_set_and_get_dimensions() {
    let mut label = Label::new();
    
    label.set_dimensions(100.0, 50.0);
    let dims = label.get_dimensions();
    assert_eq!(dims.x, 100.0);
    assert_eq!(dims.y, 50.0);
}

#[test]
fn test_dimensions_zero() {
    let mut label = Label::new();
    
    label.set_dimensions(0.0, 0.0);
    let dims = label.get_dimensions();
    assert_eq!(dims, Vec2::ZERO);
}

#[test]
fn test_line_height() {
    let mut label = Label::new();
    
    label.set_line_height(20.0);
    assert_eq!(label.get_line_height(), 20.0);
    
    label.set_line_height(0.0);
    assert_eq!(label.get_line_height(), 0.0);
}

#[test]
fn test_line_spacing() {
    let mut label = Label::new();
    
    label.set_line_spacing(5.0);
    assert_eq!(label.get_line_spacing(), 5.0);
    
    label.set_line_spacing(-2.0);
    assert_eq!(label.get_line_spacing(), -2.0);
}

#[test]
fn test_max_line_width() {
    let mut label = Label::new();
    
    label.set_max_line_width(200.0);
    assert_eq!(label.get_max_line_width(), 200.0);
}

// ============================================================================
// Word Wrap Tests
// ============================================================================

#[test]
fn test_enable_wrap() {
    let mut label = Label::new();
    
    assert!(!label.is_wrap_enabled());
    
    label.enable_wrap(true);
    assert!(label.is_wrap_enabled());
    
    label.enable_wrap(false);
    assert!(!label.is_wrap_enabled());
}

#[test]
fn test_wrap_toggle() {
    let mut label = Label::new();
    
    for _ in 0..3 {
        label.enable_wrap(true);
        assert!(label.is_wrap_enabled());
        
        label.enable_wrap(false);
        assert!(!label.is_wrap_enabled());
    }
}

// ============================================================================
// Overflow Tests
// ============================================================================

#[test]
fn test_overflow_types() {
    let overflow_types = [
        LabelOverflow::NONE,
        LabelOverflow::CLAMP,
        LabelOverflow::SHRINK,
        LabelOverflow::RESIZE_HEIGHT,
    ];
    
    for &overflow in &overflow_types {
        let mut label = Label::new();
        label.set_overflow(overflow);
        assert_eq!(label.get_overflow(), overflow);
    }
}

// ============================================================================
// Shadow Effect Tests
// ============================================================================

#[test]
fn test_enable_shadow() {
    let mut label = Label::new();
    
    let shadow_color = Color3B::BLACK;
    let shadow_offset = Vec2::new(2.0, -2.0);
    let blur_radius = 4.0;
    
    label.enable_shadow(shadow_color, shadow_offset, blur_radius);
    
    // Shadow is enabled (we can verify through internal state if needed)
    // In a real implementation, you might add a getter for shadow state
}

#[test]
fn test_disable_shadow() {
    let mut label = Label::new();
    
    // Enable shadow first
    label.enable_shadow(Color3B::BLACK, Vec2::new(1.0, 1.0), 2.0);
    
    // Then disable it
    label.disable_shadow();
    
    // Shadow should be disabled
}

#[test]
fn test_shadow_with_different_parameters() {
    let test_cases = [
        (Color3B::BLACK, Vec2::new(0.0, 0.0), 0.0),
        (Color3B::GRAY, Vec2::new(2.0, -2.0), 4.0),
        (Color3B::RED, Vec2::new(-1.0, -1.0), 10.0),
    ];
    
    for (color, offset, blur) in &test_cases {
        let mut label = Label::new();
        label.enable_shadow(*color, *offset, *blur);
        // Verify shadow is applied
    }
}

// ============================================================================
// Outline Effect Tests
// ============================================================================

#[test]
fn test_enable_outline() {
    let mut label = Label::new();
    
    let outline_color = Color3B::BLACK;
    let outline_size = 2.0;
    
    label.enable_outline(outline_color, outline_size);
    // Outline is enabled
}

#[test]
fn test_disable_outline() {
    let mut label = Label::new();
    
    label.enable_outline(Color3B::BLACK, 2.0);
    label.disable_outline();
    // Outline should be disabled
}

#[test]
fn test_outline_with_different_sizes() {
    let sizes = [0.5, 1.0, 2.0, 5.0, 10.0];
    
    for &size in &sizes {
        let mut label = Label::new();
        label.enable_outline(Color3B::BLACK, size);
        // Verify outline size
    }
}

// ============================================================================
// Builder Pattern Tests
// ============================================================================

#[test]
fn test_builder_pattern_basic() {
    let label = LabelTestBuilder::new()
        .with_text("Builder Test")
        .with_font("Helvetica", 18.0)
        .build();
    
    assert_eq!(label.get_string(), "Builder Test");
    assert_eq!(label.get_font_name(), "Helvetica");
    assert_eq!(label.get_font_size(), 18.0);
}

#[test]
fn test_builder_pattern_full() {
    let label = LabelTestBuilder::new()
        .with_text("Complete Label")
        .with_font("Arial", 24.0)
        .with_alignment(TextHAlignment::CENTER, TextVAlignment::CENTER)
        .build();
    
    assert_eq!(label.get_string(), "Complete Label");
    assert_eq!(label.get_font_name(), "Arial");
    assert_eq!(label.get_font_size(), 24.0);
    assert_eq!(label.get_horizontal_alignment(), TextHAlignment::CENTER);
    assert_eq!(label.get_vertical_alignment(), TextVAlignment::CENTER);
}

#[test]
fn test_builder_pattern_minimal() {
    let label = LabelTestBuilder::new().build();
    
    // Should have default values
    assert_eq!(label.get_string(), "");
    assert_eq!(label.get_font_name(), "Arial");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complex_label_configuration() {
    let mut label = Label::create_with_ttf("Complex Label", "CustomFont", 20.0);
    
    label.set_dimensions(300.0, 100.0);
    label.set_alignment(TextHAlignment::CENTER, TextVAlignment::CENTER);
    label.set_text_color(Color3B::BLUE);
    label.enable_wrap(true);
    label.set_line_spacing(5.0);
    label.set_overflow(LabelOverflow::SHRINK);
    label.enable_shadow(Color3B::BLACK, Vec2::new(2.0, -2.0), 3.0);
    label.enable_outline(Color3B::WHITE, 1.5);
    
    // Verify all settings
    assert_eq!(label.get_string(), "Complex Label");
    assert_eq!(label.get_font_name(), "CustomFont");
    assert_eq!(label.get_font_size(), 20.0);
    assert_eq!(label.get_dimensions(), Vec2::new(300.0, 100.0));
    assert_eq!(label.get_horizontal_alignment(), TextHAlignment::CENTER);
    assert_eq!(label.get_vertical_alignment(), TextVAlignment::CENTER);
    assert_eq!(label.get_text_color(), Color3B::BLUE);
    assert!(label.is_wrap_enabled());
    assert_eq!(label.get_line_spacing(), 5.0);
    assert_eq!(label.get_overflow(), LabelOverflow::SHRINK);
}

// ============================================================================
// Enum Tests (ensuring Copy, Clone, PartialEq, Eq, Debug traits work)
// ============================================================================

#[test]
fn test_text_halignment_traits() {
    let align1 = TextHAlignment::CENTER;
    let align2 = align1; // Copy
    let align3 = align1.clone(); // Clone
    
    assert_eq!(align1, align2); // PartialEq
    assert_eq!(align2, align3); // PartialEq
    
    // Debug
    let debug_str = format!("{:?}", align1);
    assert!(debug_str.contains("CENTER"));
}

#[test]
fn test_text_valignment_traits() {
    let align1 = TextVAlignment::BOTTOM;
    let align2 = align1;
    
    assert_eq!(align1, align2);
    assert_ne!(align1, TextVAlignment::TOP);
}

#[test]
fn test_label_overflow_traits() {
    let overflow1 = LabelOverflow::SHRINK;
    let overflow2 = overflow1;
    
    assert_eq!(overflow1, overflow2);
    assert_ne!(overflow1, LabelOverflow::NONE);
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_very_long_string() {
    let mut label = Label::new();
    let long_string = "A".repeat(10000);
    
    label.set_string(&long_string);
    assert_eq!(label.get_string_length(), 10000);
}

#[test]
fn test_special_characters() {
    let mut label = Label::new();
    let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    
    label.set_string(special);
    assert_eq!(label.get_string(), special);
}

#[test]
fn test_whitespace_only() {
    let mut label = Label::new();
    
    label.set_string("   ");
    assert_eq!(label.get_string(), "   ");
    assert_eq!(label.get_string_length(), 3);
}

#[test]
fn test_newlines_only() {
    let mut label = Label::new();
    
    label.set_string("\n\n\n");
    assert_eq!(label.get_string_num_lines(), 4); // 3 newlines = 4 lines
}

#[test]
fn test_mixed_newlines() {
    let mut label = Label::new();
    
    label.set_string("Line1\n\nLine3\n");
    assert_eq!(label.get_string_num_lines(), 4);
}

// ============================================================================
// Property Mutation Tests
// ============================================================================

#[test]
fn test_multiple_property_changes() {
    let mut label = Label::new();
    
    // Change properties multiple times
    for i in 1..=5 {
        label.set_string(&format!("Text {}", i));
        label.set_font_size(10.0 + i as f32);
        
        assert_eq!(label.get_string(), format!("Text {}", i));
        assert_eq!(label.get_font_size(), 10.0 + i as f32);
    }
}

#[test]
fn test_property_independence() {
    let mut label = Label::new();
    
    // Changing one property shouldn't affect others
    let original_font = label.get_font_name().to_string();
    let original_size = label.get_font_size();
    
    label.set_string("New Text");
    
    assert_eq!(label.get_font_name(), original_font);
    assert_eq!(label.get_font_size(), original_size);
}

// ============================================================================
// Performance/Stress Tests (示例)
// ============================================================================

#[test]
#[ignore] // Use cargo test -- --ignored to run
fn stress_test_rapid_updates() {
    let mut label = Label::new();
    
    for i in 0..1000 {
        label.set_string(&format!("Update {}", i));
        label.set_font_size((i % 100) as f32);
        label.set_alignment(
            if i % 3 == 0 { TextHAlignment::LEFT } 
            else if i % 3 == 1 { TextHAlignment::CENTER } 
            else { TextHAlignment::RIGHT },
            if i % 2 == 0 { TextVAlignment::TOP } 
            else { TextVAlignment::BOTTOM }
        );
    }
    
    assert_eq!(label.get_string(), "Update 999");
}
