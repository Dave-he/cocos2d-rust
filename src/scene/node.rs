/// Node - 所有节点基类
///
/// Node 是 Cocos2d-Rust 中所有可视元素的基础类。
/// 它提供位置、缩放、旋转、锚点等变换属性，以及父子关系管理。

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::math::Vec2;
use crate::math::Vec3;
use crate::math::Mat4;
use crate::math::geometry::{Rect, Size};
use crate::base::types::Color3B;

/// Node 标签常量
pub const TAG_INVALID: i32 = -1;

/// 节点变换标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TransformFlags {
    pub position: bool,
    pub rotation: bool,
    pub scale: bool,
    pub skew: bool,
    pub anchor: bool,
    pub content_size: bool,
}

impl TransformFlags {
    pub fn new() -> Self {
        Self {
            position: true,
            rotation: true,
            scale: true,
            skew: true,
            anchor: true,
            content_size: true,
        }
    }

    pub fn dirty(&self) -> bool {
        self.position || self.rotation || self.scale || self.skew || self.anchor
    }

    pub fn clean(&mut self) {
        self.position = false;
        self.rotation = false;
        self.scale = false;
        self.skew = false;
        self.anchor = false;
    }
}

/// 节点类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum NodeType {
    Scene,
    Layer,
    Sprite,
    Label,
    Menu,
    MenuItem,
    LayerColor,
    LayerGradient,
    #[default]
    Node,
}


/// Node - 节点基类
pub struct Node {
    // 变换属性
    position: Vec2,
    position_z: f32,
    scale_x: f32,
    scale_y: f32,
    rotation_x: f32,
    rotation_y: f32,
    skew_x: f32,
    skew_y: f32,
    anchor_point: Vec2,
    anchor_point_in_points: Vec2,
    content_size: Size,
    
    // 显示属性
    visible: bool,
    opacity: u8,
    displayed_opacity: u8,
    cascade_opacity: bool,
    color: Color3B,
    displayed_color: Color3B,
    cascade_color: bool,
    
    // 层级关系
    local_z_order: i32,
    global_z_order: i32,
    tag: i32,
    name: String,
    
    // 父子关系
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    children_by_name: HashMap<String, Rc<RefCell<Node>>>,
    
    // 变换矩阵
    model_view_transform: Mat4,
    transform: Mat4,
    inverse: Mat4,
    additional_transform: Option<Mat4>,
    transform_dirty: bool,
    inverse_dirty: bool,
    additional_transform_dirty: bool,
    transform_flags: TransformFlags,
    
    // 状态
    running: bool,
    paused: bool,
    user_data: i64,
    ignore_anchor_point_for_position: bool,
    ignore_anchor_point_for_rotation: bool,
    ignore_anchor_point_for_scale: bool,
    reorder_child_dirty: bool,
    
