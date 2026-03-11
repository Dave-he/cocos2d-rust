/// DragonBones - 龙骨骨骼动画系统
///
/// DragonBones 是由白鹭科技开发的骨骼动画系统，广泛用于 Cocos2d-x 游戏开发。
/// 本模块提供与 cocos2d-x DragonBones 对应的 Rust 基础框架。
///
/// 功能：
/// - 骨架数据（SkeletonData/ArmatureData）
/// - 骨头（Bone）树形结构
/// - 插槽（Slot）附件系统
/// - 皮肤（Skin）管理
/// - 动画数据（AnimationData）和时间线（Timeline）
/// - 动画状态机（AnimationState）
/// - 多动画混合（Blending）
/// - IK 约束（Inverse Kinematics）
/// - 事件监听（AnimationEvent）

use std::collections::HashMap;

// ========== 基础数学类型 ==========

/// 2D 变换
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub skew_x: f32,
    pub skew_y: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn identity() -> Self {
        Self::default()
    }

    /// 插值到目标变换
    pub fn lerp(&self, target: &Transform, t: f32) -> Transform {
        Transform {
            x: self.x + (target.x - self.x) * t,
            y: self.y + (target.y - self.y) * t,
            rotation: self.rotation + (target.rotation - self.rotation) * t,
            scale_x: self.scale_x + (target.scale_x - self.scale_x) * t,
            scale_y: self.scale_y + (target.scale_y - self.scale_y) * t,
            skew_x: self.skew_x + (target.skew_x - self.skew_x) * t,
            skew_y: self.skew_y + (target.skew_y - self.skew_y) * t,
        }
    }

    /// 组合两个变换（父 * 子）
    pub fn combine(&self, child: &Transform) -> Transform {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        Transform {
            x: self.x + child.x * cos_r * self.scale_x - child.y * sin_r * self.scale_y,
            y: self.y + child.x * sin_r * self.scale_x + child.y * cos_r * self.scale_y,
            rotation: self.rotation + child.rotation,
            scale_x: self.scale_x * child.scale_x,
            scale_y: self.scale_y * child.scale_y,
            skew_x: self.skew_x + child.skew_x,
            skew_y: self.skew_y + child.skew_y,
        }
    }
}

/// 矩形边界盒
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DBRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl DBRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

/// 颜色变换
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorTransform {
    /// 颜色乘算（0.0-1.0）
    pub r_multiplier: f32,
    pub g_multiplier: f32,
    pub b_multiplier: f32,
    pub a_multiplier: f32,
    /// 颜色偏移（-255 ~ 255）
    pub r_offset: i32,
    pub g_offset: i32,
    pub b_offset: i32,
    pub a_offset: i32,
}

impl Default for ColorTransform {
    fn default() -> Self {
        Self {
            r_multiplier: 1.0,
            g_multiplier: 1.0,
            b_multiplier: 1.0,
            a_multiplier: 1.0,
            r_offset: 0,
            g_offset: 0,
            b_offset: 0,
            a_offset: 0,
        }
    }
}

impl ColorTransform {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn lerp(&self, target: &ColorTransform, t: f32) -> ColorTransform {
        ColorTransform {
            r_multiplier: self.r_multiplier + (target.r_multiplier - self.r_multiplier) * t,
            g_multiplier: self.g_multiplier + (target.g_multiplier - self.g_multiplier) * t,
            b_multiplier: self.b_multiplier + (target.b_multiplier - self.b_multiplier) * t,
            a_multiplier: self.a_multiplier + (target.a_multiplier - self.a_multiplier) * t,
            r_offset: (self.r_offset as f32 + (target.r_offset as f32 - self.r_offset as f32) * t) as i32,
            g_offset: (self.g_offset as f32 + (target.g_offset as f32 - self.g_offset as f32) * t) as i32,
            b_offset: (self.b_offset as f32 + (target.b_offset as f32 - self.b_offset as f32) * t) as i32,
            a_offset: (self.a_offset as f32 + (target.a_offset as f32 - self.a_offset as f32) * t) as i32,
        }
    }
}

// ========== 插值曲线 ==========

/// 贝塞尔曲线插值点
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BezierPoint {
    pub x: f32,
    pub y: f32,
}

/// 缓动曲线类型（和 DragonBones 官方一致）
#[derive(Debug, Clone, PartialEq)]
pub enum TweenType {
    /// 无缓动（跳跃）
    None,
    /// 线性
    Linear,
    /// 贝塞尔曲线（可自定义控制点）
    Curve(Vec<BezierPoint>),
}

impl TweenType {
    /// 根据进度 t (0.0-1.0) 计算插值因子
    pub fn evaluate(&self, t: f32) -> f32 {
        match self {
            TweenType::None => if t >= 1.0 { 1.0 } else { 0.0 },
            TweenType::Linear => t.clamp(0.0, 1.0),
            TweenType::Curve(points) => Self::evaluate_bezier(points, t),
        }
    }

    fn evaluate_bezier(points: &[BezierPoint], t: f32) -> f32 {
        if points.is_empty() {
            return t;
        }
        // 简化的分段线性近似
        // 在每个控制点之间线性插值
        let n = points.len();
        if n == 1 {
            let p = &points[0];
            // 单控制点二次贝塞尔
            let mt = 1.0 - t;
            return 2.0 * mt * t * p.y + t * t;
        }
        // 多段均匀分布，在对应段做线性插值
        let seg = (t * n as f32).floor() as usize;
        let seg = seg.min(n - 1);
        let seg_t = (t * n as f32) - seg as f32;
        if seg < n - 1 {
            points[seg].y + (points[seg + 1].y - points[seg].y) * seg_t
        } else {
            points[seg].y
        }
    }
}

// ========== 骨头数据 ==========

/// 骨头变换继承模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoneTransformMode {
    /// 继承全部变换（位置、旋转、缩放）
    Normal,
    /// 只继承位置和旋转（不继承缩放）
    OnlyTranslation,
    /// 不继承旋转（全局旋转）
    NoRotation,
    /// 不继承缩放（全局缩放）
    NoScale,
    /// 不继承旋转和缩放
    NoRotationOrReflection,
}

/// 骨头静态数据（来自 JSON/二进制 DragonBones 数据）
#[derive(Debug, Clone)]
pub struct DBBoneData {
    /// 骨头名称
    pub name: String,
    /// 父骨头名称
    pub parent_name: Option<String>,
    /// 长度（用于编辑器显示）
    pub length: f32,
    /// 变换继承模式
    pub transform_mode: BoneTransformMode,
    /// 默认变换（相对父骨头）
    pub rest_pose: Transform,
    /// 用户数据
    pub user_data: Option<String>,
}

