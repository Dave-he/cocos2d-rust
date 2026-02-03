use crate::action::{Action, FiniteTimeAction};
use crate::base::{Color3B, Color4F, Director, Node, Ref, RefPtr, Scene};
use crate::math::Vec2;
use crate::sprite::Sprite;

use std::rc::Rc;
use std::cell::RefCell;

use super::node::NodeType;

/// Layer - 图层类
///
/// Layer 是 Scene 中的可交互容器，继承自 Node。
/// 它提供触摸和键盘事件处理功能。
pub struct Layer {
    node: Node,
    touch_enabled: bool,
    mouse_enabled: bool,
    keyboard_enabled: bool,
    accelerometer_enabled: bool,
}

impl std::fmt::Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layer")
            .field("node", &self.node)
            .field("touch_enabled", &self.touch_enabled)
            .field("keyboard_enabled", &self.keyboard_enabled)
            .finish()
    }
}

impl AsRef<Node> for Layer {
    fn as_ref(&self) -> &Node {
        &self.node
    }
}

impl AsMut<Node> for Layer {
    fn as_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Layer {
    pub fn new() -> Self {
        let mut node = Node::with_type(NodeType::Layer);
        node.set_anchor_point(Vec2::new(0.5, 0.5));
        
        Self {
            node,
            touch_enabled: false,
            mouse_enabled: false,
            keyboard_enabled: false,
            accelerometer_enabled: false,
        }
    }

    pub fn create() -> Self {
        Self::new()
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    // ===== 事件启用 =====
    
    pub fn set_touch_enabled(&mut self, enabled: bool) {
        self.touch_enabled = enabled;
    }

    pub fn is_touch_enabled(&self) -> bool {
        self.touch_enabled
    }

    pub fn set_mouse_enabled(&mut self, enabled: bool) {
        self.mouse_enabled = enabled;
    }

    pub fn is_mouse_enabled(&self) -> bool {
        self.mouse_enabled
    }

    pub fn on_enter(&mut self) {}

    pub fn on_exit(&mut self) {}

    pub fn set_keyboard_enabled(&mut self, enabled: bool) {
        self.keyboard_enabled = enabled;
    }

    pub fn is_keyboard_enabled(&self) -> bool {
        self.keyboard_enabled
    }

    pub fn set_accelerometer_enabled(&mut self, enabled: bool) {
        self.accelerometer_enabled = enabled;
    }

    pub fn is_accelerometer_enabled(&self) -> bool {
        self.accelerometer_enabled
    }

    // ===== 触摸事件 =====
    
    pub fn on_touch_began(&mut self, _location: &Vec2) -> bool {
        false
    }

    pub fn on_touch_moved(&mut self, _location: &Vec2) {}

    pub fn on_touch_ended(&mut self, _location: &Vec2) {}

    pub fn on_touch_cancelled(&mut self, _location: &Vec2) {}

    // ===== 鼠标事件 =====
    
    pub fn on_mouse_down(&mut self, _button: u32, _location: &Vec2) -> bool {
        false
    }

    pub fn on_mouse_up(&mut self, _button: u32, _location: &Vec2) {}

    pub fn on_mouse_move(&mut self, _location: &Vec2) {}

    pub fn on_mouse_scroll(&mut self, _delta: f32) {}

    // ===== 键盘事件 =====
    
    pub fn on_key_pressed(&mut self, _key_code: u32) -> bool {
        false
    }

    pub fn on_key_released(&mut self, _key_code: u32) {
    }

    // ===== 加速器事件 =====
    
    pub fn on_acceleration(&mut self, _accel: (f32, f32, f32)) {
    }

    // ===== 生命周期 =====
    
    pub fn on_enter(&mut self) {
        self.node.on_enter();
    }

    pub fn on_exit(&mut self) {
        self.node.on_exit();
    }

    pub fn on_enter_transition_did_finish(&mut self) {
    }

    pub fn on_exit_transition_did_start(&mut self) {
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}

/// LayerColor - 颜色图层
///
/// 可以设置背景颜色的图层。
use crate::math::Size;

pub struct LayerColor {
    layer: Layer,
    blend_func_src: u32,
    blend_func_dst: u32,
}

impl std::fmt::Debug for LayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerColor")
            .field("layer", &self.layer)
            .finish()
    }
}

impl AsRef<Node> for LayerColor {
    fn as_ref(&self) -> &Node {
        self.layer.as_ref()
    }
}

impl AsMut<Node> for LayerColor {
    fn as_mut(&mut self) -> &mut Node {
        self.layer.as_mut()
    }
}

impl AsRef<Layer> for LayerColor {
    fn as_ref(&self) -> &Layer {
        &self.layer
    }
}

impl AsMut<Layer> for LayerColor {
    fn as_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }
}

