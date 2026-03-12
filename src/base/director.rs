#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::base::event::EventDispatcher;
use crate::base::scheduler::Scheduler;
use crate::base::{Ref, RefPtr, Size};
use std::cell::RefCell;

use crate::renderer::Renderer;
use glow::Context;
use std::rc::Rc;

/// 投影类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Projection {
    /// 2D 正交投影
    Projection2D,
    /// 3D 透视投影
    Projection3D,
    /// 自定义投影
    Custom,
}

/// 分辨率适配策略（对应 cocos2d-x ResolutionPolicy）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionPolicy {
    /// 拉伸填满，可能变形
    ExactFit,
    /// 不裁剪，可能有黑边
    NoBorder,
    /// 显示全部，保持宽高比，可能有黑边
    ShowAll,
    /// 固定高度，宽度自适应
    FixedHeight,
    /// 固定宽度，高度自适应
    FixedWidth,
    /// 不做任何缩放
    Unknown,
}

#[derive(Debug)]
pub struct Director {
    running_scene: RefPtr<Scene>,
    next_scene: Option<RefPtr<Scene>>,
    /// 场景栈（用于 push/pop 场景切换）
    scene_stack: Vec<RefPtr<Scene>>,
    scheduler: RefPtr<Scheduler>,
    event_dispatcher: RefPtr<EventDispatcher>,
    renderer: RefPtr<Renderer>,
    delta_time: f32,
    total_time: f32,
    last_update_time: std::time::Instant,
    is_paused: bool,
    is_cleanup: bool,
    // === 新增：投影和分辨率 ===
    /// 当前投影类型
    projection: Projection,
    /// 投影矩阵
    projection_matrix: crate::math::Mat4,
    /// 窗口实际大小（像素）
    win_size_in_pixels: Size,
    /// 窗口逻辑大小
    win_size: Size,
    /// 设计分辨率
    design_resolution_size: Size,
    /// 分辨率适配策略
    resolution_policy: ResolutionPolicy,
    /// 可见区域大小（考虑适配后）
    visible_size: Size,
    /// 可见区域原点
    visible_origin: crate::math::Vec2,
    /// 缩放因子
    scale_x: f32,
    scale_y: f32,
    /// 帧率
    animation_interval: f64,
    /// 当前帧数
    total_frames: u64,
    /// 内容缩放因子（Retina屏幕等）
    content_scale_factor: f32,
    /// 通知暂停时是否也暂停渲染
    send_cleanup_to_scene: bool,
}

impl Default for Director {
    fn default() -> Self {
        Self::new()
    }
}

impl Director {
    #[allow(static_mut_refs)]
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
            scene_stack: Vec::new(),
            scheduler: RefPtr::new(Scheduler::new()),
            event_dispatcher: RefPtr::new(EventDispatcher::new()),
            renderer: RefPtr::new(Renderer::new()),
            delta_time: 0.0,
            total_time: 0.0,
            last_update_time: std::time::Instant::now(),
            is_paused: false,
            is_cleanup: false,
            projection: Projection::Projection2D,
            projection_matrix: crate::math::Mat4::IDENTITY,
            win_size_in_pixels: Size::new(960.0, 640.0),
            win_size: Size::new(960.0, 640.0),
            design_resolution_size: Size::new(960.0, 640.0),
            resolution_policy: ResolutionPolicy::ShowAll,
            visible_size: Size::new(960.0, 640.0),
            visible_origin: crate::math::Vec2::ZERO,
            scale_x: 1.0,
            scale_y: 1.0,
            animation_interval: 1.0 / 60.0,
            total_frames: 0,
            content_scale_factor: 1.0,
            send_cleanup_to_scene: false,
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

    /// 压入场景（保留当前场景在栈中）
    pub fn push_scene(&mut self, scene: RefPtr<Scene>) {
        self.scene_stack.push(self.running_scene.clone());
        self.next_scene = Some(scene);
    }

    /// 弹出场景（恢复上一个场景）
    pub fn pop_scene(&mut self) {
        if let Some(prev_scene) = self.scene_stack.pop() {
            self.next_scene = Some(prev_scene);
        }
    }

