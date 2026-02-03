use crate::base::event::EventDispatcher;
use crate::base::scheduler::Scheduler;
use crate::base::{Ref, RefPtr, Size};
use std::cell::RefCell;

use crate::renderer::Renderer;
use glow::Context;
use std::rc::Rc;

#[derive(Debug)]
pub struct Director {
    running_scene: RefPtr<Scene>,
    next_scene: Option<RefPtr<Scene>>,
    scheduler: RefPtr<Scheduler>,
    event_dispatcher: RefPtr<EventDispatcher>,
    renderer: RefPtr<Renderer>,
    delta_time: f32,
    total_time: f32,
    last_update_time: std::time::Instant,
    is_paused: bool,
    is_cleanup: bool,
}

impl Director {
    pub fn get_instance() -> RefPtr<Director> {
        static mut DIRECTOR: Option<RefCell<RefPtr<Director>>> = None;
        unsafe {
            if DIRECTOR.is_none() {
                DIRECTOR = Some(RefCell::new(RefPtr::new(Director::new())));
            }
            DIRECTOR.as_ref().unwrap().borrow().clone()
        }
    }

    pub fn new() -> Director {
        Director {
            running_scene: RefPtr::new(Scene::new()),
            next_scene: None,
            scheduler: RefPtr::new(Scheduler::new()),
            event_dispatcher: RefPtr::new(EventDispatcher::new()),
            renderer: RefPtr::new(Renderer::new()),
            delta_time: 0.0,
            total_time: 0.0,
            last_update_time: std::time::Instant::now(),
            is_paused: false,
            is_cleanup: false,
        }
    }

    pub fn set_gl_context(&mut self, context: Rc<Context>) {
        self.renderer.borrow_mut().init_backend(context);
    }

    pub fn get_renderer(&self) -> &RefPtr<Renderer> {
        &self.renderer
    }

    pub fn get_running_scene(&self) -> &RefPtr<Scene> {
        &self.running_scene
    }

    pub fn get_scheduler(&self) -> &RefPtr<Scheduler> {
        &self.scheduler
    }

    pub fn get_event_dispatcher(&self) -> &RefPtr<EventDispatcher> {
        &self.event_dispatcher
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_total_time(&self) -> f32 {
        self.total_time
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn run_scene(&mut self, scene: RefPtr<Scene>) {
        self.next_scene = Some(scene);
    }

    pub fn push_scene(&mut self, scene: RefPtr<Scene>) {
        self.next_scene = Some(scene);
    }

    /// Pops the running scene
    pub fn pop_scene(&mut self) {
    }

    pub fn replace_scene(&mut self, scene: RefPtr<Scene>) {
        self.running_scene = scene;
    }

    pub fn main_loop(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_update_time);
        self.last_update_time = now;

        self.delta_time = elapsed.as_secs_f32();
        self.total_time += self.delta_time;

        if !self.is_paused {
            self.scheduler.borrow_mut().update(self.delta_time);
        }

        if let Some(scene) = self.next_scene.take() {
            self.running_scene = scene;
        }

<<<<<<< HEAD
        self.running_scene.borrow().visit(&mut self.renderer.borrow_mut(), &crate::math::Mat4::IDENTITY, 0);
=======
        // Render the current scene
        self.running_scene.borrow().visit(
            &mut self.renderer.borrow_mut(),
            &crate::math::Mat4::IDENTITY,
            0,
        );
>>>>>>> feature/warning-cleanup
        self.renderer.borrow_mut().render();
        log::info!("Director loop running. Delta: {}", self.delta_time);
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
        self.last_update_time = std::time::Instant::now();
    }

    pub fn stop(&mut self) {
        self.is_cleanup = true;
    }

    pub fn get_win_size(&self) -> Size {
        Size::new(960.0, 640.0)
    }

    pub fn get_visible_size(&self) -> Size {
        self.get_win_size()
    }

    pub fn get_visible_origin(&self) -> crate::math::Vec2 {
        crate::math::Vec2::ZERO
    }
}

#[derive(Debug)]
pub struct Scene {
    base: Ref,
    children: Vec<RefPtr<Node>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            base: Ref::new(),
            children: Vec::new(),
        }
    }

    pub fn get_children(&self) -> &Vec<RefPtr<Node>> {
        &self.children
    }

    pub fn add_child(&mut self, child: RefPtr<Node>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &RefPtr<Node>) {
        self.children
            .retain(|c| !c.borrow().get_id() == child.borrow().get_id());
    }

    pub fn update(&mut self, delta_time: f32) {
        for child in &mut self.children {
            child.borrow_mut().update(delta_time);
        }
    }

    pub fn visit(
        &self,
        renderer: &mut Renderer,
        parent_transform: &crate::math::Mat4,
        parent_flags: u32,
    ) {
        for child in &self.children {
            child
                .borrow_mut()
                .visit(renderer, parent_transform, parent_flags);
        }
    }
}