impl LayerColor {
    pub fn new() -> Self {
        let mut layer = Layer::new();
        layer.node_mut().set_color(Color3B::white());
        
        Self {
            layer,
            blend_func_src: 1, // GL_ONE
            blend_func_dst: 771, // GL_ONE_MINUS_SRC_ALPHA
        }
    }

    pub fn create_with_color(color: Color4B, size: Size) -> Self {
        let mut layer_color = Self::new();
        layer_color.set_color(color);
        layer_color.set_content_size(size);
        layer_color
    }

    pub fn create_with_color_width_height(
        r: u8, g: u8, b: u8, a: u8,
        width: f32, height: f32,
    ) -> Self {
        let color = Color4B::new(r, g, b, a);
        let size = Size::new(width, height);
        Self::create_with_color(color, size)
    }

    pub fn layer(&self) -> &Layer {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    pub fn node(&self) -> &Node {
        self.layer.node()
    }

    pub fn node_mut(&mut self) -> &mut Node {
        self.layer.node_mut()
    }

    pub fn set_color(&mut self, color: Color4B) {
        self.layer.node_mut().set_color(Color3B::new(color.r, color.g, color.b));
        self.layer.node_mut().set_opacity(color.a);
    }

    pub fn set_color3b(&mut self, color: Color3B) {
        self.layer.node_mut().set_color(color);
    }

    pub fn get_color(&self) -> Color4B {
        let color = self.layer.node().color();
        Color4B::new(color.r, color.g, color.b, self.layer.node().opacity())
    }

    pub fn get_color3b(&self) -> Color3B {
        self.layer.node().color()
    }

    pub fn set_content_size(&mut self, size: Size) {
        self.layer.node_mut().set_content_size(size);
    }

    pub fn content_size(&self) -> Size {
        self.layer.node().content_size()
    }

    pub fn set_blend_func(&mut self, src: u32, dst: u32) {
        self.blend_func_src = src;
        self.blend_func_dst = dst;
    }

    pub fn blend_func_src(&self) -> u32 {
        self.blend_func_src
    }

    pub fn blend_func_dst(&self) -> u32 {
        self.blend_func_dst
    }

    pub fn change_width(&mut self, width: f32) {
        let mut size = self.content_size();
        size.width = width;
        self.set_content_size(size);
    }

    pub fn change_height(&mut self, height: f32) {
        let mut size = self.content_size();
        size.height = height;
        self.set_content_size(size);
    }

    pub fn change_width_and_height(&mut self, width: f32, height: f32) {
        self.set_content_size(Size::new(width, height));
    }

    pub fn on_enter(&mut self) {
        self.layer.on_enter();
    }

    pub fn on_exit(&mut self) {
        self.layer.on_exit();
    }
}

impl Default for LayerColor {
    fn default() -> Self {
        Self::new()
    }
}

/// LayerGradient - 渐变图层
///
/// 支持颜色渐变的图层。
pub struct LayerGradient {
    layer_color: LayerColor,
    start_color: Color3B,
    end_color: Color3B,
    start_vector: Vec2,
    end_vector: Vec2,
    compressed_interpolation: bool,
}

impl std::fmt::Debug for LayerGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerGradient")
            .field("layer_color", &self.layer_color)
            .field("start_color", &self.start_color)
            .field("end_color", &self.end_color)
            .finish()
    }
}

impl AsRef<Node> for LayerGradient {
    fn as_ref(&self) -> &Node {
        self.layer_color.as_ref()
    }
}

impl AsMut<Node> for LayerGradient {
    fn as_mut(&mut self) -> &mut Node {
        self.layer_color.as_mut()
    }
}

impl AsRef<LayerColor> for LayerGradient {
    fn as_ref(&self) -> &LayerColor {
        &self.layer_color
    }
}

impl AsMut<LayerColor> for LayerGradient {
    fn as_mut(&mut self) -> &mut LayerColor {
        &mut self.layer_color
    }
}

impl LayerGradient {
    pub fn new() -> Self {
        Self {
            layer_color: LayerColor::new(),
            start_color: Color3B::black(),
            end_color: Color3B::white(),
            start_vector: Vec2::zero(),
            end_vector: Vec2::new(0.0, 1.0),
            compressed_interpolation: false,
        }
    }

