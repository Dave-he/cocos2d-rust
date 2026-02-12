use cocos2d_rust::ui::{Button, Label, Slider, TextField};
use cocos2d_rust::math::Vec2;

#[test]
fn test_button_creation() {
    let button = Button::new();
    assert!(!button.is_enabled());
}

#[test]
fn test_button_title() {
    let mut button = Button::new();
    
    button.set_title_text("Click Me");
    assert_eq!(button.get_title_text(), "Click Me");
}

#[test]
fn test_button_enabled_state() {
    let mut button = Button::new();
    
    button.set_enabled(true);
    assert!(button.is_enabled());
    
    button.set_enabled(false);
    assert!(!button.is_enabled());
}

#[test]
fn test_label_creation() {
    let label = Label::new();
    assert!(label.get_string().is_empty());
}

#[test]
fn test_label_string() {
    let mut label = Label::new();
    
    label.set_string("Hello World");
    assert_eq!(label.get_string(), "Hello World");
}

#[test]
fn test_label_font_size() {
    let mut label = Label::new();
    
    label.set_font_size(24.0);
    assert_eq!(label.get_font_size(), 24.0);
}

#[test]
fn test_slider_creation() {
    let slider = Slider::new();
    assert_eq!(slider.get_percent(), 0);
}

#[test]
fn test_slider_value() {
    let mut slider = Slider::new();
    
    slider.set_percent(50);
    assert_eq!(slider.get_percent(), 50);
    
    slider.set_percent(100);
    assert_eq!(slider.get_percent(), 100);
}

#[test]
fn test_slider_max_percent() {
    let mut slider = Slider::new();
    
    slider.set_max_percent(200);
    assert_eq!(slider.get_max_percent(), 200);
}

#[test]
fn test_textfield_creation() {
    let textfield = TextField::new();
    assert!(textfield.get_string().is_empty());
}

#[test]
fn test_textfield_placeholder() {
    let mut textfield = TextField::new();
    
    textfield.set_placeholder_text("Enter text...");
    assert_eq!(textfield.get_placeholder_text(), "Enter text...");
}

#[test]
fn test_textfield_max_length() {
    let mut textfield = TextField::new();
    
    textfield.set_max_length(100);
    assert_eq!(textfield.get_max_length(), 100);
}

#[test]
fn test_textfield_password_mode() {
    let mut textfield = TextField::new();
    
    textfield.set_password_enabled(true);
    assert!(textfield.is_password_enabled());
}

#[test]
fn test_button_click_callback() {
    let mut button = Button::new();
    let mut clicked = false;
    
    button.add_click_event_listener(|| {
        clicked = true;
    });
    
    button.simulate_click();
    assert!(clicked);
}

#[test]
fn test_ui_widget_position() {
    let mut button = Button::new();
    
    button.set_position(Vec2::new(100.0, 200.0));
    assert_eq!(button.get_position(), Vec2::new(100.0, 200.0));
}

#[test]
fn test_ui_widget_size() {
    let mut button = Button::new();
    
    button.set_content_size(Size::new(150.0, 50.0));
    assert_eq!(button.get_content_size().width, 150.0);
    assert_eq!(button.get_content_size().height, 50.0);
}

#[test]
fn test_ui_widget_touch_enabled() {
    let mut button = Button::new();
    
    button.set_touch_enabled(true);
    assert!(button.is_touch_enabled());
    
    button.set_touch_enabled(false);
    assert!(!button.is_touch_enabled());
}
