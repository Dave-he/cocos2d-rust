use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

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
                self.full_path_cache.insert(filename.to_string(), full_path.clone());
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
        if let Ok(_) = fs::create_dir_all(dir_path) {
            true
        } else {
            false
        }
    }

    /// Removes a directory
    pub fn remove_directory(&self, dir_path: &str) -> bool {
        if let Ok(_) = fs::remove_dir_all(dir_path) {
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
        if let Ok(content) = fs::read_to_string(filename) {
            Some(content)
        } else {
            None
        }
    }

    /// Reads file to bytes
    pub fn get_bytes_from_file(&self, filename: &str) -> Option<Vec<u8>> {
        if let Ok(content) = fs::read(filename) {
            Some(content)
        } else {
            None
        }
    }

    /// Writes string to file
    pub fn write_string_to_file(&self, data: &str, filename: &str) -> bool {
        if let Ok(_) = fs::write(filename, data) {
            true
        } else {
            false
        }
    }

    /// Writes bytes to file
    pub fn write_bytes_to_file(&self, data: &[u8], filename: &str) -> bool {
        if let Ok(_) = fs::write(filename, data) {
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
        if let Ok(_) = fs::remove_file(filename) {
            true
        } else {
            false
        }
    }

    /// Renames a file
    pub fn rename_file(&self, old_name: &str, new_name: &str) -> bool {
        if let Ok(_) = fs::rename(old_name, new_name) {
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
}