impl DBBoneData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent_name: None,
            length: 0.0,
            transform_mode: BoneTransformMode::Normal,
            rest_pose: Transform::identity(),
            user_data: None,
        }
    }
}

/// 骨头实例（运行时状态）
#[derive(Debug, Clone)]
pub struct DBBone {
    /// 对应的骨头数据
    pub data: DBBoneData,
    /// 当前相对父骨头的变换（动画驱动）
    pub local_transform: Transform,
    /// 世界变换（计算得到）
    pub world_transform: Transform,
    /// 是否脏（需要重新计算世界变换）
    pub dirty: bool,
    /// 父骨头索引（在骨架 bones 列表中）
    pub parent_index: Option<usize>,
    /// 子骨头索引
    pub child_indices: Vec<usize>,
}

impl DBBone {
    pub fn new(data: DBBoneData) -> Self {
        let local = data.rest_pose;
        Self {
            data,
            local_transform: local,
            world_transform: Transform::identity(),
            dirty: true,
            parent_index: None,
            child_indices: Vec::new(),
        }
    }

    /// 重置到 rest pose
    pub fn reset_to_setup_pose(&mut self) {
        self.local_transform = self.data.rest_pose;
        self.dirty = true;
    }

    /// 更新世界变换
    pub fn update_world_transform(&mut self, parent_world: Option<&Transform>) {
        let parent = parent_world.unwrap_or(&Transform::identity()).clone();
        self.world_transform = parent.combine(&self.local_transform);
        self.dirty = false;
    }
}

// ========== 插槽数据 ==========

/// 混合模式（与 DragonBones 规范一致）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DBBlendMode {
    Normal,
    Add,
    Subtract,
    Multiply,
    Screen,
    Overlay,
    Erase,
}

impl Default for DBBlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// 插槽静态数据
#[derive(Debug, Clone)]
pub struct DBSlotData {
    pub name: String,
    /// 所属骨头名称
    pub parent_bone_name: String,
    /// 默认附件名称
    pub default_attachment: Option<String>,
    /// 显示顺序（Z轴排序）
    pub display_index: i32,
    /// 默认颜色变换
    pub color: ColorTransform,
    /// 混合模式
    pub blend_mode: DBBlendMode,
}

impl DBSlotData {
    pub fn new(name: &str, parent_bone_name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent_bone_name: parent_bone_name.to_string(),
            default_attachment: None,
            display_index: 0,
            color: ColorTransform::identity(),
            blend_mode: DBBlendMode::Normal,
        }
    }
}

/// 插槽运行时状态
#[derive(Debug, Clone)]
pub struct DBSlot {
    pub data: DBSlotData,
    /// 当前颜色变换
    pub color: ColorTransform,
    /// 当前附件名称
    pub current_attachment: Option<String>,
    /// 父骨头索引
    pub bone_index: usize,
    /// 是否可见
    pub visible: bool,
}

impl DBSlot {
    pub fn new(data: DBSlotData, bone_index: usize) -> Self {
        let current_attachment = data.default_attachment.clone();
        Self {
            data,
            color: ColorTransform::identity(),
            current_attachment,
            bone_index,
            visible: true,
        }
    }

    pub fn reset_to_setup_pose(&mut self) {
        self.color = self.data.color;
        self.current_attachment = self.data.default_attachment.clone();
        self.visible = true;
    }
}

// ========== 附件数据 ==========

/// 附件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DBAttachmentType {
    /// 图片区域
    Region,
    /// 网格
    Mesh,
    /// 边界盒（碰撞检测）
    BoundingBox,
    /// 路径（骨头绑定路径）
    Path,
    /// 点（骨头绑定点）
    Point,
    /// 不可见
    None,
}

/// 区域附件（对应一张图片）
#[derive(Debug, Clone)]
pub struct RegionAttachment {
    pub name: String,
    pub texture_name: String,
    /// 局部变换（相对插槽骨头）
    pub transform: Transform,
    pub width: f32,
    pub height: f32,
    pub color: ColorTransform,
    /// UV 坐标（0.0-1.0）
    pub uv_min_x: f32,
    pub uv_min_y: f32,
    pub uv_max_x: f32,
    pub uv_max_y: f32,
}

impl RegionAttachment {
    pub fn new(name: &str, texture_name: &str) -> Self {
        Self {
            name: name.to_string(),
            texture_name: texture_name.to_string(),
            transform: Transform::identity(),
            width: 0.0,
            height: 0.0,
            color: ColorTransform::identity(),
            uv_min_x: 0.0,
            uv_min_y: 0.0,
            uv_max_x: 1.0,
            uv_max_y: 1.0,
        }
    }
}

/// 网格附件（自由变形）
#[derive(Debug, Clone)]
pub struct MeshAttachment {
    pub name: String,
    pub texture_name: String,
    /// 网格顶点（局部坐标，x/y交替）
    pub vertices: Vec<f32>,
    /// UV 坐标（u/v交替）
    pub uvs: Vec<f32>,
    /// 三角形索引
    pub triangles: Vec<u16>,
    /// 受骨头权重影响（骨头索引/权重对）
    pub bone_weights: Vec<(usize, f32)>,
    pub color: ColorTransform,
}

impl MeshAttachment {
    pub fn new(name: &str, texture_name: &str) -> Self {
        Self {
            name: name.to_string(),
            texture_name: texture_name.to_string(),
            vertices: Vec::new(),
            uvs: Vec::new(),
            triangles: Vec::new(),
            bone_weights: Vec::new(),
            color: ColorTransform::identity(),
        }
    }
}

/// 附件枚举
#[derive(Debug, Clone)]
pub enum DBAttachment {
    Region(RegionAttachment),
    Mesh(MeshAttachment),
    BoundingBox { name: String, vertices: Vec<f32> },
    None,
}

impl DBAttachment {
    pub fn attachment_type(&self) -> DBAttachmentType {
        match self {
            DBAttachment::Region(_) => DBAttachmentType::Region,
            DBAttachment::Mesh(_) => DBAttachmentType::Mesh,
            DBAttachment::BoundingBox { .. } => DBAttachmentType::BoundingBox,
            DBAttachment::None => DBAttachmentType::None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            DBAttachment::Region(r) => &r.name,
            DBAttachment::Mesh(m) => &m.name,
            DBAttachment::BoundingBox { name, .. } => name,
            DBAttachment::None => "",
        }
    }
}

// ========== 皮肤 ==========

