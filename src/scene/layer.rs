use crate::base::{Director, Scene, Node, Color3B, Color4F};
use crate::math::Vec2;
use crate::sprite::Sprite;
use crate::action::{Action, FiniteTimeAction, Ease, MoveTo, RotateTo, Sequence};

#[derive(Debug)]
pub struct Layer {
    node: Node,
    touch_enabled: bool,
    keyboard_enabled: bool,
}

impl Layer {
    pub fn new() -> Layer {
        Layer {
            node: Node::new(),
            touch_enabled: false,
            keyboard_enabled: false,
        }
    }

    pub fn create() -> Layer {
        Layer::new()
    }

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn add_child(&mut self, child: Ref<Node>) {
        self.node.add_child(child);
    }

    pub fn set_touch_enabled(&mut self, enabled: bool) {
        self.touch_enabled = enabled;
    }

    pub fn is_touch_enabled(&self) -> bool {
        self.touch_enabled
    }

    pub fn on_enter(&mut self) {
    }

    pub fn on_exit(&mut self) {
    }

    pub fn on_touch_began(&mut self, _location: &Vec2) -> bool {
        false
    }

    pub fn on_touch_moved(&mut self, _location: &Vec2) {
    }

    pub fn on_touch_ended(&mut self, _location: &Vec2) {
    }
}

#[derive(Debug)]
pub struct LayerColor {
    layer: Layer,
    color: Color4F,
    opacity: u8,
}

impl LayerColor {
    pub fn new() -> LayerColor {
        LayerColor {
            layer: Layer::new(),
            color: Color4F::BLACK,
            opacity: 255,
        }
    }

    pub fn create_with_color(r: f32, g: f32, b: f32, a: f32) -> LayerColor {
        LayerColor {
            layer: Layer::new(),
            color: Color4F::new(r, g, b, a),
            opacity: (a * 255.0) as u8,
        }
    }

    pub fn set_color(&mut self, color: Color4F) {
        self.color = color;
    }

    pub fn get_color(&self) -> Color4F {
        self.color
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }
}
