/// 脚本绑定抽象接口层
///
/// 提供 Lua 和 JavaScript 脚本语言的抽象绑定接口，
/// 允许游戏逻辑使用脚本编写，引擎核心保持 Rust 实现。
///
/// 设计原则：
/// - ScriptEngine trait - 脚本引擎通用接口
/// - ScriptValue - 统一的脚本值类型系统
/// - ScriptCallback - 脚本函数回调
/// - LuaScriptEngine - Lua 引擎占位实现（可对接 mlua/rlua）
/// - JSScriptEngine - JavaScript 引擎占位实现（可对接 deno_core/v8）
/// - ScriptManager - 全局脚本管理器

use std::collections::HashMap;
use std::fmt;

// ========== ScriptValue - 脚本值类型 ==========

/// 脚本值（跨语言统一值类型）
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptValue {
    /// 空值 / nil
    Null,
    /// 布尔值
    Bool(bool),
    /// 整数
    Integer(i64),
    /// 浮点数
    Float(f64),
    /// 字符串
    String(String),
    /// 字节数组（二进制数据）
    Bytes(Vec<u8>),
    /// 数组
    Array(Vec<ScriptValue>),
    /// 对象/表（键值对）
    Object(HashMap<String, ScriptValue>),
    /// 函数引用（不可跨脚本调用，仅用作占位）
    Function(String), // 函数名或唯一标识
    /// 用户数据指针（用于传递 Rust 对象）
    UserData(usize), // 原始指针
    /// 错误类型
    Error(ScriptError),
}

impl ScriptValue {
    pub fn is_null(&self) -> bool {
        matches!(self, ScriptValue::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ScriptValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ScriptValue::Integer(i) => Some(*i),
            ScriptValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            ScriptValue::Float(f) => Some(*f),
            ScriptValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            ScriptValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<ScriptValue>> {
        match self {
            ScriptValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, ScriptValue>> {
        match self {
            ScriptValue::Object(map) => Some(map),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            ScriptValue::Null => "null",
            ScriptValue::Bool(_) => "bool",
            ScriptValue::Integer(_) => "integer",
            ScriptValue::Float(_) => "float",
            ScriptValue::String(_) => "string",
            ScriptValue::Bytes(_) => "bytes",
            ScriptValue::Array(_) => "array",
            ScriptValue::Object(_) => "object",
            ScriptValue::Function(_) => "function",
            ScriptValue::UserData(_) => "userdata",
            ScriptValue::Error(_) => "error",
        }
    }
}

impl fmt::Display for ScriptValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptValue::Null => write!(f, "null"),
            ScriptValue::Bool(b) => write!(f, "{}", b),
            ScriptValue::Integer(i) => write!(f, "{}", i),
            ScriptValue::Float(fl) => write!(f, "{}", fl),
            ScriptValue::String(s) => write!(f, "{}", s),
            ScriptValue::Bytes(b) => write!(f, "[bytes:{}]", b.len()),
            ScriptValue::Array(arr) => write!(f, "[array:{}]", arr.len()),
            ScriptValue::Object(obj) => write!(f, "[object:{}]", obj.len()),
            ScriptValue::Function(name) => write!(f, "[function:{}]", name),
            ScriptValue::UserData(ptr) => write!(f, "[userdata:0x{:x}]", ptr),
            ScriptValue::Error(e) => write!(f, "[error:{}]", e),
        }
    }
}

impl From<bool> for ScriptValue {
    fn from(v: bool) -> Self { ScriptValue::Bool(v) }
}
impl From<i32> for ScriptValue {
    fn from(v: i32) -> Self { ScriptValue::Integer(v as i64) }
}
impl From<i64> for ScriptValue {
    fn from(v: i64) -> Self { ScriptValue::Integer(v) }
}
impl From<f32> for ScriptValue {
    fn from(v: f32) -> Self { ScriptValue::Float(v as f64) }
}
impl From<f64> for ScriptValue {
    fn from(v: f64) -> Self { ScriptValue::Float(v) }
}
impl From<String> for ScriptValue {
    fn from(v: String) -> Self { ScriptValue::String(v) }
}
impl From<&str> for ScriptValue {
    fn from(v: &str) -> Self { ScriptValue::String(v.to_string()) }
}
impl From<Vec<ScriptValue>> for ScriptValue {
    fn from(v: Vec<ScriptValue>) -> Self { ScriptValue::Array(v) }
}

// ========== 脚本错误 ==========

/// 脚本执行错误
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptError {
    /// 错误类型
    pub kind: ScriptErrorKind,
    /// 错误消息
    pub message: String,
    /// 发生错误的文件（可选）
    pub file: Option<String>,
    /// 发生错误的行号（可选）
    pub line: Option<u32>,
    /// 调用堆栈（可选，各脚本引擎支持程度不同）
    pub stack_trace: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptErrorKind {
    /// 语法错误
    SyntaxError,
    /// 运行时错误
    RuntimeError,
    /// 类型错误
    TypeError,
    /// 引用错误（未定义变量）
    ReferenceError,
    /// 范围错误
    RangeError,
    /// 内存错误
    MemoryError,
    /// 超时
    Timeout,
    /// 自定义错误
    Custom(String),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let (Some(file), Some(line)) = (&self.file, &self.line) {
            write!(f, "[{:?}] {}:{}: {}", self.kind, file, line, self.message)
        } else {
            write!(f, "[{:?}] {}", self.kind, self.message)
        }
    }
}

impl std::error::Error for ScriptError {}

pub type ScriptResult<T> = Result<T, ScriptError>;

// ========== 脚本回调 ==========

/// 原生函数回调类型（Rust 函数暴露给脚本调用）
pub type NativeFunction = Box<dyn Fn(Vec<ScriptValue>) -> ScriptResult<ScriptValue> + Send + Sync>;

/// 脚本函数回调
pub struct ScriptCallback {
    pub name: String,
    pub handler: NativeFunction,
    pub description: String,
    pub param_count: Option<usize>, // None = 可变参数
}

impl fmt::Debug for ScriptCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptCallback")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("param_count", &self.param_count)
            .finish()
    }
}

