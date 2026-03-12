#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
/// Spine 骨骼动画基础框架
///
/// 对 Spine Runtime 的 Rust 端封装，支持：
/// - 骨骼(Skeleton)数据加载和管理
/// - 骨头(Bone)层次结构
/// - 插槽(Slot)附件系统
/// - 动画(Animation)混合和过渡
/// - 皮肤(Skin)系统
/// - 事件(Event)系统
/// - 约束(Constraint)系统
///
/// 注意：本模块提供 Spine 数据结构和基础逻辑框架，
/// 完整实现需对接 spine-c 或 spine-rs 底层库。

use std::collections::HashMap;
use crate::math::Vec2;

/// 骨头(Bone)数据 —— 骨架的基础组成元素
#[derive(Debug, Clone)]
pub struct BoneData {
    pub index: usize,
    pub name: String,
    pub parent: Option<usize>,
    pub length: f32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub shear_x: f32,
    pub shear_y: f32,
    pub transform_mode: TransformMode,
}

impl BoneData {
    pub fn new(index: usize, name: &str) -> Self {
        Self {
            index,
            name: name.to_string(),
            parent: None,
            length: 0.0,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            shear_x: 0.0,
            shear_y: 0.0,
            transform_mode: TransformMode::Normal,
        }
    }

    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
}

/// 骨头变换模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformMode {
    Normal,
    OnlyTranslation,
    NoRotationOrReflection,
    NoScale,
    NoScaleOrReflection,
}

/// 运行时骨头实例
#[derive(Debug, Clone)]
pub struct Bone {
    pub data: BoneData,
    pub world_x: f32,
    pub world_y: f32,
    pub world_rotation: f32,
    pub world_scale_x: f32,
    pub world_scale_y: f32,
    // 本地覆盖值
    pub local_x: f32,
    pub local_y: f32,
    pub local_rotation: f32,
    pub local_scale_x: f32,
    pub local_scale_y: f32,
    /// 是否激活
    pub active: bool,
}

impl Bone {
    pub fn from_data(data: &BoneData) -> Self {
        Self {
            data: data.clone(),
            world_x: 0.0,
            world_y: 0.0,
            world_rotation: 0.0,
            world_scale_x: 1.0,
            world_scale_y: 1.0,
            local_x: data.x,
            local_y: data.y,
            local_rotation: data.rotation,
            local_scale_x: data.scale_x,
            local_scale_y: data.scale_y,
            active: true,
        }
    }

    /// 重置到初始状态
    pub fn set_to_setup_pose(&mut self) {
        self.local_x = self.data.x;
        self.local_y = self.data.y;
        self.local_rotation = self.data.rotation;
        self.local_scale_x = self.data.scale_x;
        self.local_scale_y = self.data.scale_y;
    }

    /// 计算世界变换（需要父骨头的世界变换）
    pub fn update_world_transform(&mut self, parent: Option<&Bone>) {
        if let Some(p) = parent {
            let rad = p.world_rotation.to_radians();
            let cos_r = rad.cos();
            let sin_r = rad.sin();
            self.world_x = p.world_x + self.local_x * cos_r * p.world_scale_x
                - self.local_y * sin_r * p.world_scale_y;
            self.world_y = p.world_y + self.local_x * sin_r * p.world_scale_x
                + self.local_y * cos_r * p.world_scale_y;
            self.world_rotation = p.world_rotation + self.local_rotation;
            self.world_scale_x = p.world_scale_x * self.local_scale_x;
            self.world_scale_y = p.world_scale_y * self.local_scale_y;
        } else {
            self.world_x = self.local_x;
            self.world_y = self.local_y;
            self.world_rotation = self.local_rotation;
            self.world_scale_x = self.local_scale_x;
            self.world_scale_y = self.local_scale_y;
        }
    }

    /// 获取世界位置
    pub fn world_position(&self) -> Vec2 {
        Vec2::new(self.world_x, self.world_y)
    }
}

/// 附件类型
#[derive(Debug, Clone)]
pub enum AttachmentType {
    Region,
    Mesh,
    BoundingBox,
    Path,
    Point,
    Clipping,
}

