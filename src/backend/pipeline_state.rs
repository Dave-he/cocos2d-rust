/// 渲染管线状态（PipelineState）
///
/// 对应 cocos2d-x 后端的 PipelineDescriptor 和 BlendDescriptor，
/// 描述一次绘制调用所需的所有 GPU 状态，不依赖具体图形 API。
///
/// 设计原则：
/// - 纯数据结构，序列化/反序列化友好
/// - 与 OpenGL/Vulkan/Metal 无关
/// - 可以在 CPU 侧做相等性比较，用于管线缓存

// ─── 着色器描述 ──────────────────────────────────────────────────

/// 顶点属性格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexFormat {
    Float,
    Float2,
    Float3,
    Float4,
    Uint8x4,
    Short2,
    Short4,
}

impl VertexFormat {
    pub fn size_bytes(&self) -> u32 {
        match self {
            VertexFormat::Float    => 4,
            VertexFormat::Float2   => 8,
            VertexFormat::Float3   => 12,
            VertexFormat::Float4   => 16,
            VertexFormat::Uint8x4  => 4,
            VertexFormat::Short2   => 4,
            VertexFormat::Short4   => 8,
        }
    }

    pub fn component_count(&self) -> u32 {
        match self {
            VertexFormat::Float    => 1,
            VertexFormat::Float2   => 2,
            VertexFormat::Float3   => 3,
            VertexFormat::Float4   => 4,
            VertexFormat::Uint8x4  => 4,
            VertexFormat::Short2   => 2,
            VertexFormat::Short4   => 4,
        }
    }
}

/// 顶点属性描述
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexAttribute {
    /// 属性名称（如 "a_position"）
    pub name: String,
    /// 绑定槽位
    pub location: u32,
    pub format: VertexFormat,
    /// 在顶点结构中的字节偏移
    pub offset: u32,
    /// 是否归一化（int → float）
    pub normalized: bool,
}

impl VertexAttribute {
    pub fn new(name: &str, location: u32, format: VertexFormat, offset: u32) -> Self {
        Self {
            name: name.to_string(),
            location,
            format,
            offset,
            normalized: false,
        }
    }

    pub fn normalized(mut self) -> Self {
        self.normalized = true;
        self
    }
}

/// 顶点布局（VBO 的内存布局）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexLayout {
    pub attributes: Vec<VertexAttribute>,
    pub stride: u32,
}

impl VertexLayout {
    pub fn new() -> Self {
        Self { attributes: Vec::new(), stride: 0 }
    }

    /// 添加属性并自动计算步长
    pub fn add_attribute(&mut self, name: &str, location: u32, format: VertexFormat) -> &mut Self {
        let offset = self.stride;
        self.stride += format.size_bytes();
        self.attributes.push(VertexAttribute::new(name, location, format, offset));
        self
    }

    pub fn get_stride(&self) -> u32 { self.stride }
    pub fn get_attribute_count(&self) -> usize { self.attributes.len() }
}

impl Default for VertexLayout {
    fn default() -> Self { Self::new() }
}

// ─── 混合模式 ────────────────────────────────────────────────────

/// 混合因子
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    Zero,
    One,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
}

/// 混合运算
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

/// 混合描述符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlendDescriptor {
    pub enabled: bool,
    pub rgb_op:   BlendOp,
    pub alpha_op: BlendOp,
    pub src_rgb:   BlendFactor,
    pub dst_rgb:   BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
}

impl BlendDescriptor {
    /// 不透明（默认）
    pub const OPAQUE: Self = Self {
        enabled: false,
        rgb_op: BlendOp::Add, alpha_op: BlendOp::Add,
        src_rgb: BlendFactor::One,  dst_rgb: BlendFactor::Zero,
        src_alpha: BlendFactor::One, dst_alpha: BlendFactor::Zero,
    };

    /// 预乘 Alpha
    pub const ALPHA_PREMULTIPLIED: Self = Self {
        enabled: true,
        rgb_op: BlendOp::Add, alpha_op: BlendOp::Add,
        src_rgb: BlendFactor::One,      dst_rgb: BlendFactor::OneMinusSrcAlpha,
        src_alpha: BlendFactor::One,    dst_alpha: BlendFactor::OneMinusSrcAlpha,
    };