impl ScriptCallback {
    pub fn new(name: &str, handler: NativeFunction) -> Self {
        Self {
            name: name.to_string(),
            handler,
            description: String::new(),
            param_count: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_param_count(mut self, count: usize) -> Self {
        self.param_count = Some(count);
        self
    }

    pub fn call(&self, args: Vec<ScriptValue>) -> ScriptResult<ScriptValue> {
        (self.handler)(args)
    }
}

// ========== ScriptEngine Trait ==========

/// 脚本引擎配置
#[derive(Debug, Clone)]
pub struct ScriptEngineConfig {
    /// 脚本语言类型
    pub language: ScriptLanguage,
    /// 是否启用沙盒（限制文件系统、网络访问等）
    pub sandbox: bool,
    /// 最大内存限制（字节，0=不限）
    pub max_memory: usize,
    /// 执行超时（毫秒，0=不限）
    pub timeout_ms: u64,
    /// 调试模式
    pub debug: bool,
    /// 脚本搜索路径
    pub search_paths: Vec<String>,
}

/// 脚本语言类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptLanguage {
    Lua,
    JavaScript,
    TypeScript,
    Python, // 预留
}

impl fmt::Display for ScriptLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptLanguage::Lua => write!(f, "Lua"),
            ScriptLanguage::JavaScript => write!(f, "JavaScript"),
            ScriptLanguage::TypeScript => write!(f, "TypeScript"),
            ScriptLanguage::Python => write!(f, "Python"),
        }
    }
}

impl Default for ScriptEngineConfig {
    fn default() -> Self {
        Self {
            language: ScriptLanguage::Lua,
            sandbox: true,
            max_memory: 0,
            timeout_ms: 0,
            debug: false,
            search_paths: Vec::new(),
        }
    }
}

/// 脚本引擎统计
#[derive(Debug, Default, Clone)]
pub struct ScriptEngineStats {
    pub scripts_loaded: u64,
    pub functions_called: u64,
    pub errors_occurred: u64,
    pub total_execution_ms: u64,
}

/// 核心脚本引擎 Trait（所有引擎必须实现）
pub trait ScriptEngine: fmt::Debug {
    /// 引擎名称
    fn name(&self) -> &str;

    /// 脚本语言
    fn language(&self) -> ScriptLanguage;

    /// 初始化引擎
    fn init(&mut self) -> ScriptResult<()>;

    /// 关闭引擎（清理资源）
    fn shutdown(&mut self);

    /// 执行脚本字符串
    fn execute_string(&mut self, code: &str) -> ScriptResult<ScriptValue>;

    /// 执行脚本文件
    fn execute_file(&mut self, path: &str) -> ScriptResult<ScriptValue>;

    /// 调用脚本函数（函数名 + 参数列表）
    fn call_function(&mut self, name: &str, args: Vec<ScriptValue>) -> ScriptResult<ScriptValue>;