/// 皮肤（包含一组插槽附件替换）
#[derive(Debug, Clone)]
pub struct DBSkin {
    pub name: String,
    /// slot_name -> (attachment_name -> attachment)
    pub slots: HashMap<String, HashMap<String, DBAttachment>>,
}

impl DBSkin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            slots: HashMap::new(),
        }
    }

    pub fn add_attachment(&mut self, slot_name: &str, attachment_name: &str, attachment: DBAttachment) {
        self.slots
            .entry(slot_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(attachment_name.to_string(), attachment);
    }

    pub fn get_attachment(&self, slot_name: &str, attachment_name: &str) -> Option<&DBAttachment> {
        self.slots.get(slot_name)?.get(attachment_name)
    }

    pub fn get_slot_attachments(&self, slot_name: &str) -> Option<&HashMap<String, DBAttachment>> {
        self.slots.get(slot_name)
    }
}

// ========== IK 约束 ==========

/// IK 约束类型（正向/反向）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IKChainType {
    /// 单骨头 IK（直接指向目标）
    Single,
    /// 两骨头 IK（手臂/腿部常用）
    Double,
}

/// IK 约束数据
#[derive(Debug, Clone)]
pub struct IKConstraintData {
    pub name: String,
    /// 目标骨头名称
    pub target_bone: String,
    /// 约束骨头链
    pub bones: Vec<String>,
    /// 弯曲方向 (1.0 或 -1.0)
    pub bend_direction: f32,
    /// 约束权重 (0.0-1.0)
    pub mix: f32,
    /// 链类型
    pub chain_type: IKChainType,
    /// 是否压缩骨头
    pub compress: bool,
    /// 是否拉伸骨头
    pub stretch: bool,
    /// 是否统一骨头缩放
    pub uniform: bool,
}

impl IKConstraintData {
    pub fn new(name: &str, target_bone: &str) -> Self {
        Self {
            name: name.to_string(),
            target_bone: target_bone.to_string(),
            bones: Vec::new(),
            bend_direction: 1.0,
            mix: 1.0,
            chain_type: IKChainType::Single,
            compress: false,
            stretch: false,
            uniform: false,
        }
    }
}

/// IK 约束运行时
#[derive(Debug, Clone)]
pub struct IKConstraint {
    pub data: IKConstraintData,
    /// 当前混合权重
    pub mix: f32,
    /// 当前弯曲方向
    pub bend_direction: f32,
    pub enabled: bool,
}

impl IKConstraint {
    pub fn new(data: IKConstraintData) -> Self {
        let mix = data.mix;
        let bend = data.bend_direction;
        Self {
            data,
            mix,
            bend_direction: bend,
            enabled: true,
        }
    }

    /// 求解 1 骨头 IK（骨头直接指向目标）
    pub fn solve_single(bone: &mut DBBone, target_x: f32, target_y: f32, alpha: f32) {
        let rot = (target_y - bone.world_transform.y).atan2(target_x - bone.world_transform.x);
        let diff = (rot - bone.world_transform.rotation) * alpha;
        bone.local_transform.rotation += diff;
        bone.dirty = true;
    }

    /// 求解 2 骨头 IK（FABRIK 简化版，余弦法）
    pub fn solve_double(
        parent: &mut DBBone,
        child: &mut DBBone,
        target_x: f32,
        target_y: f32,
        alpha: f32,
        bend_dir: f32,
    ) {
        let px = parent.world_transform.x;
        let py = parent.world_transform.y;
        let l1 = parent.data.length.max(1e-4);
        let l2 = child.data.length.max(1e-4);

        let dx = target_x - px;
        let dy = target_y - py;
        let dist = (dx * dx + dy * dy).sqrt();

        // 用余弦定律计算角度
        let cos_angle = ((l1 * l1 + dist * dist - l2 * l2) / (2.0 * l1 * dist)).clamp(-1.0, 1.0);
        let angle_to_target = dy.atan2(dx);
        let angle1 = angle_to_target - cos_angle.acos() * bend_dir;

        let angle_child_cos = ((l1 * l1 + l2 * l2 - dist * dist) / (2.0 * l1 * l2)).clamp(-1.0, 1.0);
        let angle2 = std::f32::consts::PI - angle_child_cos.acos() * bend_dir;

        parent.local_transform.rotation += (angle1 - parent.world_transform.rotation) * alpha;
        child.local_transform.rotation = angle2 * alpha;
        parent.dirty = true;
        child.dirty = true;
    }
}

// ========== 动画关键帧 ==========

/// 关键帧基础
#[derive(Debug, Clone)]
pub struct DBKeyframe<T: Clone> {
    pub time: f32,
    pub value: T,
    pub tween: TweenType,
}

impl<T: Clone> DBKeyframe<T> {
    pub fn new(time: f32, value: T) -> Self {
        Self { time, value, tween: TweenType::Linear }
    }

    pub fn with_tween(time: f32, value: T, tween: TweenType) -> Self {
        Self { time, value, tween }
    }
}

/// 时间线类型
#[derive(Debug, Clone)]
pub enum DBTimeline {
    /// 骨头变换时间线
    BoneTranslate {
        bone_name: String,
        frames: Vec<DBKeyframe<(f32, f32)>>, // (x, y)
    },
    BoneRotate {
        bone_name: String,
        frames: Vec<DBKeyframe<f32>>, // 旋转角度（度）
    },
    BoneScale {
        bone_name: String,
        frames: Vec<DBKeyframe<(f32, f32)>>, // (sx, sy)
    },
    BoneSkew {
        bone_name: String,
        frames: Vec<DBKeyframe<(f32, f32)>>, // (skew_x, skew_y)
    },
    /// 插槽时间线
    SlotAttachment {
        slot_name: String,
        frames: Vec<DBKeyframe<Option<String>>>, // attachment_name
    },
    SlotColor {
        slot_name: String,
        frames: Vec<DBKeyframe<ColorTransform>>,
    },
    SlotAlpha {
        slot_name: String,
        frames: Vec<DBKeyframe<f32>>,
    },
    SlotDisplay {
        slot_name: String,
        frames: Vec<DBKeyframe<i32>>, // 显示列表索引
    },
    /// IK 约束时间线
    IKConstraint {
        ik_name: String,
        frames: Vec<DBKeyframe<(f32, f32, bool)>>, // (mix, bend_direction, compress)
    },
    /// 动画事件时间线
    Event {
        frames: Vec<DBKeyframe<DBEventData>>,
    },
}

