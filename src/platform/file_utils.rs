#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// FileUtils provides file system operations
#[derive(Debug)]
pub struct FileUtils {
    default_res_search_order: Vec<SearchPathType>,
    search_paths: Vec<String>,
    resolution_directories: HashMap<String, Vec<String>>,
    full_path_cache: HashMap<String, PathBuf>,
    writable_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum SearchPathType {
    Resources,
    Search,
    Doc,
    Caches,
    Temp,
}

impl Default for FileUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl FileUtils {
    /// Creates a new FileUtils
    pub fn new() -> FileUtils {
        FileUtils {
            default_res_search_order: vec![SearchPathType::Resources],
            search_paths: Vec::new(),
            resolution_directories: HashMap::new(),
            full_path_cache: HashMap::new(),
            writable_path: PathBuf::from("./"),
        }
    }

    /// Gets the singleton instance
    #[allow(static_mut_refs)]
    pub fn get_instance() -> &'static mut FileUtils {
        static mut FILE_UTILS: Option<FileUtils> = None;
        unsafe {
            if FILE_UTILS.is_none() {
                FILE_UTILS = Some(FileUtils::new());
            }
            FILE_UTILS.as_mut().unwrap()
        }
    }

    /// Adds a search path
    pub fn add_search_path(&mut self, path: &str, front: bool) {
        if front {
            self.search_paths.insert(0, path.to_string());
        } else {
            self.search_paths.push(path.to_string());
        }
    }

    /// Adds a resolution directory
    pub fn add_resolution_directory(&mut self, directory: &str) {
        self.resolution_directories
            .insert(directory.to_string(), vec![directory.to_string()]);
    }

    /// Gets the writable path
    pub fn get_writable_path(&self) -> &PathBuf {
        &self.writable_path
    }

    /// Gets the full path for a file
    pub fn get_full_path(&mut self, filename: &str) -> Option<PathBuf> {
        // Check cache first
        if let Some(path) = self.full_path_cache.get(filename) {
            return Some(path.clone());
        }

        // Try to find the file in search paths
        for search_path in &self.search_paths {
            let mut full_path = PathBuf::from(search_path);
            full_path.push(filename);

            if full_path.exists() {
                self.full_path_cache
                    .insert(filename.to_string(), full_path.clone());
                return Some(full_path);
            }
        }

        None
    }

    /// Checks if a file exists
    pub fn is_file_exist(&self, filename: &str) -> bool {
        let path = PathBuf::from(filename);
        path.exists() || path.is_file()
    }

    /// Checks if a directory exists
    pub fn is_directory_exist(&self, dir_path: &str) -> bool {
        let path = PathBuf::from(dir_path);
        path.exists() && path.is_dir()
    }

    /// Creates a directory
    pub fn create_directory(&self, dir_path: &str) -> bool {
        if fs::create_dir_all(dir_path).is_ok() {
            true
        } else {
            false
        }
    }

    /// Removes a directory
    pub fn remove_directory(&self, dir_path: &str) -> bool {
        if fs::remove_dir_all(dir_path).is_ok() {
            true
        } else {
            false
        }
    }

    /// Gets the file size
    pub fn get_file_size(&self, filename: &str) -> u64 {
        if let Ok(metadata) = fs::metadata(filename) {
            metadata.len()
        } else {
            0
        }
    }

    /// Reads file to string
    pub fn get_string_from_file(&self, filename: &str) -> Option<String> {
        fs::read_to_string(filename).ok()
    }

    /// Reads file to bytes
    pub fn get_bytes_from_file(&self, filename: &str) -> Option<Vec<u8>> {
        fs::read(filename).ok()
    }

    /// Writes string to file
    pub fn write_string_to_file(&self, data: &str, filename: &str) -> bool {
        if fs::write(filename, data).is_ok() {
            true
        } else {
            false
        }
    }

    /// Writes bytes to file
    pub fn write_bytes_to_file(&self, data: &[u8], filename: &str) -> bool {
        if fs::write(filename, data).is_ok() {
            true
        } else {
            false
        }
    }