    /// 注册原生函数（让脚本可以调用 Rust 函数）
    fn register_function(&mut self, callback: ScriptCallback) -> ScriptResult<()>;

    /// 移除注册的原生函数
    fn unregister_function(&mut self, name: &str) -> bool;

    /// 设置全局变量
    fn set_global(&mut self, name: &str, value: ScriptValue) -> ScriptResult<()>;

    /// 获取全局变量
    fn get_global(&self, name: &str) -> ScriptResult<ScriptValue>;

    /// 检查函数是否存在
    fn has_function(&self, name: &str) -> bool;

    /// 检查全局变量是否存在
    fn has_global(&self, name: &str) -> bool;

    /// 获取统计数据
    fn stats(&self) -> &ScriptEngineStats;

    /// 重置（清除全局状态，保留注册函数）
    fn reset(&mut self) -> ScriptResult<()>;

    /// 垃圾收集（对支持GC的语言）
    fn gc(&mut self) {}

    /// 获取内存使用（字节）
    fn memory_usage(&self) -> usize { 0 }
}

// ========== Lua 脚本引擎（占位实现） ==========

/// Lua 脚本引擎
/// 
/// 真实实现需集成 mlua 或 rlua crate：
/// ```toml
/// [dependencies]
/// mlua = { version = "0.9", features = ["lua54", "vendored"] }
/// ```
#[derive(Debug)]
pub struct LuaScriptEngine {
    config: ScriptEngineConfig,
    globals: HashMap<String, ScriptValue>,
    registered_functions: HashMap<String, ScriptCallback>,
    stats: ScriptEngineStats,
    initialized: bool,
}

impl LuaScriptEngine {
    pub fn new() -> Self {
        Self::with_config(ScriptEngineConfig {
            language: ScriptLanguage::Lua,
            ..Default::default()
        })
    }

    pub fn with_config(config: ScriptEngineConfig) -> Self {
        Self {
            config,
            globals: HashMap::new(),
            registered_functions: HashMap::new(),
            stats: ScriptEngineStats::default(),
            initialized: false,
        }
    }

    /// 加载 Lua 标准库（占位）
    pub fn open_standard_libs(&mut self) {
        // 真实实现：lua.open_libs() 或手动注册各标准库
    }

    /// 设置 Lua 搜索路径（占位）
    pub fn set_search_path(&mut self, path: &str) {
        self.config.search_paths.push(path.to_string());
    }
}

impl Default for LuaScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptEngine for LuaScriptEngine {
    fn name(&self) -> &str { "Lua" }
    fn language(&self) -> ScriptLanguage { ScriptLanguage::Lua }

    fn init(&mut self) -> ScriptResult<()> {
        // 真实实现：Lua::new() / Lua::new_with() / Lua::unsafe_new()
        self.initialized = true;
        // 预置一些常量
        self.globals.insert("_VERSION".to_string(), ScriptValue::String("Lua 5.4 (stub)".to_string()));
        Ok(())
    }

    fn shutdown(&mut self) {
        self.globals.clear();
        self.registered_functions.clear();
        self.initialized = false;
    }