    /// 非预乘 Alpha（cocos2d-x 默认）
    pub const ALPHA_NON_PREMULTIPLIED: Self = Self {
        enabled: true,
        rgb_op: BlendOp::Add, alpha_op: BlendOp::Add,
        src_rgb: BlendFactor::SrcAlpha, dst_rgb: BlendFactor::OneMinusSrcAlpha,
        src_alpha: BlendFactor::SrcAlpha, dst_alpha: BlendFactor::OneMinusSrcAlpha,
    };

    /// 加法混合（粒子光效）
    pub const ADDITIVE: Self = Self {
        enabled: true,
        rgb_op: BlendOp::Add, alpha_op: BlendOp::Add,
        src_rgb: BlendFactor::SrcAlpha,  dst_rgb: BlendFactor::One,
        src_alpha: BlendFactor::SrcAlpha, dst_alpha: BlendFactor::One,
    };

    /// 乘法混合
    pub const MULTIPLY: Self = Self {
        enabled: true,
        rgb_op: BlendOp::Add, alpha_op: BlendOp::Add,
        src_rgb: BlendFactor::DstColor, dst_rgb: BlendFactor::Zero,
        src_alpha: BlendFactor::DstAlpha, dst_alpha: BlendFactor::Zero,
    };
}

impl Default for BlendDescriptor {
    fn default() -> Self { Self::ALPHA_NON_PREMULTIPLIED }
}

// ─── 深度/模板 ────────────────────────────────────────────────────

/// 比较函数（深度/模板测试）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareFunction {
    Never, Less, Equal, LessEqual,
    Greater, NotEqual, GreaterEqual, Always,
}

/// 深度测试描述符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepthDescriptor {
    pub test_enabled: bool,
    pub write_enabled: bool,
    pub compare: CompareFunction,
}

impl DepthDescriptor {
    pub const DISABLED: Self = Self {
        test_enabled: false, write_enabled: false, compare: CompareFunction::Always,
    };
    pub const DEFAULT_3D: Self = Self {
        test_enabled: true, write_enabled: true, compare: CompareFunction::Less,
    };
}

impl Default for DepthDescriptor {
    fn default() -> Self { Self::DISABLED }
}

/// 模板操作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilOp {
    Keep, Zero, Replace, Increment, Decrement, Invert,
    IncrementWrap, DecrementWrap,
}

/// 模板测试描述符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StencilDescriptor {
    pub test_enabled: bool,
    pub compare: CompareFunction,
    pub ref_value: i32,
    pub read_mask: u32,
    pub write_mask: u32,
    pub stencil_fail_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub pass_op: StencilOp,
}

impl StencilDescriptor {
    pub const DISABLED: Self = Self {
        test_enabled: false,
        compare: CompareFunction::Always,
        ref_value: 0, read_mask: 0xFF, write_mask: 0xFF,
        stencil_fail_op: StencilOp::Keep,
        depth_fail_op: StencilOp::Keep,
        pass_op: StencilOp::Keep,
    };
}

impl Default for StencilDescriptor {
    fn default() -> Self { Self::DISABLED }
}

// ─── 光栅化状态 ──────────────────────────────────────────────────

/// 剔除模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CullMode { None, Front, Back }

/// 多边形填充模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillMode { Solid, Wireframe }

/// 光栅化描述符
#[derive(Debug, Clone, PartialEq)]
pub struct RasterizationDescriptor {
    pub cull_mode: CullMode,
    pub fill_mode: FillMode,
    pub front_face_ccw: bool,
    pub scissor_enabled: bool,
    /// 多边形偏移（阴影绘制用）
    pub polygon_offset_enabled: bool,
    pub polygon_offset_factor: f32,
    pub polygon_offset_units: f32,
}