    // 组件
    node_type: NodeType,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("position", &self.position)
            .field("scale", &(self.scale_x, self.scale_y))
            .field("rotation", &(self.rotation_x, self.rotation_y))
            .field("anchor_point", &self.anchor_point)
            .field("visible", &self.visible)
            .field("opacity", &self.opacity)
            .field("local_z_order", &self.local_z_order)
            .field("global_z_order", &self.global_z_order)
            .field("tag", &self.tag)
            .field("node_type", &self.node_type)
            .field("children_count", &self.children.len())
            .finish()
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            position_z: self.position_z,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            rotation_x: self.rotation_x,
            rotation_y: self.rotation_y,
            skew_x: self.skew_x,
            skew_y: self.skew_y,
            anchor_point: self.anchor_point,
            anchor_point_in_points: self.anchor_point_in_points,
            content_size: self.content_size,
            visible: self.visible,
            opacity: self.opacity,
            displayed_opacity: self.displayed_opacity,
            cascade_opacity: self.cascade_opacity,
            color: self.color,
            displayed_color: self.displayed_color,
            cascade_color: self.cascade_color,
            local_z_order: self.local_z_order,
            global_z_order: self.global_z_order,
            tag: self.tag,
            name: self.name.clone(),
            parent: self.parent.clone(),
            children: self.children.clone(),
            children_by_name: self.children_by_name.clone(),
            model_view_transform: self.model_view_transform,
            transform: self.transform,
            inverse: self.inverse,
            additional_transform: self.additional_transform,
            transform_dirty: self.transform_dirty,
            inverse_dirty: self.inverse_dirty,
            additional_transform_dirty: self.additional_transform_dirty,
            transform_flags: self.transform_flags,
            running: self.running,
            paused: self.paused,
            user_data: self.user_data,
            ignore_anchor_point_for_position: self.ignore_anchor_point_for_position,
            ignore_anchor_point_for_rotation: self.ignore_anchor_point_for_rotation,
            ignore_anchor_point_for_scale: self.ignore_anchor_point_for_scale,
            reorder_child_dirty: self.reorder_child_dirty,
            node_type: self.node_type,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            position: Vec2::zero(),
            position_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            anchor_point: Vec2::new(0.5, 0.5),
            anchor_point_in_points: Vec2::zero(),
            content_size: Size::zero(),
            visible: true,
            opacity: 255,
            displayed_opacity: 255,
            cascade_opacity: true,
            color: Color3B::white(),
            displayed_color: Color3B::white(),
            cascade_color: true,
            local_z_order: 0,
            global_z_order: 0,
            tag: TAG_INVALID,
            name: String::new(),
            parent: None,
            children: Vec::new(),
            children_by_name: HashMap::new(),
            model_view_transform: Mat4::IDENTITY,
            transform: Mat4::IDENTITY,
            inverse: Mat4::IDENTITY,
            additional_transform: None,
            transform_dirty: true,
            inverse_dirty: true,
            additional_transform_dirty: false,
            transform_flags: TransformFlags::new(),
            running: false,
            paused: false,
            user_data: 0,
            ignore_anchor_point_for_position: false,
            ignore_anchor_point_for_rotation: false,
            ignore_anchor_point_for_scale: false,
            reorder_child_dirty: false,
            node_type: NodeType::Node,
        }
    }

    pub fn with_type(node_type: NodeType) -> Self {
        let mut node = Self::new();
        node.node_type = node_type;
        node
    }

    // ===== 位置 =====
    
    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
        self.transform_flags.position = true;
        self.set_transform_dirty();
    }

    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.set_position(Vec2::new(x, y));
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position_z(&mut self, z: f32) {
        self.position_z = z;
    }

    pub fn position_z(&self) -> f32 {
        self.position_z
    }

    pub fn set_position3d(&mut self, pos: Vec3) {
        self.position = Vec2::new(pos.x, pos.y);
        self.position_z = pos.z;
        self.set_transform_dirty();
    }

    pub fn get_position3d(&self) -> Vec3 {
        Vec3::new(self.position.x, self.position.y, self.position_z)
    }

    pub fn get_position_x(&self) -> f32 {
        self.position.x
    }

    pub fn get_position_y(&self) -> f32 {
        self.position.y
    }

    // ===== 缩放 =====
    
    pub fn set_scale(&mut self, scale: f32) {
        self.scale_x = scale;
        self.scale_y = scale;
        self.transform_flags.scale = true;
        self.set_transform_dirty();
    }

    pub fn set_scale_xy(&mut self, scale_x: f32, scale_y: f32) {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self.transform_flags.scale = true;
        self.set_transform_dirty();
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        self.scale_x = scale_x;
        self.transform_flags.scale = true;
        self.set_transform_dirty();
    }

    pub fn set_scale_y(&mut self, scale_y: f32) {
        self.scale_y = scale_y;
        self.transform_flags.scale = true;
        self.set_transform_dirty();
    }

    pub fn scale(&self) -> f32 {
        if self.scale_x == self.scale_y {
            self.scale_x
        } else {
            (self.scale_x, self.scale_y).0
        }
    }

    pub fn scale_x(&self) -> f32 {
        self.scale_x
    }

    pub fn scale_y(&self) -> f32 {
        self.scale_y
    }

    // ===== 旋转 =====
    
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation_x = rotation;
        self.rotation_y = rotation;
        self.transform_flags.rotation = true;
        self.set_transform_dirty();
    }

    pub fn set_rotation_x(&mut self, rotation: f32) {
        self.rotation_x = rotation;
        self.transform_flags.rotation = true;
        self.set_transform_dirty();
    }

    pub fn set_rotation_y(&mut self, rotation: f32) {
        self.rotation_y = rotation;
        self.transform_flags.rotation = true;
        self.set_transform_dirty();
    }

    pub fn rotation(&self) -> f32 {
        self.rotation_x
    }

    pub fn rotation_x(&self) -> f32 {
        self.rotation_x
    }

    pub fn rotation_y(&self) -> f32 {
        self.rotation_y
    }

    // ===== 倾斜 =====
    
    pub fn set_skew(&mut self, skew: f32) {
        self.skew_x = skew;
        self.skew_y = skew;
        self.transform_flags.skew = true;
        self.set_transform_dirty();
    }

    pub fn set_skew_xy(&mut self, skew_x: f32, skew_y: f32) {
        self.skew_x = skew_x;
        self.skew_y = skew_y;
        self.transform_flags.skew = true;
        self.set_transform_dirty();
    }

    pub fn skew_x(&self) -> f32 {
        self.skew_x
    }

    pub fn skew_y(&self) -> f32 {
        self.skew_y
    }

    // ===== 锚点 =====
    
    pub fn set_anchor_point(&mut self, point: Vec2) {
        self.anchor_point = point;
        self.anchor_point_in_points = Vec2::new(
            self.content_size.width * point.x,
            self.content_size.height * point.y,
        );
        self.transform_flags.anchor = true;
        self.set_transform_dirty();
    }

    pub fn anchor_point(&self) -> Vec2 {
        self.anchor_point
    }

    pub fn anchor_point_in_points(&self) -> Vec2 {
        self.anchor_point_in_points
    }

    pub fn set_ignore_anchor_point_for_position(&mut self, ignore: bool) {
        self.ignore_anchor_point_for_position = ignore;
        self.set_transform_dirty();
    }

    pub fn is_ignore_anchor_point_for_position(&self) -> bool {
        self.ignore_anchor_point_for_position
    }

    // ===== 内容尺寸 =====
    
    pub fn set_content_size(&mut self, size: Size) {
        self.content_size = size;
        self.anchor_point_in_points = Vec2::new(
            size.width * self.anchor_point.x,
            size.height * self.anchor_point.y,
        );
        self.transform_flags.content_size = true;
        self.set_transform_dirty();
    }

    pub fn set_content_size_wh(&mut self, width: f32, height: f32) {
        self.set_content_size(Size::new(width, height));
    }

    pub fn content_size(&self) -> Size {
        self.content_size
    }

    pub fn get_content_size_width(&self) -> f32 {
        self.content_size.width
    }

    pub fn get_content_size_height(&self) -> f32 {
        self.content_size.height
    }

    // ===== 可见性 =====
    
    pub fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    // ===== 透明度 =====
    
    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
        self.set_displayed_opacity(opacity);
        self.update_cascade_opacity();
    }

    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn set_displayed_opacity(&mut self, opacity: u8) {
        self.displayed_opacity = opacity;
    }

    pub fn displayed_opacity(&self) -> u8 {
        self.displayed_opacity
    }

    pub fn set_cascade_opacity(&mut self, cascade: bool) {
        self.cascade_opacity = cascade;
    }

    pub fn is_cascade_opacity_enabled(&self) -> bool {
        self.cascade_opacity
    }

    pub fn update_cascade_opacity(&mut self) {
        if self.cascade_opacity {
            let parent_opacity = if let Some(ref parent) = self.parent {
                parent.borrow().displayed_opacity()
            } else {
                255
            };
            self.set_displayed_opacity((self.opacity as f32 * parent_opacity as f32 / 255.0) as u8);
        }
    }

    pub fn update_cascade_opacity_children(&mut self) {
        for child in &self.children {
            child.borrow_mut().update_cascade_opacity();
        }
    }

    pub fn update_cascade_opacity_with_parent(&mut self, parent_opacity: u8) {
        if self.cascade_opacity {
            self.displayed_opacity = (self.opacity as f32 * parent_opacity as f32 / 255.0) as u8;
        }
    }

    // ===== 颜色 =====
    
    pub fn set_color(&mut self, color: Color3B) {
        self.color = color;
        self.displayed_color = color;
        self.update_cascade_color();
    }

    pub fn color(&self) -> Color3B {
        self.color
    }

    pub fn displayed_color(&self) -> Color3B {
        self.displayed_color
    }

    pub fn set_cascade_color(&mut self, cascade: bool) {
        self.cascade_color = cascade;
    }

    pub fn is_cascade_color_enabled(&self) -> bool {
        self.cascade_color
    }

    pub fn update_cascade_color(&mut self) {
        if self.cascade_color {
            if let Some(ref parent) = self.parent {
                let parent_color = parent.borrow().displayed_color();
                self.displayed_color = Color3B::new(
                    (self.color.r as f32 * parent_color.r as f32 / 255.0) as u8,
                    (self.color.g as f32 * parent_color.g as f32 / 255.0) as u8,
                    (self.color.b as f32 * parent_color.b as f32 / 255.0) as u8,
                );
            }
        }
    }

    pub fn update_cascade_color_children(&mut self) {
        for child in &self.children {
            child.borrow_mut().update_cascade_color();
        }
    }

    pub fn update_cascade_color_with_parent(&mut self, parent_color: Color3B) {
        if self.cascade_color {
            self.displayed_color = Color3B::new(
                (self.color.r as f32 * parent_color.r as f32 / 255.0) as u8,
                (self.color.g as f32 * parent_color.g as f32 / 255.0) as u8,
                (self.color.b as f32 * parent_color.b as f32 / 255.0) as u8,
            );
        }
    }

    // ===== Z-order =====
    
    pub fn set_local_z_order(&mut self, z_order: i32) {
        self.local_z_order = z_order;
        self.reorder_child_dirty = true;
    }

    pub fn local_z_order(&self) -> i32 {
        self.local_z_order
    }

    pub fn set_global_z_order(&mut self, z_order: i32) {
        self.global_z_order = z_order;
    }

    pub fn global_z_order(&self) -> i32 {
        self.global_z_order
    }

    // ===== 标签和名称 =====
    
    pub fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    pub fn tag(&self) -> i32 {
        self.tag
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // ===== 父子关系 =====
    
    pub fn add_child(&mut self, child: Rc<RefCell<Node>>, z_order: i32, name: Option<&str>) {
        let parent_displayed_opacity = self.displayed_opacity;
        let parent_displayed_color = self.displayed_color;
        
        {
            let mut child_mut = child.borrow_mut();
            child_mut.set_local_z_order(z_order);
            if let Some(n) = name {
                child_mut.set_name(n);
            }
            child_mut.update_cascade_opacity_with_parent(parent_displayed_opacity);
            child_mut.update_cascade_color_with_parent(parent_displayed_color);
        }
        
        self.children.push(child.clone());
        if let Some(n) = name {
            self.children_by_name.insert(n.to_string(), child);
        }
        
        self.reorder_child_dirty = true;
    }

    pub fn add_child_with_z(&mut self, child: Rc<RefCell<Node>>, z_order: i32) {
        self.add_child(child, z_order, None);
    }

    pub fn add_child_simple(&mut self, child: Rc<RefCell<Node>>) {
        self.add_child(child, self.local_z_order, None);
    }

    pub fn add_child_to_parent(
        parent: &Rc<RefCell<Node>>,
        child: Rc<RefCell<Node>>,
        z_order: i32,
        name: Option<&str>,
    ) {
        let (parent_displayed_opacity, parent_displayed_color) = {
            let p = parent.borrow();
            (p.displayed_opacity, p.displayed_color)
        };
        
        {
            let mut child_mut = child.borrow_mut();
            child_mut.parent = Some(Rc::clone(parent));
            child_mut.set_local_z_order(z_order);
            if let Some(n) = name {
                child_mut.set_name(n);
            }
            child_mut.update_cascade_opacity_with_parent(parent_displayed_opacity);
            child_mut.update_cascade_color_with_parent(parent_displayed_color);
        }
        
        let mut parent_mut = parent.borrow_mut();
        parent_mut.children.push(child.clone());
        if let Some(n) = name {
            parent_mut.children_by_name.insert(n.to_string(), child);
        }
        parent_mut.reorder_child_dirty = true;
    }

    pub fn get_child_by_tag(&self, tag: i32) -> Option<Rc<RefCell<Node>>> {
        for child in &self.children {
            if child.borrow().tag == tag {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    pub fn get_child_by_name(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        self.children_by_name.get(name).cloned()
    }

    pub fn get_children(&self) -> &[Rc<RefCell<Node>>] {
        &self.children
    }

    pub fn get_children_count(&self) -> usize {
        self.children.len()
    }

    pub fn remove_child(&mut self, child: &Rc<RefCell<Node>>, cleanup: bool) {
        let tag = child.borrow().tag();
        let name = child.borrow().name().to_string();
        
        self.children.retain(|c| !Rc::ptr_eq(c, child));
        self.children_by_name.remove(&name);
        
        if cleanup {
            let mut child_mut = child.borrow_mut();
            child_mut.parent = None;
            child_mut.cleanup();
        }
    }

    pub fn remove_child_by_tag(&mut self, tag: i32, cleanup: bool) {
        if let Some(child) = self.get_child_by_tag(tag) {
            self.remove_child(&child, cleanup);
        }
    }

    pub fn remove_child_by_name(&mut self, name: &str, cleanup: bool) {
        if let Some(child) = self.get_child_by_name(name) {
            self.remove_child(&child, cleanup);
        }
    }

    pub fn remove_all_children(&mut self, cleanup: bool) {
        if cleanup {
            for child in &self.children {
                let mut child_mut = child.borrow_mut();
                child_mut.parent = None;
                child_mut.cleanup();
            }
        }
        self.children.clear();
        self.children_by_name.clear();
    }

    pub fn remove_from_parent(&mut self, cleanup: bool) {
        // 注意：此方法无法完全实现，因为我们无法从&mut self获取Rc来与children比较
        // 如果需要从父节点移除，请使用parent.borrow_mut().remove_child()
        if cleanup {
            self.cleanup();
        }
        self.parent = None;
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.as_ref().map(Rc::clone)
    }

    pub fn set_parent(&mut self, parent: Option<Rc<RefCell<Node>>>) {
        self.parent = parent;
    }

    pub fn cleanup(&mut self) {
        self.remove_all_children(true);
    }

    // ===== 变换矩阵 =====
    
    pub fn set_transform_dirty(&mut self) {
        self.transform_dirty = true;
        self.inverse_dirty = true;
    }

    pub fn is_transform_dirty(&self) -> bool {
        self.transform_dirty
    }

    pub fn update_transform(&mut self) {
        self.calculate_transform();
        self.transform_dirty = false;
    }

    pub fn calculate_transform(&mut self) {
        self.transform = Mat4::IDENTITY;
        
        // 平移
        let mut pos = self.position;
        if !self.ignore_anchor_point_for_position {
            pos.x -= self.anchor_point_in_points.x;
            pos.y -= self.anchor_point_in_points.y;
        }
        let mut transform = self.transform;
        transform.translate(pos.x, pos.y, self.position_z);
        
        // 旋转
        if self.rotation_x != 0.0 || self.rotation_y != 0.0 {
            // X 轴旋转
            if self.rotation_x != 0.0 {
                let angle_x = self.rotation_x.to_radians();
                transform.rotate_x(-angle_x);
            }
            // Y 轴旋转
            if self.rotation_y != 0.0 {
                let angle_y = self.rotation_y.to_radians();
                transform.rotate_y(-angle_y);
            }
        }
        
        // 缩放
        transform.scale(self.scale_x, self.scale_y, 1.0);
        
        self.transform = transform;
        
        // 倾斜 (简化处理)
        if self.skew_x != 0.0 || self.skew_y != 0.0 {
            // 倾斜矩阵 (简化)
        }
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
    }

    pub fn get_node_to_parent_transform(&self) -> Mat4 {
        if self.transform_dirty {
            // 需要重新计算，但这是一个不可变方法
            // 返回基于当前属性计算的变换矩阵
            return self.calculate_transform_immutable();
        }
        self.transform
    }
    
    fn calculate_transform_immutable(&self) -> Mat4 {
        let mut transform = Mat4::IDENTITY;
        
        // 平移
        let mut pos = self.position;
        if !self.ignore_anchor_point_for_position {
            pos.x -= self.anchor_point_in_points.x;
            pos.y -= self.anchor_point_in_points.y;
        }
        
        // 创建平移矩阵
        transform = Mat4::create_translation(&Vec3::new(pos.x, pos.y, self.position_z));
        
        // 旋转 (简化：只处理 Z 轴，在 cocos2d-x 中 rotation 就是 Z 轴旋转)
        if self.rotation_x != 0.0 {
            let angle_rad = self.rotation_x.to_radians();
            let mut rotation = Mat4::IDENTITY;
            rotation.rotate_z(angle_rad);
            transform = transform * rotation;
        }
        
        // 缩放
        let scale_mat = Mat4::create_scale(&Vec3::new(self.scale_x, self.scale_y, 1.0));
        transform = transform * scale_mat;
        
        transform
    }

    pub fn get_node_to_parent_transform_mut(&mut self) -> Mat4 {
        self.update_transform();
        self.transform
    }

    pub fn get_parent_to_node_transform(&self) -> Mat4 {
        let node_to_parent = self.get_node_to_parent_transform();
        node_to_parent.inverted().unwrap_or(Mat4::IDENTITY)
    }

    pub fn get_node_to_world_transform(&self) -> Mat4 {
        let mut transform = self.get_node_to_parent_transform();
        
        if let Some(ref parent) = self.parent {
            let parent_transform = parent.borrow().get_node_to_world_transform();
            transform = parent_transform * transform;
        }
        
        transform
    }

    pub fn get_world_to_node_transform(&self) -> Mat4 {
        self.get_node_to_world_transform().inverted().unwrap_or(Mat4::IDENTITY)
    }

    pub fn convert_to_node_space(&self, world_point: Vec2) -> Vec2 {
        let world_transform = self.get_node_to_world_transform();
        let point3 = Vec3::new(world_point.x, world_point.y, 0.0);
        let local_point = world_transform.inverted().unwrap_or(Mat4::IDENTITY) * point3;
        Vec2::new(local_point.x, local_point.y)
    }

    pub fn convert_to_world_space(&self, node_point: Vec2) -> Vec2 {
        let world_transform = self.get_node_to_world_transform();
        let point3 = Vec3::new(node_point.x, node_point.y, 0.0);
        let world_point = world_transform * point3;
        Vec2::new(world_point.x, world_point.y)
    }

    // ===== 额外变换 =====
    
    pub fn set_additional_transform(&mut self, transform: Option<Mat4>) {
        self.additional_transform = transform;
        self.additional_transform_dirty = true;
        self.set_transform_dirty();
    }

    pub fn get_additional_transform(&self) -> Option<Mat4> {
        self.additional_transform
    }

    // ===== 暂停/恢复 =====
    
    /// 是否已暂停
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// 暂停节点（停止运行但不隐藏）
    pub fn pause(&mut self) {
        self.paused = true;
        for child in &self.children {
            child.borrow_mut().pause();
        }
    }

    /// 恢复节点
    pub fn resume(&mut self) {
        self.paused = false;
        for child in &self.children {
            child.borrow_mut().resume();
        }
    }
    
    // ===== 运行状态 =====
    
    pub fn set_running(&mut self, running: bool) {
        if self.running != running {
            self.running = running;
            for child in &self.children {
                child.borrow_mut().set_running(running);
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn on_enter(&mut self) {
        self.set_running(true);
    }

    pub fn on_exit(&mut self) {
        self.set_running(false);
    }

    // Placeholder for draw callback - sprite模块需要但暂未完整实现
    pub fn set_on_draw(&mut self, _callback: Box<dyn Fn(&dyn std::any::Any, &Mat4)>) {
        // TODO: 实现绘制回调机制
    }

    // ===== 节点类型 =====
    
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }

    pub fn is_scene(&self) -> bool {
        self.node_type == NodeType::Scene
    }

    pub fn is_layer(&self) -> bool {
        matches!(
            self.node_type,
            NodeType::Layer | NodeType::LayerColor | NodeType::LayerGradient
        )
    }

    // ===== 排序 =====
    
    pub fn sort_all_children(&mut self) {
        if self.reorder_child_dirty {
            self.children.sort_by(|a, b| {
                let a_z = a.borrow().local_z_order;
                let b_z = b.borrow().local_z_order;
                a_z.cmp(&b_z)
            });
            self.reorder_child_dirty = false;
        }
    }

    // ===== 遍历 =====
    
    pub fn traverse<F>(&self, callback: &mut F)
    where
        F: FnMut(&Node),
    {
        callback(self);
        for child in &self.children {
            child.borrow().traverse(callback);
        }
    }

    pub fn traverse_mut<F>(&mut self, callback: &mut F)
    where
        F: FnMut(&mut Node),
    {
        callback(self);
        for child in &self.children {
            child.borrow_mut().traverse_mut(callback);
        }
    }

    // ===== 边界框 =====
    
    pub fn get_bounding_box(&self) -> Rect {
        let size = self.content_size;
        Rect::new(0.0, 0.0, size.width, size.height)
    }

    pub fn get_bounding_box_in_parent(&self) -> Rect {
        let bbox = self.get_bounding_box();
        let transform = self.get_node_to_parent_transform();
        
        let origin = Vec3::new(bbox.origin.x, bbox.origin.y, 0.0);
        let transformed = transform * origin;
        
        Rect::new(transformed.x, transformed.y, bbox.size.width, bbox.size.height)
    }

    // ===== cocos2d-x API 兼容方法（别名方法）=====

    /// 获取位置（别名方法，兼容 cocos2d-x API）
    #[inline]
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// 设置 X 坐标
    pub fn set_position_x(&mut self, x: f32) {
        self.position.x = x;
        self.transform_flags.position = true;
        self.set_transform_dirty();
    }

    /// 设置 Y 坐标
    pub fn set_position_y(&mut self, y: f32) {
        self.position.y = y;
        self.transform_flags.position = true;
        self.set_transform_dirty();
    }

    /// 获取旋转角度（别名方法）
    #[inline]
    pub fn get_rotation(&self) -> f32 {
        self.rotation_x
    }

    /// 获取 X 轴缩放（别名方法）
    #[inline]
    pub fn get_scale_x(&self) -> f32 {
        self.scale_x
    }

    /// 获取 Y 轴缩放（别名方法）
    #[inline]
    pub fn get_scale_y(&self) -> f32 {
        self.scale_y
    }

    /// 获取透明度（别名方法）
    #[inline]
    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    /// 获取锚点（别名方法）
    #[inline]
    pub fn get_anchor_point(&self) -> Vec2 {
        self.anchor_point
    }

    /// 获取本地 Z-order（别名方法）
    #[inline]
    pub fn get_local_z_order(&self) -> i32 {
        self.local_z_order
    }

    /// 获取全局 Z-order（别名方法）
    #[inline]
    pub fn get_global_z_order(&self) -> i32 {
        self.global_z_order
    }

    /// 获取标签（别名方法）
    #[inline]
    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    /// 获取名称（别名方法，返回 String 克隆）
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// 获取内容尺寸（别名方法）
    #[inline]
    pub fn get_content_size(&self) -> Size {
        self.content_size
    }

    /// 获取世界坐标
    pub fn get_world_position(&self) -> Vec2 {
        if let Some(ref parent) = self.parent {
            let parent_pos = parent.borrow().get_world_position();
            parent_pos + self.position
        } else {
            self.position
        }
    }

    /// 设置用户数据（整数类型，简化版）
    pub fn set_user_data(&mut self, data: i64) {
        self.user_data = data;
    }

    /// 获取用户数据
    pub fn get_user_data(&self) -> i64 {
        self.user_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new();
        assert_eq!(node.position(), Vec2::zero());
        assert_eq!(node.scale(), 1.0);
        assert_eq!(node.rotation(), 0.0);
        assert_eq!(node.anchor_point(), Vec2::new(0.5, 0.5));
        assert!(node.is_visible());
        assert_eq!(node.opacity(), 255);
        assert_eq!(node.tag(), TAG_INVALID);
    }

    #[test]
    fn test_node_position() {
        let mut node = Node::new();
        node.set_position_xy(100.0, 200.0);
        assert_eq!(node.position(), Vec2::new(100.0, 200.0));
        assert_eq!(node.get_position_x(), 100.0);
        assert_eq!(node.get_position_y(), 200.0);
    }

    #[test]
    fn test_node_scale() {
        let mut node = Node::new();
        node.set_scale(2.0);
        assert_eq!(node.scale(), 2.0);
        assert_eq!(node.scale_x(), 2.0);
        assert_eq!(node.scale_y(), 2.0);

        node.set_scale_xy(3.0, 4.0);
        assert_eq!(node.scale_x(), 3.0);
        assert_eq!(node.scale_y(), 4.0);
    }

    #[test]
    fn test_node_rotation() {
        let mut node = Node::new();
        node.set_rotation(45.0);
        assert_eq!(node.rotation(), 45.0);
        assert_eq!(node.rotation_x(), 45.0);
        assert_eq!(node.rotation_y(), 45.0);

        node.set_rotation_x(90.0);
        assert_eq!(node.rotation_x(), 90.0);
    }

    #[test]
    fn test_node_anchor_point() {
        let mut node = Node::new();
        node.set_anchor_point(Vec2::new(0.0, 0.0));
        assert_eq!(node.anchor_point(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_node_content_size() {
        let mut node = Node::new();
        node.set_content_size(Size::new(100.0, 50.0));
        assert_eq!(node.content_size(), Size::new(100.0, 50.0));
        assert_eq!(node.get_content_size_width(), 100.0);
        assert_eq!(node.get_content_size_height(), 50.0);
    }

    #[test]
    fn test_node_visible() {
        let mut node = Node::new();
        assert!(node.is_visible());

        node.set_visible(false);
        assert!(!node.is_visible());

        node.set_visible(true);
        assert!(node.is_visible());
    }

    #[test]
    fn test_node_opacity() {
        let mut node = Node::new();
        assert_eq!(node.opacity(), 255);

        node.set_opacity(128);
        assert_eq!(node.opacity(), 128);
        assert_eq!(node.displayed_opacity(), 128);
    }

    #[test]
    fn test_node_tag() {
        let mut node = Node::new();
        assert_eq!(node.tag(), TAG_INVALID);

        node.set_tag(100);
        assert_eq!(node.tag(), 100);
    }

    #[test]
    fn test_node_name() {
        let mut node = Node::new();
        node.set_name("TestNode");
        assert_eq!(node.name(), "TestNode");
    }

    #[test]
    fn test_node_parent_child() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child = Rc::new(RefCell::new(Node::new()));

        Node::add_child_to_parent(&parent, Rc::clone(&child), 0, None);

        let child_borrow = child.borrow();
        assert!(child_borrow.get_parent().is_some());
        assert_eq!(parent.borrow().get_children_count(), 1);
    }

    #[test]
    fn test_node_remove_child() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child = Rc::new(RefCell::new(Node::new()));
        child.borrow_mut().set_tag(100);

        Node::add_child_to_parent(&parent, Rc::clone(&child), 0, None);

        let mut parent_mut = parent.borrow_mut();
        parent_mut.remove_child_by_tag(100, true);

        assert_eq!(parent_mut.get_children_count(), 0);
    }

    #[test]
    fn test_node_get_child_by_tag() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child1 = Rc::new(RefCell::new(Node::new()));
        let child2 = Rc::new(RefCell::new(Node::new()));
        
        child1.borrow_mut().set_tag(100);
        child2.borrow_mut().set_tag(200);

        Node::add_child_to_parent(&parent, Rc::clone(&child1), 0, None);
        Node::add_child_to_parent(&parent, Rc::clone(&child2), 0, None);

        let found = parent.borrow().get_child_by_tag(100);
        assert!(found.is_some());
        assert_eq!(found.unwrap().borrow().tag(), 100);
    }

    #[test]
    fn test_node_get_child_by_name() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child1 = Rc::new(RefCell::new(Node::new()));
        
        Node::add_child_to_parent(&parent, Rc::clone(&child1), 0, Some("Child1"));

        let found = parent.borrow().get_child_by_name("Child1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().borrow().name(), "Child1");
    }

    #[test]
    fn test_node_sort_children() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child1 = Rc::new(RefCell::new(Node::new()));
        let child2 = Rc::new(RefCell::new(Node::new()));
        let child3 = Rc::new(RefCell::new(Node::new()));

        child1.borrow_mut().set_tag(1);
        child2.borrow_mut().set_tag(2);
        child3.borrow_mut().set_tag(3);

        Node::add_child_to_parent(&parent, Rc::clone(&child1), 3, None);
        Node::add_child_to_parent(&parent, Rc::clone(&child2), 1, None);
        Node::add_child_to_parent(&parent, Rc::clone(&child3), 2, None);
        parent.borrow_mut().sort_all_children();

        let parent_ref = parent.borrow();
        assert_eq!(parent_ref.get_children()[0].borrow().tag(), 2);
        assert_eq!(parent_ref.get_children()[1].borrow().tag(), 3);
        assert_eq!(parent_ref.get_children()[2].borrow().tag(), 1);
    }

    #[test]
    fn test_node_z_order() {
        let mut node = Node::new();
        node.set_local_z_order(5);
        assert_eq!(node.local_z_order(), 5);

        node.set_global_z_order(10);
        assert_eq!(node.global_z_order(), 10);
    }

    #[test]
    fn test_node_skew() {
        let mut node = Node::new();
        node.set_skew(10.0);
        assert_eq!(node.skew_x(), 10.0);
        assert_eq!(node.skew_y(), 10.0);

        node.set_skew_xy(5.0, 15.0);
        assert_eq!(node.skew_x(), 5.0);
        assert_eq!(node.skew_y(), 15.0);
    }

    #[test]
    fn test_node_cascade_opacity() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child = Rc::new(RefCell::new(Node::new()));

        parent.borrow_mut().set_opacity(128);
        child.borrow_mut().set_opacity(255);
        child.borrow_mut().set_cascade_opacity(true);

        Node::add_child_to_parent(&parent, Rc::clone(&child), 0, None);

        let child_borrow = child.borrow();
        assert!(child_borrow.displayed_opacity() <= 128);
    }

    #[test]
    fn test_node_convert_space() {
        let mut node = Node::new();
        node.set_position_xy(100.0, 200.0);

        let world_point = Vec2::new(110.0, 210.0);
        let local_point = node.convert_to_node_space(world_point);
        
        // 世界坐标 (110, 210) 相对于节点位置 (100, 200)
        // 在节点本地空间应该是 (10, 10)
        // 因此 local_point 的值应该远小于 world_point
        assert!((local_point.x - 10.0).abs() < 0.01, 
            "Expected local_point.x ≈ 10.0, got {}", local_point.x);
        assert!((local_point.y - 10.0).abs() < 0.01,
            "Expected local_point.y ≈ 10.0, got {}", local_point.y);
    }

    #[test]
    fn test_node_traverse() {
        let parent = Rc::new(RefCell::new(Node::new()));
        parent.borrow_mut().set_tag(0);
        let child1 = Rc::new(RefCell::new(Node::new()));
        child1.borrow_mut().set_tag(1);
        let child2 = Rc::new(RefCell::new(Node::new()));
        child2.borrow_mut().set_tag(2);

        Node::add_child_to_parent(&parent, Rc::clone(&child1), 0, None);
        Node::add_child_to_parent(&parent, Rc::clone(&child2), 0, None);

        let mut count = 0;
        let mut tags = Vec::new();
        parent.borrow().traverse(&mut |node| {
            count += 1;
            tags.push(node.tag());
        });

        assert_eq!(count, 3);
        assert_eq!(tags, vec![0, 1, 2]);
    }

    #[test]
    fn test_node_bounding_box() {
        let mut node = Node::new();
        node.set_content_size(Size::new(100.0, 50.0));
        node.set_anchor_point(Vec2::new(0.0, 0.0));

        let bbox = node.get_bounding_box();
        assert_eq!(bbox.origin, Vec2::zero());
        assert_eq!(bbox.size, Size::new(100.0, 50.0));
    }

    #[test]
    fn test_node_on_enter_exit() {
        let mut node = Node::new();
        assert!(!node.is_running());

        node.on_enter();
        assert!(node.is_running());

        node.on_exit();
        assert!(!node.is_running());
    }

    #[test]
    fn test_node_with_type() {
        let node = Node::with_type(NodeType::Scene);
        assert!(node.is_scene());
        assert!(!node.is_layer());

        let layer = Node::with_type(NodeType::Layer);
        assert!(!layer.is_scene());
        assert!(layer.is_layer());
    }

    #[test]
    fn test_node_transform_flags() {
        let mut flags = TransformFlags::new();
        assert!(flags.dirty());

        flags.clean();
        assert!(!flags.dirty());

        flags.position = true;
        assert!(flags.dirty());
    }

    #[test]
    fn test_node_remove_all_children() {
        let parent = Rc::new(RefCell::new(Node::new()));
        let child1 = Rc::new(RefCell::new(Node::new()));
        let child2 = Rc::new(RefCell::new(Node::new()));

        Node::add_child_to_parent(&parent, Rc::clone(&child1), 0, None);
        Node::add_child_to_parent(&parent, Rc::clone(&child2), 0, None);
        assert_eq!(parent.borrow().get_children_count(), 2);

        parent.borrow_mut().remove_all_children(false);
        assert_eq!(parent.borrow().get_children_count(), 0);
    }

    #[test]
    fn test_node_ignore_anchor_point() {
        let mut node = Node::new();
        assert!(!node.is_ignore_anchor_point_for_position());

        node.set_ignore_anchor_point_for_position(true);
        assert!(node.is_ignore_anchor_point_for_position());
    }
}