    /// 弹出到根场景（清空栈）
    pub fn pop_to_root_scene(&mut self) {
        if let Some(root) = self.scene_stack.first().cloned() {
            self.scene_stack.clear();
            self.next_scene = Some(root);
        }
    }

    /// 获取场景栈深度
    pub fn scene_stack_depth(&self) -> usize {
        self.scene_stack.len()
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
        self.total_frames += 1;

        if !self.is_paused {
            self.scheduler.borrow_mut().update(self.delta_time);
        }

        if let Some(scene) = self.next_scene.take() {
            self.running_scene = scene;
        }

        // Render the current scene (使用投影矩阵)
        self.running_scene.borrow().visit(
            &mut self.renderer.borrow_mut(),
            &self.projection_matrix,
            0,
        );
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
        self.win_size
    }

    /// 获取窗口像素大小
    pub fn get_win_size_in_pixels(&self) -> Size {
        self.win_size_in_pixels
    }

    /// 设置窗口大小（通常由平台层调用）
    pub fn set_win_size(&mut self, width: f32, height: f32) {
        self.win_size = Size::new(width, height);
        self.win_size_in_pixels = Size::new(
            width * self.content_scale_factor,
            height * self.content_scale_factor,
        );
        self.update_design_resolution();
    }

    pub fn get_visible_size(&self) -> Size {
        self.visible_size
    }

    pub fn get_visible_origin(&self) -> crate::math::Vec2 {
        self.visible_origin
    }

    // ========== 设计分辨率 ==========

    /// 设置设计分辨率和适配策略
    pub fn set_design_resolution_size(&mut self, width: f32, height: f32, policy: ResolutionPolicy) {
        if width <= 0.0 || height <= 0.0 {
            return;
        }
        self.design_resolution_size = Size::new(width, height);
        self.resolution_policy = policy;
        self.update_design_resolution();
    }

    /// 获取设计分辨率
    pub fn get_design_resolution_size(&self) -> Size {
        self.design_resolution_size
    }

    /// 获取分辨率策略
    pub fn get_resolution_policy(&self) -> ResolutionPolicy {
        self.resolution_policy
    }

    /// 更新设计分辨率的内部计算
    fn update_design_resolution(&mut self) {
        let frame_w = self.win_size_in_pixels.width;
        let frame_h = self.win_size_in_pixels.height;
        let design_w = self.design_resolution_size.width;
        let design_h = self.design_resolution_size.height;

        if design_w <= 0.0 || design_h <= 0.0 || frame_w <= 0.0 || frame_h <= 0.0 {
            return;
        }

        let (sx, sy) = match self.resolution_policy {
            ResolutionPolicy::ExactFit => {
                (frame_w / design_w, frame_h / design_h)
            }
            ResolutionPolicy::NoBorder => {
                let s = (frame_w / design_w).max(frame_h / design_h);
                (s, s)
            }
            ResolutionPolicy::ShowAll => {
                let s = (frame_w / design_w).min(frame_h / design_h);
                (s, s)
            }
            ResolutionPolicy::FixedHeight => {
                let s = frame_h / design_h;
                (s, s)
            }
            ResolutionPolicy::FixedWidth => {
                let s = frame_w / design_w;
                (s, s)
            }
            ResolutionPolicy::Unknown => {
                (1.0, 1.0)
            }
        };

        self.scale_x = sx;
        self.scale_y = sy;

        // 计算可见区域
        self.visible_size = Size::new(frame_w / sx, frame_h / sy);
        self.visible_origin = crate::math::Vec2::new(
            (design_w - self.visible_size.width) / 2.0,
            (design_h - self.visible_size.height) / 2.0,
        );

        self.update_projection();
    }

    // ========== 投影矩阵 ==========

    /// 设置投影类型
    pub fn set_projection(&mut self, projection: Projection) {
        self.projection = projection;
        self.update_projection();
    }

    /// 获取投影类型
    pub fn get_projection(&self) -> Projection {
        self.projection
    }

    /// 获取投影矩阵
    pub fn get_projection_matrix(&self) -> &crate::math::Mat4 {
        &self.projection_matrix
    }