/// 附件 —— 附着在插槽上的可视元素
#[derive(Debug, Clone)]
pub struct Attachment {
    pub name: String,
    pub attachment_type: AttachmentType,
    /// 区域附件：对应的图片区域名
    pub region_name: Option<String>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub width: f32,
    pub height: f32,
    pub color: (f32, f32, f32, f32), // RGBA
}

impl Attachment {
    pub fn region(name: &str, region: &str) -> Self {
        Self {
            name: name.to_string(),
            attachment_type: AttachmentType::Region,
            region_name: Some(region.to_string()),
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            width: 0.0,
            height: 0.0,
            color: (1.0, 1.0, 1.0, 1.0),
        }
    }
}

/// 插槽(Slot)数据
#[derive(Debug, Clone)]
pub struct SlotData {
    pub index: usize,
    pub name: String,
    pub bone_index: usize,
    pub color: (f32, f32, f32, f32),
    pub attachment_name: Option<String>,
    pub blend_mode: BlendMode,
}

impl SlotData {
    pub fn new(index: usize, name: &str, bone_index: usize) -> Self {
        Self {
            index,
            name: name.to_string(),
            bone_index,
            color: (1.0, 1.0, 1.0, 1.0),
            attachment_name: None,
            blend_mode: BlendMode::Normal,
        }
    }
}

/// 混合模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Additive,
    Multiply,
    Screen,
}

/// 运行时插槽实例
#[derive(Debug, Clone)]
pub struct Slot {
    pub data: SlotData,
    pub attachment: Option<Attachment>,
    pub color: (f32, f32, f32, f32),
}

impl Slot {
    pub fn from_data(data: &SlotData) -> Self {
        Self {
            data: data.clone(),
            attachment: None,
            color: data.color,
        }
    }

    pub fn set_to_setup_pose(&mut self) {
        self.color = self.data.color;
        self.attachment = None;
    }
}

/// 皮肤(Skin)
#[derive(Debug, Clone)]
pub struct Skin {
    pub name: String,
    /// (slot_index, attachment_name) -> Attachment
    attachments: HashMap<(usize, String), Attachment>,
}

impl Skin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attachments: HashMap::new(),
        }
    }

    pub fn add_attachment(&mut self, slot_index: usize, name: &str, attachment: Attachment) {
        self.attachments.insert((slot_index, name.to_string()), attachment);
    }

    pub fn get_attachment(&self, slot_index: usize, name: &str) -> Option<&Attachment> {
        self.attachments.get(&(slot_index, name.to_string()))
    }

    pub fn attachment_count(&self) -> usize {
        self.attachments.len()
    }
}

/// Spine 事件数据
#[derive(Debug, Clone)]
pub struct SpineEventData {
    pub name: String,
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: String,
}

impl SpineEventData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            int_value: 0,
            float_value: 0.0,
            string_value: String::new(),
        }
    }
}

/// 触发的事件实例
#[derive(Debug, Clone)]
pub struct SpineEvent {
    pub data: SpineEventData,
    pub time: f32,
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: String,
}

/// 动画关键帧类型
#[derive(Debug, Clone)]
pub enum TimelineType {
    Rotate,
    Translate,
    Scale,
    Shear,
    Color,
    Attachment,
    Event,
    DrawOrder,
}

/// 关键帧
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub value2: Option<f32>,
    pub curve_type: CurveType,
}

impl Keyframe {
    pub fn linear(time: f32, value: f32) -> Self {
        Self { time, value, value2: None, curve_type: CurveType::Linear }
    }

    pub fn stepped(time: f32, value: f32) -> Self {
        Self { time, value, value2: None, curve_type: CurveType::Stepped }
    }
}

/// 插值曲线类型
#[derive(Debug, Clone)]
pub enum CurveType {
    Linear,
    Stepped,
    Bezier { cx1: f32, cy1: f32, cx2: f32, cy2: f32 },
}

/// 时间线
#[derive(Debug, Clone)]
pub struct Timeline {
    pub timeline_type: TimelineType,
    pub bone_index: Option<usize>,
    pub slot_index: Option<usize>,
    pub keyframes: Vec<Keyframe>,
}