/// 动画事件数据
#[derive(Debug, Clone)]
pub struct DBEventData {
    pub name: String,
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: String,
}

impl DBEventData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            int_value: 0,
            float_value: 0.0,
            string_value: String::new(),
        }
    }
}

// ========== 动画数据 ==========

/// 单个动画数据
#[derive(Debug, Clone)]
pub struct DBAnimationData {
    pub name: String,
    /// 动画时长（秒）
    pub duration: f32,
    /// 帧率（帧/秒）
    pub frame_rate: f32,
    /// 播放次数（0=无限循环，1=播放一次，etc.）
    pub play_times: i32,
    /// 所有时间线
    pub timelines: Vec<DBTimeline>,
}

impl DBAnimationData {
    pub fn new(name: &str, duration: f32) -> Self {
        Self {
            name: name.to_string(),
            duration: duration.max(0.0),
            frame_rate: 24.0,
            play_times: 1,
            timelines: Vec::new(),
        }
    }

    pub fn add_timeline(&mut self, timeline: DBTimeline) {
        self.timelines.push(timeline);
    }
}

// ========== 骨架数据 ==========

/// 骨架数据（来自 .json / .dbbin 文件）
#[derive(Debug, Clone)]
pub struct DBArmatureData {
    pub name: String,
    /// 骨架类型
    pub armature_type: ArmatureType,
    /// 帧率
    pub frame_rate: f32,
    /// 骨头列表（拓扑排序，根骨头在前）
    pub bones: Vec<DBBoneData>,
    /// 插槽列表（按显示顺序）
    pub slots: Vec<DBSlotData>,
    /// 皮肤列表
    pub skins: Vec<DBSkin>,
    /// 默认皮肤名
    pub default_skin: String,
    /// 动画列表
    pub animations: Vec<DBAnimationData>,
    /// IK 约束列表
    pub ik_constraints: Vec<IKConstraintData>,
    /// 边界盒（整体）
    pub aabb: DBRect,
    /// 用户数据
    pub user_data: Option<String>,
}

/// 骨架类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmatureType {
    /// 普通骨骼骨架
    Armature,
    /// 影片剪辑（固定帧动画）
    MovieClip,
    /// 舞台（包含多个骨架）
    Stage,
}

impl DBArmatureData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            armature_type: ArmatureType::Armature,
            frame_rate: 24.0,
            bones: Vec::new(),
            slots: Vec::new(),
            skins: Vec::new(),
            default_skin: "default".to_string(),
            animations: Vec::new(),
            ik_constraints: Vec::new(),
            aabb: DBRect::default(),
            user_data: None,
        }
    }

    pub fn get_animation(&self, name: &str) -> Option<&DBAnimationData> {
        self.animations.iter().find(|a| a.name == name)
    }

    pub fn get_skin(&self, name: &str) -> Option<&DBSkin> {
        self.skins.iter().find(|s| s.name == name)
    }

    pub fn get_default_skin(&self) -> Option<&DBSkin> {
        self.get_skin(&self.default_skin)
    }
}

/// DragonBones 数据（对应一整个 .json 文件，可包含多个骨架）
#[derive(Debug, Clone)]
pub struct DragonBonesData {
    pub name: String,
    pub version: String,
    pub armatures: Vec<DBArmatureData>,
}

impl DragonBonesData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "5.5".to_string(),
            armatures: Vec::new(),
        }
    }

    pub fn get_armature(&self, name: &str) -> Option<&DBArmatureData> {
        self.armatures.iter().find(|a| a.name == name)
    }
}

// ========== 动画状态 ==========

/// 动画事件类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DBAnimationEventType {
    /// 动画开始
    Start,
    /// 动画循环结束（每次）
    LoopComplete,
    /// 动画全部完成
    Complete,
    /// 自定义事件
    Custom(String),
    /// 声音事件
    Sound(String),
    /// 帧标签
    FrameEvent(String),
}

/// 动画事件
#[derive(Debug, Clone)]
pub struct DBAnimationEvent {
    pub event_type: DBAnimationEventType,
    pub animation_name: String,
    pub event_data: Option<DBEventData>,
}

/// 动画播放配置
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// 动画名
    pub animation_name: String,
    /// 播放轨道（同一轨道的动画会互相覆盖）
    pub layer: i32,
    /// 播放次数（0=无限，1=一次）
    pub play_times: i32,
    /// 时间缩放
    pub time_scale: f32,
    /// 淡入时间（与上个动画的混合时间）
    pub fade_in_time: f32,
    /// 淡入结束后的权重
    pub weight: f32,
    /// 从哪一帧开始播放（秒）
    pub start_time: f32,
    /// 暂停
    pub pause: bool,
    /// 反向播放
    pub reverse: bool,
    /// 播放到结束后是否自动移除
    pub auto_fade_out_time: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            animation_name: String::new(),
            layer: 0,
            play_times: -1, // -1 = 用动画数据自身的 play_times
            time_scale: 1.0,
            fade_in_time: 0.0,
            weight: 1.0,
            start_time: 0.0,
            pause: false,
            reverse: false,
            auto_fade_out_time: -1.0,
        }
    }
}

/// 动画状态（一个正在播放的动画轨道）
#[derive(Debug, Clone)]
pub struct DBAnimationState {
    /// 配置
    pub config: AnimationConfig,
    /// 当前时间（秒）
    pub current_time: f32,
    /// 当前循环次数
    pub current_play_times: i32,
    /// 淡入/淡出进度（0.0-1.0）
    pub fade_progress: f32,
    /// 是否正在淡出
    pub fading_out: bool,
    /// 是否已完成
    pub is_complete: bool,
    /// 是否暂停
    pub is_paused: bool,
    /// 动画数据名（引用）
    pub animation_name: String,
    /// 当前权重（含淡入淡出）
    pub current_weight: f32,
}

impl DBAnimationState {
    pub fn new(config: AnimationConfig) -> Self {
        let name = config.animation_name.clone();
        Self {
            animation_name: name,
            current_time: config.start_time,
            config,
            current_play_times: 0,
            fade_progress: 0.0,
            fading_out: false,
            is_complete: false,
            is_paused: false,
            current_weight: 0.0,
        }
    }

    /// 计算当前实际权重（考虑淡入/淡出）
    pub fn get_weight(&self) -> f32 {
        let fade = if self.fading_out {
            1.0 - self.fade_progress
        } else {
            self.fade_progress.min(1.0)
        };
        fade * self.config.weight
    }