    pub fn create_with_colors(start: Color3B, end: Color3B, size: Size) -> Self {
        let mut gradient = Self::new();
        gradient.set_start_color(start);
        gradient.set_end_color(end);
        gradient.set_content_size(size);
        gradient
    }

    pub fn layer_color(&self) -> &LayerColor {
        &self.layer_color
    }

    pub fn layer_color_mut(&mut self) -> &mut LayerColor {
        &mut self.layer_color
    }

    pub fn set_start_color(&mut self, color: Color3B) {
        self.start_color = color;
    }

    pub fn start_color(&self) -> Color3B {
        self.start_color
    }

    pub fn set_end_color(&mut self, color: Color3B) {
        self.end_color = color;
    }

    pub fn end_color(&self) -> Color3B {
        self.end_color
    }

    pub fn set_vector(&mut self, start: Vec2, end: Vec2) {
        self.start_vector = start;
        self.end_vector = end;
    }

    pub fn start_vector(&self) -> Vec2 {
        self.start_vector
    }

    pub fn end_vector(&self) -> Vec2 {
        self.end_vector
    }

    pub fn set_compressed_interpolation(&mut self, compressed: bool) {
        self.compressed_interpolation = compressed;
    }

    pub fn is_compressed_interpolation(&self) -> bool {
        self.compressed_interpolation
    }

    pub fn set_content_size(&mut self, size: Size) {
        self.layer_color.set_content_size(size);
    }

    pub fn content_size(&self) -> Size {
        self.layer_color.content_size()
    }

    pub fn on_enter(&mut self) {
        self.layer_color.on_enter();
    }

    pub fn on_exit(&mut self) {
        self.layer_color.on_exit();
    }
}