pub struct Node {
    base: Ref,
    parent: Option<RefPtr<Node>>,
    children: Vec<RefPtr<Node>>,
    position: crate::math::Vec2,
    rotation: f32,
    scale_x: f32,
    scale_y: f32,
    visible: bool,
    tag: i32,
    name: String,
    local_transform: crate::math::Mat4,
    global_transform: crate::math::Mat4,
    content_size: crate::math::Vec2,
    on_draw: Option<Box<dyn Fn(&mut Renderer, &crate::math::Mat4)>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            base: self.base.clone(),
            parent: None,
            children: Vec::new(),
            position: self.position,
            rotation: self.rotation,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            visible: self.visible,
            tag: self.tag,
            name: self.name.clone(),
            local_transform: self.local_transform,
            global_transform: self.global_transform,
            content_size: self.content_size,
            on_draw: None,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("base", &self.base)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("position", &self.position)
            .field("rotation", &self.rotation)
            .field("scale_x", &self.scale_x)
            .field("scale_y", &self.scale_y)
            .field("visible", &self.visible)
            .field("tag", &self.tag)
            .field("name", &self.name)
            .field("content_size", &self.content_size)
            .field("on_draw", &"Fn(...)")
            .finish()
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            base: Ref::new(),
            parent: None,
            children: Vec::new(),
            position: crate::math::Vec2::ZERO,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            visible: true,
            tag: 0,
            name: String::new(),
            local_transform: crate::math::Mat4::IDENTITY,
            global_transform: crate::math::Mat4::IDENTITY,
            content_size: crate::math::Vec2::ZERO,
            on_draw: None,
        }
    }

    pub fn set_on_draw(&mut self, callback: Box<dyn Fn(&mut Renderer, &crate::math::Mat4)>) {
        self.on_draw = Some(callback);
    }

    pub fn visit(
        &mut self,
        renderer: &mut Renderer,
        parent_transform: &crate::math::Mat4,
        _parent_flags: u32,
    ) {
        if !self.visible {
            return;
        }

        self.global_transform = *parent_transform * self.local_transform;

        for child in &self.children {
            child
                .borrow_mut()
                .visit(renderer, &self.global_transform, _parent_flags);
        }

        if let Some(callback) = &self.on_draw {
            callback(renderer, &self.global_transform);
        }
    }

    pub fn get_parent(&self) -> Option<&RefPtr<Node>> {
        self.parent.as_ref()
    }

    pub fn set_parent(&mut self, parent: RefPtr<Node>) {
        self.parent = Some(parent);
    }

    pub fn get_children(&self) -> &Vec<RefPtr<Node>> {
        &self.children
    }

    pub fn add_child(&mut self, child: RefPtr<Node>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &RefPtr<Node>) {
        self.children
            .retain(|c| c.borrow().get_id() != child.borrow().get_id());
    }

    pub fn remove_all_children(&mut self) {
        self.children.clear();
    }

    pub fn get_position(&self) -> &crate::math::Vec2 {
        &self.position
    }

    pub fn set_position(&mut self, position: crate::math::Vec2) {
        self.position = position;
        self.update_local_transform();
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.update_local_transform();
    }

    pub fn get_scale_x(&self) -> f32 {
        self.scale_x
    }

    pub fn get_scale_y(&self) -> f32 {
        self.scale_y
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale_x = scale;
        self.scale_y = scale;
        self.update_local_transform();
    }

    pub fn set_scale_xy(&mut self, scale_x: f32, scale_y: f32) {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self.update_local_transform();
    }

    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    pub fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_local_transform(&self) -> &crate::math::Mat4 {
        &self.local_transform
    }

    pub fn get_global_transform(&self) -> &crate::math::Mat4 {
        &self.global_transform
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn get_content_size(&self) -> crate::math::Vec2 {
        self.content_size
    }

    pub fn set_content_size(&mut self, size: crate::math::Vec2) {
        self.content_size = size;
    }

    fn update_local_transform(&mut self) {
        self.local_transform = crate::math::Mat4::create_translation(&crate::math::Vec3::new(
            self.position.x,
            self.position.y,
            0.0,
        ));
    }

<<<<<<< HEAD
    pub fn update(&mut self, delta_time: f32) {
    }
=======
    /// Updates the node
    pub fn update(&mut self, delta_time: f32) {}
>>>>>>> feature/warning-cleanup

    pub fn get_id(&self) -> usize {
        let ptr = &self.base as *const Ref as *const u8 as usize;
        ptr
    }

    pub fn get_base(&self) -> &Ref {
        &self.base
    }

    pub fn get_base_mut(&mut self) -> &mut Ref {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::RefPtr;

    #[test]
    fn test_scene_new() {
        let scene = Scene::new();
        assert_eq!(scene.get_children().len(), 0);
    }

    #[test]
    fn test_scene_add_child() {
        let mut scene = Scene::new();
        let node = RefPtr::new(Node::new());
        scene.add_child(node.clone());
        assert_eq!(scene.get_children().len(), 1);
    }

    #[test]
    fn test_scene_remove_child() {
        let mut scene = Scene::new();
        let node = RefPtr::new(Node::new());
        scene.add_child(node.clone());
        assert_eq!(scene.get_children().len(), 1);

        scene.remove_child(&node);
        assert_eq!(scene.get_children().len(), 0);
    }

    #[test]
    fn test_scene_multiple_children() {
        let mut scene = Scene::new();
        for i in 0..5 {
            let node = RefPtr::new(Node::new());
            scene.add_child(node);
        }
        assert_eq!(scene.get_children().len(), 5);
    }

    #[test]
    fn test_node_new() {
        let node = Node::new();
        assert_eq!(node.get_position().x, 0.0);
        assert_eq!(node.get_position().y, 0.0);
        assert_eq!(node.get_rotation(), 0.0);
        assert_eq!(node.get_scale_x(), 1.0);
        assert_eq!(node.get_scale_y(), 1.0);
        assert!(node.is_visible());
        assert_eq!(node.get_tag(), 0);
        assert_eq!(node.get_name(), "");
    }

    #[test]
    fn test_node_set_position() {
        let mut node = Node::new();
        node.set_position(crate::math::Vec2::new(100.0, 200.0));
        assert_eq!(node.get_position().x, 100.0);
        assert_eq!(node.get_position().y, 200.0);
    }

    #[test]
    fn test_node_set_rotation() {
        let mut node = Node::new();
        node.set_rotation(45.0);
        assert_eq!(node.get_rotation(), 45.0);

        node.set_rotation(-90.0);
        assert_eq!(node.get_rotation(), -90.0);
    }

    #[test]
    fn test_node_set_scale() {
        let mut node = Node::new();
        node.set_scale(2.0);
        assert_eq!(node.get_scale_x(), 2.0);
        assert_eq!(node.get_scale_y(), 2.0);
    }

    #[test]
    fn test_node_set_scale_xy() {
        let mut node = Node::new();
        node.set_scale_xy(1.5, 2.5);
        assert_eq!(node.get_scale_x(), 1.5);
        assert_eq!(node.get_scale_y(), 2.5);
    }

    #[test]
    fn test_node_set_visible() {
        let mut node = Node::new();
        assert!(node.is_visible());
        node.set_visible(false);
        assert!(!node.is_visible());
    }

    #[test]
    fn test_node_set_tag() {
        let mut node = Node::new();
        node.set_tag(42);
        assert_eq!(node.get_tag(), 42);
    }

    #[test]
    fn test_node_set_name() {
        let mut node = Node::new();
        node.set_name(String::from("test_node"));
        assert_eq!(node.get_name(), "test_node");
    }

    #[test]
    fn test_node_set_content_size() {
        let mut node = Node::new();
        node.set_content_size(crate::math::Vec2::new(100.0, 50.0));
        assert_eq!(node.get_content_size().x, 100.0);
        assert_eq!(node.get_content_size().y, 50.0);
    }

    #[test]
    fn test_node_get_id() {
        let node = Node::new();
        let id1 = node.get_id();
        let node2 = Node::new();
        let id2 = node2.get_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_node_add_child() {
        let mut node = Node::new();
        let child = RefPtr::new(Node::new());
        node.add_child(child.clone());
        assert_eq!(node.get_children().len(), 1);
    }

    #[test]
    fn test_node_remove_child() {
        let mut node = Node::new();
        let child = RefPtr::new(Node::new());
        node.add_child(child.clone());
        assert_eq!(node.get_children().len(), 1);

        node.remove_child(&child);
        assert_eq!(node.get_children().len(), 0);
    }

    #[test]
    fn test_node_remove_all_children() {
        let mut node = Node::new();
        for _ in 0..10 {
            node.add_child(RefPtr::new(Node::new()));
        }
        assert_eq!(node.get_children().len(), 10);

        node.remove_all_children();
        assert_eq!(node.get_children().len(), 0);
    }

    #[test]
    fn test_node_set_parent() {
        let mut node = Node::new();
        let parent = RefPtr::new(Node::new());
        node.set_parent(parent.clone());
        assert!(node.get_parent().is_some());
    }

    #[test]
    fn test_node_hierarchy() {
        let mut parent = Node::new();
        let child1 = RefPtr::new(Node::new());
        let child2 = RefPtr::new(Node::new());

        parent.add_child(child1.clone());
        parent.add_child(child2.clone());

        assert_eq!(parent.get_children().len(), 2);
    }

    #[test]
    fn test_node_transform_updates() {
        let mut node = Node::new();
        node.set_position(crate::math::Vec2::new(100.0, 50.0));
        let transform = node.get_local_transform();
        assert!(!transform.is_identity());
    }

    #[test]
    fn test_node_multiple_property_changes() {
        let mut node = Node::new();

        node.set_position(crate::math::Vec2::new(10.0, 20.0));
        node.set_rotation(30.0);
        node.set_scale(1.5);
        node.set_visible(false);
        node.set_tag(100);

        assert_eq!(node.get_position().x, 10.0);
        assert_eq!(node.get_rotation(), 30.0);
        assert_eq!(node.get_scale_x(), 1.5);
        assert!(!node.is_visible());
        assert_eq!(node.get_tag(), 100);
    }

    #[test]
    fn test_scene_with_nodes() {
        let mut scene = Scene::new();

        for i in 0..3 {
            let mut node = Node::new();
            node.set_position(crate::math::Vec2::new(i as f32 * 10.0, 0.0));
            node.set_tag(i as i32);
            scene.add_child(RefPtr::new(node));
        }

        assert_eq!(scene.get_children().len(), 3);
    }

    #[test]
    fn test_director_new() {
        let director = Director::new();
        assert_eq!(director.get_delta_time(), 0.0);
        assert_eq!(director.get_total_time(), 0.0);
        assert!(!director.is_paused());
    }

    #[test]
    fn test_director_get_win_size() {
        let director = Director::new();
        let size = director.get_win_size();
        assert_eq!(size.width, 960.0);
        assert_eq!(size.height, 640.0);
    }

    #[test]
    fn test_director_get_visible_size() {
        let director = Director::new();
        let size = director.get_visible_size();
        assert_eq!(size.width, 960.0);
        assert_eq!(size.height, 640.0);
    }

    #[test]
    fn test_director_get_visible_origin() {
        let director = Director::new();
        let origin = director.get_visible_origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
    }

    #[test]
    fn test_director_pause_resume() {
        let mut director = Director::new();
        assert!(!director.is_paused());

        director.pause();
        assert!(director.is_paused());

        director.resume();
        assert!(!director.is_paused());
    }

    #[test]
    fn test_director_stop() {
        let mut director = Director::new();
        director.stop();
    }

    #[test]
    fn test_director_run_scene() {
        let mut director = Director::new();
        let scene = RefPtr::new(Scene::new());
        director.run_scene(scene);
    }

    #[test]
    fn test_director_replace_scene() {
        let mut director = Director::new();
        let scene = RefPtr::new(Scene::new());
        director.replace_scene(scene);
    }

    #[test]
    fn test_director_push_scene() {
        let mut director = Director::new();
        let scene = RefPtr::new(Scene::new());
        director.push_scene(scene);
    }
}