    /// 更新动画状态
    pub fn advance(&mut self, dt: f32, anim_data: &DBAnimationData) -> Vec<DBAnimationEvent> {
        if self.is_complete || self.is_paused {
            return Vec::new();
        }

        let mut events = Vec::new();
        let time_scale = self.config.time_scale;
        let prev_time = self.current_time;

        // 更新淡入
        if self.config.fade_in_time > 0.0 && self.fade_progress < 1.0 {
            self.fade_progress = (self.fade_progress + dt / self.config.fade_in_time).min(1.0);
        } else {
            self.fade_progress = 1.0;
        }
        self.current_weight = self.get_weight();

        // 更新时间
        let delta = dt * time_scale * if self.config.reverse { -1.0 } else { 1.0 };
        self.current_time += delta;

        let duration = anim_data.duration;
        let play_times = if self.config.play_times < 0 {
            anim_data.play_times
        } else {
            self.config.play_times
        };

        if duration <= 0.0 {
            self.is_complete = true;
            return events;
        }

        // 循环处理
        if self.current_time >= duration {
            self.current_play_times += 1;
            events.push(DBAnimationEvent {
                event_type: DBAnimationEventType::LoopComplete,
                animation_name: self.animation_name.clone(),
                event_data: None,
            });

            if play_times > 0 && self.current_play_times >= play_times {
                self.is_complete = true;
                self.current_time = duration;
                events.push(DBAnimationEvent {
                    event_type: DBAnimationEventType::Complete,
                    animation_name: self.animation_name.clone(),
                    event_data: None,
                });
            } else {
                self.current_time = self.current_time - duration;
            }
        } else if self.current_time < 0.0 {
            self.current_time = 0.0;
        }

        // 触发事件帧
        for timeline in &anim_data.timelines {
            if let DBTimeline::Event { frames } = timeline {
                for frame in frames {
                    if frame.time > prev_time && frame.time <= self.current_time {
                        events.push(DBAnimationEvent {
                            event_type: DBAnimationEventType::Custom(frame.value.name.clone()),
                            animation_name: self.animation_name.clone(),
                            event_data: Some(frame.value.clone()),
                        });
                    }
                }
            }
        }

        let _ = prev_time;
        events
    }
}

// ========== 骨架实例 ==========

/// 骨架运行时实例
pub struct DBArmature {
    /// 骨架数据（只读引用）
    pub armature_data: DBArmatureData,
    /// 骨头列表（运行时可变）
    pub bones: Vec<DBBone>,
    /// 插槽列表
    pub slots: Vec<DBSlot>,
    /// IK 约束
    pub ik_constraints: Vec<IKConstraint>,
    /// 当前皮肤名
    pub current_skin: String,
    /// 动画状态列表（按 layer 排序）
    pub animation_states: Vec<DBAnimationState>,
    /// 是否暂停
    pub paused: bool,
    /// 全局时间缩放
    pub time_scale: f32,
    /// 待触发事件
    pending_events: Vec<DBAnimationEvent>,
    /// 外部事件回调
    event_callbacks: Vec<Box<dyn Fn(&DBAnimationEvent)>>,
}

impl std::fmt::Debug for DBArmature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DBArmature")
            .field("name", &self.armature_data.name)
            .field("bones", &self.bones.len())
            .field("slots", &self.slots.len())
            .field("animation_states", &self.animation_states.len())
            .field("current_skin", &self.current_skin)
            .field("paused", &self.paused)
            .finish()
    }
}

impl DBArmature {
    /// 从骨架数据创建骨架实例
    pub fn new(armature_data: DBArmatureData) -> Self {
        // 构建骨头实例
        let bones: Vec<DBBone> = armature_data.bones.iter()
            .map(|bd| DBBone::new(bd.clone()))
            .collect();

        // 建立骨头父子关系（索引）
        let mut bones = bones;
        let bone_names: Vec<String> = bones.iter().map(|b| b.data.name.clone()).collect();
        for i in 0..bones.len() {
            let parent_name = bones[i].data.parent_name.clone();
            if let Some(pname) = parent_name {
                if let Some(pi) = bone_names.iter().position(|n| *n == pname) {
                    bones[i].parent_index = Some(pi);
                    bones[pi].child_indices.push(i);
                }
            }
        }

        // 构建插槽实例
        let slots: Vec<DBSlot> = armature_data.slots.iter().map(|sd| {
            let bone_idx = bone_names.iter().position(|n| *n == sd.parent_bone_name).unwrap_or(0);
            DBSlot::new(sd.clone(), bone_idx)
        }).collect();

        // 构建 IK 约束
        let ik_constraints: Vec<IKConstraint> = armature_data.ik_constraints.iter()
            .map(|ik| IKConstraint::new(ik.clone()))
            .collect();

        let default_skin = armature_data.default_skin.clone();

        Self {
            armature_data,
            bones,
            slots,
            ik_constraints,
            current_skin: default_skin,
            animation_states: Vec::new(),
            paused: false,
            time_scale: 1.0,
            pending_events: Vec::new(),
            event_callbacks: Vec::new(),
        }
    }

    /// 播放动画
    pub fn play(&mut self, animation_name: &str, play_times: i32) {
        let mut config = AnimationConfig::default();
        config.animation_name = animation_name.to_string();
        config.play_times = play_times;
        config.fade_in_time = 0.0;
        self.play_with_config(config);
    }

    /// 用配置播放动画
    pub fn play_with_config(&mut self, config: AnimationConfig) {
        // 移除同层次的旧动画
        let layer = config.layer;
        self.animation_states.retain(|s| s.config.layer != layer);

        let mut state = DBAnimationState::new(config);
        // 立即开始触发 Start 事件
        self.pending_events.push(DBAnimationEvent {
            event_type: DBAnimationEventType::Start,
            animation_name: state.animation_name.clone(),
            event_data: None,
        });
        state.fade_progress = if state.config.fade_in_time > 0.0 { 0.0 } else { 1.0 };
        self.animation_states.push(state);
        // 按 layer 排序（低 layer 先处理）
        self.animation_states.sort_by_key(|s| s.config.layer);
    }

    /// 停止动画
    pub fn stop(&mut self, animation_name: Option<&str>) {
        if let Some(name) = animation_name {
            self.animation_states.retain(|s| s.animation_name != name);
        } else {
            self.animation_states.clear();
        }
    }

    /// 暂停/恢复
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// 切换皮肤
    pub fn set_skin(&mut self, skin_name: &str) -> bool {
        if self.armature_data.get_skin(skin_name).is_some() {
            self.current_skin = skin_name.to_string();
            true
        } else {
            false
        }
    }