impl Default for RasterizationDescriptor {
    fn default() -> Self {
        Self {
            cull_mode: CullMode::None,
            fill_mode: FillMode::Solid,
            front_face_ccw: false,
            scissor_enabled: false,
            polygon_offset_enabled: false,
            polygon_offset_factor: 0.0,
            polygon_offset_units: 0.0,
        }
    }
}

// ─── 着色器引用 ──────────────────────────────────────────────────

/// 着色器程序引用（用名称或 ID 引用预编译的着色器）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderRef {
    pub program_id: u32,
    pub vertex_shader_name: String,
    pub fragment_shader_name: String,
}

impl ShaderRef {
    pub fn new(program_id: u32, vs: &str, fs: &str) -> Self {
        Self {
            program_id,
            vertex_shader_name: vs.to_string(),
            fragment_shader_name: fs.to_string(),
        }
    }

    pub fn default_2d() -> Self {
        Self::new(1, "position_texture_color_vert", "position_texture_color_frag")
    }

    pub fn default_3d() -> Self {
        Self::new(2, "3d_position_texture_vert", "3d_position_texture_frag")
    }

    pub fn ui() -> Self {
        Self::new(3, "ui_vert", "ui_frag")
    }

    pub fn particle() -> Self {
        Self::new(4, "particle_vert", "particle_frag")
    }
}

// ─── 完整管线状态 ─────────────────────────────────────────────

/// 渲染管线状态 — 一次绘制调用所需的所有 GPU 状态
#[derive(Debug, Clone)]
pub struct PipelineState {
    /// 着色器引用
    pub shader: ShaderRef,
    /// 顶点布局
    pub vertex_layout: VertexLayout,
    /// 混合状态
    pub blend: BlendDescriptor,
    /// 深度测试
    pub depth: DepthDescriptor,
    /// 模板测试
    pub stencil: StencilDescriptor,
    /// 光栅化设置
    pub rasterization: RasterizationDescriptor,
    /// 颜色写入掩码（rgba）
    pub color_write_mask: [bool; 4],
    /// 是否启用 Alpha-to-Coverage（MSAA 边缘）
    pub alpha_to_coverage: bool,
}

impl PipelineState {
    /// 创建默认 2D 精灵管线状态
    pub fn new_sprite_2d() -> Self {
        Self {
            shader: ShaderRef::default_2d(),
            vertex_layout: {
                let mut layout = VertexLayout::new();
                layout
                    .add_attribute("a_position", 0, VertexFormat::Float3)
                    .add_attribute("a_texCoord", 1, VertexFormat::Float2)
                    .add_attribute("a_color", 2, VertexFormat::Uint8x4);
                layout
            },
            blend: BlendDescriptor::ALPHA_NON_PREMULTIPLIED,
            depth: DepthDescriptor::DISABLED,
            stencil: StencilDescriptor::DISABLED,
            rasterization: RasterizationDescriptor::default(),
            color_write_mask: [true; 4],
            alpha_to_coverage: false,
        }
    }

    /// 3D 网格管线
    pub fn new_mesh_3d() -> Self {
        let mut state = Self::new_sprite_2d();
        state.shader = ShaderRef::default_3d();
        state.depth = DepthDescriptor::DEFAULT_3D;
        state.rasterization.cull_mode = CullMode::Back;
        state
    }

    /// UI 不混合管线（全遮挡）
    pub fn new_ui_opaque() -> Self {
        let mut state = Self::new_sprite_2d();
        state.shader = ShaderRef::ui();
        state.blend = BlendDescriptor::OPAQUE;
        state
    }

    /// 粒子加法混合管线
    pub fn new_particle_additive() -> Self {
        let mut state = Self::new_sprite_2d();
        state.shader = ShaderRef::particle();
        state.blend = BlendDescriptor::ADDITIVE;
        state
    }

    /// 计算顶点步长
    pub fn vertex_stride(&self) -> u32 {
        self.vertex_layout.get_stride()
    }

    /// 是否启用了深度测试
    pub fn has_depth_test(&self) -> bool {
        self.depth.test_enabled
    }