    /// 更新投影矩阵
    fn update_projection(&mut self) {
        let size = self.visible_size;
        match self.projection {
            Projection::Projection2D => {
                // 正交投影（off-center 形式，原点在左下角）
                self.projection_matrix = crate::math::Mat4::create_orthographic_off_center(
                    0.0,
                    size.width,
                    0.0,
                    size.height,
                    -1024.0,
                    1024.0,
                );
            }
            Projection::Projection3D => {
                // 透视投影（field_of_view 传度数，方法内部会转弧度）
                let aspect = size.width / size.height.max(1.0);
                let fov_deg = 60.0f32; // 度数
                let near = 0.1;
                let far = (size.height / 1.1566) * 2.0; // 类似 cocos2d-x 默认 eye 距离
                self.projection_matrix = crate::math::Mat4::create_perspective(
                    fov_deg, aspect, near, far,
                );
            }
            Projection::Custom => {
                // 自定义投影，保持当前矩阵不变
            }
        }
    }

    /// 设置自定义投影矩阵
    pub fn set_projection_matrix(&mut self, matrix: crate::math::Mat4) {
        self.projection = Projection::Custom;
        self.projection_matrix = matrix;
    }

    // ========== 内容缩放 ==========

    /// 设置内容缩放因子（Retina 屏幕为 2.0）
    pub fn set_content_scale_factor(&mut self, factor: f32) {
        self.content_scale_factor = factor.max(0.1);
        self.win_size_in_pixels = Size::new(
            self.win_size.width * self.content_scale_factor,
            self.win_size.height * self.content_scale_factor,
        );
        self.update_design_resolution();
    }

    /// 获取内容缩放因子
    pub fn get_content_scale_factor(&self) -> f32 {
        self.content_scale_factor
    }

    /// 获取缩放因子 X/Y
    pub fn get_scale_x(&self) -> f32 {
        self.scale_x
    }

    pub fn get_scale_y(&self) -> f32 {
        self.scale_y
    }

    // ========== 帧率 ==========

    /// 设置动画间隔（秒）
    pub fn set_animation_interval(&mut self, interval: f64) {
        self.animation_interval = interval.max(0.001);
    }

    /// 获取动画间隔
    pub fn get_animation_interval(&self) -> f64 {
        self.animation_interval
    }

    /// 获取总帧数
    pub fn get_total_frames(&self) -> u64 {
        self.total_frames
    }

    /// 将 GL 坐标（像素）转换为设计分辨率坐标
    pub fn convert_to_gl(&self, ui_point: &crate::math::Vec2) -> crate::math::Vec2 {
        crate::math::Vec2::new(
            ui_point.x / self.scale_x + self.visible_origin.x,
            ui_point.y / self.scale_y + self.visible_origin.y,
        )
    }

    /// 将设计分辨率坐标转换为 GL 坐标（像素）
    pub fn convert_to_ui(&self, gl_point: &crate::math::Vec2) -> crate::math::Vec2 {
        crate::math::Vec2::new(
            (gl_point.x - self.visible_origin.x) * self.scale_x,
            (gl_point.y - self.visible_origin.y) * self.scale_y,
        )
    }
}

#[derive(Debug)]
pub struct Scene {
    base: Ref,
    children: Vec<RefPtr<Node>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
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
    color: crate::base::Color3B,
    opacity: u8,
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
            color: self.color,
            opacity: self.opacity,
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

impl Default for Node {
    fn default() -> Self {
        Self::new()
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
            color: crate::base::Color3B::WHITE,
            opacity: 255,
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
    
    pub fn get_anchor_point(&self) -> crate::math::Vec2 {
        crate::math::Vec2::ANCHOR_MIDDLE
    }
    
    pub fn set_anchor_point(&mut self, _anchor: crate::math::Vec2) {
        // TODO: implement anchor point logic
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

    /// Updates the node
    pub fn update(&mut self, delta_time: f32) {}

    pub fn get_id(&self) -> usize {
        
        &self.base as *const Ref as *const u8 as usize
    }

    pub fn get_base(&self) -> &Ref {
        &self.base
    }

    pub fn get_base_mut(&mut self) -> &mut Ref {
        &mut self.base
    }
    
    pub fn color(&self) -> crate::base::Color3B {
        self.color
    }
    
    pub fn set_color(&mut self, color: crate::base::Color3B) {
        self.color = color;
    }
    
    pub fn opacity(&self) -> u8 {
        self.opacity
    }
    
    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }
    
    pub fn on_enter(&mut self) {
        // Hook for when node enters the scene
    }
    
    pub fn on_exit(&mut self) {
        // Hook for when node exits the scene
    }

    // ===== 兼容 scene::Node 的别名方法 =====

    /// 获取位置（scene::Node 风格别名）
    pub fn position(&self) -> crate::math::Vec2 {
        self.position
    }

    /// 设置位置（接受 x, y 两个参数）
    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.position = crate::math::Vec2::new(x, y);
        self.update_local_transform();
    }