impl Timeline {
    pub fn new(timeline_type: TimelineType) -> Self {
        Self {
            timeline_type,
            bone_index: None,
            slot_index: None,
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
    }

    pub fn duration(&self) -> f32 {
        self.keyframes.last().map(|k| k.time).unwrap_or(0.0)
    }

    /// 在时间 t 处采样（线性插值）
    pub fn sample(&self, time: f32) -> f32 {
        if self.keyframes.is_empty() { return 0.0; }
        if self.keyframes.len() == 1 { return self.keyframes[0].value; }

        // 找到左右关键帧
        if time <= self.keyframes[0].time { return self.keyframes[0].value; }
        if time >= self.keyframes.last().unwrap().time { return self.keyframes.last().unwrap().value; }

        for i in 0..self.keyframes.len() - 1 {
            let k0 = &self.keyframes[i];
            let k1 = &self.keyframes[i + 1];
            if time >= k0.time && time < k1.time {
                match &k0.curve_type {
                    CurveType::Stepped => return k0.value,
                    CurveType::Linear => {
                        let t = (time - k0.time) / (k1.time - k0.time);
                        return k0.value + (k1.value - k0.value) * t;
                    }
                    CurveType::Bezier { cx1, cy1, cx2, cy2 } => {
                        let t = (time - k0.time) / (k1.time - k0.time);
                        let bez = Self::cubic_bezier(t, *cx1, *cy1, *cx2, *cy2);
                        return k0.value + (k1.value - k0.value) * bez;
                    }
                }
            }
        }

        self.keyframes.last().unwrap().value
    }

    fn cubic_bezier(t: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32) -> f32 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        // 简化计算：P0=(0,0), P1=(cx1,cy1), P2=(cx2,cy2), P3=(1,1)
        3.0 * mt2 * t * cy1 + 3.0 * mt * t2 * cy2 + t3
    }
}

/// 动画
#[derive(Debug, Clone)]
pub struct SpineAnimation {
    pub name: String,
    pub duration: f32,
    pub timelines: Vec<Timeline>,
}

impl SpineAnimation {
    pub fn new(name: &str, duration: f32) -> Self {
        Self {
            name: name.to_string(),
            duration,
            timelines: Vec::new(),
        }
    }

    pub fn add_timeline(&mut self, timeline: Timeline) {
        self.timelines.push(timeline);
    }
}

/// 动画混合模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixBlend {
    Setup,
    First,
    Replace,
    Add,
}

/// 动画播放轨道
#[derive(Debug, Clone)]
pub struct TrackEntry {
    pub animation: SpineAnimation,
    pub track_index: usize,
    pub loop_animation: bool,
    pub time: f32,
    pub time_scale: f32,
    pub alpha: f32,
    pub mix_time: f32,
    pub mix_duration: f32,
    pub mix_blend: MixBlend,
    pub event_threshold: f32,
    pub complete: bool,
}

impl TrackEntry {
    pub fn new(animation: SpineAnimation, track_index: usize) -> Self {
        Self {
            animation,
            track_index,
            loop_animation: false,
            time: 0.0,
            time_scale: 1.0,
            alpha: 1.0,
            mix_time: 0.0,
            mix_duration: 0.0,
            mix_blend: MixBlend::Replace,
            event_threshold: 0.0,
            complete: false,
        }
    }

    /// 更新轨道时间
    pub fn update(&mut self, delta: f32) {
        self.time += delta * self.time_scale;
        if self.time >= self.animation.duration {
            if self.loop_animation {
                self.time %= self.animation.duration;
            } else {
                self.time = self.animation.duration;
                self.complete = true;
            }
        }
    }

    /// 获取当前是否正在混合
    pub fn is_mixing(&self) -> bool {
        self.mix_time < self.mix_duration
    }
}

/// 骨骼数据
#[derive(Debug, Clone)]
pub struct SkeletonData {
    pub name: String,
    pub bones: Vec<BoneData>,
    pub slots: Vec<SlotData>,
    pub skins: Vec<Skin>,
    pub animations: Vec<SpineAnimation>,
    pub events: Vec<SpineEventData>,
    pub default_skin: Option<String>,
    pub width: f32,
    pub height: f32,
    pub version: String,
}

