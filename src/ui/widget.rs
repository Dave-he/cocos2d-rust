use crate::base::Ref;
use crate::base::types::Color3B;
use crate::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchEventType {
    BEGAN,
    MOVED,
    ENDED,
    CANCELED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetBrightStyle {
    NONE,
    NORMAL,
    BRIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetSizeType {
    ABSOLUTE,
    PERCENT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetTextureType {
    PLAIN,
    SLICED,
    FILLED,
}

#[derive(Debug)]
pub struct Widget {
    name: String,
    tag: i32,
    position: Vec2,
    size: Vec2,
    anchor_point: Vec2,
    color: Color3B,
    opacity: u8,
    enabled: bool,
    bright: bool,
    bright_style: WidgetBrightStyle,
    touch_pass_through: bool,
    pass_through_lb: Vec2,
    pass_through_rb: Vec2,
    layout_parameter: Option<LayoutParameter>,
    parent: Option<Ref<Widget>>,
    children: Vec<Ref<Widget>>,
}

impl Widget {
    pub fn new() -> Widget {
        Widget {
            name: String::new(),
            tag: 0,
            position: Vec2::ZERO,
            size: Vec2::new(100.0, 100.0),
            anchor_point: Vec2::new(0.5, 0.5),
            color: Color3B::WHITE,
            opacity: 255,
            enabled: true,
            bright: true,
            bright_style: WidgetBrightStyle::NORMAL,
            touch_pass_through: false,
            pass_through_lb: Vec2::ZERO,
            pass_through_rb: Vec2::ZERO,
            layout_parameter: None,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn set_anchor_point(&mut self, anchor_point: Vec2) {
        self.anchor_point = anchor_point;
    }

    pub fn get_anchor_point(&self) -> Vec2 {
        self.anchor_point
    }

    pub fn set_color(&mut self, color: Color3B) {
        self.color = color;
    }

    pub fn get_color(&self) -> Color3B {
        self.color
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_bright(&mut self, bright: bool) {
        self.bright = bright;
    }

    pub fn is_bright(&self) -> bool {
        self.bright
    }

    pub fn set_bright_style(&mut self, style: WidgetBrightStyle) {
        self.bright_style = style;
    }

    pub fn get_bright_style(&self) -> WidgetBrightStyle {
        self.bright_style
    }

    pub fn add_child(&mut self, child: Ref<Widget>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &Ref<Widget>) {
        self.children.retain(|c| c.get_tag() != child.get_tag());
    }

    pub fn get_children(&self) -> &Vec<Ref<Widget>> {
        &self.children
    }

    pub fn on_touch_began(&mut self, touch: &Vec2) -> bool {
        false
    }

    pub fn on_touch_moved(&mut self, touch: &Vec2) {
    }

    pub fn on_touch_ended(&mut self, touch: &Vec2) {
    }

    pub fn on_touch_canceled(&mut self, touch: &Vec2) {
    }
}

#[derive(Debug)]
pub struct Button {
    widget: Widget,
    title_text: String,
    title_color: Color3B,
    normal_image: String,
    pressed_image: String,
    disabled_image: String,
}

impl Button {
    pub fn new() -> Button {
        Button {
            widget: Widget::new(),
            title_text: String::new(),
            title_color: Color3B::WHITE,
            normal_image: String::new(),
            pressed_image: String::new(),
            disabled_image: String::new(),
        }
    }

    pub fn set_title_text(&mut self, text: &str) {
        self.title_text = text.to_string();
    }

    pub fn get_title_text(&self) -> &str {
        &self.title_text
    }

    pub fn set_title_color(&mut self, color: Color3B) {
        self.title_color = color;
    }

    pub fn loadTextures(&mut self, normal: &str, pressed: &str, disabled: &str) {
        self.normal_image = normal.to_string();
        self.pressed_image = pressed.to_string();
        self.disabled_image = disabled.to_string();
    }
}

#[derive(Debug)]
pub struct TextField {
    widget: Widget,
    text: String,
    place_holder: String,
    font_size: f32,
    font_name: String,
    color: Color3B,
    max_length: i32,
    password_enabled: bool,
    password_char: char,
}

impl TextField {
    pub fn new() -> TextField {
        TextField {
            widget: Widget::new(),
            text: String::new(),
            place_holder: String::new(),
            font_size: 24.0,
            font_name: String::from("Arial"),
            color: Color3B::WHITE,
            max_length: 0,
            password_enabled: false,
            password_char: '*',
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_place_holder(&mut self, text: &str) {
        self.place_holder = text.to_string();
    }

    pub fn get_place_holder(&self) -> &str {
        &self.place_holder
    }

    pub fn set_max_length(&mut self, length: i32) {
        self.max_length = length;
    }

    pub fn get_max_length(&self) -> i32 {
        self.max_length
    }

    pub fn set_password_enabled(&mut self, enabled: bool) {
        self.password_enabled = enabled;
    }

    pub fn is_password_enabled(&self) -> bool {
        self.password_enabled
    }
}

#[derive(Debug)]
pub struct Slider {
    widget: Widget,
    bar_image: String,
    progress_bar_image: String,
    ball_normal_image: String,
    ball_pressed_image: String,
    ball_disabled_image: String,
    percent: i32,
    min_percent: i32,
    max_percent: i32,
}

impl Slider {
    pub fn new() -> Slider {
        Slider {
            widget: Widget::new(),
            bar_image: String::new(),
            progress_bar_image: String::new(),
            ball_normal_image: String::new(),
            ball_pressed_image: String::new(),
            ball_disabled_image: String::new(),
            percent: 50,
            min_percent: 0,
            max_percent: 100,
        }
    }

    pub fn set_percent(&mut self, percent: i32) {
        self.percent = percent.clamp(self.min_percent, self.max_percent);
    }

    pub fn get_percent(&self) -> i32 {
        self.percent
    }

    pub fn set_min_percent(&mut self, percent: i32) {
        self.min_percent = percent;
    }

    pub fn get_min_percent(&self) -> i32 {
        self.min_percent
    }

    pub fn set_max_percent(&mut self, percent: i32) {
        self.max_percent = percent;
    }

    pub fn get_max_percent(&self) -> i32 {
        self.max_percent
    }

    pub fn loadSlidingBar(&mut self, bar: &str) {
        self.bar_image = bar.to_string();
    }
}

#[derive(Debug)]
pub struct CheckBox {
    widget: Widget,
    on_off: bool,
    off_normal_image: String,
    on_normal_image: String,
    off_disabled_image: String,
    on_disabled_image: String,
    check_mark_image: String,
}

impl CheckBox {
    pub fn new() -> CheckBox {
        CheckBox {
            widget: Widget::new(),
            on_off: false,
            off_normal_image: String::new(),
            on_normal_image: String::new(),
            off_disabled_image: String::new(),
            on_disabled_image: String::new(),
            check_mark_image: String::new(),
        }
    }

    pub fn set_selected(&mut self, on: bool) {
        self.on_off = on;
    }

    pub fn is_selected(&self) -> bool {
        self.on_off
    }

    pub fn loadTextures(&mut self, off_normal: &str, on_normal: &str, off_disabled: &str, on_disabled: &str, check_mark: &str) {
        self.off_normal_image = off_normal.to_string();
        self.on_normal_image = on_normal.to_string();
        self.off_disabled_image = off_disabled.to_string();
        self.on_disabled_image = on_disabled.to_string();
        self.check_mark_image = check_mark.to_string();
    }
}

#[derive(Debug)]
pub struct ImageView {
    widget: Widget,
    image_texture: String,
    scale_type: WidgetTextureType,
}

impl ImageView {
    pub fn new() -> ImageView {
        ImageView {
            widget: Widget::new(),
            image_texture: String::new(),
            scale_type: WidgetTextureType::PLAIN,
        }
    }

    pub fn load_texture(&mut self, file: &str) {
        self.image_texture = file.to_string();
    }

    pub fn set_scale_type(&mut self, scale_type: WidgetTextureType) {
        self.scale_type = scale_type;
    }
}

#[derive(Debug)]
pub struct Text {
    widget: Widget,
    text: String,
    font_size: f32,
    font_name: String,
    color: Color3B,
    horizontal_alignment: TextHAlignment,
    vertical_alignment: TextVAlignment,
    shadow_enabled: bool,
    shadow_color: Color3B,
    shadow_offset: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextHAlignment {
    LEFT,
    CENTER,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextVAlignment {
    TOP,
    CENTER,
    BOTTOM,
}

impl Text {
    pub fn new() -> Text {
        Text {
            widget: Widget::new(),
            text: String::new(),
            font_size: 24.0,
            font_name: String::from("Arial"),
            color: Color3B::WHITE,
            horizontal_alignment: TextHAlignment::CENTER,
            vertical_alignment: TextVAlignment::CENTER,
            shadow_enabled: false,
            shadow_color: Color3B::BLACK,
            shadow_offset: Vec2::new(2.0, -2.0),
        }
    }

    pub fn set_string(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn get_string(&self) -> &str {
        &self.text
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
    }

    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    pub fn set_font_name(&mut self, name: &str) {
        self.font_name = name.to_string();
    }

    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }

    pub fn set_text_color(&mut self, color: Color3B) {
        self.color = color;
    }

    pub fn get_text_color(&self) -> Color3B {
        self.color
    }
}

#[derive(Debug)]
pub struct LayoutParameter {
    margin_left: f32,
    margin_top: f32,
    margin_right: f32,
    margin_bottom: f32,
}

impl LayoutParameter {
    pub fn new() -> LayoutParameter {
        LayoutParameter {
            margin_left: 0.0,
            margin_top: 0.0,
            margin_right: 0.0,
            margin_bottom: 0.0,
        }
    }

    pub fn set_margin(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.margin_left = left;
        self.margin_top = top;
        self.margin_right = right;
        self.margin_bottom = bottom;
    }
}