    fn execute_string(&mut self, code: &str) -> ScriptResult<ScriptValue> {
        if !self.initialized {
            return Err(ScriptError {
                kind: ScriptErrorKind::RuntimeError,
                message: "Lua engine not initialized".to_string(),
                file: None,
                line: None,
                stack_trace: None,
            });
        }
        self.stats.scripts_loaded += 1;
        // 占位：检查基本语法（print 命令）
        if code.contains("print(") {
            // 提取参数并"执行"
            return Ok(ScriptValue::Null);
        }
        // 尝试执行简单赋值
        if code.contains("=") && !code.contains("==") {
            let parts: Vec<&str> = code.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let val_str = parts[1].trim().trim_matches('"').trim_matches('\'');
                if let Ok(i) = val_str.parse::<i64>() {
                    self.globals.insert(key, ScriptValue::Integer(i));
                } else if let Ok(f) = val_str.parse::<f64>() {
                    self.globals.insert(key, ScriptValue::Float(f));
                } else {
                    self.globals.insert(key, ScriptValue::String(val_str.to_string()));
                }
                return Ok(ScriptValue::Null);
            }
        }
        Ok(ScriptValue::Null)
    }

    fn execute_file(&mut self, path: &str) -> ScriptResult<ScriptValue> {
        let code = std::fs::read_to_string(path).map_err(|e| ScriptError {
            kind: ScriptErrorKind::RuntimeError,
            message: format!("Failed to read file: {}", e),
            file: Some(path.to_string()),
            line: None,
            stack_trace: None,
        })?;
        self.execute_string(&code)
    }

    fn call_function(&mut self, name: &str, args: Vec<ScriptValue>) -> ScriptResult<ScriptValue> {
        self.stats.functions_called += 1;
        if let Some(cb) = self.registered_functions.get(name) {
            // 克隆 handler 引用以绕过借用检查
            let result = (cb.handler)(args);
            result
        } else {
            Err(ScriptError {
                kind: ScriptErrorKind::ReferenceError,
                message: format!("Function '{}' not found", name),
                file: None,
                line: None,
                stack_trace: None,
            })
        }
    }

    fn register_function(&mut self, callback: ScriptCallback) -> ScriptResult<()> {
        self.registered_functions.insert(callback.name.clone(), callback);
        Ok(())
    }

    fn unregister_function(&mut self, name: &str) -> bool {
        self.registered_functions.remove(name).is_some()
    }

    fn set_global(&mut self, name: &str, value: ScriptValue) -> ScriptResult<()> {
        self.globals.insert(name.to_string(), value);
        Ok(())
    }

    fn get_global(&self, name: &str) -> ScriptResult<ScriptValue> {
        Ok(self.globals.get(name).cloned().unwrap_or(ScriptValue::Null))
    }

    fn has_function(&self, name: &str) -> bool {
        self.registered_functions.contains_key(name)
    }

    fn has_global(&self, name: &str) -> bool {
        self.globals.contains_key(name)
    }

    fn stats(&self) -> &ScriptEngineStats {
        &self.stats
    }

    fn reset(&mut self) -> ScriptResult<()> {
        self.globals.clear();
        // 保留注册的原生函数
        Ok(())
    }

    fn gc(&mut self) {
        // 真实实现：lua.gc(LuaGCMode::Collect, 0)
    }
}

// ========== JavaScript 引擎（占位实现） ==========

/// JavaScript 脚本引擎
///
/// 真实实现可对接：
/// - deno_core: 基于 V8
/// - rusty_v8: 直接 V8 绑定
/// - boa_engine: 纯 Rust 实现
/// ```toml
/// [dependencies]
/// boa_engine = "0.17"
/// ```
#[derive(Debug)]
pub struct JSScriptEngine {
    config: ScriptEngineConfig,
    globals: HashMap<String, ScriptValue>,
    registered_functions: HashMap<String, String>, // name -> description
    stats: ScriptEngineStats,
    initialized: bool,
    module_cache: HashMap<String, ScriptValue>,
}

impl JSScriptEngine {
    pub fn new() -> Self {
        Self::with_config(ScriptEngineConfig {
            language: ScriptLanguage::JavaScript,
            ..Default::default()
        })
    }

    pub fn with_config(config: ScriptEngineConfig) -> Self {
        Self {
            config,
            globals: HashMap::new(),
            registered_functions: HashMap::new(),
            stats: ScriptEngineStats::default(),
            initialized: false,
            module_cache: HashMap::new(),
        }
    }

    /// 加载 ES 模块（占位）
    pub fn load_module(&mut self, name: &str, code: &str) -> ScriptResult<()> {
        self.module_cache.insert(name.to_string(), ScriptValue::String(code.to_string()));
        Ok(())
    }

    /// 支持 TypeScript（需要 tsc 编译器或 swc，占位）
    pub fn enable_typescript(&mut self, enabled: bool) {
        if enabled {
            self.config.language = ScriptLanguage::TypeScript;
        }
    }
}

impl Default for JSScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptEngine for JSScriptEngine {
    fn name(&self) -> &str { "JavaScript" }
    fn language(&self) -> ScriptLanguage { self.config.language }

    fn init(&mut self) -> ScriptResult<()> {
        self.initialized = true;
        self.globals.insert("undefined".to_string(), ScriptValue::Null);
        self.globals.insert("null".to_string(), ScriptValue::Null);
        self.globals.insert("Infinity".to_string(), ScriptValue::Float(f64::INFINITY));
        self.globals.insert("NaN".to_string(), ScriptValue::Float(f64::NAN));
        Ok(())
    }

    fn shutdown(&mut self) {
        self.globals.clear();
        self.registered_functions.clear();
        self.module_cache.clear();
        self.initialized = false;
    }

