use crate::base::types::Color4F;
use crate::base::{Ref, RefPtr};
use crate::math::{Mat4, Vec4};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    technique: Option<RefPtr<Technique>>,
    techniques: HashMap<String, RefPtr<Technique>>,
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

    pub fn set_technique(&mut self, technique: RefPtr<Technique>) {
        self.technique = Some(technique);
    }

    pub fn get_technique(&self) -> Option<&RefPtr<Technique>> {
        self.technique.as_ref()
    }

    pub fn add_technique(&mut self, name: &str, technique: RefPtr<Technique>) {
        self.techniques.insert(name.to_string(), technique);
    }

    pub fn get_technique_by_name(&self, name: &str) -> Option<&RefPtr<Technique>> {
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
    passes: Vec<RefPtr<Pass>>,
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

    pub fn add_pass(&mut self, pass: RefPtr<Pass>) {
        self.passes.push(pass);
    }

    pub fn get_passes(&self) -> &Vec<RefPtr<Pass>> {
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
    program: Option<RefPtr<Program>>,
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

    pub fn set_program(&mut self, program: RefPtr<Program>) {
        self.program = Some(program);
    }

    pub fn get_program(&self) -> Option<&RefPtr<Program>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_new() {
        let material = Material::new();
        assert_eq!(material.get_name(), "");
        assert!(material.get_technique().is_none());
    }

    #[test]
    fn test_material_with_name() {
        let material = Material::with_name("test_material");
        assert_eq!(material.get_name(), "test_material");
    }

    #[test]
    fn test_material_set_name() {
        let mut material = Material::new();
        material.set_name("new_name");
        assert_eq!(material.get_name(), "new_name");
    }

    #[test]
    fn test_material_state_new() {
        let state = MaterialState::new();
        assert!(state.depth_write);
        assert!(state.depth_test);
        assert!(!state.blend);
        assert_eq!(state.blend_src, 770);
        assert_eq!(state.blend_dst, 771);
        assert_eq!(state.cull_mode, 2);
    }

    #[test]
    fn test_material_set_depth_write() {
        let mut material = Material::new();
        assert!(material.get_state().depth_write);
        material.set_depth_write(false);
        assert!(!material.get_state().depth_write);
    }

    #[test]
    fn test_material_set_depth_test() {
        let mut material = Material::new();
        assert!(material.get_state().depth_test);
        material.set_depth_test(false);
        assert!(!material.get_state().depth_test);
    }

    #[test]
    fn test_material_set_blend() {
        let mut material = Material::new();
        assert!(!material.get_state().blend);
        material.set_blend(true);
        assert!(material.get_state().blend);
    }

    #[test]
    fn test_material_set_blend_func() {
        let mut material = Material::new();
        material.set_blend_func(1, 0);
        assert_eq!(material.get_state().blend_src, 1);
        assert_eq!(material.get_state().blend_dst, 0);
    }

    #[test]
    fn test_material_set_cull_mode() {
        let mut material = Material::new();
        assert_eq!(material.get_state().cull_mode, 2);
        material.set_cull_mode(0);
        assert_eq!(material.get_state().cull_mode, 0);
    }

    #[test]
    fn test_technique_new() {
        let technique = Technique::new();
        assert_eq!(technique.get_name(), "");
        assert_eq!(technique.get_pass_count(), 0);
    }

    #[test]
    fn test_technique_with_name() {
        let technique = Technique::with_name("deferred");
        assert_eq!(technique.get_name(), "deferred");
    }

    #[test]
    fn test_technique_set_name() {
        let mut technique = Technique::new();
        technique.set_name("forward");
        assert_eq!(technique.get_name(), "forward");
    }

    #[test]
    fn test_technique_add_pass() {
        let mut technique = Technique::new();
        let pass = RefPtr::new(Pass::new());
        technique.add_pass(pass.clone());
        assert_eq!(technique.get_pass_count(), 1);
        assert_eq!(technique.get_passes().len(), 1);
    }

    #[test]
    fn test_pass_new() {
        let pass = Pass::new();
        assert_eq!(pass.get_name(), "");
        assert!(pass.get_program().is_none());
    }

    #[test]
    fn test_pass_set_name() {
        let mut pass = Pass::new();
        pass.set_name("main_pass");
        assert_eq!(pass.get_name(), "main_pass");
    }

    #[test]
    fn test_pass_uniforms() {
        let mut pass = Pass::new();
        pass.set_uniform("u_color", UniformValue::Vec4(Vec4::new(1.0, 0.0, 0.0, 1.0)));
        pass.set_uniform("u_intensity", UniformValue::Float(0.5));

        assert!(pass.get_uniform("u_color").is_some());
        assert!(pass.get_uniform("u_nonexistent").is_none());
    }

    #[test]
    fn test_render_state_new() {
        let state = RenderState::new();
        assert!(state.depth_write);
        assert!(state.depth_test);
        assert!(!state.blend);
        assert_eq!(state.blend_src, 770);
        assert_eq!(state.depth_func, 3);
    }

    #[test]
    fn test_uniform_value_variants() {
        let float_val = UniformValue::Float(1.0);
        let vec4_val = UniformValue::Vec4(Vec4::new(1.0, 2.0, 3.0, 4.0));
        let int_val = UniformValue::Int(42);
        let mat4_val = UniformValue::Mat4(Mat4::IDENTITY);
        let sampler_val = UniformValue::Sampler(0);

        match float_val {
            UniformValue::Float(v) => assert!((v - 1.0).abs() < 0.001),
            _ => panic!("Expected Float"),
        }

        match vec4_val {
            UniformValue::Vec4(v) => {
                assert!((v.x - 1.0).abs() < 0.001);
                assert!((v.y - 2.0).abs() < 0.001);
            }
            _ => panic!("Expected Vec4"),
        }

        match int_val {
            UniformValue::Int(v) => assert_eq!(v, 42),
            _ => panic!("Expected Int"),
        }

        match sampler_val {
            UniformValue::Sampler(v) => assert_eq!(v, 0),
            _ => panic!("Expected Sampler"),
        }
    }

    #[test]
    fn test_program_new() {
        let program = Program::new();
        assert_eq!(program.get_name(), "");
    }


    #[test]
    fn test_program_with_name() {
        let program = Program::with_name("default");
        assert_eq!(program.get_name(), "default");
    }

    #[test]
    fn test_program_set_shaders() {
        let mut program = Program::new();
        program.set_vertex_shader("void main() { gl_Position = vec4(0.0); }");
        program.set_fragment_shader("void main() { gl_FragColor = vec4(1.0); }");
    }

    #[test]
    fn test_program_add_uniform() {
        let mut program = Program::new();
        let uniform = UniformInfo::new("u_color", UniformType::Vec4);
        program.add_uniform("u_color", uniform);
        assert!(program.get_uniform("u_color").is_some());
    }

    #[test]
    fn test_uniform_type_size() {
        assert_eq!(UniformType::Float.get_size(), 4);
        assert_eq!(UniformType::Vec2.get_size(), 8);
        assert_eq!(UniformType::Vec3.get_size(), 12);
        assert_eq!(UniformType::Vec4.get_size(), 16);
        assert_eq!(UniformType::Mat4.get_size(), 64);
        assert_eq!(UniformType::Mat2.get_size(), 16);
        assert_eq!(UniformType::Mat3.get_size(), 36);
    }

    #[test]
    fn test_uniform_info_new() {
        let info = UniformInfo::new("u_tex0", UniformType::Sampler2D);
        assert_eq!(info.name, "u_tex0");
        assert_eq!(info.uniform_type, UniformType::Sampler2D);
        assert_eq!(info.location, -1);
        assert_eq!(info.count, 1);
    }

    #[test]
    fn test_material_technique_by_name() {
        let mut material = Material::new();
        let technique = RefPtr::new(Technique::with_name("forward"));
        material.add_technique("forward", technique.clone());
        assert!(material.get_technique_by_name("forward").is_some());
        assert!(material.get_technique_by_name("deferred").is_none());
    }
}

impl UniformType {
    pub fn get_size(&self) -> u32 {
        match self {
            UniformType::Float
            | UniformType::Int
            | UniformType::Bool
            | UniformType::Sampler2D
            | UniformType::SamplerCube => 4,
            UniformType::Vec2 | UniformType::IVec2 | UniformType::BVec2 => 8,
            UniformType::Vec3 | UniformType::IVec3 | UniformType::BVec3 => 12,
            UniformType::Vec4 | UniformType::IVec4 | UniformType::BVec4 => 16,
            UniformType::Mat2 => 16,
            UniformType::Mat3 => 36,
            UniformType::Mat4 => 64,
        }
    }
}
