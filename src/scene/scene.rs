/// Scene - 场景类
///
/// Scene 是所有游戏场景的容器，继承自 Node。
/// 它通常作为场景图的根节点。

use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

use super::node::{Node, NodeType};

/// Scene - 场景类
pub struct Scene {
    node: Node,
}

impl std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene")
            .field("node", &self.node)
            .finish()
    }
}

impl AsRef<Node> for Scene {
    fn as_ref(&self) -> &Node {
        &self.node
    }
}

impl AsMut<Node> for Scene {
    fn as_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Scene {
    pub fn new() -> Self {
        let mut node = Node::with_type(NodeType::Scene);
        node.set_local_z_order(0);
        
        Self { node }
    }

    pub fn create() -> Self {
        Self::new()
    }

    pub fn create_with_size() -> Self {
        Self::new()
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    // ===== 转发 Node 方法 =====
    
    pub fn add_child(&mut self, child: Rc<RefCell<Node>>, z_order: i32, name: Option<&str>) {
        self.node.add_child(child, z_order, name);
    }

    pub fn add_child_simple(&mut self, child: Rc<RefCell<Node>>) {
        self.node.add_child_simple(child);
    }

    pub fn get_child_by_tag(&self, tag: i32) -> Option<Rc<RefCell<Node>>> {
        self.node.get_child_by_tag(tag)
    }

    pub fn get_child_by_name(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        self.node.get_child_by_name(name)
    }

    pub fn remove_child(&mut self, child: &Rc<RefCell<Node>>, cleanup: bool) {
        self.node.remove_child(child, cleanup);
    }

    pub fn remove_child_by_tag(&mut self, tag: i32, cleanup: bool) {
        self.node.remove_child_by_tag(tag, cleanup);
    }

    pub fn remove_all_children(&mut self, cleanup: bool) {
        self.node.remove_all_children(cleanup);
    }

    pub fn get_children(&self) -> &[Rc<RefCell<Node>>] {
        self.node.get_children()
    }

    pub fn get_children_count(&self) -> usize {
        self.node.get_children_count()
    }

    pub fn set_position(&mut self, pos: crate::math::Vec2) {
        self.node.set_position(pos);
    }

    pub fn position(&self) -> crate::math::Vec2 {
        self.node.position()
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.node.set_scale(scale);
    }

    pub fn scale(&self) -> f32 {
        self.node.scale()
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.node.set_rotation(rotation);
    }

    pub fn rotation(&self) -> f32 {
        self.node.rotation()
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.node.set_visible(visible);
    }

    pub fn is_visible(&self) -> bool {
        self.node.is_visible()
    }

    pub fn set_tag(&mut self, tag: i32) {
        self.node.set_tag(tag);
    }

    pub fn tag(&self) -> i32 {
        self.node.tag()
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.node.set_name(name);
    }

    pub fn name(&self) -> &str {
        self.node.name()
    }

    pub fn on_enter(&mut self) {
        self.node.on_enter();
    }

    pub fn on_exit(&mut self) {
        self.node.on_exit();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

// ===== 场景类型特征 =====

/// 场景过渡接口
pub trait SceneTransition {
    fn get_in_scene(&self) -> Option<Rc<RefCell<Scene>>>;
    fn get_out_scene(&self) -> Option<Rc<RefCell<Scene>>>;
    fn duration(&self) -> f32;
    fn update(&mut self, delta: f32) -> bool;
    fn is_done(&self) -> bool;
}

/// 可以使用 Any 进行转换
impl Scene {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_creation() {
        let scene = Scene::new();
        assert!(scene.is_visible());
        assert_eq!(scene.get_children_count(), 0);
        assert_eq!(scene.tag(), super::super::node::TAG_INVALID);
    }

    #[test]
    fn test_scene_create() {
        let scene = Scene::create();
        assert_eq!(scene.get_children_count(), 0);
    }

    #[test]
    fn test_scene_add_child() {
        let mut scene = Scene::new();
        let child = Rc::new(RefCell::new(Node::new()));
        child.borrow_mut().set_tag(100);

        scene.add_child_simple(Rc::clone(&child));

        assert_eq!(scene.get_children_count(), 1);
        let found = scene.get_child_by_tag(100);
        assert!(found.is_some());
    }

    #[test]
    fn test_scene_remove_child() {
        let mut scene = Scene::new();
        let child = Rc::new(RefCell::new(Node::new()));
        child.borrow_mut().set_tag(100);

        scene.add_child_simple(Rc::clone(&child));
        scene.remove_child_by_tag(100, true);

        assert_eq!(scene.get_children_count(), 0);
    }

    #[test]
    fn test_scene_position() {
        let mut scene = Scene::new();
        scene.set_position(crate::math::Vec2::new(100.0, 200.0));
        assert_eq!(scene.position(), crate::math::Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_scene_scale() {
        let mut scene = Scene::new();
        scene.set_scale(2.0);
        assert_eq!(scene.scale(), 2.0);
    }

    #[test]
    fn test_scene_rotation() {
        let mut scene = Scene::new();
        scene.set_rotation(45.0);
        assert_eq!(scene.rotation(), 45.0);
    }

    #[test]
    fn test_scene_visible() {
        let mut scene = Scene::new();
        assert!(scene.is_visible());

        scene.set_visible(false);
        assert!(!scene.is_visible());
    }

    #[test]
    fn test_scene_tag() {
        let mut scene = Scene::new();
        scene.set_tag(999);
        assert_eq!(scene.tag(), 999);
    }

    #[test]
    fn test_scene_name() {
        let mut scene = Scene::new();
        scene.set_name("MainScene");
        assert_eq!(scene.name(), "MainScene");
    }

    #[test]
    fn test_scene_on_enter_exit() {
        let mut scene = Scene::new();
        
        scene.on_enter();
        assert!(scene.node().is_running());

        scene.on_exit();
        assert!(!scene.node().is_running());
    }

    #[test]
    fn test_scene_node_ref() {
        let scene = Scene::new();
        let node = scene.node();
        assert_eq!(node.node_type(), NodeType::Scene);
    }

    #[test]
    fn test_scene_multiple_children() {
        let mut scene = Scene::new();
        
        for i in 0..5 {
            let child = Rc::new(RefCell::new(Node::new()));
            child.borrow_mut().set_tag(i);
            scene.add_child_simple(child);
        }

        assert_eq!(scene.get_children_count(), 5);
        
        for i in 0..5 {
            assert!(scene.get_child_by_tag(i).is_some());
        }
    }
}
