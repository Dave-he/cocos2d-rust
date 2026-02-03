use crate::base::types::Color4F;

#[derive(Debug, Clone)]
pub struct PipelineState {
    name: String,
    program: String,
    blend_state: BlendState,
    depth_stencil_state: DepthStencilState,
    rasterizer_state: RasterizerState,
    vertex_layout: String,
    primitive_type: PrimitiveType,
    render_target: String,
}

impl PipelineState {
    pub fn new() -> PipelineState {
        PipelineState {
            name: String::new(),
            program: String::new(),
            blend_state: BlendState::new(),
            depth_stencil_state: DepthStencilState::new(),
            rasterizer_state: RasterizerState::new(),
            vertex_layout: String::new(),
            primitive_type: PrimitiveType::TRIANGLES,
            render_target: String::new(),
        }
    }

    pub fn with_name(name: &str) -> PipelineState {
        PipelineState {
            name: name.to_string(),
            program: String::new(),
            blend_state: BlendState::new(),
            depth_stencil_state: DepthStencilState::new(),
            rasterizer_state: RasterizerState::new(),
            vertex_layout: String::new(),
            primitive_type: PrimitiveType::TRIANGLES,
            render_target: String::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_program(&mut self, program: &str) {
        self.program = program.to_string();
    }

    pub fn get_program(&self) -> &str {
        &self.program
    }

    pub fn get_blend_state(&self) -> &BlendState {
        &self.blend_state
    }

    pub fn get_blend_state_mut(&mut self) -> &mut BlendState {
        &mut self.blend_state
    }

    pub fn get_depth_stencil_state(&self) -> &DepthStencilState {
        &self.depth_stencil_state
    }

    pub fn get_depth_stencil_state_mut(&mut self) -> &mut DepthStencilState {
        &mut self.depth_stencil_state
    }

    pub fn get_rasterizer_state(&self) -> &RasterizerState {
        &self.rasterizer_state
    }

    pub fn get_rasterizer_state_mut(&mut self) -> &mut RasterizerState {
        &mut self.rasterizer_state
    }

    pub fn set_vertex_layout(&mut self, layout: &str) {
        self.vertex_layout = layout.to_string();
    }

    pub fn set_primitive_type(&mut self, primitive: PrimitiveType) {
        self.primitive_type = primitive;
    }

    pub fn set_render_target(&mut self, target: &str) {
        self.render_target = target.to_string();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    POINTS,
    LINES,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
}

#[derive(Debug, Clone)]
pub struct BlendState {
    enabled: bool,
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
    rgb_op: u32,
    alpha_op: u32,
    write_mask: ColorWriteMask,
}

impl BlendState {
    pub fn new() -> BlendState {
        BlendState {
            enabled: false,
            src_rgb: 770,
            dst_rgb: 771,
            src_alpha: 770,
            dst_alpha: 771,
            rgb_op: 32774,
            alpha_op: 32774,
            write_mask: ColorWriteMask::ALL,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn get_src_rgb(&self) -> u32 {
        self.src_rgb
    }

    pub fn set_src_rgb(&mut self, src: u32) {
        self.src_rgb = src;
    }

    pub fn get_dst_rgb(&self) -> u32 {
        self.dst_rgb
    }

    pub fn set_dst_rgb(&mut self, dst: u32) {
        self.dst_rgb = dst;
    }

    pub fn get_src_alpha(&self) -> u32 {
        self.src_alpha
    }

    pub fn set_src_alpha(&mut self, src: u32) {
        self.src_alpha = src;
    }

    pub fn get_dst_alpha(&self) -> u32 {
        self.dst_alpha
    }

    pub fn set_dst_alpha(&mut self, dst: u32) {
        self.dst_alpha = dst;
    }

    pub fn set_blend_func(&mut self, src: u32, dst: u32) {
        self.src_rgb = src;
        self.dst_rgb = dst;
        self.src_alpha = src;
        self.dst_alpha = dst;
    }

    pub fn set_blend_func_separate(&mut self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
        self.src_rgb = src_rgb;
        self.dst_rgb = dst_rgb;
        self.src_alpha = src_alpha;
        self.dst_alpha = dst_alpha;
    }

    pub fn get_write_mask(&self) -> ColorWriteMask {
        self.write_mask
    }

    pub fn set_write_mask(&mut self, mask: ColorWriteMask) {
        self.write_mask = mask;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorWriteMask(u32);

impl ColorWriteMask {
    pub const NONE: ColorWriteMask = ColorWriteMask(0);
    pub const RED: ColorWriteMask = ColorWriteMask(1);
    pub const GREEN: ColorWriteMask = ColorWriteMask(2);
    pub const BLUE: ColorWriteMask = ColorWriteMask(4);
    pub const ALPHA: ColorWriteMask = ColorWriteMask(8);
    pub const ALL: ColorWriteMask = ColorWriteMask(15);

    pub fn get_red(&self) -> bool {
        (self.0 & 1) != 0
    }

    pub fn get_green(&self) -> bool {
        (self.0 & 2) != 0
    }

    pub fn get_blue(&self) -> bool {
        (self.0 & 4) != 0
    }

    pub fn get_alpha(&self) -> bool {
        (self.0 & 8) != 0
    }
}

use std::ops::BitOr;
impl BitOr for ColorWriteMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        ColorWriteMask(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone)]
pub struct DepthStencilState {
    depth_test_enabled: bool,
    depth_write_enabled: bool,
    depth_func: CompareFunc,
    stencil_enabled: bool,
    stencil_read_mask: u32,
    stencil_write_mask: u32,
    front_stencil: StencilState,
    back_stencil: StencilState,
}

impl DepthStencilState {
    pub fn new() -> DepthStencilState {
        DepthStencilState {
            depth_test_enabled: true,
            depth_write_enabled: true,
            depth_func: CompareFunc::LEQUAL,
            stencil_enabled: false,
            stencil_read_mask: 255,
            stencil_write_mask: 255,
            front_stencil: StencilState::new(),
            back_stencil: StencilState::new(),
        }
    }

    pub fn is_depth_test_enabled(&self) -> bool {
        self.depth_test_enabled
    }

    pub fn set_depth_test_enabled(&mut self, enabled: bool) {
        self.depth_test_enabled = enabled;
    }

    pub fn is_depth_write_enabled(&self) -> bool {
        self.depth_write_enabled
    }

    pub fn set_depth_write_enabled(&mut self, enabled: bool) {
        self.depth_write_enabled = enabled;
    }

    pub fn get_depth_func(&self) -> CompareFunc {
        self.depth_func
    }

    pub fn set_depth_func(&mut self, func: CompareFunc) {
        self.depth_func = func;
    }

    pub fn is_stencil_enabled(&self) -> bool {
        self.stencil_enabled
    }

    pub fn set_stencil_enabled(&mut self, enabled: bool) {
        self.stencil_enabled = enabled;
    }

    pub fn get_stencil_read_mask(&self) -> u32 {
        self.stencil_read_mask
    }

    pub fn set_stencil_read_mask(&mut self, mask: u32) {
        self.stencil_read_mask = mask;
    }

    pub fn get_stencil_write_mask(&self) -> u32 {
        self.stencil_write_mask
    }

    pub fn set_stencil_write_mask(&mut self, mask: u32) {
        self.stencil_write_mask = mask;
    }

    pub fn get_front_stencil(&self) -> &StencilState {
        &self.front_stencil
    }

    pub fn get_back_stencil(&self) -> &StencilState {
        &self.back_stencil
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunc {
    NEVER,
    LESS,
    EQUAL,
    LEQUAL,
    GREATER,
    NOTEQUAL,
    GEQUAL,
    ALWAYS,
}

#[derive(Debug, Clone)]
pub struct StencilState {
    stencil_func: CompareFunc,
    stencil_ref: i32,
    stencil_fail_op: StencilOp,
    stencil_pass_depth_fail_op: StencilOp,
    stencil_pass_depth_pass_op: StencilOp,
}

impl StencilState {
    pub fn new() -> StencilState {
        StencilState {
            stencil_func: CompareFunc::ALWAYS,
            stencil_ref: 0,
            stencil_fail_op: StencilOp::KEEP,
            stencil_pass_depth_fail_op: StencilOp::KEEP,
            stencil_pass_depth_pass_op: StencilOp::KEEP,
        }
    }

    pub fn get_stencil_func(&self) -> CompareFunc {
        self.stencil_func
    }

    pub fn set_stencil_func(&mut self, func: CompareFunc) {
        self.stencil_func = func;
    }

    pub fn get_stencil_ref(&self) -> i32 {
        self.stencil_ref
    }

    pub fn set_stencil_ref(&mut self, ref_val: i32) {
        self.stencil_ref = ref_val;
    }

    pub fn get_stencil_fail_op(&self) -> StencilOp {
        self.stencil_fail_op
    }

    pub fn set_stencil_fail_op(&mut self, op: StencilOp) {
        self.stencil_fail_op = op;
    }

    pub fn get_stencil_pass_depth_fail_op(&self) -> StencilOp {
        self.stencil_pass_depth_fail_op
    }

    pub fn set_stencil_pass_depth_fail_op(&mut self, op: StencilOp) {
        self.stencil_pass_depth_fail_op = op;
    }

    pub fn get_stencil_pass_depth_pass_op(&self) -> StencilOp {
        self.stencil_pass_depth_pass_op
    }

    pub fn set_stencil_pass_depth_pass_op(&mut self, op: StencilOp) {
        self.stencil_pass_depth_pass_op = op;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOp {
    KEEP,
    ZERO,
    REPLACE,
    INCR,
    INCR_WRAP,
    DECR,
    DECR_WRAP,
    INV,
}

#[derive(Debug, Clone)]
pub struct RasterizerState {
    cull_mode: CullMode,
    depth_bias: f32,
    depth_bias_clamp: f32,
    slope_scaled_depth_bias: f32,
    depth_clip_enabled: bool,
    scissor_test_enabled: bool,
    multisample_antialiasing_enabled: bool,
    line_width: f32,
}

impl RasterizerState {
    pub fn new() -> RasterizerState {
        RasterizerState {
            cull_mode: CullMode::BACK,
            depth_bias: 0.0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enabled: true,
            scissor_test_enabled: false,
            multisample_antialiasing_enabled: true,
            line_width: 1.0,
        }
    }

    pub fn get_cull_mode(&self) -> CullMode {
        self.cull_mode
    }

    pub fn set_cull_mode(&mut self, mode: CullMode) {
        self.cull_mode = mode;
    }

    pub fn get_depth_bias(&self) -> f32 {
        self.depth_bias
    }

    pub fn set_depth_bias(&mut self, bias: f32) {
        self.depth_bias = bias;
    }

    pub fn is_scissor_test_enabled(&self) -> bool {
        self.scissor_test_enabled
    }

    pub fn set_scissor_test_enabled(&mut self, enabled: bool) {
        self.scissor_test_enabled = enabled;
    }

    pub fn is_multisample_antialiasing_enabled(&self) -> bool {
        self.multisample_antialiasing_enabled
    }

    pub fn set_multisample_antialiasing_enabled(&mut self, enabled: bool) {
        self.multisample_antialiasing_enabled = enabled;
    }

    pub fn get_line_width(&self) -> f32 {
        self.line_width
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
    }

    pub fn is_depth_clip_enabled(&self) -> bool {
        self.depth_clip_enabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    NONE,
    FRONT,
    BACK,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_state_new() {
        let pipeline = PipelineState::new();
        assert_eq!(pipeline.get_name(), "");
        assert_eq!(pipeline.get_program(), "");
        assert_eq!(pipeline.primitive_type, PrimitiveType::TRIANGLES);
    }

    #[test]
    fn test_pipeline_state_with_name() {
        let pipeline = PipelineState::with_name("opaque");
        assert_eq!(pipeline.get_name(), "opaque");
    }

    #[test]
    fn test_pipeline_state_set_name() {
        let mut pipeline = PipelineState::new();
        pipeline.set_name("transparent");
        assert_eq!(pipeline.get_name(), "transparent");
    }

    #[test]
    fn test_pipeline_state_set_program() {
        let mut pipeline = PipelineState::new();
        pipeline.set_program("default");
        assert_eq!(pipeline.get_program(), "default");
    }

    #[test]
    fn test_pipeline_state_set_primitive_type() {
        let mut pipeline = PipelineState::new();
        pipeline.set_primitive_type(PrimitiveType::LINES);
        assert_eq!(pipeline.primitive_type, PrimitiveType::LINES);
    }

    #[test]
    fn test_primitive_type_variants() {
        let types = [
            PrimitiveType::POINTS,
            PrimitiveType::LINES,
            PrimitiveType::LINE_STRIP,
            PrimitiveType::TRIANGLES,
            PrimitiveType::TRIANGLE_STRIP,
            PrimitiveType::TRIANGLE_FAN,
        ];
        assert_eq!(types.len(), 6);
    }

    #[test]
    fn test_blend_state_new() {
        let blend = BlendState::new();
        assert!(!blend.is_enabled());
        assert_eq!(blend.get_src_rgb(), 770);
        assert_eq!(blend.get_dst_rgb(), 771);
    }

    #[test]
    fn test_blend_state_set_enabled() {
        let mut blend = BlendState::new();
        assert!(!blend.is_enabled());
        blend.set_enabled(true);
        assert!(blend.is_enabled());
    }

    #[test]
    fn test_blend_state_set_blend_func() {
        let mut blend = BlendState::new();
        blend.set_blend_func(1, 0);
        assert_eq!(blend.get_src_rgb(), 1);
        assert_eq!(blend.get_dst_rgb(), 0);
    }

    #[test]
    fn test_blend_state_set_blend_func_separate() {
        let mut blend = BlendState::new();
        blend.set_blend_func_separate(1, 0, 2, 3);
        assert_eq!(blend.get_src_rgb(), 1);
        assert_eq!(blend.get_dst_rgb(), 0);
        assert_eq!(blend.get_src_alpha(), 2);
        assert_eq!(blend.get_dst_alpha(), 3);
    }

    #[test]
    fn test_color_write_mask() {
        assert_eq!(ColorWriteMask::NONE.0, 0);
        assert_eq!(ColorWriteMask::RED.0, 1);
        assert_eq!(ColorWriteMask::GREEN.0, 2);
        assert_eq!(ColorWriteMask::BLUE.0, 4);
        assert_eq!(ColorWriteMask::ALPHA.0, 8);
        assert_eq!(ColorWriteMask::ALL.0, 15);
    }

    #[test]
    fn test_color_write_mask_getters() {
        let mask = ColorWriteMask::ALL;
        assert!(mask.get_red());
        assert!(mask.get_green());
        assert!(mask.get_blue());
        assert!(mask.get_alpha());

        let mask = ColorWriteMask::RED | ColorWriteMask::BLUE;
        assert!(mask.get_red());
        assert!(!mask.get_green());
        assert!(mask.get_blue());
        assert!(!mask.get_alpha());
    }

    #[test]
    fn test_depth_stencil_state_new() {
        let ds = DepthStencilState::new();
        assert!(ds.is_depth_test_enabled());
        assert!(ds.is_depth_write_enabled());
        assert_eq!(ds.get_depth_func(), CompareFunc::LEQUAL);
        assert!(!ds.is_stencil_enabled());
    }

    #[test]
    fn test_depth_stencil_state_setters() {
        let mut ds = DepthStencilState::new();
        ds.set_depth_test_enabled(false);
        ds.set_depth_write_enabled(false);
        ds.set_depth_func(CompareFunc::LESS);
        ds.set_stencil_enabled(true);

        assert!(!ds.is_depth_test_enabled());
        assert!(!ds.is_depth_write_enabled());
        assert_eq!(ds.get_depth_func(), CompareFunc::LESS);
        assert!(ds.is_stencil_enabled());
    }

    #[test]
    fn test_compare_func_variants() {
        let funcs = [
            CompareFunc::NEVER,
            CompareFunc::LESS,
            CompareFunc::EQUAL,
            CompareFunc::LEQUAL,
            CompareFunc::GREATER,
            CompareFunc::NOTEQUAL,
            CompareFunc::GEQUAL,
            CompareFunc::ALWAYS,
        ];
        assert_eq!(funcs.len(), 8);
    }

    #[test]
    fn test_stencil_state_new() {
        let stencil = StencilState::new();
        assert_eq!(stencil.get_stencil_func(), CompareFunc::ALWAYS);
        assert_eq!(stencil.get_stencil_ref(), 0);
        assert_eq!(stencil.get_stencil_fail_op(), StencilOp::KEEP);
    }

    #[test]
    fn test_stencil_state_setters() {
        let mut stencil = StencilState::new();
        stencil.set_stencil_func(CompareFunc::LESS);
        stencil.set_stencil_ref(1);
        stencil.set_stencil_fail_op(StencilOp::ZERO);
        stencil.set_stencil_pass_depth_fail_op(StencilOp::INCR);
        stencil.set_stencil_pass_depth_pass_op(StencilOp::DECR);

        assert_eq!(stencil.get_stencil_func(), CompareFunc::LESS);
        assert_eq!(stencil.get_stencil_ref(), 1);
        assert_eq!(stencil.get_stencil_fail_op(), StencilOp::ZERO);
        assert_eq!(stencil.get_stencil_pass_depth_fail_op(), StencilOp::INCR);
        assert_eq!(stencil.get_stencil_pass_depth_pass_op(), StencilOp::DECR);
    }

    #[test]
    fn test_stencil_op_variants() {
        let ops = [
            StencilOp::KEEP,
            StencilOp::ZERO,
            StencilOp::REPLACE,
            StencilOp::INCR,
            StencilOp::INCR_WRAP,
            StencilOp::DECR,
            StencilOp::DECR_WRAP,
            StencilOp::INV,
        ];
        assert_eq!(ops.len(), 8);
    }

    #[test]
    fn test_rasterizer_state_new() {
        let rs = RasterizerState::new();
        assert_eq!(rs.get_cull_mode(), CullMode::BACK);
        assert!((rs.get_depth_bias() - 0.0).abs() < 0.001);
        assert!((rs.get_line_width() - 1.0).abs() < 0.001);
        assert!(rs.is_depth_clip_enabled());
        assert!(!rs.is_scissor_test_enabled());
        assert!(rs.is_multisample_antialiasing_enabled());
    }

    #[test]
    fn test_rasterizer_state_setters() {
        let mut rs = RasterizerState::new();
        rs.set_cull_mode(CullMode::FRONT);
        rs.set_depth_bias(0.5);
        rs.set_scissor_test_enabled(true);
        rs.set_line_width(2.0);

        assert_eq!(rs.get_cull_mode(), CullMode::FRONT);
        assert!((rs.get_depth_bias() - 0.5).abs() < 0.001);
        assert!(rs.is_scissor_test_enabled());
        assert!((rs.get_line_width() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_cull_mode_variants() {
        assert_eq!(CullMode::NONE, CullMode::NONE);
        assert_eq!(CullMode::FRONT, CullMode::FRONT);
        assert_eq!(CullMode::BACK, CullMode::BACK);
        assert_ne!(CullMode::NONE, CullMode::FRONT);
    }
}
