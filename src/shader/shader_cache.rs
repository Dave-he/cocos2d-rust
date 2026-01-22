use super::shader_program::ShaderProgram;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// 着色器缓存
/// 管理所有加载的着色器程序，避免重复编译
pub struct ShaderCache {
    /// 着色器程序缓存
    programs: HashMap<String, Rc<RefCell<ShaderProgram>>>,
}

impl ShaderCache {
    /// 创建新的着色器缓存
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    /// 添加着色器程序
    pub fn add_program(&mut self, program: ShaderProgram) {
        let name = program.name().to_string();
        self.programs.insert(name, Rc::new(RefCell::new(program)));
    }

    /// 添加着色器程序（使用 Rc）
    pub fn add_program_rc(&mut self, program: Rc<RefCell<ShaderProgram>>) {
        let name = program.borrow().name().to_string();
        self.programs.insert(name, program);
    }

    /// 通过名称获取着色器程序
    pub fn get_program(&self, name: &str) -> Option<Rc<RefCell<ShaderProgram>>> {
        self.programs.get(name).cloned()
    }

    /// 移除着色器程序
    pub fn remove_program(&mut self, name: &str) -> bool {
        self.programs.remove(name).is_some()
    }

    /// 清空所有着色器程序
    pub fn clear(&mut self) {
        self.programs.clear();
    }

    /// 获取着色器程序数量
    pub fn program_count(&self) -> usize {
        self.programs.len()
    }

    /// 检查是否存在指定着色器程序
    pub fn has_program(&self, name: &str) -> bool {
        self.programs.contains_key(name)
    }

    /// 获取所有着色器程序名称
    pub fn program_names(&self) -> Vec<String> {
        self.programs.keys().cloned().collect()
    }

    /// 从文件加载着色器
    pub fn load_program_from_files(
        &mut self,
        name: impl Into<String>,
        vertex_file: &str,
        fragment_file: &str,
    ) -> Result<Rc<RefCell<ShaderProgram>>, String> {
        use std::fs;
        
        let name = name.into();
        
        // 检查是否已存在
        if let Some(program) = self.get_program(&name) {
            return Ok(program);
        }
        
        // 读取顶点着色器源码
        let vertex_source = fs::read_to_string(vertex_file)
            .map_err(|e| format!("Failed to read vertex shader file '{}': {}", vertex_file, e))?;
        
        // 读取片段着色器源码
        let fragment_source = fs::read_to_string(fragment_file)
            .map_err(|e| format!("Failed to read fragment shader file '{}': {}", fragment_file, e))?;
        
        // 创建并编译着色器程序
        self.load_program_from_source(name, vertex_source, fragment_source)
    }

    /// 从源码创建并缓存着色器
    pub fn load_program_from_source(
        &mut self,
        name: impl Into<String>,
        vertex_source: impl Into<String>,
        fragment_source: impl Into<String>,
    ) -> Result<Rc<RefCell<ShaderProgram>>, String> {
        let name = name.into();
        
        // 检查是否已存在
        if let Some(program) = self.get_program(&name) {
            return Ok(program);
        }

        // 创建并编译新程序
        let mut program = ShaderProgram::from_source(
            name.clone(),
            vertex_source,
            fragment_source,
        );
        
        program.compile()?;
        
        let program = Rc::new(RefCell::new(program));
        self.programs.insert(name, program.clone());
        
        Ok(program)
    }

    /// 重新加载着色器（用于热重载）
    pub fn reload_program(&mut self, name: &str) -> Result<(), String> {
        if let Some(program) = self.get_program(name) {
            let mut program_mut = program.borrow_mut();
            program_mut.compile()?;
            Ok(())
        } else {
            Err(format!("Shader program '{}' not found", name))
        }
    }

    /// 重新加载所有着色器
    pub fn reload_all(&mut self) -> Result<(), String> {
        for name in self.program_names() {
            self.reload_program(&name)?;
        }
        Ok(())
    }