    /// 获取缩放
    pub fn scale(&self) -> f32 {
        (self.scale_x + self.scale_y) / 2.0
    }

    /// 获取旋转
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// tag() 别名
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// name() 别名
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取内容大小（兼容 Size 类型）
    pub fn content_size(&self) -> crate::math::geometry::Size {
        crate::math::geometry::Size::new(self.content_size.x, self.content_size.y)
    }

    /// set_content_size（兼容 Size 类型）
    pub fn set_content_size_from_size(&mut self, size: crate::math::geometry::Size) {
        self.content_size = crate::math::Vec2::new(size.width, size.height);
    }

    /// 获取锚点
    pub fn anchor_point(&self) -> crate::math::Vec2 {
        crate::math::Vec2::new(0.5, 0.5)
    }

    /// 设置本地Z轴排序
    pub fn set_local_z_order(&mut self, _z: i32) {
        // director::Node 暂不支持Z排序，保留接口兼容性
    }

    /// 获取本地Z轴排序
    pub fn local_z_order(&self) -> i32 {
        0
    }

    /// 添加子节点（简化版，不需要z轴参数）
    pub fn add_child_simple(&mut self, child: RefPtr<Node>) {
        self.children.push(child);
    }

    /// 获取子节点数量
    pub fn get_children_count(&self) -> usize {
        self.children.len()
    }

    /// 通过标签获取子节点
    pub fn get_child_by_tag(&self, tag: i32) -> Option<RefPtr<Node>> {
        for child in &self.children {
            if child.borrow().tag() == tag {
                return Some(child.clone());
            }
        }
        None
    }