    /// Lists files in a directory
    pub fn list_files(&self, dir_path: &str) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                if let Some(path) = entry.path().to_str() {
                    files.push(path.to_string());
                }
            }
        }
        files
    }

    /// Removes a file
    pub fn remove_file(&self, filename: &str) -> bool {
        if fs::remove_file(filename).is_ok() {
            true
        } else {
            false
        }
    }

    /// Renames a file
    pub fn rename_file(&self, old_name: &str, new_name: &str) -> bool {
        if fs::rename(old_name, new_name).is_ok() {
            true
        } else {
            false
        }
    }

    /// Gets the file extension
    pub fn get_file_extension(&self, filename: &str) -> Option<String> {
        PathBuf::from(filename)
            .extension()
            .map(|ext| ext.to_string_lossy().to_string())
    }

    /// Gets the file name from a path
    pub fn get_file_name(&self, path: &str) -> String {
        PathBuf::from(path)
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Gets the directory from a path
    pub fn get_directory_from_path(&self, path: &str) -> String {
        PathBuf::from(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// 拼接路径（跨平台）
    pub fn path_join(&self, base: &str, sub: &str) -> String {
        PathBuf::from(base).join(sub).to_string_lossy().to_string()
    }

    /// 规范化路径（去除 `.` `..` 等）
    pub fn normalize_path(&self, path: &str) -> String {
        let p = PathBuf::from(path);
        // 尝试 canonicalize，失败则返回原路径
        p.canonicalize()
            .map(|cp| cp.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string())
    }

    /// 获取文件大小（字节）
    pub fn get_file_size_bytes(&self, filename: &str) -> Option<u64> {
        fs::metadata(filename).ok().map(|m| m.len())
    }

    /// 检查路径是否是绝对路径
    pub fn is_absolute_path(&self, path: &str) -> bool {
        PathBuf::from(path).is_absolute()
    }

    /// 检查路径是否有指定扩展名（忽略大小写）
    pub fn has_extension(&self, path: &str, ext: &str) -> bool {
        let ext_lower = ext.to_lowercase();
        let ext_lower = ext_lower.trim_start_matches('.');
        PathBuf::from(path)
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase() == ext_lower)
            .unwrap_or(false)
    }

    /// 递归列出目录下所有文件（不包含子目录）
    pub fn list_files_recursive(&self, dir_path: &str) -> Vec<String> {
        let mut files = Vec::new();
        self.collect_files_recursive(dir_path, &mut files);
        files
    }

    fn collect_files_recursive(&self, dir_path: &str, files: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(s) = path.to_str() {
                        files.push(s.to_string());
                    }
                } else if path.is_dir() {
                    if let Some(s) = path.to_str() {
                        self.collect_files_recursive(s, files);
                    }
                }
            }
        }
    }

    /// 拷贝文件
    pub fn copy_file(&self, src: &str, dst: &str) -> bool {
        fs::copy(src, dst).is_ok()
    }

    /// 读取 JSON 字符串（就是文本读取，JSON 解析由调用方完成）
    pub fn read_json_string(&self, filename: &str) -> Option<String> {
        if self.has_extension(filename, "json") {
            self.get_string_from_file(filename)
        } else {
            None
        }
    }

    /// 获取文件最后修改时间（Unix 时间戳，秒）
    pub fn get_file_modified_time(&self, filename: &str) -> Option<u64> {
        fs::metadata(filename).ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_temp_file(name: &str, content: &str) -> String {
        let path = format!("/tmp/cocos_test_{}", name);
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        path
    }

    #[test]
    fn test_file_utils_new() {
        let fu = FileUtils::new();
        assert!(!fu.get_writable_path().to_str().unwrap().is_empty());
    }

    #[test]
    fn test_file_utils_add_search_path() {
        let mut fu = FileUtils::new();
        fu.add_search_path("/tmp", false);
        fu.add_search_path("/usr/share", true);
        // 不崩溃即通过
    }

    #[test]
    fn test_file_utils_get_file_extension() {
        let fu = FileUtils::new();
        assert_eq!(fu.get_file_extension("game.png"), Some("png".to_string()));
        assert_eq!(fu.get_file_extension("scene.json"), Some("json".to_string()));
        assert_eq!(fu.get_file_extension("no_ext"), None);
        assert_eq!(fu.get_file_extension("path/to/sprite.atlas"), Some("atlas".to_string()));
    }

    #[test]
    fn test_file_utils_has_extension() {
        let fu = FileUtils::new();
        assert!(fu.has_extension("game.PNG", "png")); // 忽略大小写
        assert!(fu.has_extension("scene.json", ".json"));
        assert!(!fu.has_extension("game.png", "jpg"));
        assert!(!fu.has_extension("no_ext", "png"));
    }

    #[test]
    fn test_file_utils_get_file_name() {
        let fu = FileUtils::new();
        assert_eq!(fu.get_file_name("path/to/file.png"), "file.png");
        assert_eq!(fu.get_file_name("/usr/share/game.data"), "game.data");
    }

    #[test]
    fn test_file_utils_get_directory_from_path() {
        let fu = FileUtils::new();
        assert_eq!(fu.get_directory_from_path("/tmp/game/file.png"), "/tmp/game");
        assert_eq!(fu.get_directory_from_path("relative/path/file.txt"), "relative/path");
    }

    #[test]
    fn test_file_utils_path_join() {
        let fu = FileUtils::new();
        let joined = fu.path_join("/tmp", "game/assets");
        assert!(joined.contains("tmp"));
        assert!(joined.contains("game"));
    }

    #[test]
    fn test_file_utils_is_absolute_path() {
        let fu = FileUtils::new();
        assert!(fu.is_absolute_path("/tmp/game"));
        assert!(!fu.is_absolute_path("relative/path"));
        assert!(!fu.is_absolute_path("file.png"));
    }

    #[test]
    fn test_file_utils_write_and_read_string() {
        let fu = FileUtils::new();
        let path = "/tmp/cocos_test_write_read.txt";
        let content = "Hello, cocos2d-rust!";

        assert!(fu.write_string_to_file(content, path));
        assert!(fu.is_file_exist(path));

        let read_back = fu.get_string_from_file(path);
        assert_eq!(read_back, Some(content.to_string()));

        fu.remove_file(path);
    }

    #[test]
    fn test_file_utils_write_and_read_bytes() {
        let fu = FileUtils::new();
        let path = "/tmp/cocos_test_bytes.bin";
        let data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF];

        assert!(fu.write_bytes_to_file(&data, path));
        let read_back = fu.get_bytes_from_file(path);
        assert_eq!(read_back, Some(data));

        fu.remove_file(path);
    }

    #[test]
    fn test_file_utils_file_size() {
        let fu = FileUtils::new();
        let path = create_temp_file("size_test.txt", "Hello World");
        let size = fu.get_file_size(&path);
        assert!(size > 0);

        let size_opt = fu.get_file_size_bytes(&path);
        assert!(size_opt.is_some());
        assert_eq!(size_opt.unwrap(), 11);

        fu.remove_file(&path);
    }

    #[test]
    fn test_file_utils_create_remove_directory() {
        let fu = FileUtils::new();
        let dir = "/tmp/cocos_test_dir_create_remove";

        // 先确保不存在
        let _ = fs::remove_dir_all(dir);
        assert!(!fu.is_directory_exist(dir));

        assert!(fu.create_directory(dir));
        assert!(fu.is_directory_exist(dir));

        assert!(fu.remove_directory(dir));
        assert!(!fu.is_directory_exist(dir));
    }

    #[test]
    fn test_file_utils_rename_file() {
        let fu = FileUtils::new();
        let old = "/tmp/cocos_test_rename_old.txt";
        let new = "/tmp/cocos_test_rename_new.txt";

        fu.write_string_to_file("data", old);
        assert!(fu.rename_file(old, new));
        assert!(!fu.is_file_exist(old));
        assert!(fu.is_file_exist(new));

        fu.remove_file(new);
    }

    #[test]
    fn test_file_utils_copy_file() {
        let fu = FileUtils::new();
        let src = "/tmp/cocos_test_copy_src.txt";
        let dst = "/tmp/cocos_test_copy_dst.txt";

        fu.write_string_to_file("copy me", src);
        assert!(fu.copy_file(src, dst));
        assert!(fu.is_file_exist(src));
        assert!(fu.is_file_exist(dst));

        let dst_content = fu.get_string_from_file(dst);
        assert_eq!(dst_content, Some("copy me".to_string()));

        fu.remove_file(src);
        fu.remove_file(dst);
    }

    #[test]
    fn test_file_utils_list_files() {
        let fu = FileUtils::new();
        let dir = "/tmp/cocos_test_list_dir";
        fu.create_directory(dir);

        let f1 = format!("{}/file1.txt", dir);
        let f2 = format!("{}/file2.txt", dir);
        fu.write_string_to_file("1", &f1);
        fu.write_string_to_file("2", &f2);

        let files = fu.list_files(dir);
        assert_eq!(files.len(), 2);

        fu.remove_directory(dir);
    }

    #[test]
    fn test_file_utils_read_json_string() {
        let fu = FileUtils::new();
        let path = "/tmp/cocos_test.json";
        fu.write_string_to_file(r#"{"key": "value"}"#, path);

        let result = fu.read_json_string(path);
        assert!(result.is_some());
        assert!(result.unwrap().contains("key"));

        fu.remove_file(path);

        // 非 JSON 文件应返回 None
        let txt_path = "/tmp/cocos_test.txt";
        fu.write_string_to_file("text content", txt_path);
        let result = fu.read_json_string(txt_path);
        assert!(result.is_none());
        fu.remove_file(txt_path);
    }

    #[test]
    fn test_file_utils_nonexistent_file() {
        let fu = FileUtils::new();
        assert!(!fu.is_file_exist("/nonexistent/path/file.png"));
        assert!(fu.get_string_from_file("/nonexistent/path/file.txt").is_none());
        assert!(fu.get_bytes_from_file("/nonexistent/path/file.bin").is_none());
        assert_eq!(fu.get_file_size("/nonexistent/file"), 0);
    }
}

