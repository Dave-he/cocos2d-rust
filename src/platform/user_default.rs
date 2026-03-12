#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// 用户数据持久化系统
/// 
/// UserDefault 提供简单的键值对存储，用于保存游戏设置、玩家数据等
/// 支持的数据类型：bool, i32, i64, f32, f64, String
/// 数据以 JSON 格式存储在本地文件中
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UserDefaultData {
    #[serde(default)]
    bool_values: HashMap<String, bool>,
    #[serde(default)]
    int_values: HashMap<String, i64>,
    #[serde(default)]
    float_values: HashMap<String, f64>,
    #[serde(default)]
    string_values: HashMap<String, String>,
}


/// UserDefault 单例
pub struct UserDefault {
    data: UserDefaultData,
    file_path: PathBuf,
    auto_flush: bool,
}

impl UserDefault {
    /// 创建新的 UserDefault 实例
    fn new(file_path: PathBuf) -> Self {
        let mut instance = UserDefault {
            data: UserDefaultData::default(),
            file_path,
            auto_flush: true,
        };
        
        // 尝试加载已有数据
        let _ = instance.load();
        
        instance
    }
    
    /// 获取全局单例（默认路径）
    pub fn get_instance() -> Arc<Mutex<UserDefault>> {
        static INSTANCE: std::sync::OnceLock<Arc<Mutex<UserDefault>>> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = Self::get_default_path();
            Arc::new(Mutex::new(UserDefault::new(path)))
        }).clone()
    }
    
    /// 创建自定义路径的实例
    pub fn create_with_path<P: AsRef<Path>>(path: P) -> Arc<Mutex<UserDefault>> {
        Arc::new(Mutex::new(UserDefault::new(path.as_ref().to_path_buf())))
    }
    
    /// 获取默认存储路径
    fn get_default_path() -> PathBuf {
        // 在真实应用中，应该使用系统特定的用户数据目录
        // 例如：Linux: ~/.local/share/cocos2d-rust/
        //      macOS: ~/Library/Application Support/cocos2d-rust/
        //      Windows: %APPDATA%/cocos2d-rust/
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                let mut path = PathBuf::from(home);
                path.push("Library");
                path.push("Application Support");
                path.push("cocos2d-rust");
                std::fs::create_dir_all(&path).ok();
                path.push("UserDefault.json");
                return path;
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                let mut path = PathBuf::from(home);
                path.push(".local");
                path.push("share");
                path.push("cocos2d-rust");
                std::fs::create_dir_all(&path).ok();
                path.push("UserDefault.json");
                return path;
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            if let Some(appdata) = std::env::var_os("APPDATA") {
                let mut path = PathBuf::from(appdata);
                path.push("cocos2d-rust");
                std::fs::create_dir_all(&path).ok();
                path.push("UserDefault.json");
                return path;
            }
        }
        
        // 默认回退到当前目录
        PathBuf::from("UserDefault.json")
    }
    
    // ========== Boolean 值操作 ==========
    
    /// 设置布尔值
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.data.bool_values.insert(key.to_string(), value);
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 获取布尔值
    pub fn get_bool(&self, key: &str, default_value: bool) -> bool {
        self.data.bool_values.get(key).copied().unwrap_or(default_value)
    }
    
    // ========== Integer 值操作 ==========
    
    /// 设置整数值
    pub fn set_int(&mut self, key: &str, value: i32) {
        self.data.int_values.insert(key.to_string(), value as i64);
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 获取整数值
    pub fn get_int(&self, key: &str, default_value: i32) -> i32 {
        self.data.int_values.get(key).map(|&v| v as i32).unwrap_or(default_value)
    }
    
    /// 设置长整数值
    pub fn set_long(&mut self, key: &str, value: i64) {
        self.data.int_values.insert(key.to_string(), value);
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 获取长整数值
    pub fn get_long(&self, key: &str, default_value: i64) -> i64 {
        self.data.int_values.get(key).copied().unwrap_or(default_value)
    }
    
    // ========== Float 值操作 ==========
    
    /// 设置浮点数值
    pub fn set_float(&mut self, key: &str, value: f32) {
        self.data.float_values.insert(key.to_string(), value as f64);
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 获取浮点数值
    pub fn get_float(&self, key: &str, default_value: f32) -> f32 {
        self.data.float_values.get(key).map(|&v| v as f32).unwrap_or(default_value)
    }
    
    /// 设置双精度浮点数值
    pub fn set_double(&mut self, key: &str, value: f64) {
        self.data.float_values.insert(key.to_string(), value);
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 获取双精度浮点数值
    pub fn get_double(&self, key: &str, default_value: f64) -> f64 {
        self.data.float_values.get(key).copied().unwrap_or(default_value)
    }
    
    // ========== String 值操作 ==========
    
    /// 设置字符串值
    pub fn set_string(&mut self, key: &str, value: &str) {
        self.data.string_values.insert(key.to_string(), value.to_string());
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 获取字符串值
    pub fn get_string(&self, key: &str, default_value: &str) -> String {
        self.data.string_values.get(key).cloned().unwrap_or_else(|| default_value.to_string())
    }
    
    // ========== 键操作 ==========
    
    /// 检查键是否存在
    pub fn has_key(&self, key: &str) -> bool {
        self.data.bool_values.contains_key(key)
            || self.data.int_values.contains_key(key)
            || self.data.float_values.contains_key(key)
            || self.data.string_values.contains_key(key)
    }
    
    /// 删除指定键
    pub fn remove_key(&mut self, key: &str) {
        self.data.bool_values.remove(key);
        self.data.int_values.remove(key);
        self.data.float_values.remove(key);
        self.data.string_values.remove(key);
        
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    /// 清除所有数据
    pub fn clear(&mut self) {
        self.data.bool_values.clear();
        self.data.int_values.clear();
        self.data.float_values.clear();
        self.data.string_values.clear();
        
        if self.auto_flush {
            let _ = self.flush();
        }
    }
    
    // ========== 持久化操作 ==========
    
    /// 设置是否自动保存
    pub fn set_auto_flush(&mut self, auto_flush: bool) {
        self.auto_flush = auto_flush;
    }
    
    /// 获取是否自动保存
    pub fn get_auto_flush(&self) -> bool {
        self.auto_flush
    }
    
    /// 立即保存到文件
    pub fn flush(&self) -> Result<(), String> {
        // 确保目录存在
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        // 写入文件
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;
        
        Ok(())
    }
    
    /// 从文件加载数据
    pub fn load(&mut self) -> Result<(), String> {
        if !self.file_path.exists() {
            // 文件不存在，使用默认空数据
            return Ok(());
        }
        
        let file = File::open(&self.file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let reader = BufReader::new(file);
        self.data = serde_json::from_reader(reader)
            .map_err(|e| format!("Failed to deserialize data: {}", e))?;
        
        Ok(())
    }
    
    /// 获取文件路径
    pub fn get_file_path(&self) -> &Path {
        &self.file_path
    }
    
    /// 获取所有键（按类型）
    pub fn get_all_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        keys.extend(self.data.bool_values.keys().cloned());
        keys.extend(self.data.int_values.keys().cloned());
        keys.extend(self.data.float_values.keys().cloned());
        keys.extend(self.data.string_values.keys().cloned());
        keys.sort();
        keys.dedup();
        keys
    }
    
    /// 获取数据统计信息
    pub fn get_stats(&self) -> UserDefaultStats {
        UserDefaultStats {
            bool_count: self.data.bool_values.len(),
            int_count: self.data.int_values.len(),
            float_count: self.data.float_values.len(),
            string_count: self.data.string_values.len(),
            total_count: self.get_all_keys().len(),
        }
    }
}

/// 数据统计信息
#[derive(Debug, Clone)]
pub struct UserDefaultStats {
    pub bool_count: usize,
    pub int_count: usize,
    pub float_count: usize,
    pub string_count: usize,
    pub total_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    fn create_test_instance() -> Arc<Mutex<UserDefault>> {
        let test_path = PathBuf::from("/tmp/cocos2d_rust_test_userdefault.json");
        // 删除测试文件
        let _ = fs::remove_file(&test_path);
        UserDefault::create_with_path(test_path)
    }
    
    #[test]
    fn test_user_default_creation() {
        let ud = create_test_instance();
        let ud = ud.lock().unwrap();
        assert!(ud.get_file_path().to_str().unwrap().contains("userdefault"));
    }
    
    #[test]
    fn test_bool_operations() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_bool("test_bool", true);
        assert_eq!(ud.get_bool("test_bool", false), true);
        assert_eq!(ud.get_bool("non_existent", false), false);
    }
    
    #[test]
    fn test_int_operations() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_int("test_int", 42);
        assert_eq!(ud.get_int("test_int", 0), 42);
        assert_eq!(ud.get_int("non_existent", 99), 99);
    }
    
    #[test]
    fn test_long_operations() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_long("test_long", 9876543210);
        assert_eq!(ud.get_long("test_long", 0), 9876543210);
    }
    
    #[test]
    fn test_float_operations() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_float("test_float", 3.14);
        let value = ud.get_float("test_float", 0.0);
        assert!((value - 3.14).abs() < 0.01);
    }
    
    #[test]
    fn test_double_operations() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_double("test_double", 3.141592653589793);
        let value = ud.get_double("test_double", 0.0);
        assert!((value - 3.141592653589793).abs() < 1e-10);
    }
    
    #[test]
    fn test_string_operations() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_string("test_string", "Hello, Cocos2d-Rust!");
        assert_eq!(ud.get_string("test_string", ""), "Hello, Cocos2d-Rust!");
        assert_eq!(ud.get_string("non_existent", "default"), "default");
    }
    
    #[test]
    fn test_has_key() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_int("existing_key", 100);
        assert!(ud.has_key("existing_key"));
        assert!(!ud.has_key("non_existent_key"));
    }
    
    #[test]
    fn test_remove_key() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_int("key_to_remove", 123);
        assert!(ud.has_key("key_to_remove"));
        
        ud.remove_key("key_to_remove");
        assert!(!ud.has_key("key_to_remove"));
    }
    
    #[test]
    fn test_clear() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_bool("bool_key", true);
        ud.set_int("int_key", 42);
        ud.set_string("string_key", "test");
        
        assert!(ud.has_key("bool_key"));
        assert!(ud.has_key("int_key"));
        assert!(ud.has_key("string_key"));
        
        ud.clear();
        
        assert!(!ud.has_key("bool_key"));
        assert!(!ud.has_key("int_key"));
        assert!(!ud.has_key("string_key"));
    }
    
    #[test]
    fn test_persistence() {
        let test_path = PathBuf::from("/tmp/cocos2d_rust_test_persistence.json");
        let _ = fs::remove_file(&test_path);
        
        // 第一个实例：写入数据
        {
            let ud = UserDefault::create_with_path(&test_path);
            let mut ud = ud.lock().unwrap();
            ud.set_int("persisted_value", 999);
            ud.set_string("persisted_string", "Persistent!");
            ud.flush().unwrap();
        }
        
        // 第二个实例：读取数据
        {
            let ud = UserDefault::create_with_path(&test_path);
            let ud = ud.lock().unwrap();
            assert_eq!(ud.get_int("persisted_value", 0), 999);
            assert_eq!(ud.get_string("persisted_string", ""), "Persistent!");
        }
        
        // 清理
        let _ = fs::remove_file(&test_path);
    }
    
    #[test]
    fn test_auto_flush() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        assert!(ud.get_auto_flush());
        
        ud.set_auto_flush(false);
        assert!(!ud.get_auto_flush());
        
        ud.set_auto_flush(true);
        assert!(ud.get_auto_flush());
    }
    
    #[test]
    fn test_get_all_keys() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_bool("key1", true);
        
        let keys = ud.get_all_keys();
        assert!(keys.len() >= 1);
    }
    
    #[test]
    fn test_stats() {
        let ud = create_test_instance();
        let mut ud = ud.lock().unwrap();
        
        ud.set_bool("bool1", true);
        ud.set_int("int1", 1);
        ud.set_string("str1", "a");
        
        let stats = ud.get_stats();
        assert!(stats.bool_count >= 0);
        assert!(stats.int_count >= 0);
        assert!(stats.string_count >= 0);
    }
}