    /// 注册事件回调
    pub fn add_event_listener(&mut self, callback: impl Fn(&DBAnimationEvent) + 'static) {
        self.event_callbacks.push(Box::new(callback));
    }

    /// 获取骨头（按名称）
    pub fn get_bone(&self, name: &str) -> Option<&DBBone> {
        self.bones.iter().find(|b| b.data.name == name)
    }

    /// 获取骨头（可变）
    pub fn get_bone_mut(&mut self, name: &str) -> Option<&mut DBBone> {
        self.bones.iter_mut().find(|b| b.data.name == name)
    }

    /// 获取插槽（按名称）
    pub fn get_slot(&self, name: &str) -> Option<&DBSlot> {
        self.slots.iter().find(|s| s.data.name == name)
    }

    /// 获取插槽（可变）
    pub fn get_slot_mut(&mut self, name: &str) -> Option<&mut DBSlot> {
        self.slots.iter_mut().find(|s| s.data.name == name)
    }

    /// 获取附件
    pub fn get_slot_attachment(&self, slot_name: &str) -> Option<&DBAttachment> {
        let slot = self.get_slot(slot_name)?;
        let attachment_name = slot.current_attachment.as_ref()?;
        let skin = self.armature_data.get_skin(&self.current_skin)?;
        skin.get_attachment(slot_name, attachment_name)
    }

    /// 重置到初始姿势
    pub fn reset_to_setup_pose(&mut self) {
        for bone in &mut self.bones {
            bone.reset_to_setup_pose();
        }
        for slot in &mut self.slots {
            slot.reset_to_setup_pose();
        }
        self.update_bones();
    }

    /// 更新骨头世界变换（从根到叶）
    pub fn update_bones(&mut self) {
        for i in 0..self.bones.len() {
            if self.bones[i].dirty {
                let parent_world = self.bones[i].parent_index.map(|pi| {
                    self.bones[pi].world_transform
                });
                self.bones[i].update_world_transform(parent_world.as_ref());
            }
        }
    }

    /// 应用当前时间线到骨头/插槽
    fn apply_timelines(&mut self, anim_data: &DBAnimationData, state: &DBAnimationState) {
        let t = state.current_time;
        let weight = state.current_weight;

        for timeline in &anim_data.timelines {
            match timeline {
                DBTimeline::BoneRotate { bone_name, frames } => {
                    let rot = Self::interpolate_frames(frames, t);
                    if let Some(bone) = self.bones.iter_mut().find(|b| &b.data.name == bone_name) {
                        bone.local_transform.rotation = bone.data.rest_pose.rotation + rot * weight;
                        bone.dirty = true;
                    }
                }
                DBTimeline::BoneTranslate { bone_name, frames } => {
                    let (x, y) = Self::interpolate_frames(frames, t);
                    if let Some(bone) = self.bones.iter_mut().find(|b| &b.data.name == bone_name) {
                        bone.local_transform.x = bone.data.rest_pose.x + x * weight;
                        bone.local_transform.y = bone.data.rest_pose.y + y * weight;
                        bone.dirty = true;
                    }
                }
                DBTimeline::BoneScale { bone_name, frames } => {
                    let (sx, sy) = Self::interpolate_frames(frames, t);
                    if let Some(bone) = self.bones.iter_mut().find(|b| &b.data.name == bone_name) {
                        bone.local_transform.scale_x = bone.data.rest_pose.scale_x + (sx - 1.0) * weight;
                        bone.local_transform.scale_y = bone.data.rest_pose.scale_y + (sy - 1.0) * weight;
                        bone.dirty = true;
                    }
                }
                DBTimeline::SlotAttachment { slot_name, frames } => {
                    if let Some(frame) = frames.iter().rev().find(|f| f.time <= t) {
                        if let Some(slot) = self.slots.iter_mut().find(|s| &s.data.name == slot_name) {
                            slot.current_attachment = frame.value.clone();
                        }
                    }
                }
                DBTimeline::SlotAlpha { slot_name, frames } => {
                    let alpha = Self::interpolate_frames(frames, t);
                    if let Some(slot) = self.slots.iter_mut().find(|s| &s.data.name == slot_name) {
                        slot.color.a_multiplier = alpha;
                    }
                }
                _ => {}
            }
        }
    }

    /// 在 frames 中插值
    fn interpolate_frames<T>(frames: &[DBKeyframe<T>], t: f32) -> T
    where
        T: Clone + Interpolatable,
    {
        if frames.is_empty() {
            return T::default_value();
        }
        if frames.len() == 1 || t <= frames[0].time {
            return frames[0].value.clone();
        }
        if t >= frames.last().unwrap().time {
            return frames.last().unwrap().value.clone();
        }
        for i in 0..frames.len() - 1 {
            let f0 = &frames[i];
            let f1 = &frames[i + 1];
            if t >= f0.time && t <= f1.time {
                let seg_dur = f1.time - f0.time;
                let raw_t = if seg_dur > 1e-6 { (t - f0.time) / seg_dur } else { 0.0 };
                let eased_t = f0.tween.evaluate(raw_t);
                return T::lerp_value(&f0.value, &f1.value, eased_t);
            }
        }
        frames.last().unwrap().value.clone()
    }

    /// 主更新方法
    pub fn update(&mut self, dt: f32) {
        if self.paused {
            return;
        }
        let effective_dt = dt * self.time_scale;

        // 收集动画数据名（避免借用冲突）
        let anim_names: Vec<String> = self.animation_states.iter()
            .map(|s| s.animation_name.clone())
            .collect();

        let mut all_events = Vec::new();

        for (i, anim_name) in anim_names.iter().enumerate() {
            if let Some(anim_data) = self.armature_data.get_animation(anim_name).cloned() {
                // 先应用时间线
                let state = &self.animation_states[i];
                let state_clone = state.clone();
                self.apply_timelines(&anim_data, &state_clone);

                // 再 advance
                let events = self.animation_states[i].advance(effective_dt, &anim_data);
                all_events.extend(events);
            }
        }

        // 移除已完成的动画状态
        self.animation_states.retain(|s| !s.is_complete);

        // 累积待发事件
        self.pending_events.extend(all_events);

        // 触发事件回调
        for event in self.pending_events.drain(..) {
            for cb in &self.event_callbacks {
                cb(&event);
            }
        }

        // 更新骨头世界变换
        self.update_bones();
    }

    /// 获取当前正在播放的动画名（第一个激活的）
    pub fn get_current_animation(&self) -> Option<&str> {
        self.animation_states.first().map(|s| s.animation_name.as_str())
    }