impl Default for LayerGradient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new();
        assert!(!layer.is_touch_enabled());
        assert!(!layer.is_keyboard_enabled());
        assert_eq!(layer.node().node_type(), NodeType::Layer);
    }

    #[test]
    fn test_layer_touch_enabled() {
        let mut layer = Layer::new();
        assert!(!layer.is_touch_enabled());

        layer.set_touch_enabled(true);
        assert!(layer.is_touch_enabled());

        layer.set_touch_enabled(false);
        assert!(!layer.is_touch_enabled());
    }

    #[test]
    fn test_layer_keyboard_enabled() {
        let mut layer = Layer::new();
        assert!(!layer.is_keyboard_enabled());

        layer.set_keyboard_enabled(true);
        assert!(layer.is_keyboard_enabled());
    }

    #[test]
    fn test_layer_mouse_enabled() {
        let mut layer = Layer::new();
        assert!(!layer.is_mouse_enabled());

        layer.set_mouse_enabled(true);
        assert!(layer.is_mouse_enabled());
    }

    #[test]
    fn test_layer_accelerometer_enabled() {
        let mut layer = Layer::new();
        assert!(!layer.is_accelerometer_enabled());

        layer.set_accelerometer_enabled(true);
        assert!(layer.is_accelerometer_enabled());
    }

    #[test]
    fn test_layer_touch_events() {
        let mut layer = Layer::new();
        let location = Vec2::new(100.0, 200.0);

        assert!(!layer.on_touch_began(&location));
        layer.on_touch_moved(&location);
        layer.on_touch_ended(&location);
        layer.on_touch_cancelled(&location);
    }

    #[test]
    fn test_layer_mouse_events() {
        let mut layer = Layer::new();
        let location = Vec2::new(100.0, 200.0);

        assert!(!layer.on_mouse_down(0, &location));
        layer.on_mouse_up(0, &location);
        layer.on_mouse_move(&location);
        layer.on_mouse_scroll(1.0);
    }

    #[test]
    fn test_layer_keyboard_events() {
        let mut layer = Layer::new();

        assert!(!layer.on_key_pressed(65)); // 'A'
        layer.on_key_released(65);
    }

    #[test]
    fn test_layer_on_enter_exit() {
        let mut layer = Layer::new();
        assert!(!layer.node().is_running());

        layer.on_enter();
        assert!(layer.node().is_running());

        layer.on_exit();
        assert!(!layer.node().is_running());
    }

    #[test]
    fn test_layer_creation_create() {
        let layer = Layer::create();
        assert_eq!(layer.node().node_type(), NodeType::Layer);
    }

    #[test]
    fn test_layer_color_creation() {
        let layer_color = LayerColor::new();
        assert!(!layer_color.layer().is_touch_enabled());
    }

    #[test]
    fn test_layer_color_create_with_color() {
        let layer_color = LayerColor::create_with_color(
            Color4B::new(255, 0, 0, 128),
            Size::new(100.0, 50.0)
        );
        assert_eq!(layer_color.content_size(), Size::new(100.0, 50.0));
        assert_eq!(layer_color.get_color().r, 255);
        assert_eq!(layer_color.get_color().g, 0);
        assert_eq!(layer_color.get_color().b, 0);
        assert_eq!(layer_color.get_color().a, 128);
    }

    #[test]
    fn test_layer_color_create_with_width_height() {
        let layer_color = LayerColor::create_with_color_width_height(
            255, 0, 0, 128,
            200.0, 100.0
        );
        assert_eq!(layer_color.content_size(), Size::new(200.0, 100.0));
    }

    #[test]
    fn test_layer_color_set_color() {
        let mut layer_color = LayerColor::new();
        layer_color.set_color(Color4B::new(100, 150, 200, 255));
        
        let color = layer_color.get_color();
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_layer_color_set_color3b() {
        let mut layer_color = LayerColor::new();
        layer_color.set_color3b(Color3B::new(50, 100, 150));
        
        let color = layer_color.get_color3b();
        assert_eq!(color.r, 50);
        assert_eq!(color.g, 100);
        assert_eq!(color.b, 150);
    }

    #[test]
    fn test_layer_color_blend_func() {
        let mut layer_color = LayerColor::new();
        
        assert_eq!(layer_color.blend_func_src(), 1);
        assert_eq!(layer_color.blend_func_dst(), 771);

        layer_color.set_blend_func(2, 772);
        assert_eq!(layer_color.blend_func_src(), 2);
        assert_eq!(layer_color.blend_func_dst(), 772);
    }

    #[test]
    fn test_layer_color_change_width() {
        let mut layer_color = LayerColor::new();
        layer_color.set_content_size(Size::new(100.0, 50.0));

        layer_color.change_width(200.0);
        assert_eq!(layer_color.content_size().width, 200.0);
        assert_eq!(layer_color.content_size().height, 50.0);
    }

    #[test]
    fn test_layer_color_change_height() {
        let mut layer_color = LayerColor::new();
        layer_color.set_content_size(Size::new(100.0, 50.0));

        layer_color.change_height(100.0);
        assert_eq!(layer_color.content_size().width, 100.0);
        assert_eq!(layer_color.content_size().height, 100.0);
    }

    #[test]
    fn test_layer_color_change_width_and_height() {
        let mut layer_color = LayerColor::new();
        layer_color.change_width_and_height(300.0, 200.0);
        assert_eq!(layer_color.content_size(), Size::new(300.0, 200.0));
    }

    #[test]
    fn test_layer_gradient_creation() {
        let gradient = LayerGradient::new();
        assert_eq!(gradient.start_color(), Color3B::black());
        assert_eq!(gradient.end_color(), Color3B::white());
    }

    #[test]
    fn test_layer_gradient_create_with_colors() {
        let gradient = LayerGradient::create_with_colors(
            Color3B::new(255, 0, 0),
            Color3B::new(0, 0, 255),
            Size::new(100.0, 100.0)
        );
        assert_eq!(gradient.start_color(), Color3B::new(255, 0, 0));
        assert_eq!(gradient.end_color(), Color3B::new(0, 0, 255));
        assert_eq!(gradient.content_size(), Size::new(100.0, 100.0));
    }

    #[test]
    fn test_layer_gradient_set_colors() {
        let mut gradient = LayerGradient::new();
        gradient.set_start_color(Color3B::new(128, 128, 128));
        gradient.set_end_color(Color3B::new(200, 200, 200));
        
        assert_eq!(gradient.start_color(), Color3B::new(128, 128, 128));
        assert_eq!(gradient.end_color(), Color3B::new(200, 200, 200));
    }

    #[test]
    fn test_layer_gradient_set_vector() {
        let mut gradient = LayerGradient::new();
        gradient.set_vector(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        
        assert_eq!(gradient.start_vector(), Vec2::new(0.0, 0.0));
        assert_eq!(gradient.end_vector(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_layer_gradient_compressed_interpolation() {
        let mut gradient = LayerGradient::new();
        assert!(!gradient.is_compressed_interpolation());

        gradient.set_compressed_interpolation(true);
        assert!(gradient.is_compressed_interpolation());
    }

    #[test]
    fn test_layer_gradient_on_enter_exit() {
        let mut gradient = LayerGradient::new();
        assert!(!gradient.node().is_running());

        gradient.on_enter();
        assert!(gradient.node().is_running());

        gradient.on_exit();
        assert!(!gradient.node().is_running());
    }
}