    /// 通过名称获取子节点
    pub fn get_child_by_name(&self, name: &str) -> Option<RefPtr<Node>> {
        for child in &self.children {
            if child.borrow().name() == name {
                return Some(child.clone());
            }
        }
        None
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

    // ========== 新增：场景栈 / 投影 / 分辨率 测试 ==========

    #[test]
    fn test_director_push_pop_scene() {
        let mut director = Director::new();
        assert_eq!(director.scene_stack_depth(), 0);

        let scene1 = RefPtr::new(Scene::new());
        let scene2 = RefPtr::new(Scene::new());
        director.run_scene(scene1);
        director.push_scene(scene2);
        assert_eq!(director.scene_stack_depth(), 1);

        director.pop_scene();
        // 场景栈减少
        assert_eq!(director.scene_stack_depth(), 0);
    }

    #[test]
    fn test_director_pop_to_root_scene() {
        let mut director = Director::new();
        let root = RefPtr::new(Scene::new());
        director.run_scene(root.clone());

        let s1 = RefPtr::new(Scene::new());
        let s2 = RefPtr::new(Scene::new());
        director.push_scene(s1);
        director.push_scene(s2);
        assert_eq!(director.scene_stack_depth(), 2);

        director.pop_to_root_scene();
        // 清空后栈深为 0
        assert_eq!(director.scene_stack_depth(), 0);
    }

    #[test]
    fn test_director_design_resolution_show_all() {
        let mut director = Director::new();
        director.set_win_size(1280.0, 720.0);
        director.set_design_resolution_size(960.0, 640.0, ResolutionPolicy::ShowAll);
        let vis = director.get_visible_size();
        // ShowAll 情况下，设计分辨率等比缩放，可见区域不超过窗口
        assert!(vis.width <= 1280.0 / director.get_scale_x() + 1.0);
        assert!(vis.height <= 720.0 / director.get_scale_y() + 1.0);
    }

    #[test]
    fn test_director_design_resolution_exact_fit() {
        let mut director = Director::new();
        director.set_win_size(1280.0, 720.0);
        director.set_design_resolution_size(960.0, 640.0, ResolutionPolicy::ExactFit);
        // ExactFit：sx = 1280/960, sy = 720/640
        let sx = director.get_scale_x();
        let sy = director.get_scale_y();
        assert!((sx - 1280.0 / 960.0).abs() < 0.01);
        assert!((sy - 720.0 / 640.0).abs() < 0.01);
    }

    #[test]
    fn test_director_design_resolution_no_border() {
        let mut director = Director::new();
        director.set_win_size(1280.0, 720.0);
        director.set_design_resolution_size(960.0, 640.0, ResolutionPolicy::NoBorder);
        // NoBorder：取较大缩放
        let sx = director.get_scale_x();
        let sy = director.get_scale_y();
        assert!((sx - sy).abs() < 0.01); // 等比
        let expected = (1280.0f32 / 960.0).max(720.0 / 640.0);
        assert!((sx - expected).abs() < 0.01);
    }

    #[test]
    fn test_director_projection_2d() {
        let mut director = Director::new();
        director.set_win_size(960.0, 640.0);
        director.set_design_resolution_size(960.0, 640.0, ResolutionPolicy::ShowAll);
        director.set_projection(Projection::Projection2D);
        assert_eq!(director.get_projection(), Projection::Projection2D);
        // 投影矩阵不应该是全零
        let m = director.get_projection_matrix();
        let is_nonzero = m.m.iter().any(|&v| v.abs() > 1e-6);
        assert!(is_nonzero, "2D projection matrix should be non-zero");
    }

    #[test]
    fn test_director_projection_3d() {
        let mut director = Director::new();
        director.set_win_size(960.0, 640.0);
        director.set_design_resolution_size(960.0, 640.0, ResolutionPolicy::ShowAll);
        director.set_projection(Projection::Projection3D);
        assert_eq!(director.get_projection(), Projection::Projection3D);
        let m = director.get_projection_matrix();
        let is_nonzero = m.m.iter().any(|&v| v.abs() > 1e-6);
        assert!(is_nonzero, "3D projection matrix should be non-zero");
    }

    #[test]
    fn test_director_custom_projection() {
        let mut director = Director::new();
        let custom = crate::math::Mat4::IDENTITY;
        director.set_projection_matrix(custom);
        assert_eq!(director.get_projection(), Projection::Custom);
    }

    #[test]
    fn test_director_content_scale_factor() {
        let mut director = Director::new();
        director.set_win_size(960.0, 640.0);
        director.set_content_scale_factor(2.0);
        assert!((director.get_content_scale_factor() - 2.0).abs() < 1e-5);
        let pixels = director.get_win_size_in_pixels();
        assert!((pixels.width - 1920.0).abs() < 1.0);
        assert!((pixels.height - 1280.0).abs() < 1.0);
    }

    #[test]
    fn test_director_animation_interval() {
        let mut director = Director::new();
        director.set_animation_interval(1.0 / 30.0);
        assert!((director.get_animation_interval() - 1.0 / 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_director_convert_to_gl_ui() {
        let mut director = Director::new();
        director.set_win_size(960.0, 640.0);
        director.set_design_resolution_size(960.0, 640.0, ResolutionPolicy::ExactFit);
        let pt = crate::math::Vec2::new(100.0, 200.0);
        let gl = director.convert_to_gl(&pt);
        let back = director.convert_to_ui(&gl);
        assert!((back.x - pt.x).abs() < 0.01);
        assert!((back.y - pt.y).abs() < 0.01);
    }

    #[test]
    fn test_director_total_frames() {
        let director = Director::new();
        assert_eq!(director.get_total_frames(), 0);
    }
}