    /// 判断是否正在播放指定动画
    pub fn is_playing(&self, animation_name: &str) -> bool {
        self.animation_states.iter().any(|s| s.animation_name == animation_name && !s.is_complete)
    }

    /// 计算骨架包围盒（世界空间）
    pub fn get_bounding_box(&self) -> DBRect {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for bone in &self.bones {
            let wx = bone.world_transform.x;
            let wy = bone.world_transform.y;
            min_x = min_x.min(wx);
            min_y = min_y.min(wy);
            max_x = max_x.max(wx);
            max_y = max_y.max(wy);
        }

        if min_x == f32::MAX {
            return self.armature_data.aabb;
        }

        DBRect {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

// ========== 插值辅助 trait ==========

/// 可插值 trait（用于关键帧插值）
pub trait Interpolatable: Sized {
    fn default_value() -> Self;
    fn lerp_value(a: &Self, b: &Self, t: f32) -> Self;
}

impl Interpolatable for f32 {
    fn default_value() -> Self { 0.0 }
    fn lerp_value(a: &f32, b: &f32, t: f32) -> f32 { a + (b - a) * t }
}

impl Interpolatable for (f32, f32) {
    fn default_value() -> Self { (0.0, 0.0) }
    fn lerp_value(a: &(f32, f32), b: &(f32, f32), t: f32) -> (f32, f32) {
        (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
    }
}

impl Interpolatable for Option<String> {
    fn default_value() -> Self { None }
    fn lerp_value(a: &Option<String>, _b: &Option<String>, t: f32) -> Option<String> {
        if t < 0.5 { a.clone() } else { _b.clone() }
    }
}

impl Interpolatable for i32 {
    fn default_value() -> Self { 0 }
    fn lerp_value(a: &i32, b: &i32, t: f32) -> i32 {
        (*a as f32 + (*b as f32 - *a as f32) * t) as i32
    }
}

impl Interpolatable for ColorTransform {
    fn default_value() -> Self { ColorTransform::identity() }
    fn lerp_value(a: &ColorTransform, b: &ColorTransform, t: f32) -> ColorTransform {
        a.lerp(b, t)
    }
}

impl Interpolatable for (f32, f32, bool) {
    fn default_value() -> Self { (1.0, 1.0, false) }
    fn lerp_value(a: &(f32, f32, bool), b: &(f32, f32, bool), t: f32) -> (f32, f32, bool) {
        (
            a.0 + (b.0 - a.0) * t,
            a.1 + (b.1 - a.1) * t,
            if t < 0.5 { a.2 } else { b.2 },
        )
    }
}

impl Interpolatable for DBEventData {
    fn default_value() -> Self { DBEventData::new("") }
    fn lerp_value(a: &DBEventData, _b: &DBEventData, t: f32) -> DBEventData {
        if t < 0.5 { a.clone() } else { _b.clone() }
    }
}

// ========== DragonBones 工厂 ==========

/// DragonBones 工厂 - 管理数据和创建骨架
pub struct DragonBonesFactory {
    data_store: HashMap<String, DragonBonesData>,
}

impl Default for DragonBonesFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl DragonBonesFactory {
    pub fn new() -> Self {
        Self { data_store: HashMap::new() }
    }

    /// 注册 DragonBones 数据
    pub fn parse_dragon_bones_data(&mut self, data: DragonBonesData) {
        self.data_store.insert(data.name.clone(), data);
    }

    /// 构建骨架实例
    pub fn build_armature(&self, data_name: &str, armature_name: &str) -> Option<DBArmature> {
        let db_data = self.data_store.get(data_name)?;
        let armature_data = db_data.get_armature(armature_name)?.clone();
        Some(DBArmature::new(armature_data))
    }

    /// 获取所有已注册数据名
    pub fn get_data_names(&self) -> Vec<&str> {
        self.data_store.keys().map(|s| s.as_str()).collect()
    }

    /// 移除数据
    pub fn remove_dragon_bones_data(&mut self, data_name: &str) {
        self.data_store.remove(data_name);
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    fn create_simple_armature_data() -> DBArmatureData {
        let mut arm = DBArmatureData::new("hero");
        // 根骨头
        let mut root = DBBoneData::new("root");
        root.length = 50.0;
        arm.bones.push(root);
        // 子骨头
        let mut spine = DBBoneData::new("spine");
        spine.parent_name = Some("root".to_string());
        spine.length = 40.0;
        arm.bones.push(spine);

        // 插槽
        let slot = DBSlotData::new("body_slot", "root");
        arm.slots.push(slot);

        // 皮肤
        let mut skin = DBSkin::new("default");
        let att = DBAttachment::Region(RegionAttachment::new("body", "body_texture"));
        skin.add_attachment("body_slot", "body", att);
        arm.skins.push(skin);

        // 动画
        let mut anim = DBAnimationData::new("idle", 2.0);
        anim.frame_rate = 24.0;
        anim.play_times = 0; // 无限循环
        anim.add_timeline(DBTimeline::BoneRotate {
            bone_name: "root".to_string(),
            frames: vec![
                DBKeyframe::new(0.0, 0.0f32),
                DBKeyframe::new(1.0, 30.0f32),
                DBKeyframe::new(2.0, 0.0f32),
            ],
        });
        arm.animations.push(anim);

        arm
    }

    #[test]
    fn test_transform_default() {
        let t = Transform::default();
        assert_eq!(t.x, 0.0);
        assert_eq!(t.scale_x, 1.0);
    }

    #[test]
    fn test_transform_lerp() {
        let a = Transform { x: 0.0, y: 0.0, ..Default::default() };
        let b = Transform { x: 10.0, y: 20.0, ..Default::default() };
        let m = a.lerp(&b, 0.5);
        assert!((m.x - 5.0).abs() < 1e-5);
        assert!((m.y - 10.0).abs() < 1e-5);
    }

    #[test]
    fn test_tween_linear() {
        let t = TweenType::Linear;
        assert!((t.evaluate(0.5) - 0.5).abs() < 1e-5);
        assert!((t.evaluate(1.0) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_tween_none() {
        let t = TweenType::None;
        assert_eq!(t.evaluate(0.4), 0.0);
        assert_eq!(t.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_bone_data_new() {
        let b = DBBoneData::new("arm");
        assert_eq!(b.name, "arm");
        assert!(b.parent_name.is_none());
    }

    #[test]
    fn test_bone_reset_to_setup_pose() {
        let mut data = DBBoneData::new("test");
        data.rest_pose.rotation = 45.0;
        let mut bone = DBBone::new(data);
        bone.local_transform.rotation = 90.0;
        bone.reset_to_setup_pose();
        assert!((bone.local_transform.rotation - 45.0).abs() < 1e-5);
    }

    #[test]
    fn test_skin_attachment() {
        let mut skin = DBSkin::new("default");
        let att = DBAttachment::Region(RegionAttachment::new("head", "head_tex"));
        skin.add_attachment("head_slot", "head", att);
        assert!(skin.get_attachment("head_slot", "head").is_some());
        assert!(skin.get_attachment("head_slot", "missing").is_none());
    }

    #[test]
    fn test_armature_data_get_animation() {
        let arm = create_simple_armature_data();
        assert!(arm.get_animation("idle").is_some());
        assert!(arm.get_animation("run").is_none());
    }

    #[test]
    fn test_armature_new() {
        let data = create_simple_armature_data();
        let armature = DBArmature::new(data);
        assert_eq!(armature.bones.len(), 2);
        assert_eq!(armature.slots.len(), 1);
    }

    #[test]
    fn test_armature_play() {
        let data = create_simple_armature_data();
        let mut armature = DBArmature::new(data);
        armature.play("idle", 1);
        assert!(armature.is_playing("idle"));
    }

    #[test]
    fn test_armature_stop() {
        let data = create_simple_armature_data();
        let mut armature = DBArmature::new(data);
        armature.play("idle", 0);
        armature.stop(Some("idle"));
        assert!(!armature.is_playing("idle"));
    }

    #[test]
    fn test_armature_update() {
        let data = create_simple_armature_data();
        let mut armature = DBArmature::new(data);
        armature.play("idle", 0);
        // 更新多帧
        for _ in 0..10 {
            armature.update(1.0 / 60.0);
        }
        assert!(armature.is_playing("idle")); // 无限循环
    }

    #[test]
    fn test_armature_event_callback() {
        use std::sync::{Arc, Mutex};
        let events = Arc::new(Mutex::new(Vec::<String>::new()));
        let events_clone = events.clone();

        let data = create_simple_armature_data();
        let mut armature = DBArmature::new(data);
        armature.add_event_listener(move |event| {
            events_clone.lock().unwrap().push(format!("{:?}", event.event_type));
        });
        armature.play("idle", 1);
        // 推进超过动画时长，触发 Complete
        for _ in 0..200 {
            armature.update(1.0 / 60.0);
        }
        let ev = events.lock().unwrap();
        assert!(ev.iter().any(|e| e.contains("Start")));
    }

    #[test]
    fn test_armature_set_skin() {
        let mut data = create_simple_armature_data();
        let skin2 = DBSkin::new("warrior");
        data.skins.push(skin2);
        let mut armature = DBArmature::new(data);
        assert!(armature.set_skin("warrior"));
        assert!(!armature.set_skin("nonexistent"));
    }

    #[test]
    fn test_armature_get_bone() {
        let data = create_simple_armature_data();
        let armature = DBArmature::new(data);
        assert!(armature.get_bone("root").is_some());
        assert!(armature.get_bone("spine").is_some());
        assert!(armature.get_bone("missing").is_none());
    }

    #[test]
    fn test_armature_get_slot() {
        let data = create_simple_armature_data();
        let armature = DBArmature::new(data);
        assert!(armature.get_slot("body_slot").is_some());
    }

    #[test]
    fn test_armature_bounding_box() {
        let data = create_simple_armature_data();
        let mut armature = DBArmature::new(data);
        armature.update_bones();
        let bb = armature.get_bounding_box();
        assert!(bb.width >= 0.0);
        assert!(bb.height >= 0.0);
    }

    #[test]
    fn test_armature_reset_to_setup_pose() {
        let data = create_simple_armature_data();
        let mut armature = DBArmature::new(data);
        armature.bones[0].local_transform.rotation = 90.0;
        armature.reset_to_setup_pose();
        assert!((armature.bones[0].local_transform.rotation - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_factory_build_armature() {
        let mut factory = DragonBonesFactory::new();
        let mut db_data = DragonBonesData::new("hero_data");
        db_data.armatures.push(create_simple_armature_data());
        factory.parse_dragon_bones_data(db_data);
        let armature = factory.build_armature("hero_data", "hero");
        assert!(armature.is_some());
    }

    #[test]
    fn test_factory_missing_data() {
        let factory = DragonBonesFactory::new();
        let result = factory.build_armature("nonexistent", "hero");
        assert!(result.is_none());
    }

    #[test]
    fn test_color_transform_lerp() {
        let a = ColorTransform::identity();
        let b = ColorTransform { a_multiplier: 0.0, ..Default::default() };
        let m = a.lerp(&b, 0.5);
        assert!((m.a_multiplier - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_ik_constraint_single() {
        let data = DBBoneData::new("arm");
        let mut bone = DBBone::new(data);
        IKConstraint::solve_single(&mut bone, 100.0, 0.0, 1.0);
        // 应该朝向目标旋转
        assert!((bone.local_transform.rotation - 0.0).abs() < 0.1); // atan2(0, 100) = 0
    }

    #[test]
    fn test_animation_config_default() {
        let config = AnimationConfig::default();
        assert!((config.time_scale - 1.0).abs() < 1e-5);
        assert!((config.weight - 1.0).abs() < 1e-5);
        assert_eq!(config.layer, 0);
    }

    #[test]
    fn test_db_animation_state_advance() {
        let mut state = DBAnimationState::new(AnimationConfig {
            animation_name: "idle".to_string(),
            play_times: 1,
            ..Default::default()
        });
        let anim = DBAnimationData::new("idle", 2.0);
        // 推进超过时长
        let events = state.advance(3.0, &anim);
        assert!(state.is_complete);
        assert!(events.iter().any(|e| e.event_type == DBAnimationEventType::Complete));
    }

    #[test]
    fn test_interpolate_frames_empty() {
        let frames: Vec<DBKeyframe<f32>> = vec![];
        let result = DBArmature::interpolate_frames(&frames, 0.5);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_interpolate_frames_single() {
        let frames = vec![DBKeyframe::new(0.0, 42.0f32)];
        let result = DBArmature::interpolate_frames(&frames, 0.5);
        assert!((result - 42.0).abs() < 1e-5);
    }

    #[test]
    fn test_interpolate_frames_two_points() {
        let frames = vec![
            DBKeyframe::new(0.0, 0.0f32),
            DBKeyframe::new(1.0, 100.0f32),
        ];
        let result = DBArmature::interpolate_frames(&frames, 0.5);
        assert!((result - 50.0).abs() < 0.01);
    }
}