impl SkeletonData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            bones: Vec::new(),
            slots: Vec::new(),
            skins: Vec::new(),
            animations: Vec::new(),
            events: Vec::new(),
            default_skin: None,
            width: 0.0,
            height: 0.0,
            version: String::new(),
        }
    }

    pub fn find_bone(&self, name: &str) -> Option<&BoneData> {
        self.bones.iter().find(|b| b.name == name)
    }

    pub fn find_slot(&self, name: &str) -> Option<&SlotData> {
        self.slots.iter().find(|s| s.name == name)
    }

    pub fn find_skin(&self, name: &str) -> Option<&Skin> {
        self.skins.iter().find(|s| s.name == name)
    }

    pub fn find_animation(&self, name: &str) -> Option<&SpineAnimation> {
        self.animations.iter().find(|a| a.name == name)
    }
}

/// 骨骼动画状态
#[derive(Debug, Clone)]
pub struct AnimationState {
    tracks: Vec<Option<TrackEntry>>,
    pub time_scale: f32,
    events: Vec<SpineEvent>,
}

impl AnimationState {
    pub fn new(num_tracks: usize) -> Self {
        Self {
            tracks: vec![None; num_tracks.max(1)],
            time_scale: 1.0,
            events: Vec::new(),
        }
    }

    /// 设置动画到指定轨道
    pub fn set_animation(&mut self, track: usize, animation: SpineAnimation, loop_anim: bool) {
        if track >= self.tracks.len() {
            self.tracks.resize(track + 1, None);
        }
        let mut entry = TrackEntry::new(animation, track);
        entry.loop_animation = loop_anim;
        self.tracks[track] = Some(entry);
    }

    /// 添加动画到轨道队列（TODO: 实现排队机制）
    pub fn add_animation(&mut self, track: usize, animation: SpineAnimation, loop_anim: bool, delay: f32) {
        // 简化实现：直接替换
        self.set_animation(track, animation, loop_anim);
    }

    /// 清空轨道
    pub fn clear_track(&mut self, track: usize) {
        if track < self.tracks.len() {
            self.tracks[track] = None;
        }
    }

    /// 清空所有轨道
    pub fn clear_tracks(&mut self) {
        for track in &mut self.tracks {
            *track = None;
        }
    }

    /// 更新所有轨道
    pub fn update(&mut self, delta: f32) {
        let scaled_delta = delta * self.time_scale;
        for track in &mut self.tracks {
            if let Some(entry) = track {
                entry.update(scaled_delta);
            }
        }
    }

    /// 获取轨道
    pub fn get_current(&self, track: usize) -> Option<&TrackEntry> {
        self.tracks.get(track).and_then(|t| t.as_ref())
    }

    /// 获取已触发的事件
    pub fn poll_events(&mut self) -> Vec<SpineEvent> {
        self.events.drain(..).collect()
    }
}

/// 骨架实例
#[derive(Debug)]
pub struct Skeleton {
    pub data: SkeletonData,
    pub bones: Vec<Bone>,
    pub slots: Vec<Slot>,
    pub skin: Option<String>,
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub color: (f32, f32, f32, f32),
}

impl Skeleton {
    pub fn from_data(data: &SkeletonData) -> Self {
        let bones = data.bones.iter().map(Bone::from_data).collect();
        let slots = data.slots.iter().map(Slot::from_data).collect();

        Self {
            data: data.clone(),
            bones,
            slots,
            skin: data.default_skin.clone(),
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            color: (1.0, 1.0, 1.0, 1.0),
        }
    }

    /// 重置到初始状态
    pub fn set_to_setup_pose(&mut self) {
        for bone in &mut self.bones {
            bone.set_to_setup_pose();
        }
        for slot in &mut self.slots {
            slot.set_to_setup_pose();
        }
    }