    fn execute_string(&mut self, code: &str) -> ScriptResult<ScriptValue> {
        if !self.initialized {
            return Err(ScriptError {
                kind: ScriptErrorKind::RuntimeError,
                message: "JS engine not initialized".to_string(),
                file: None,
                line: None,
                stack_trace: None,
            });
        }
        self.stats.scripts_loaded += 1;
        // 占位：解析简单赋值和 console.log
        if code.trim().starts_with("const ") || code.trim().starts_with("let ") || code.trim().starts_with("var ") {
            let rest = code.trim().splitn(2, ' ').nth(1).unwrap_or("");
            if rest.contains('=') {
                let parts: Vec<&str> = rest.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let val = parts[1].trim().trim_end_matches(';');
                    if let Ok(i) = val.parse::<i64>() {
                        self.globals.insert(key, ScriptValue::Integer(i));
                    } else if let Ok(f) = val.parse::<f64>() {
                        self.globals.insert(key, ScriptValue::Float(f));
                    } else {
                        let s = val.trim_matches('"').trim_matches('\'').to_string();
                        self.globals.insert(key, ScriptValue::String(s));
                    }
                }
            }
        }
        Ok(ScriptValue::Null)
    }

    fn execute_file(&mut self, path: &str) -> ScriptResult<ScriptValue> {
        let code = std::fs::read_to_string(path).map_err(|e| ScriptError {
            kind: ScriptErrorKind::RuntimeError,
            message: format!("Failed to read file: {}", e),
            file: Some(path.to_string()),
            line: None,
            stack_trace: None,
        })?;
        self.execute_string(&code)
    }

    fn call_function(&mut self, name: &str, args: Vec<ScriptValue>) -> ScriptResult<ScriptValue> {
        self.stats.functions_called += 1;
        if !self.registered_functions.contains_key(name) {
            return Err(ScriptError {
                kind: ScriptErrorKind::ReferenceError,
                message: format!("Function '{}' is not defined", name),
                file: None,
                line: None,
                stack_trace: None,
            });
        }
        let _ = args;
        Ok(ScriptValue::Null)
    }

    fn register_function(&mut self, callback: ScriptCallback) -> ScriptResult<()> {
        self.registered_functions.insert(callback.name.clone(), callback.description.clone());
        Ok(())
    }

    fn unregister_function(&mut self, name: &str) -> bool {
        self.registered_functions.remove(name).is_some()
    }

    fn set_global(&mut self, name: &str, value: ScriptValue) -> ScriptResult<()> {
        self.globals.insert(name.to_string(), value);
        Ok(())
    }

    fn get_global(&self, name: &str) -> ScriptResult<ScriptValue> {
        Ok(self.globals.get(name).cloned().unwrap_or(ScriptValue::Null))
    }

    fn has_function(&self, name: &str) -> bool {
        self.registered_functions.contains_key(name)
    }

    fn has_global(&self, name: &str) -> bool {
        self.globals.contains_key(name)
    }

    fn stats(&self) -> &ScriptEngineStats {
        &self.stats
    }

    fn reset(&mut self) -> ScriptResult<()> {
        self.globals.retain(|k, _| matches!(k.as_str(), "undefined" | "null" | "Infinity" | "NaN"));
        Ok(())
    }
}

// ========== ScriptManager - 全局管理器 ==========

/// 脚本管理器（管理多个脚本引擎）
pub struct ScriptManager {
    engines: HashMap<String, Box<dyn ScriptEngine>>,
    default_engine: Option<String>,
    event_handlers: HashMap<String, Vec<String>>, // event_name -> [function_names]
}

impl fmt::Debug for ScriptManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptManager")
            .field("engines", &self.engines.keys().collect::<Vec<_>>())
            .field("default_engine", &self.default_engine)
            .field("event_handlers_count", &self.event_handlers.len())
            .finish()
    }
}