    /// 获取共享实例（单例模式）
    pub fn shared() -> &'static RefCell<ShaderCache> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<RefCell<ShaderCache>> = OnceLock::new();
        INSTANCE.get_or_init(|| RefCell::new(ShaderCache::new()))
    }

    /// 预加载内置着色器
    pub fn preload_built_in_shaders(&mut self) {
        use super::built_in_shaders::BuiltInShaders;
        
        // 遍历所有内置着色器并加载
        for shader_name in BuiltInShaders::shader_names() {
            if let Some((vertex_source, fragment_source)) = BuiltInShaders::get_shader_source(shader_name) {
                match self.load_program_from_source(shader_name, vertex_source, fragment_source) {
                    Ok(_) => {
                        // 着色器加载成功
                    }
                    Err(e) => {
                        eprintln!("Failed to load built-in shader '{}': {}", shader_name, e);
                    }
                }
            }
        }
    }
}

impl Default for ShaderCache {
    fn default() -> Self {
        Self::new()
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
            FragColor = vec4(1.0);
        }
    "#;

    #[test]
    fn test_shader_cache_creation() {
        let cache = ShaderCache::new();
        assert_eq!(cache.program_count(), 0);
    }

    #[test]
    fn test_add_and_get_program() {
        let mut cache = ShaderCache::new();
        let program = ShaderProgram::from_source("test", VERTEX_SHADER, FRAGMENT_SHADER);
        
        cache.add_program(program);
        assert_eq!(cache.program_count(), 1);
        
        let retrieved = cache.get_program("test");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_remove_program() {
        let mut cache = ShaderCache::new();
        cache.add_program(ShaderProgram::from_source("test1", VERTEX_SHADER, FRAGMENT_SHADER));
        cache.add_program(ShaderProgram::from_source("test2", VERTEX_SHADER, FRAGMENT_SHADER));
        
        assert_eq!(cache.program_count(), 2);
        
        assert!(cache.remove_program("test1"));
        assert_eq!(cache.program_count(), 1);
        
        assert!(!cache.remove_program("nonexistent"));
    }

    #[test]
    fn test_has_program() {
        let mut cache = ShaderCache::new();
        cache.add_program(ShaderProgram::from_source("test", VERTEX_SHADER, FRAGMENT_SHADER));
        
        assert!(cache.has_program("test"));
        assert!(!cache.has_program("nonexistent"));
    }

    #[test]
    fn test_clear() {
        let mut cache = ShaderCache::new();
        cache.add_program(ShaderProgram::from_source("test1", VERTEX_SHADER, FRAGMENT_SHADER));
        cache.add_program(ShaderProgram::from_source("test2", VERTEX_SHADER, FRAGMENT_SHADER));
        
        assert_eq!(cache.program_count(), 2);
        
        cache.clear();
        assert_eq!(cache.program_count(), 0);
    }

    #[test]
    fn test_program_names() {
        let mut cache = ShaderCache::new();
        cache.add_program(ShaderProgram::from_source("shader1", VERTEX_SHADER, FRAGMENT_SHADER));
        cache.add_program(ShaderProgram::from_source("shader2", VERTEX_SHADER, FRAGMENT_SHADER));
        
        let names = cache.program_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"shader1".to_string()));
        assert!(names.contains(&"shader2".to_string()));
    }

    #[test]
    fn test_load_program_from_source() {
        let mut cache = ShaderCache::new();
        
        let result = cache.load_program_from_source("test", VERTEX_SHADER, FRAGMENT_SHADER);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.borrow().name(), "test");
        assert!(program.borrow().is_ready());
        
        // 再次加载应该返回缓存的版本
        let result2 = cache.load_program_from_source("test", VERTEX_SHADER, FRAGMENT_SHADER);
        assert!(result2.is_ok());
        
        // 验证是同一个实例
        assert!(Rc::ptr_eq(&program, &result2.unwrap()));
    }

    #[test]
    fn test_shared_instance() {
        let cache1 = ShaderCache::shared();
        let cache2 = ShaderCache::shared();
        
        // 验证是同一个实例
        cache1.borrow_mut().add_program(
            ShaderProgram::from_source("shared", VERTEX_SHADER, FRAGMENT_SHADER)
        );
        assert!(cache2.borrow().has_program("shared"));
        
        // 清理
        cache1.borrow_mut().clear();
    }
}