    /// 是否启用了模板测试
    pub fn has_stencil_test(&self) -> bool {
        self.stencil.test_enabled
    }

    /// 是否写入颜色（任何通道）
    pub fn writes_color(&self) -> bool {
        self.color_write_mask.iter().any(|&v| v)
    }
}

impl Default for PipelineState {
    fn default() -> Self {
        Self::new_sprite_2d()
    }
}

/// 管线状态缓存（避免重复创建相同的 PipelineState）
#[derive(Debug, Default)]
pub struct PipelineCache {
    states: Vec<PipelineState>,
    hits: u32,
    misses: u32,
}

impl PipelineCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// 如果缓存中没有，则存入并返回引用
    pub fn get_or_insert(&mut self, state: PipelineState) -> &PipelineState {
        // 简单线性搜索（实际可用 HashMap + 哈希）
        if let Some(idx) = self.states.iter().position(|s| {
            s.shader == state.shader
                && s.blend == state.blend
                && s.depth == state.depth
        }) {
            self.hits += 1;
            &self.states[idx]
        } else {
            self.misses += 1;
            self.states.push(state);
            self.states.last().unwrap()
        }
    }

    pub fn get_cache_size(&self) -> usize { self.states.len() }
    pub fn get_hit_count(&self) -> u32 { self.hits }
    pub fn get_miss_count(&self) -> u32 { self.misses }

    pub fn clear(&mut self) {
        self.states.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_format_size() {
        assert_eq!(VertexFormat::Float.size_bytes(), 4);
        assert_eq!(VertexFormat::Float2.size_bytes(), 8);
        assert_eq!(VertexFormat::Float3.size_bytes(), 12);
        assert_eq!(VertexFormat::Float4.size_bytes(), 16);
        assert_eq!(VertexFormat::Uint8x4.size_bytes(), 4);
    }

    #[test]
    fn test_vertex_format_component_count() {
        assert_eq!(VertexFormat::Float.component_count(), 1);
        assert_eq!(VertexFormat::Float3.component_count(), 3);
        assert_eq!(VertexFormat::Uint8x4.component_count(), 4);
    }

    #[test]
    fn test_vertex_layout_stride() {
        let mut layout = VertexLayout::new();
        layout
            .add_attribute("a_position", 0, VertexFormat::Float3)
            .add_attribute("a_texCoord", 1, VertexFormat::Float2)
            .add_attribute("a_color", 2, VertexFormat::Uint8x4);

        // 12 + 8 + 4 = 24 bytes
        assert_eq!(layout.get_stride(), 24);
        assert_eq!(layout.get_attribute_count(), 3);
    }

    #[test]
    fn test_vertex_attribute_offset() {
        let mut layout = VertexLayout::new();
        layout
            .add_attribute("a_pos", 0, VertexFormat::Float3)   // offset 0
            .add_attribute("a_uv",  1, VertexFormat::Float2)   // offset 12
            .add_attribute("a_col", 2, VertexFormat::Uint8x4); // offset 20

        assert_eq!(layout.attributes[0].offset, 0);
        assert_eq!(layout.attributes[1].offset, 12);
        assert_eq!(layout.attributes[2].offset, 20);
    }

    #[test]
    fn test_blend_descriptor_constants() {
        assert!(!BlendDescriptor::OPAQUE.enabled);
        assert!(BlendDescriptor::ALPHA_NON_PREMULTIPLIED.enabled);
        assert!(BlendDescriptor::ADDITIVE.enabled);
        assert_eq!(BlendDescriptor::ADDITIVE.dst_rgb, BlendFactor::One);
        assert_eq!(BlendDescriptor::MULTIPLY.src_rgb, BlendFactor::DstColor);
    }

    #[test]
    fn test_depth_descriptor_constants() {
        assert!(!DepthDescriptor::DISABLED.test_enabled);
        assert!(DepthDescriptor::DEFAULT_3D.test_enabled);
        assert!(DepthDescriptor::DEFAULT_3D.write_enabled);
        assert_eq!(DepthDescriptor::DEFAULT_3D.compare, CompareFunction::Less);
    }

    #[test]
    fn test_pipeline_state_sprite_2d() {
        let state = PipelineState::new_sprite_2d();
        assert_eq!(state.shader.program_id, 1);
        assert!(state.blend.enabled);
        assert!(!state.depth.test_enabled);
        assert!(!state.stencil.test_enabled);
        assert_eq!(state.vertex_stride(), 24); // 12+8+4
    }

    #[test]
    fn test_pipeline_state_mesh_3d() {
        let state = PipelineState::new_mesh_3d();
        assert_eq!(state.shader.program_id, 2);
        assert!(state.depth.test_enabled);
        assert!(state.depth.write_enabled);
        assert_eq!(state.rasterization.cull_mode, CullMode::Back);
    }

    #[test]
    fn test_pipeline_state_particle_additive() {
        let state = PipelineState::new_particle_additive();
        assert!(state.blend.enabled);
        assert_eq!(state.blend.dst_rgb, BlendFactor::One); // 加法混合
    }

    #[test]
    fn test_pipeline_state_ui_opaque() {
        let state = PipelineState::new_ui_opaque();
        assert!(!state.blend.enabled);
        assert_eq!(state.shader.program_id, 3);
    }

    #[test]
    fn test_pipeline_state_writes_color() {
        let state = PipelineState::new_sprite_2d();
        assert!(state.writes_color());
    }

    #[test]
    fn test_pipeline_state_has_depth() {
        let sprite = PipelineState::new_sprite_2d();
        let mesh = PipelineState::new_mesh_3d();
        assert!(!sprite.has_depth_test());
        assert!(mesh.has_depth_test());
    }

    #[test]
    fn test_pipeline_cache_basic() {
        let mut cache = PipelineCache::new();
        let state1 = PipelineState::new_sprite_2d();
        let state2 = PipelineState::new_sprite_2d(); // 相同配置

        let _ = cache.get_or_insert(state1);
        assert_eq!(cache.get_miss_count(), 1);
        assert_eq!(cache.get_cache_size(), 1);

        let _ = cache.get_or_insert(state2);
        assert_eq!(cache.get_hit_count(), 1);
        assert_eq!(cache.get_cache_size(), 1); // 不重复插入
    }

    #[test]
    fn test_pipeline_cache_different_states() {
        let mut cache = PipelineCache::new();

        let _ = cache.get_or_insert(PipelineState::new_sprite_2d());
        let _ = cache.get_or_insert(PipelineState::new_mesh_3d());
        let _ = cache.get_or_insert(PipelineState::new_particle_additive());

        assert_eq!(cache.get_cache_size(), 3);
        assert_eq!(cache.get_miss_count(), 3);
        assert_eq!(cache.get_hit_count(), 0);
    }

    #[test]
    fn test_pipeline_cache_clear() {
        let mut cache = PipelineCache::new();
        let _ = cache.get_or_insert(PipelineState::new_sprite_2d());
        let _ = cache.get_or_insert(PipelineState::new_mesh_3d());
        assert_eq!(cache.get_cache_size(), 2);

        cache.clear();
        assert_eq!(cache.get_cache_size(), 0);
    }

    #[test]
    fn test_shader_ref_default() {
        let vs = ShaderRef::default_2d();
        assert_eq!(vs.program_id, 1);
        assert!(vs.vertex_shader_name.contains("position"));

        let mesh = ShaderRef::default_3d();
        assert_eq!(mesh.program_id, 2);
    }

    #[test]
    fn test_vertex_attribute_normalized() {
        let attr = VertexAttribute::new("a_col", 2, VertexFormat::Uint8x4, 16).normalized();
        assert!(attr.normalized);
        assert_eq!(attr.name, "a_col");
        assert_eq!(attr.location, 2);
    }

    #[test]
    fn test_rasterization_default() {
        let r = RasterizationDescriptor::default();
        assert_eq!(r.cull_mode, CullMode::None);
        assert_eq!(r.fill_mode, FillMode::Solid);
        assert!(!r.scissor_enabled);
    }
}