    /// 更新世界变换
    pub fn update_world_transform(&mut self) {
        // 使用索引避免借用冲突
        for i in 0..self.bones.len() {
            let parent_idx = self.bones[i].data.parent;
            if let Some(pidx) = parent_idx {
                // 需要从父骨头复制世界变换
                let parent_world = (
                    self.bones[pidx].world_x,
                    self.bones[pidx].world_y,
                    self.bones[pidx].world_rotation,
                    self.bones[pidx].world_scale_x,
                    self.bones[pidx].world_scale_y,
                );
                let bone = &mut self.bones[i];
                let rad = parent_world.2.to_radians();
                let cos_r = rad.cos();
                let sin_r = rad.sin();
                bone.world_x = parent_world.0 + bone.local_x * cos_r * parent_world.3
                    - bone.local_y * sin_r * parent_world.4;
                bone.world_y = parent_world.1 + bone.local_x * sin_r * parent_world.3
                    + bone.local_y * cos_r * parent_world.4;
                bone.world_rotation = parent_world.2 + bone.local_rotation;
                bone.world_scale_x = parent_world.3 * bone.local_scale_x;
                bone.world_scale_y = parent_world.4 * bone.local_scale_y;
            } else {
                let bone = &mut self.bones[i];
                bone.world_x = bone.local_x + self.x;
                bone.world_y = bone.local_y + self.y;
                bone.world_rotation = bone.local_rotation + self.rotation;
                bone.world_scale_x = bone.local_scale_x * self.scale_x;
                bone.world_scale_y = bone.local_scale_y * self.scale_y;
            }
        }
    }

    /// 设置皮肤
    pub fn set_skin(&mut self, skin_name: &str) -> bool {
        if self.data.find_skin(skin_name).is_some() {
            self.skin = Some(skin_name.to_string());
            true
        } else {
            false
        }
    }

    /// 查找骨头
    pub fn find_bone(&self, name: &str) -> Option<&Bone> {
        self.bones.iter().find(|b| b.data.name == name)
    }

    /// 查找骨头（可变引用）
    pub fn find_bone_mut(&mut self, name: &str) -> Option<&mut Bone> {
        self.bones.iter_mut().find(|b| b.data.name == name)
    }

    /// 查找插槽
    pub fn find_slot(&self, name: &str) -> Option<&Slot> {
        self.slots.iter().find(|s| s.data.name == name)
    }

    /// 获取骨架包围盒
    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        // 简化：返回所有骨头的极值范围
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for bone in &self.bones {
            min_x = min_x.min(bone.world_x);
            min_y = min_y.min(bone.world_y);
            max_x = max_x.max(bone.world_x);
            max_y = max_y.max(bone.world_y);
        }