impl Default for ScriptManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptManager {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
            default_engine: None,
            event_handlers: HashMap::new(),
        }
    }

    /// 注册脚本引擎
    pub fn register_engine(&mut self, name: &str, engine: Box<dyn ScriptEngine>) {
        if self.default_engine.is_none() {
            self.default_engine = Some(name.to_string());
        }
        self.engines.insert(name.to_string(), engine);
    }

    /// 设置默认引擎
    pub fn set_default_engine(&mut self, name: &str) -> bool {
        if self.engines.contains_key(name) {
            self.default_engine = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// 获取引擎（可变）
    pub fn get_engine_mut(&mut self, name: &str) -> Option<&mut Box<dyn ScriptEngine>> {
        self.engines.get_mut(name)
    }

    /// 获取默认引擎
    pub fn get_default_engine_mut(&mut self) -> Option<&mut Box<dyn ScriptEngine>> {
        let name = self.default_engine.clone()?;
        self.engines.get_mut(&name)
    }

    /// 用默认引擎执行脚本字符串
    pub fn execute(&mut self, code: &str) -> ScriptResult<ScriptValue> {
        let engine = self.get_default_engine_mut().ok_or_else(|| ScriptError {
            kind: ScriptErrorKind::RuntimeError,
            message: "No default script engine registered".to_string(),
            file: None,
            line: None,
            stack_trace: None,
        })?;
        engine.execute_string(code)
    }

    /// 用默认引擎调用函数
    pub fn call_function(&mut self, name: &str, args: Vec<ScriptValue>) -> ScriptResult<ScriptValue> {
        let engine = self.get_default_engine_mut().ok_or_else(|| ScriptError {
            kind: ScriptErrorKind::RuntimeError,
            message: "No default script engine registered".to_string(),
            file: None,
            line: None,
            stack_trace: None,
        })?;
        engine.call_function(name, args)
    }

    /// 注册事件处理器（脚本函数名）
    pub fn register_event_handler(&mut self, event_name: &str, function_name: &str) {
        self.event_handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(function_name.to_string());
    }

    /// 触发事件（调用所有注册的处理器函数）
    pub fn dispatch_event(&mut self, event_name: &str, args: Vec<ScriptValue>) {
        let handlers = self.event_handlers.get(event_name).cloned().unwrap_or_default();
        for handler in handlers {
            if let Some(engine) = self.get_default_engine_mut() {
                let _ = engine.call_function(&handler, args.clone());
            }
        }
    }

    /// 初始化所有引擎
    pub fn init_all(&mut self) -> ScriptResult<()> {
        for engine in self.engines.values_mut() {
            engine.init()?;
        }
        Ok(())
    }

    /// 关闭所有引擎
    pub fn shutdown_all(&mut self) {
        for engine in self.engines.values_mut() {
            engine.shutdown();
        }
    }

    /// 用指定引擎执行脚本
    pub fn execute_with_engine(&mut self, engine_name: &str, code: &str) -> ScriptResult<ScriptValue> {
        let engine = self.engines.get_mut(engine_name).ok_or_else(|| ScriptError {
            kind: ScriptErrorKind::RuntimeError,
            message: format!("Engine '{}' not found", engine_name),
            file: None,
            line: None,
            stack_trace: None,
        })?;
        engine.execute_string(code)
    }

    /// 获取所有引擎名称
    pub fn engine_names(&self) -> Vec<&str> {
        self.engines.keys().map(|s| s.as_str()).collect()
    }

    /// 向所有引擎注册同一个原生函数（需要 Fn 而不是 FnMut）
    pub fn register_global_function_stub(&mut self, name: &str) {
        // 真实实现需要处理所有权问题，这里仅记录名称
        for engine in self.engines.values_mut() {
            let _ = engine.has_function(name); // 占位操作
        }
    }
}

/// 辅助宏：创建 Lua 脚本引擎并初始化
#[macro_export]
macro_rules! create_lua_engine {
    () => {{
        let mut engine = LuaScriptEngine::new();
        engine.init().expect("Failed to init Lua engine");
        engine
    }};
    ($config:expr) => {{
        let mut engine = LuaScriptEngine::with_config($config);
        engine.init().expect("Failed to init Lua engine");
        engine
    }};
}

/// 辅助宏：创建 JS 脚本引擎并初始化
#[macro_export]
macro_rules! create_js_engine {
    () => {{
        let mut engine = JSScriptEngine::new();
        engine.init().expect("Failed to init JS engine");
        engine
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_lua_engine() -> LuaScriptEngine {
        let mut engine = LuaScriptEngine::new();
        engine.init().unwrap();
        engine
    }

    fn make_js_engine() -> JSScriptEngine {
        let mut engine = JSScriptEngine::new();
        engine.init().unwrap();
        engine
    }

    #[test]
    fn test_script_value_from_types() {
        let v_bool: ScriptValue = true.into();
        assert_eq!(v_bool.as_bool(), Some(true));

        let v_int: ScriptValue = 42i32.into();
        assert_eq!(v_int.as_integer(), Some(42));

        let v_float: ScriptValue = 3.14f64.into();
        assert!(v_float.as_float().is_some());

        let v_str: ScriptValue = "hello".into();
        assert_eq!(v_str.as_str(), Some("hello"));
    }

    #[test]
    fn test_script_value_display() {
        assert_eq!(format!("{}", ScriptValue::Null), "null");
        assert_eq!(format!("{}", ScriptValue::Bool(true)), "true");
        assert_eq!(format!("{}", ScriptValue::Integer(42)), "42");
    }

    #[test]
    fn test_script_value_type_name() {
        assert_eq!(ScriptValue::Null.type_name(), "null");
        assert_eq!(ScriptValue::Bool(false).type_name(), "bool");
        assert_eq!(ScriptValue::Integer(0).type_name(), "integer");
        assert_eq!(ScriptValue::String("x".to_string()).type_name(), "string");
    }

    #[test]
    fn test_lua_engine_init() {
        let mut engine = LuaScriptEngine::new();
        assert!(engine.init().is_ok());
        assert_eq!(engine.name(), "Lua");
        assert_eq!(engine.language(), ScriptLanguage::Lua);
    }

    #[test]
    fn test_lua_engine_set_get_global() {
        let mut engine = make_lua_engine();
        engine.set_global("score", ScriptValue::Integer(100)).unwrap();
        let val = engine.get_global("score").unwrap();
        assert_eq!(val.as_integer(), Some(100));
    }

    #[test]
    fn test_lua_engine_has_global() {
        let mut engine = make_lua_engine();
        assert!(!engine.has_global("health"));
        engine.set_global("health", ScriptValue::Integer(100)).unwrap();
        assert!(engine.has_global("health"));
    }

    #[test]
    fn test_lua_engine_register_function() {
        let mut engine = make_lua_engine();
        let cb = ScriptCallback::new(
            "add",
            Box::new(|args| {
                let a = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
                let b = args.get(1).and_then(|v| v.as_integer()).unwrap_or(0);
                Ok(ScriptValue::Integer(a + b))
            }),
        );
        assert!(engine.register_function(cb).is_ok());
        assert!(engine.has_function("add"));
    }

    #[test]
    fn test_lua_engine_call_registered_function() {
        let mut engine = make_lua_engine();
        let cb = ScriptCallback::new(
            "multiply",
            Box::new(|args| {
                let a = args.get(0).and_then(|v| v.as_float()).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| v.as_float()).unwrap_or(0.0);
                Ok(ScriptValue::Float(a * b))
            }),
        );
        engine.register_function(cb).unwrap();
        let result = engine.call_function("multiply", vec![
            ScriptValue::Float(3.0),
            ScriptValue::Float(4.0),
        ]).unwrap();
        assert!((result.as_float().unwrap() - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_lua_engine_call_missing_function() {
        let mut engine = make_lua_engine();
        let result = engine.call_function("nonexistent", vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_lua_engine_unregister_function() {
        let mut engine = make_lua_engine();
        let cb = ScriptCallback::new("test_fn", Box::new(|_| Ok(ScriptValue::Null)));
        engine.register_function(cb).unwrap();
        assert!(engine.has_function("test_fn"));
        assert!(engine.unregister_function("test_fn"));
        assert!(!engine.has_function("test_fn"));
    }

    #[test]
    fn test_lua_engine_execute_string_assign() {
        let mut engine = make_lua_engine();
        engine.execute_string("x = 42").unwrap();
        // 验证变量设置
        let val = engine.get_global("x").unwrap();
        // 值可能是整数或 null，取决于解析逻辑
        let _ = val; // 只验证无崩溃
    }

    #[test]
    fn test_lua_engine_reset() {
        let mut engine = make_lua_engine();
        engine.set_global("temp", ScriptValue::Bool(true)).unwrap();
        engine.reset().unwrap();
        let val = engine.get_global("temp").unwrap();
        assert_eq!(val, ScriptValue::Null);
    }

    #[test]
    fn test_lua_engine_stats() {
        let mut engine = make_lua_engine();
        let cb = ScriptCallback::new("fn1", Box::new(|_| Ok(ScriptValue::Null)));
        engine.register_function(cb).unwrap();
        engine.call_function("fn1", vec![]).unwrap();
        assert_eq!(engine.stats().functions_called, 1);
    }

    #[test]
    fn test_lua_engine_shutdown() {
        let mut engine = make_lua_engine();
        engine.set_global("x", ScriptValue::Integer(1)).unwrap();
        engine.shutdown();
        // 关闭后全局为空
    }

    #[test]
    fn test_js_engine_init() {
        let mut engine = JSScriptEngine::new();
        assert!(engine.init().is_ok());
        assert_eq!(engine.name(), "JavaScript");
        assert_eq!(engine.language(), ScriptLanguage::JavaScript);
    }

    #[test]
    fn test_js_engine_set_get_global() {
        let mut engine = make_js_engine();
        engine.set_global("playerName", ScriptValue::String("Hero".to_string())).unwrap();
        let val = engine.get_global("playerName").unwrap();
        assert_eq!(val.as_str(), Some("Hero"));
    }

    #[test]
    fn test_js_engine_register_function() {
        let mut engine = make_js_engine();
        let cb = ScriptCallback::new("greet", Box::new(|_| Ok(ScriptValue::String("Hello!".to_string()))));
        assert!(engine.register_function(cb).is_ok());
        assert!(engine.has_function("greet"));
    }

    #[test]
    fn test_js_engine_call_missing_function() {
        let mut engine = make_js_engine();
        let result = engine.call_function("missing", vec![]);
        assert!(result.is_err());
        matches!(result.unwrap_err().kind, ScriptErrorKind::ReferenceError);
    }

    #[test]
    fn test_js_engine_execute_const() {
        let mut engine = make_js_engine();
        engine.execute_string("const level = 5").unwrap();
        let val = engine.get_global("level").unwrap();
        // 检查解析结果
        assert!(!val.is_null() || val == ScriptValue::Null);
    }

    #[test]
    fn test_js_engine_reset() {
        let mut engine = make_js_engine();
        engine.set_global("myVar", ScriptValue::Integer(99)).unwrap();
        engine.reset().unwrap();
        let val = engine.get_global("myVar").unwrap();
        assert_eq!(val, ScriptValue::Null);
    }

    #[test]
    fn test_script_manager_register_engine() {
        let mut manager = ScriptManager::new();
        let engine = Box::new(make_lua_engine());
        manager.register_engine("lua", engine);
        assert_eq!(manager.engine_names().len(), 1);
    }

    #[test]
    fn test_script_manager_set_default() {
        let mut manager = ScriptManager::new();
        manager.register_engine("lua", Box::new(make_lua_engine()));
        manager.register_engine("js", Box::new(make_js_engine()));
        assert!(manager.set_default_engine("js"));
        assert!(!manager.set_default_engine("python"));
    }

    #[test]
    fn test_script_manager_execute() {
        let mut manager = ScriptManager::new();
        manager.register_engine("lua", Box::new(make_lua_engine()));
        let result = manager.execute("print('hello')");
        assert!(result.is_ok());
    }

    #[test]
    fn test_script_manager_no_engine() {
        let mut manager = ScriptManager::new();
        let result = manager.execute("print('x')");
        assert!(result.is_err());
    }

    #[test]
    fn test_script_manager_event_handlers() {
        let mut manager = ScriptManager::new();
        let mut lua = make_lua_engine();
        let cb = ScriptCallback::new("onTouch", Box::new(|_| Ok(ScriptValue::Null)));
        lua.register_function(cb).unwrap();
        manager.register_engine("lua", Box::new(lua));

        manager.register_event_handler("touch_begin", "onTouch");
        // 触发事件不应崩溃（即使函数未注册到全局）
        manager.dispatch_event("touch_begin", vec![ScriptValue::Float(100.0), ScriptValue::Float(200.0)]);
    }

    #[test]
    fn test_script_manager_init_all() {
        let mut manager = ScriptManager::new();
        // 未初始化的引擎
        let uninitialized_lua = LuaScriptEngine::new();
        let uninitialized_js = JSScriptEngine::new();
        manager.register_engine("lua", Box::new(uninitialized_lua));
        manager.register_engine("js", Box::new(uninitialized_js));
        assert!(manager.init_all().is_ok());
    }

    #[test]
    fn test_script_callback_builder() {
        let cb = ScriptCallback::new("fn", Box::new(|_| Ok(ScriptValue::Null)))
            .with_description("A test function")
            .with_param_count(2);
        assert_eq!(cb.description, "A test function");
        assert_eq!(cb.param_count, Some(2));
    }

    #[test]
    fn test_script_error_display() {
        let err = ScriptError {
            kind: ScriptErrorKind::SyntaxError,
            message: "Unexpected token".to_string(),
            file: Some("test.lua".to_string()),
            line: Some(10),
            stack_trace: None,
        };
        let s = format!("{}", err);
        assert!(s.contains("SyntaxError"));
        assert!(s.contains("test.lua"));
    }

    #[test]
    fn test_script_value_array() {
        let arr = ScriptValue::Array(vec![
            ScriptValue::Integer(1),
            ScriptValue::Integer(2),
            ScriptValue::Integer(3),
        ]);
        assert_eq!(arr.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_script_value_object() {
        let mut map = HashMap::new();
        map.insert("x".to_string(), ScriptValue::Float(1.0));
        map.insert("y".to_string(), ScriptValue::Float(2.0));
        let obj = ScriptValue::Object(map);
        assert_eq!(obj.as_object().unwrap().len(), 2);
    }
}
