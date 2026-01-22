use std::collections::HashMap;

/// 着色器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    /// 顶点着色器
    Vertex,
    /// 片段着色器
    Fragment,
    /// 几何着色器
    Geometry,
    /// 计算着色器
    Compute,
}

/// Uniform 变量位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniformLocation(pub i32);

/// Attribute 变量位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeLocation(pub i32);

/// 着色器程序状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderProgramState {
    /// 未初始化
    Uninitialized,
    /// 编译中
    Compiling,
    /// 链接中
    Linking,
    /// 就绪
    Ready,
    /// 错误
    Error,
}

/// 着色器程序
pub struct ShaderProgram {
    /// 程序名称
    name: String,
    /// 程序 ID（OpenGL 程序对象）
    program_id: u32,
    /// 顶点着色器源码
    vertex_source: String,
    /// 片段着色器源码
    fragment_source: String,
    /// Uniform 位置缓存
    uniform_locations: HashMap<String, UniformLocation>,
    /// Attribute 位置缓存
    attribute_locations: HashMap<String, AttributeLocation>,
    /// 程序状态
    state: ShaderProgramState,
    /// 编译日志
    compile_log: String,
}

impl ShaderProgram {
    /// 创建新的着色器程序
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            program_id: 0,
            vertex_source: String::new(),
            fragment_source: String::new(),
            uniform_locations: HashMap::new(),
            attribute_locations: HashMap::new(),
            state: ShaderProgramState::Uninitialized,
            compile_log: String::new(),
        }
    }

    /// 从源码创建
    pub fn from_source(
        name: impl Into<String>,
        vertex_source: impl Into<String>,
        fragment_source: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            program_id: 0,
            vertex_source: vertex_source.into(),
            fragment_source: fragment_source.into(),
            uniform_locations: HashMap::new(),
            attribute_locations: HashMap::new(),
            state: ShaderProgramState::Uninitialized,
            compile_log: String::new(),
        }
    }

    /// 获取程序名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取程序 ID
    pub fn program_id(&self) -> u32 {
        self.program_id
    }

    /// 设置顶点着色器源码
    pub fn set_vertex_source(&mut self, source: impl Into<String>) {
        self.vertex_source = source.into();
        self.state = ShaderProgramState::Uninitialized;
    }

    /// 设置片段着色器源码
    pub fn set_fragment_source(&mut self, source: impl Into<String>) {
        self.fragment_source = source.into();
        self.state = ShaderProgramState::Uninitialized;
    }

    /// 获取顶点着色器源码
    pub fn vertex_source(&self) -> &str {
        &self.vertex_source
    }

    /// 获取片段着色器源码
    pub fn fragment_source(&self) -> &str {
        &self.fragment_source
    }

    /// 编译和链接着色器程序
    pub fn compile(&mut self) -> Result<(), String> {
        if self.vertex_source.is_empty() || self.fragment_source.is_empty() {
            return Err("Vertex or fragment shader source is empty".to_string());
        }

        self.state = ShaderProgramState::Compiling;
        
        // TODO: 实现实际的 OpenGL 编译逻辑
        // 这里需要调用 OpenGL API：
        // 1. glCreateShader
        // 2. glShaderSource
        // 3. glCompileShader
        // 4. glGetShaderiv (检查编译状态)
        // 5. glCreateProgram
        // 6. glAttachShader
        // 7. glLinkProgram
        // 8. glGetProgramiv (检查链接状态)

        // 模拟成功编译
        self.program_id = 1; // 实际应该从 glCreateProgram 获取
        self.state = ShaderProgramState::Ready;
        self.compile_log = "Compilation successful (simulated)".to_string();

        Ok(())
    }

    /// 使用此着色器程序
    pub fn use_program(&self) {
        if self.state != ShaderProgramState::Ready {
            return;
        }
        
        // TODO: 调用 glUseProgram(self.program_id)
    }

    /// 获取 Uniform 位置
    pub fn get_uniform_location(&mut self, name: &str) -> Option<UniformLocation> {
        // 先查缓存
        if let Some(&location) = self.uniform_locations.get(name) {
            return Some(location);
        }

        if self.state != ShaderProgramState::Ready {
            return None;
        }

        // TODO: 调用 glGetUniformLocation
        // 这里模拟返回一个位置
        let location = UniformLocation(self.uniform_locations.len() as i32);
        self.uniform_locations.insert(name.to_string(), location);
        Some(location)
    }

    /// 获取 Attribute 位置
    pub fn get_attribute_location(&mut self, name: &str) -> Option<AttributeLocation> {
        // 先查缓存
        if let Some(&location) = self.attribute_locations.get(name) {
            return Some(location);
        }

        if self.state != ShaderProgramState::Ready {
            return None;
        }

        // TODO: 调用 glGetAttribLocation
        let location = AttributeLocation(self.attribute_locations.len() as i32);
        self.attribute_locations.insert(name.to_string(), location);
        Some(location)
    }

    /// 设置 Uniform float
    pub fn set_uniform_float(&self, location: UniformLocation, value: f32) {
        // TODO: 调用 glUniform1f(location.0, value)
        let _ = (location, value); // 避免未使用警告
    }

    /// 设置 Uniform vec2
    pub fn set_uniform_vec2(&self, location: UniformLocation, x: f32, y: f32) {
        // TODO: 调用 glUniform2f(location.0, x, y)
        let _ = (location, x, y);
    }

    /// 设置 Uniform vec3
    pub fn set_uniform_vec3(&self, location: UniformLocation, x: f32, y: f32, z: f32) {
        // TODO: 调用 glUniform3f(location.0, x, y, z)
        let _ = (location, x, y, z);
    }

    /// 设置 Uniform vec4
    pub fn set_uniform_vec4(&self, location: UniformLocation, x: f32, y: f32, z: f32, w: f32) {
        // TODO: 调用 glUniform4f(location.0, x, y, z, w)
        let _ = (location, x, y, z, w);
    }

    /// 设置 Uniform mat4
    pub fn set_uniform_mat4(&self, location: UniformLocation, matrix: &[f32; 16]) {
        // TODO: 调用 glUniformMatrix4fv(location.0, 1, GL_FALSE, matrix.as_ptr())
        let _ = (location, matrix);
    }

    /// 设置 Uniform int
    pub fn set_uniform_int(&self, location: UniformLocation, value: i32) {
        // TODO: 调用 glUniform1i(location.0, value)
        let _ = (location, value);
    }

    /// 获取程序状态
    pub fn state(&self) -> ShaderProgramState {
        self.state
    }

    /// 是否就绪
    pub fn is_ready(&self) -> bool {
        self.state == ShaderProgramState::Ready
    }

    /// 获取编译日志
    pub fn compile_log(&self) -> &str {
        &self.compile_log
    }

    /// 清理资源
    pub fn destroy(&mut self) {
        if self.program_id != 0 {
            // TODO: 调用 glDeleteProgram(self.program_id)
            self.program_id = 0;
        }
        self.state = ShaderProgramState::Uninitialized;
        self.uniform_locations.clear();
        self.attribute_locations.clear();
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl std::fmt::Debug for ShaderProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShaderProgram")
            .field("name", &self.name)
            .field("program_id", &self.program_id)
            .field("state", &self.state)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VERTEX_SHADER: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        void main() {
            gl_Position = vec4(aPos, 1.0);
        }
    "#;

    const FRAGMENT_SHADER: &str = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0, 0.5, 0.2, 1.0);
        }
    "#;

    #[test]
    fn test_shader_program_creation() {
        let program = ShaderProgram::new("test_shader");
        assert_eq!(program.name(), "test_shader");
        assert_eq!(program.state(), ShaderProgramState::Uninitialized);
    }

    #[test]
    fn test_shader_program_from_source() {
        let program = ShaderProgram::from_source(
            "test",
            VERTEX_SHADER,
            FRAGMENT_SHADER,
        );
        
        assert_eq!(program.name(), "test");
        assert!(!program.vertex_source().is_empty());
        assert!(!program.fragment_source().is_empty());
    }

    #[test]
    fn test_shader_program_compile() {
        let mut program = ShaderProgram::from_source(
            "test",
            VERTEX_SHADER,
            FRAGMENT_SHADER,
        );
        
        let result = program.compile();
        assert!(result.is_ok());
        assert_eq!(program.state(), ShaderProgramState::Ready);
    }

    #[test]
    fn test_shader_program_compile_error() {
        let mut program = ShaderProgram::new("test");
        
        let result = program.compile();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_uniform_location() {
        let mut program = ShaderProgram::from_source(
            "test",
            VERTEX_SHADER,
            FRAGMENT_SHADER,
        );
        
        program.compile().unwrap();
        
        let location1 = program.get_uniform_location("uMVP");
        assert!(location1.is_some());
        
        // 第二次应该从缓存获取
        let location2 = program.get_uniform_location("uMVP");
        assert_eq!(location1, location2);
    }

    #[test]
    fn test_get_attribute_location() {
        let mut program = ShaderProgram::from_source(
            "test",
            VERTEX_SHADER,
            FRAGMENT_SHADER,
        );
        
        program.compile().unwrap();
        
        let location = program.get_attribute_location("aPosition");
        assert!(location.is_some());
    }

    #[test]
    fn test_shader_program_state() {
        let program = ShaderProgram::new("test");
        assert!(!program.is_ready());
        
        let mut program = ShaderProgram::from_source("test", VERTEX_SHADER, FRAGMENT_SHADER);
        program.compile().unwrap();
        assert!(program.is_ready());
    }
}
