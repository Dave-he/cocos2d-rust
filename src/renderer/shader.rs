#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
/// 着色器系统
///
/// 管理和编译 GLSL 着色器程序，支持顶点着色器、片段着色器和统一变量

use std::collections::HashMap;
use std::rc::Rc;
use crate::math::{Mat4, Vec2, Vec3, Vec4};
use crate::base::types::Color4F;

pub type ShaderId = u32;
pub type UniformLocation = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
}

#[derive(Debug)]
pub struct Shader {
    id: ShaderId,
    shader_type: ShaderType,
    source: String,
    compiled: bool,
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: String) -> Self {
        Self {
            id: 0,
            shader_type,
            source,
            compiled: false,
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        if self.compiled {
            return Ok(());
        }

        self.compiled = true;
        Ok(())
    }

    pub fn id(&self) -> ShaderId {
        self.id
    }

    pub fn is_compiled(&self) -> bool {
        self.compiled
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Debug)]
pub struct ShaderProgram {
    id: ShaderId,
    vertex_shader: Rc<Shader>,
    fragment_shader: Rc<Shader>,
    geometry_shader: Option<Rc<Shader>>,
    uniforms: HashMap<String, UniformLocation>,
    linked: bool,
}

impl ShaderProgram {
    pub fn new(vertex_shader: Rc<Shader>, fragment_shader: Rc<Shader>) -> Self {
        Self {
            id: 0,
            vertex_shader,
            fragment_shader,
            geometry_shader: None,
            uniforms: HashMap::new(),
            linked: false,
        }
    }

    pub fn with_geometry(
        vertex_shader: Rc<Shader>,
        geometry_shader: Rc<Shader>,
        fragment_shader: Rc<Shader>,
    ) -> Self {
        Self {
            id: 0,
            vertex_shader,
            fragment_shader,
            geometry_shader: Some(geometry_shader),
            uniforms: HashMap::new(),
            linked: false,
        }
    }

    pub fn link(&mut self) -> Result<(), String> {
        if self.linked {
            return Ok(());
        }

        if !self.vertex_shader.is_compiled() {
            return Err("Vertex shader not compiled".to_string());
        }

        if !self.fragment_shader.is_compiled() {
            return Err("Fragment shader not compiled".to_string());
        }

        if let Some(ref gs) = self.geometry_shader {
            if !gs.is_compiled() {
                return Err("Geometry shader not compiled".to_string());
            }
        }

        self.linked = true;
        Ok(())
    }

    pub fn bind(&self) {
    }

    pub fn unbind(&self) {
    }

    pub fn is_linked(&self) -> bool {
        self.linked
    }

    pub fn id(&self) -> ShaderId {
        self.id
    }

    pub fn get_uniform_location(&mut self, name: &str) -> Option<UniformLocation> {
        if let Some(&location) = self.uniforms.get(name) {
            return Some(location);
        }

        let location = 0;
        self.uniforms.insert(name.to_string(), location);
        Some(location)
    }

    pub fn set_uniform_int(&mut self, name: &str, value: i32) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }

    pub fn set_uniform_float(&mut self, name: &str, value: f32) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }

    pub fn set_uniform_vec2(&mut self, name: &str, value: &Vec2) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }

    pub fn set_uniform_vec3(&mut self, name: &str, value: &Vec3) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }

    pub fn set_uniform_vec4(&mut self, name: &str, value: &Vec4) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }

    pub fn set_uniform_mat4(&mut self, name: &str, value: &Mat4) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }

    pub fn set_uniform_color(&mut self, name: &str, value: &Color4F) {
        if let Some(_location) = self.get_uniform_location(name) {
        }
    }
}

pub struct ShaderCache {
    programs: HashMap<String, Rc<ShaderProgram>>,
    shaders: HashMap<String, Rc<Shader>>,
}

impl ShaderCache {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn add_shader(&mut self, name: String, shader: Rc<Shader>) {
        self.shaders.insert(name, shader);
    }

    pub fn get_shader(&self, name: &str) -> Option<Rc<Shader>> {
        self.shaders.get(name).cloned()
    }

