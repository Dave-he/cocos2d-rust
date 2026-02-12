use std::cell::RefCell;
use std::rc::Rc;

use crate::base::Rect;
use crate::math::Vec2;
use crate::renderer::Renderer;

use super::node::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClippingType {
    Stencil,
    Scissor,
    AlphaMask,
}

#[derive(Debug, Clone)]
pub struct ClippingNode {
    node: Node,
    stencil: Option<Rc<RefCell<Node>>>,
    alpha_threshold: f32,
    inverted: bool,
    clipping_type: ClippingType,
    clipping_enabled: bool,
    clipping_region: Option<Rect>,
    stencil_bits: u8,
}

impl ClippingNode {
    pub fn new() -> Self {
        Self {
            node: Node::new(),
            stencil: None,
            alpha_threshold: 0.0,
            inverted: false,
            clipping_type: ClippingType::Stencil,
            clipping_enabled: true,
            clipping_region: None,
            stencil_bits: 8,
        }
    }

    pub fn with_stencil(stencil: Rc<RefCell<Node>>) -> Self {
        let mut clipping = Self::new();
        clipping.set_stencil(Some(stencil));
        clipping
    }

    pub fn with_rect(rect: Rect) -> Self {
        let mut clipping = Self::new();
        clipping.set_clipping_type(ClippingType::Scissor);
        clipping.set_clipping_region(Some(rect));
        clipping
    }

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn set_stencil(&mut self, stencil: Option<Rc<RefCell<Node>>>) {
        self.stencil = stencil;
    }

    pub fn get_stencil(&self) -> Option<&Rc<RefCell<Node>>> {
        self.stencil.as_ref()
    }

    pub fn set_alpha_threshold(&mut self, threshold: f32) {
        self.alpha_threshold = threshold.clamp(0.0, 1.0);
    }

    pub fn get_alpha_threshold(&self) -> f32 {
        self.alpha_threshold
    }

    pub fn set_inverted(&mut self, inverted: bool) {
        self.inverted = inverted;
    }

    pub fn is_inverted(&self) -> bool {
        self.inverted
    }

    pub fn set_clipping_type(&mut self, clipping_type: ClippingType) {
        self.clipping_type = clipping_type;
    }

    pub fn get_clipping_type(&self) -> ClippingType {
        self.clipping_type
    }

    pub fn set_clipping_enabled(&mut self, enabled: bool) {
        self.clipping_enabled = enabled;
    }

    pub fn is_clipping_enabled(&self) -> bool {
        self.clipping_enabled
    }

    pub fn set_clipping_region(&mut self, region: Option<Rect>) {
        self.clipping_region = region;
    }

    pub fn get_clipping_region(&self) -> Option<Rect> {
        self.clipping_region
    }

    pub fn set_stencil_bits(&mut self, bits: u8) {
        self.stencil_bits = bits.min(8);
    }

    pub fn get_stencil_bits(&self) -> u8 {
        self.stencil_bits
    }

    pub fn visit(&mut self, _renderer: &mut Renderer, _parent_transform: &crate::math::Mat4) {
        if !self.node.is_visible() {
            return;
        }

        if !self.clipping_enabled {
            return;
        }
    }

    pub fn on_enter(&mut self) {
        self.node.on_enter();
    }

    pub fn on_exit(&mut self) {
        self.node.on_exit();
    }
}

impl Default for ClippingNode {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ClippingRectangleNode {
    clipping: ClippingNode,
}

impl ClippingRectangleNode {
    pub fn new() -> Self {
        let mut clipping = ClippingNode::new();
        clipping.set_clipping_type(ClippingType::Scissor);
        
        Self { clipping }
    }

    pub fn with_rect(rect: Rect) -> Self {
        let mut node = Self::new();
        node.set_clipping_region(rect);
        node
    }

    pub fn get_clipping_node(&self) -> &ClippingNode {
        &self.clipping
    }

    pub fn get_clipping_node_mut(&mut self) -> &mut ClippingNode {
        &mut self.clipping
    }

    pub fn set_clipping_region(&mut self, rect: Rect) {
        self.clipping.set_clipping_region(Some(rect));
    }

    pub fn get_clipping_region(&self) -> Option<Rect> {
        self.clipping.get_clipping_region()
    }

    pub fn set_clipping_enabled(&mut self, enabled: bool) {
        self.clipping.set_clipping_enabled(enabled);
    }

    pub fn is_clipping_enabled(&self) -> bool {
        self.clipping.is_clipping_enabled()
    }

    pub fn get_node(&self) -> &Node {
        self.clipping.get_node()
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        self.clipping.get_node_mut()
    }