        (min_x, min_y, max_x - min_x, max_y - min_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_skeleton_data() -> SkeletonData {
        let mut data = SkeletonData::new("test_skeleton");

        // 根骨头
        data.bones.push(BoneData::new(0, "root"));

        // 身体骨头
        data.bones.push(BoneData::new(1, "body").with_parent(0).with_position(0.0, 100.0));

        // 头部骨头
        data.bones.push(BoneData::new(2, "head").with_parent(1).with_position(0.0, 50.0));

        // 左臂
        data.bones.push(BoneData::new(3, "left_arm")
            .with_parent(1)
            .with_position(-30.0, 40.0)
            .with_rotation(-45.0));

        // 右臂
        data.bones.push(BoneData::new(4, "right_arm")
            .with_parent(1)
            .with_position(30.0, 40.0)
            .with_rotation(45.0));

        // 插槽
        data.slots.push(SlotData::new(0, "body_slot", 1));
        data.slots.push(SlotData::new(1, "head_slot", 2));

        // 默认皮肤
        let mut skin = Skin::new("default");
        skin.add_attachment(0, "body", Attachment::region("body", "body_region"));
        skin.add_attachment(1, "head", Attachment::region("head", "head_region"));
        data.skins.push(skin);
        data.default_skin = Some("default".to_string());

        // 动画
        let mut idle_anim = SpineAnimation::new("idle", 1.0);
        let mut rotate_tl = Timeline::new(TimelineType::Rotate);
        rotate_tl.bone_index = Some(2);
        rotate_tl.add_keyframe(Keyframe::linear(0.0, 0.0));
        rotate_tl.add_keyframe(Keyframe::linear(0.5, 10.0));
        rotate_tl.add_keyframe(Keyframe::linear(1.0, 0.0));
        idle_anim.add_timeline(rotate_tl);
        data.animations.push(idle_anim);

        let walk_anim = SpineAnimation::new("walk", 0.8);
        data.animations.push(walk_anim);

        data
    }

    #[test]
    fn test_bone_data_creation() {
        let bone = BoneData::new(0, "root");
        assert_eq!(bone.name, "root");
        assert_eq!(bone.index, 0);
        assert!(bone.parent.is_none());
        assert_eq!(bone.scale_x, 1.0);
    }

    #[test]
    fn test_bone_data_with_parent() {
        let bone = BoneData::new(1, "child").with_parent(0).with_position(10.0, 20.0);
        assert_eq!(bone.parent, Some(0));
        assert_eq!(bone.x, 10.0);
        assert_eq!(bone.y, 20.0);
    }

    #[test]
    fn test_bone_world_transform_root() {
        let data = BoneData::new(0, "root");
        let mut bone = Bone::from_data(&data);
        bone.local_x = 100.0;
        bone.local_y = 200.0;
        bone.update_world_transform(None);
        assert_eq!(bone.world_x, 100.0);
        assert_eq!(bone.world_y, 200.0);
    }

    #[test]
    fn test_bone_world_transform_child() {
        let parent_data = BoneData::new(0, "parent");
        let mut parent = Bone::from_data(&parent_data);
        parent.world_x = 100.0;
        parent.world_y = 100.0;
        parent.world_rotation = 0.0;
        parent.world_scale_x = 1.0;
        parent.world_scale_y = 1.0;

        let child_data = BoneData::new(1, "child");
        let mut child = Bone::from_data(&child_data);
        child.local_x = 50.0;
        child.local_y = 0.0;
        child.update_world_transform(Some(&parent));

        assert!((child.world_x - 150.0).abs() < 0.01);
        assert!((child.world_y - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_skeleton_creation() {
        let data = make_test_skeleton_data();
        let skeleton = Skeleton::from_data(&data);
        assert_eq!(skeleton.bones.len(), 5);
        assert_eq!(skeleton.slots.len(), 2);
        assert_eq!(skeleton.skin.as_deref(), Some("default"));
    }

    #[test]
    fn test_skeleton_find_bone() {
        let data = make_test_skeleton_data();
        let skeleton = Skeleton::from_data(&data);
        assert!(skeleton.find_bone("root").is_some());
        assert!(skeleton.find_bone("head").is_some());
        assert!(skeleton.find_bone("nonexistent").is_none());
    }

    #[test]
    fn test_skeleton_set_to_setup_pose() {
        let data = make_test_skeleton_data();
        let mut skeleton = Skeleton::from_data(&data);
        
        // 修改骨头
        if let Some(bone) = skeleton.find_bone_mut("head") {
            bone.local_rotation = 90.0;
        }

        // 重置
        skeleton.set_to_setup_pose();
        let head = skeleton.find_bone("head").unwrap();
        assert_eq!(head.local_rotation, 0.0);
    }

    #[test]
    fn test_skeleton_update_world_transform() {
        let data = make_test_skeleton_data();
        let mut skeleton = Skeleton::from_data(&data);
        skeleton.x = 200.0;
        skeleton.y = 300.0;
        skeleton.update_world_transform();

        let root = skeleton.find_bone("root").unwrap();
        assert_eq!(root.world_x, 200.0);
        assert_eq!(root.world_y, 300.0);

        let body = skeleton.find_bone("body").unwrap();
        assert!((body.world_y - 400.0).abs() < 0.01, "body y={}", body.world_y);
    }

    #[test]
    fn test_skeleton_set_skin() {
        let data = make_test_skeleton_data();
        let mut skeleton = Skeleton::from_data(&data);
        assert!(skeleton.set_skin("default"));
        assert!(!skeleton.set_skin("nonexistent"));
    }

    #[test]
    fn test_skin_operations() {
        let mut skin = Skin::new("warrior");
        skin.add_attachment(0, "body", Attachment::region("body_warrior", "warrior_body"));
        assert_eq!(skin.attachment_count(), 1);
        assert!(skin.get_attachment(0, "body").is_some());
        assert!(skin.get_attachment(0, "nothing").is_none());
    }

    #[test]
    fn test_animation_state() {
        let data = make_test_skeleton_data();
        let mut state = AnimationState::new(4);
        state.set_animation(0, data.animations[0].clone(), true);

        let entry = state.get_current(0);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().animation.name, "idle");
        assert!(entry.unwrap().loop_animation);
    }

    #[test]
    fn test_animation_state_update() {
        let data = make_test_skeleton_data();
        let mut state = AnimationState::new(1);
        state.set_animation(0, data.animations[0].clone(), true);

        state.update(0.5);
        let entry = state.get_current(0).unwrap();
        assert!((entry.time - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_animation_state_loop() {
        let data = make_test_skeleton_data();
        let mut state = AnimationState::new(1);
        state.set_animation(0, data.animations[0].clone(), true);

        // 更新超过动画时长
        state.update(1.5);
        let entry = state.get_current(0).unwrap();
        assert!(!entry.complete); // 循环动画不应标记完成
        assert!(entry.time < 1.0); // 应该循环回来
    }

    #[test]
    fn test_animation_state_no_loop() {
        let data = make_test_skeleton_data();
        let mut state = AnimationState::new(1);
        state.set_animation(0, data.animations[0].clone(), false);

        state.update(2.0);
        let entry = state.get_current(0).unwrap();
        assert!(entry.complete);
    }

    #[test]
    fn test_animation_state_clear() {
        let data = make_test_skeleton_data();
        let mut state = AnimationState::new(2);
        state.set_animation(0, data.animations[0].clone(), true);
        state.set_animation(1, data.animations[1].clone(), true);

        state.clear_track(0);
        assert!(state.get_current(0).is_none());
        assert!(state.get_current(1).is_some());

        state.clear_tracks();
        assert!(state.get_current(1).is_none());
    }

    #[test]
    fn test_timeline_sample_linear() {
        let mut tl = Timeline::new(TimelineType::Rotate);
        tl.add_keyframe(Keyframe::linear(0.0, 0.0));
        tl.add_keyframe(Keyframe::linear(1.0, 90.0));

        assert!((tl.sample(0.0) - 0.0).abs() < 0.01);
        assert!((tl.sample(0.5) - 45.0).abs() < 0.01);
        assert!((tl.sample(1.0) - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_timeline_sample_stepped() {
        let mut tl = Timeline::new(TimelineType::Rotate);
        tl.add_keyframe(Keyframe::stepped(0.0, 0.0));
        tl.add_keyframe(Keyframe::stepped(1.0, 90.0));

        assert!((tl.sample(0.0) - 0.0).abs() < 0.01);
        assert!((tl.sample(0.5) - 0.0).abs() < 0.01, "Stepped should not interpolate");
    }

    #[test]
    fn test_timeline_duration() {
        let mut tl = Timeline::new(TimelineType::Translate);
        tl.add_keyframe(Keyframe::linear(0.0, 0.0));
        tl.add_keyframe(Keyframe::linear(2.5, 100.0));
        assert_eq!(tl.duration(), 2.5);
    }

    #[test]
    fn test_skeleton_data_find() {
        let data = make_test_skeleton_data();
        assert!(data.find_bone("root").is_some());
        assert!(data.find_slot("body_slot").is_some());
        assert!(data.find_skin("default").is_some());
        assert!(data.find_animation("idle").is_some());
        assert!(data.find_animation("walk").is_some());
    }

    #[test]
    fn test_slot_data() {
        let slot = SlotData::new(0, "body", 1);
        assert_eq!(slot.name, "body");
        assert_eq!(slot.bone_index, 1);
        assert_eq!(slot.blend_mode, BlendMode::Normal);
    }

    #[test]
    fn test_event_data() {
        let event = SpineEventData::new("footstep");
        assert_eq!(event.name, "footstep");
        assert_eq!(event.int_value, 0);
    }

    #[test]
    fn test_skeleton_bounds() {
        let data = make_test_skeleton_data();
        let mut skeleton = Skeleton::from_data(&data);
        skeleton.update_world_transform();
        let (x, y, w, h) = skeleton.get_bounds();
        // 应该有有效的包围盒
        assert!(w >= 0.0);
        assert!(h >= 0.0);
    }
}
