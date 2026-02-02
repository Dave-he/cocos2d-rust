use std::cell::RefCell;
use crate::base::{Size, Ref, RefPtr};
use crate::base::scheduler::Scheduler;
use crate::base::event::EventDispatcher;

use crate::renderer::Renderer;
use std::rc::Rc;
use glow::Context;

/// Director is the main object that runs the scene.
///
/// It is a singleton object that runs the main game loop. The Director is
/// responsible for managing the scenes and the transition between them.
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
    /// Gets the singleton instance of Director
    pub fn get_instance() -> RefPtr<Director> {
        static mut DIRECTOR: Option<RefCell<RefPtr<Director>>> = None;
        unsafe {
            if DIRECTOR.is_none() {
                DIRECTOR = Some(RefCell::new(RefPtr::new(Director::new())));
            }
            DIRECTOR.as_ref().unwrap().borrow().clone()
        }
    }

    /// Creates a new Director
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

    /// Gets the running scene
    pub fn get_running_scene(&self) -> &RefPtr<Scene> {
        &self.running_scene
    }

    /// Gets the scheduler
    pub fn get_scheduler(&self) -> &RefPtr<Scheduler> {
        &self.scheduler
    }

    /// Gets the event dispatcher
    pub fn get_event_dispatcher(&self) -> &RefPtr<EventDispatcher> {
        &self.event_dispatcher
    }

    /// Gets the delta time
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    /// Gets the total time
    pub fn get_total_time(&self) -> f32 {
        self.total_time
    }

    /// Checks if the director is paused
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Runs a scene
    pub fn run_scene(&mut self, scene: RefPtr<Scene>) {
        self.next_scene = Some(scene);
    }

    /// Pushes a scene to the stack
    pub fn push_scene(&mut self, scene: RefPtr<Scene>) {
        self.next_scene = Some(scene);
    }

    /// Pops the running scene
    pub fn pop_scene(&mut self) {
    }

    /// Replaces the running scene
    pub fn replace_scene(&mut self, scene: RefPtr<Scene>) {
        self.running_scene = scene;
    }

    /// Main loop function
    pub fn main_loop(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_update_time);
        self.last_update_time = now;

        // Calculate delta time in seconds
        self.delta_time = elapsed.as_secs_f32();
        self.total_time += self.delta_time;

        if !self.is_paused {
            // Update the scheduler
            self.scheduler.borrow_mut().update(self.delta_time);
        }

        // Process scene transitions
        if let Some(scene) = self.next_scene.take() {
            self.running_scene = scene;
        }

        // Render the current scene
        self.running_scene.borrow().visit(&mut self.renderer.borrow_mut(), &crate::math::Mat4::IDENTITY, 0);
        self.renderer.borrow_mut().render();
        log::info!("Director loop running. Delta: {}", self.delta_time);
    }

    /// Pauses the game
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Resumes the game
    pub fn resume(&mut self) {
        self.is_paused = false;
        self.last_update_time = std::time::Instant::now();
    }

    /// Stops the game
    pub fn stop(&mut self) {
        self.is_cleanup = true;
    }

    /// Gets the frame size
    pub fn get_win_size(&self) -> Size {
        Size::new(960.0, 640.0)
    }

    /// Gets the visible size
    pub fn get_visible_size(&self) -> Size {
        self.get_win_size()
    }

    /// Gets the visible origin
    pub fn get_visible_origin(&self) -> crate::math::Vec2 {
        crate::math::Vec2::ZERO
    }
}

/// Scene is a node that contains all the game elements
#[derive(Debug)]
pub struct Scene {
    base: Ref,
    children: Vec<RefPtr<Node>>,
}

impl Scene {
    /// Creates a new scene
    pub fn new() -> Scene {
        Scene {
            base: Ref::new(),
            children: Vec::new(),
        }
    }

    /// Gets the children of the scene
    pub fn get_children(&self) -> &Vec<RefPtr<Node>> {
        &self.children
    }

    /// Adds a child to the scene
    pub fn add_child(&mut self, child: RefPtr<Node>) {
        self.children.push(child);
    }

    /// Removes a child from the scene
    pub fn remove_child(&mut self, child: &RefPtr<Node>) {
        self.children.retain(|c| !c.borrow().get_id() == child.borrow().get_id());
    }