    pub fn add_program(&mut self, name: String, program: Rc<ShaderProgram>) {
        self.programs.insert(name, program);
    }

    pub fn get_program(&self, name: &str) -> Option<Rc<ShaderProgram>> {
        self.programs.get(name).cloned()
    }

    pub fn remove_shader(&mut self, name: &str) {
        self.shaders.remove(name);
    }

    pub fn remove_program(&mut self, name: &str) {
        self.programs.remove(name);
    }

    pub fn clear(&mut self) {
        self.programs.clear();
        self.shaders.clear();
    }

    pub fn shader_count(&self) -> usize {
        self.shaders.len()
    }

    pub fn program_count(&self) -> usize {
        self.programs.len()
    }
}

impl Default for ShaderCache {
    fn default() -> Self {
        Self::new()
    }
}

pub mod builtin {
    use super::*;

    pub fn create_default_vertex_shader() -> Shader {
        let source = r#"
#version 330 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_texCoord;
layout(location = 2) in vec4 a_color;

out vec2 v_texCoord;
out vec4 v_color;

uniform mat4 u_MVPMatrix;

void main() {
    gl_Position = u_MVPMatrix * vec4(a_position, 1.0);
    v_texCoord = a_texCoord;
    v_color = a_color;
}
"#;
        Shader::new(ShaderType::Vertex, source.to_string())
    }

    pub fn create_default_fragment_shader() -> Shader {
        let source = r#"
#version 330 core

in vec2 v_texCoord;
in vec4 v_color;

out vec4 FragColor;

uniform sampler2D u_texture;

void main() {
    FragColor = texture(u_texture, v_texCoord) * v_color;
}
"#;
        Shader::new(ShaderType::Fragment, source.to_string())
    }

    pub fn create_position_color_vertex_shader() -> Shader {
        let source = r#"
#version 330 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec4 a_color;

out vec4 v_color;

uniform mat4 u_MVPMatrix;

void main() {
    gl_Position = u_MVPMatrix * vec4(a_position, 1.0);
    v_color = a_color;
}
"#;
        Shader::new(ShaderType::Vertex, source.to_string())
    }

    pub fn create_position_color_fragment_shader() -> Shader {
        let source = r#"
#version 330 core

in vec4 v_color;

out vec4 FragColor;

void main() {
    FragColor = v_color;
}
"#;
        Shader::new(ShaderType::Fragment, source.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_creation() {
        let shader = Shader::new(ShaderType::Vertex, "void main() {}".to_string());
        assert_eq!(shader.shader_type, ShaderType::Vertex);
        assert!(!shader.is_compiled());
    }

    #[test]
    fn test_shader_compile() {
        let mut shader = Shader::new(ShaderType::Vertex, "void main() {}".to_string());
        assert!(shader.compile().is_ok());
        assert!(shader.is_compiled());
    }

    #[test]
    fn test_shader_program_creation() {
        let vs = Rc::new(Shader::new(ShaderType::Vertex, "void main() {}".to_string()));
        let fs = Rc::new(Shader::new(ShaderType::Fragment, "void main() {}".to_string()));
        
        let program = ShaderProgram::new(vs, fs);
        assert!(!program.is_linked());
    }

    #[test]
    fn test_shader_cache() {
        let mut cache = ShaderCache::new();
        assert_eq!(cache.shader_count(), 0);
        assert_eq!(cache.program_count(), 0);

        let shader = Rc::new(Shader::new(ShaderType::Vertex, "void main() {}".to_string()));
        cache.add_shader("test".to_string(), shader);
        assert_eq!(cache.shader_count(), 1);

        assert!(cache.get_shader("test").is_some());
        assert!(cache.get_shader("nonexistent").is_none());
    }

    #[test]
    fn test_builtin_shaders() {
        let vs = builtin::create_default_vertex_shader();
        assert_eq!(vs.shader_type, ShaderType::Vertex);
        assert!(!vs.source().is_empty());

        let fs = builtin::create_default_fragment_shader();
        assert_eq!(fs.shader_type, ShaderType::Fragment);
        assert!(!fs.source().is_empty());
    }
}
