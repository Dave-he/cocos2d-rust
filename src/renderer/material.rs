use std::collections::HashMap;
use crate::base::Ref;
use crate::base::types::Color4F;
use crate::math::Vec4;

#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    technique: Option<Ref<Technique>>,
    techniques: HashMap<String, Ref<Technique>>,
    state: MaterialState,
}

impl Material {
    pub fn new() -> Material {
        Material {
            name: String::new(),
            technique: None,
            techniques: HashMap::new(),
            state: MaterialState::new(),
        }
    }

    pub fn with_name(name: &str) -> Material {
        Material {
            name: name.to_string(),
            technique: None,
            techniques: HashMap::new(),
            state: MaterialState::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_technique(&mut self, technique: Ref<Technique>) {
        self.technique = Some(technique);
    }

    pub fn get_technique(&self) -> Option<&Ref<Technique>> {
        self.technique.as_ref()
    }

    pub fn add_technique(&mut self, name: &str, technique: Ref<Technique>) {
        self.techniques.insert(name.to_string(), technique);
    }

    pub fn get_technique_by_name(&self, name: &str) -> Option<&Ref<Technique>> {
        self.techniques.get(name)
    }

    pub fn get_state(&self) -> &MaterialState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut MaterialState {
        &mut self.state
    }

    pub fn set_depth_write(&mut self, enable: bool) {
        self.state.depth_write = enable;
    }

    pub fn set_depth_test(&mut self, enable: bool) {
        self.state.depth_test = enable;
    }

    pub fn set_blend(&mut self, enable: bool) {
        self.state.blend = enable;
    }

    pub fn set_blend_func(&mut self, src: u32, dst: u32) {
        self.state.blend_src = src;
        self.state.blend_dst = dst;
    }

    pub fn set_cull_mode(&mut self, mode: u32) {
        self.state.cull_mode = mode;
    }
}

#[derive(Debug, Clone)]
pub struct MaterialState {
    pub depth_write: bool,
    pub depth_test: bool,
    pub blend: bool,
    pub blend_src: u32,
    pub blend_dst: u32,
    pub cull_mode: u32,
    pub depth_func: u32,
    pub alpha_test: bool,
    pub alpha_test_value: f32,
}

impl MaterialState {
    pub fn new() -> MaterialState {
        MaterialState {
            depth_write: true,
            depth_test: true,
            blend: false,
            blend_src: 770,
            blend_dst: 771,
            cull_mode: 2,
            depth_func: 3,
            alpha_test: false,
            alpha_test_value: 0.5,
        }
    }
}

#[derive(Debug)]
pub struct Technique {
    name: String,
    passes: Vec<Ref<Pass>>,
    render_states: Vec<RenderState>,
}

impl Technique {
    pub fn new() -> Technique {
        Technique {
            name: String::new(),
            passes: Vec::new(),
            render_states: Vec::new(),
        }
    }

    pub fn with_name(name: &str) -> Technique {
        Technique {
            name: name.to_string(),
            passes: Vec::new(),
            render_states: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_pass(&mut self, pass: Ref<Pass>) {
        self.passes.push(pass);
    }

    pub fn get_passes(&self) -> &Vec<Ref<Pass>> {
        &self.passes
    }

    pub fn get_pass_count(&self) -> u32 {
        self.passes.len() as u32
    }

    pub fn get_render_state(&self, index: u32) -> Option<&RenderState> {
        self.render_states.get(index as usize)
    }
}

#[derive(Debug, Clone)]
pub struct Pass {
    name: String,
    program: Option<Ref<Program>>,
    render_state: RenderState,
    uniform_data: HashMap<String, UniformValue>,
}

impl Pass {
    pub fn new() -> Pass {
        Pass {
            name: String::new(),
            program: None,
            render_state: RenderState::new(),
            uniform_data: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_program(&mut self, program: Ref<Program>) {
        self.program = Some(program);
    }

    pub fn get_program(&self) -> Option<&Ref<Program>> {
        self.program.as_ref()
    }

    pub fn get_render_state(&self) -> &RenderState {
        &self.render_state
    }

    pub fn get_render_state_mut(&mut self) -> &mut RenderState {
        &mut self.render_state
    }

    pub fn set_uniform(&mut self, name: &str, value: UniformValue) {
        self.uniform_data.insert(name.to_string(), value);
    }

    pub fn get_uniform(&self, name: &str) -> Option<&UniformValue> {
        self.uniform_data.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct RenderState {
    pub depth_write: bool,
    pub depth_test: bool,
    pub depth_func: u32,
    pub blend: bool,
    pub blend_src: u32,
    pub blend_dst: u32,
    pub blend_eq: u32,
    pub cull_mode: u32,
    pub front_face: u32,
    pub color_write: u32,
    pub stencil_write: u32,
    pub stencil_func: u32,
    pub stencil_func_ref: i32,
    pub stencil_func_mask: u32,
    pub stencil_op_s_fail: u32,
    pub stencil_op_s_pass_depth_fail: u32,
    pub stencil_op_s_pass_depth_pass: u32,
    pub alpha_test: bool,
    pub alpha_test_value: f32,
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState {
            depth_write: true,
            depth_test: true,
            depth_func: 3,
            blend: false,
            blend_src: 770,
            blend_dst: 771,
            blend_eq: 32774,
            cull_mode: 2,
            front_face: 2304,
            color_write: 15,
            stencil_write: 255,
            stencil_func: 519,
            stencil_func_ref: 0,
            stencil_func_mask: 255,
            stencil_op_s_fail: 7680,
            stencil_op_s_pass_depth_fail: 7680,
            stencil_op_s_pass_depth_pass: 7680,
            alpha_test: false,
            alpha_test_value: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UniformValue {
    Float(f32),
    Vec2(Vec4),
    Vec3(Vec4),
    Vec4(Vec4),
    Mat4(Mat4),
    Int(i32),
    IVec2(Vec4),
    IVec3(Vec4),
    IVec4(Vec4),
    Sampler(i32),
}

#[derive(Debug)]
pub struct Program {
    name: String,
    vertex_shader: String,
    fragment_shader: String,
    uniforms: HashMap<String, UniformInfo>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            name: String::new(),
            vertex_shader: String::new(),
            fragment_shader: String::new(),
            uniforms: HashMap::new(),
        }
    }

    pub fn with_name(name: &str) -> Program {
        Program {
            name: name.to_string(),
            vertex_shader: String::new(),
            fragment_shader: String::new(),
            uniforms: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_vertex_shader(&mut self, shader: &str) {
        self.vertex_shader = shader.to_string();
    }

    pub fn set_fragment_shader(&mut self, shader: &str) {
        self.fragment_shader = shader.to_string();
    }

    pub fn add_uniform(&mut self, name: &str, uniform: UniformInfo) {
        self.uniforms.insert(name.to_string(), uniform);
    }

    pub fn get_uniform(&self, name: &str) -> Option<&UniformInfo> {
        self.uniforms.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct UniformInfo {
    pub name: String,
    pub location: i32,
    pub uniform_type: UniformType,
    pub count: u32,
    pub size: u32,
}

impl UniformInfo {
    pub fn new(name: &str, uniform_type: UniformType) -> UniformInfo {
        UniformInfo {
            name: name.to_string(),
            location: -1,
            uniform_type,
            count: 1,
            size: uniform_type.get_size(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniformType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Int,
    IVec2,
    IVec3,
    IVec4,
    Bool,
    BVec2,
    BVec3,
    BVec4,
    Mat2,
    Mat3,
    Mat4,
    Sampler2D,
    SamplerCube,
}

impl UniformType {
    pub fn get_size(&self) -> u32 {
        match self {
            UniformType::Float | UniformType::Int | UniformType::Bool | UniformType::Sampler2D | UniformType::SamplerCube => 4,
            UniformType::Vec2 | UniformType::IVec2 | UniformType::BVec2 => 8,
            UniformType::Vec3 | UniformType::IVec3 | UniformType::BVec3 => 12,
            UniformType::Vec4 | UniformType::IVec4 | UniformType::BVec4 => 16,
            UniformType::Mat2 => 16,
            UniformType::Mat3 => 36,
            UniformType::Mat4 => 64,
        }
    }
}