    pub fn visit(&mut self, renderer: &mut Renderer, parent_transform: &crate::math::Mat4) {
        self.clipping.visit(renderer, parent_transform);
    }
}

impl Default for ClippingRectangleNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipping_node_creation() {
        let clipping = ClippingNode::new();
        
        assert!(clipping.get_stencil().is_none());
        assert_eq!(clipping.get_alpha_threshold(), 0.0);
        assert!(!clipping.is_inverted());
        assert_eq!(clipping.get_clipping_type(), ClippingType::Stencil);
        assert!(clipping.is_clipping_enabled());
    }

    #[test]
    fn test_clipping_with_stencil() {
        let stencil = Rc::new(RefCell::new(Node::new()));
        let clipping = ClippingNode::with_stencil(stencil.clone());
        
        assert!(clipping.get_stencil().is_some());
        assert_eq!(clipping.get_clipping_type(), ClippingType::Stencil);
    }

    #[test]
    fn test_clipping_with_rect() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        let clipping = ClippingNode::with_rect(rect);
        
        assert_eq!(clipping.get_clipping_type(), ClippingType::Scissor);
        assert_eq!(clipping.get_clipping_region(), Some(rect));
    }

    #[test]
    fn test_alpha_threshold() {
        let mut clipping = ClippingNode::new();
        
        clipping.set_alpha_threshold(0.5);
        assert_eq!(clipping.get_alpha_threshold(), 0.5);
        
        clipping.set_alpha_threshold(1.5);
        assert_eq!(clipping.get_alpha_threshold(), 1.0);
        
        clipping.set_alpha_threshold(-0.5);
        assert_eq!(clipping.get_alpha_threshold(), 0.0);
    }

    #[test]
    fn test_inverted_clipping() {
        let mut clipping = ClippingNode::new();
        
        assert!(!clipping.is_inverted());
        
        clipping.set_inverted(true);
        assert!(clipping.is_inverted());
    }

    #[test]
    fn test_clipping_types() {
        let mut clipping = ClippingNode::new();
        
        assert_eq!(clipping.get_clipping_type(), ClippingType::Stencil);
        
        clipping.set_clipping_type(ClippingType::Scissor);
        assert_eq!(clipping.get_clipping_type(), ClippingType::Scissor);
        
        clipping.set_clipping_type(ClippingType::AlphaMask);
        assert_eq!(clipping.get_clipping_type(), ClippingType::AlphaMask);
    }

    #[test]
    fn test_clipping_enabled() {
        let mut clipping = ClippingNode::new();
        
        assert!(clipping.is_clipping_enabled());
        
        clipping.set_clipping_enabled(false);
        assert!(!clipping.is_clipping_enabled());
    }

    #[test]
    fn test_stencil_bits() {
        let mut clipping = ClippingNode::new();
        
        assert_eq!(clipping.get_stencil_bits(), 8);
        
        clipping.set_stencil_bits(4);
        assert_eq!(clipping.get_stencil_bits(), 4);
        
        clipping.set_stencil_bits(16);
        assert_eq!(clipping.get_stencil_bits(), 8);
    }

    #[test]
    fn test_rectangle_clipping_node() {
        let mut rect_node = ClippingRectangleNode::new();
        
        assert!(rect_node.is_clipping_enabled());
        assert_eq!(
            rect_node.get_clipping_node().get_clipping_type(),
            ClippingType::Scissor
        );
    }

    #[test]
    fn test_rectangle_clipping_with_rect() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect_node = ClippingRectangleNode::with_rect(rect);
        
        assert_eq!(rect_node.get_clipping_region(), Some(rect));
    }

    #[test]
    fn test_rectangle_clipping_set_region() {
        let mut rect_node = ClippingRectangleNode::new();
        let rect = Rect::new(50.0, 50.0, 200.0, 150.0);
        
        rect_node.set_clipping_region(rect);
        assert_eq!(rect_node.get_clipping_region(), Some(rect));
    }

    #[test]
    fn test_rectangle_clipping_toggle() {
        let mut rect_node = ClippingRectangleNode::new();
        
        rect_node.set_clipping_enabled(false);
        assert!(!rect_node.is_clipping_enabled());
        
        rect_node.set_clipping_enabled(true);
        assert!(rect_node.is_clipping_enabled());
    }

    #[test]
    fn test_stencil_node_update() {
        let mut clipping = ClippingNode::new();
        let stencil = Rc::new(RefCell::new(Node::new()));
        
        stencil.borrow_mut().set_position(Vec2::new(10.0, 20.0));
        clipping.set_stencil(Some(stencil.clone()));
        
        let retrieved_stencil = clipping.get_stencil().unwrap();
        assert_eq!(
            retrieved_stencil.borrow().position(),
            Vec2::new(10.0, 20.0)
        );
    }
}