    /// Updates the scene
    pub fn update(&mut self, delta_time: f32) {
        for child in &mut self.children {
            child.borrow_mut().update(delta_time);
        }
    }

    pub fn visit(&self, renderer: &mut Renderer, parent_transform: &crate::math::Mat4, parent_flags: u32) {
        for child in &self.children {
            child.borrow_mut().visit(renderer, parent_transform, parent_flags);
        }
    }
}

/// Base node type for all scene elements
// #[derive(Debug)] // Manual implementation below
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
    /// Creates a new node
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

    pub fn visit(&mut self, renderer: &mut Renderer, parent_transform: &crate::math::Mat4, _parent_flags: u32) {
        if !self.visible {
            return;
        }

        // Update local transform if needed (simplified)
        // self.update_local_transform(); // Already done in setters usually

        // Update global transform
        // global_transform = parent_transform * local_transform
        self.global_transform = *parent_transform * self.local_transform;

        // Visit children (simplified: just iterate, no z-order sorting yet)
        for child in &self.children {
            child.borrow_mut().visit(renderer, &self.global_transform, _parent_flags);
        }

        // Draw self
        if let Some(callback) = &self.on_draw {
            callback(renderer, &self.global_transform);
        }
    }

    /// Gets the parent node
    pub fn get_parent(&self) -> Option<&RefPtr<Node>> {
        self.parent.as_ref()
    }

    /// Sets the parent node
    pub fn set_parent(&mut self, parent: RefPtr<Node>) {
        self.parent = Some(parent);
    }

    /// Gets the children
    pub fn get_children(&self) -> &Vec<RefPtr<Node>> {
        &self.children
    }

    /// Adds a child node
    pub fn add_child(&mut self, child: RefPtr<Node>) {
        // child.borrow_mut().set_parent(self.base.clone());
        self.children.push(child);
    }

    /// Removes a child node
    pub fn remove_child(&mut self, child: &RefPtr<Node>) {
        self.children.retain(|c| c.borrow().get_id() != child.borrow().get_id());
    }

    /// Removes all children
    pub fn remove_all_children(&mut self) {
        self.children.clear();
    }

    /// Gets the position
    pub fn get_position(&self) -> &crate::math::Vec2 {
        &self.position
    }

    /// Sets the position
    pub fn set_position(&mut self, position: crate::math::Vec2) {
        self.position = position;
        self.update_local_transform();
    }

    /// Gets the rotation
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    /// Sets the rotation
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.update_local_transform();
    }

    /// Gets the scale X
    pub fn get_scale_x(&self) -> f32 {
        self.scale_x
    }

    /// Gets the scale Y
    pub fn get_scale_y(&self) -> f32 {
        self.scale_y
    }

    /// Sets the scale
    pub fn set_scale(&mut self, scale: f32) {
        self.scale_x = scale;
        self.scale_y = scale;
        self.update_local_transform();
    }

    /// Sets the scale with separate X and Y
    pub fn set_scale_xy(&mut self, scale_x: f32, scale_y: f32) {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self.update_local_transform();
    }

    /// Gets the tag
    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    /// Sets the tag
    pub fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    /// Gets the name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Sets the name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Gets the local transform matrix
    pub fn get_local_transform(&self) -> &crate::math::Mat4 {
        &self.local_transform
    }

    /// Gets the global transform matrix
    pub fn get_global_transform(&self) -> &crate::math::Mat4 {
        &self.global_transform
    }

    /// Checks if the node is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Sets the visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Gets the content size
    pub fn get_content_size(&self) -> crate::math::Vec2 {
        self.content_size
    }

    /// Sets the content size
    pub fn set_content_size(&mut self, size: crate::math::Vec2) {
        self.content_size = size;
    }

    /// Updates the local transform matrix
    fn update_local_transform(&mut self) {
        self.local_transform = crate::math::Mat4::create_translation(&crate::math::Vec3::new(self.position.x, self.position.y, 0.0));
    }

    /// Updates the node
    pub fn update(&mut self, delta_time: f32) {
    }

    /// Gets a unique ID for the node
    pub fn get_id(&self) -> usize {
        let ptr = &self.base as *const Ref as *const u8 as usize;
        ptr
    }

    /// Gets the base reference
    pub fn get_base(&self) -> &Ref {
        &self.base
    }

    /// Gets mutable base reference
    pub fn get_base_mut(&mut self) -> &mut Ref {
        &mut self.base
    }
}
