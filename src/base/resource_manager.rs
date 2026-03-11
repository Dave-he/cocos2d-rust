/// ResourceManager - 资源管理器
///
/// 功能：
/// - 资源的加载、缓存、释放
/// - 引用计数管理
/// - 异步加载支持
/// - 内存预算控制
/// - 资源分组和批量操作

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Texture,
    Audio,
    Font,
    Shader,
    Tilemap,
    Json,
    Data,
    Spine,
    Animation,
    Custom(u32),
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Texture => write!(f, "Texture"),
            ResourceType::Audio => write!(f, "Audio"),
            ResourceType::Font => write!(f, "Font"),
            ResourceType::Shader => write!(f, "Shader"),
            ResourceType::Tilemap => write!(f, "Tilemap"),
            ResourceType::Json => write!(f, "JSON"),
            ResourceType::Data => write!(f, "Data"),
            ResourceType::Spine => write!(f, "Spine"),
            ResourceType::Animation => write!(f, "Animation"),
            ResourceType::Custom(id) => write!(f, "Custom({})", id),
        }
    }
}

/// 资源状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    Unloaded,
    Loading,
    Loaded,
    Failed,
    Evicted,
}

/// 资源元数据
#[derive(Debug, Clone)]
pub struct ResourceMeta {
    pub key: String,
    pub resource_type: ResourceType,
    pub state: ResourceState,
    pub size_bytes: usize,
    pub ref_count: usize,
    pub load_time: Option<Instant>,
    pub last_access: Instant,
    pub group: Option<String>,
}

impl ResourceMeta {
    pub fn new(key: &str, res_type: ResourceType) -> Self {
        Self {
            key: key.to_string(),
            resource_type: res_type,
            state: ResourceState::Unloaded,
            size_bytes: 0,
            ref_count: 0,
            load_time: None,
            last_access: Instant::now(),
            group: None,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.state == ResourceState::Loaded
    }

    pub fn is_loading(&self) -> bool {
        self.state == ResourceState::Loading
    }
}

/// 泛型资源包装
#[derive(Debug)]
pub struct Resource<T> {
    pub meta: ResourceMeta,
    pub data: Option<Arc<T>>,
}

impl<T> Resource<T> {
    pub fn new(key: &str, res_type: ResourceType) -> Self {
        Self {
            meta: ResourceMeta::new(key, res_type),
            data: None,
        }
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(Arc::new(data));
        self.meta.state = ResourceState::Loaded;
        self.meta.load_time = Some(Instant::now());
    }

    pub fn get(&self) -> Option<Arc<T>> {
        self.data.clone()
    }

    pub fn is_loaded(&self) -> bool {
        self.data.is_some()
    }
}

/// 资源统计
#[derive(Debug, Clone, Default)]
pub struct ResourceStats {
    pub total_loaded: usize,
    pub total_size_bytes: usize,
    pub by_type: HashMap<ResourceType, usize>,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub evictions: usize,
}

impl ResourceStats {
    pub fn total_size_kb(&self) -> f32 {
        self.total_size_bytes as f32 / 1024.0
    }

    pub fn total_size_mb(&self) -> f32 {
        self.total_size_bytes as f32 / (1024.0 * 1024.0)
    }

    pub fn hit_rate(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 { 0.0 } else { self.cache_hits as f32 / total as f32 }
    }
}

/// 通用资源管理器
pub struct ResourceManager {
    /// key -> (元数据, 引用数)
    registry: Arc<Mutex<HashMap<String, ResourceMeta>>>,
    /// 内存上限（字节，0 表示无限）
    memory_budget: usize,
    /// 统计
    stats: Arc<Mutex<ResourceStats>>,
    /// 分组
    groups: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
            memory_budget: 0,
            stats: Arc::new(Mutex::new(ResourceStats::default())),
            groups: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_memory_budget(mut self, budget_mb: usize) -> Self {
        self.memory_budget = budget_mb * 1024 * 1024;
        self
    }

    /// 注册资源（标记为待加载）
    pub fn register(&self, key: &str, res_type: ResourceType) {
        let mut registry = self.registry.lock().unwrap();
        if !registry.contains_key(key) {
            registry.insert(key.to_string(), ResourceMeta::new(key, res_type));
        }
    }

    /// 标记资源已加载
    pub fn mark_loaded(&self, key: &str, size_bytes: usize) {
        let mut registry = self.registry.lock().unwrap();
        if let Some(meta) = registry.get_mut(key) {
            meta.state = ResourceState::Loaded;
            meta.size_bytes = size_bytes;
            meta.load_time = Some(Instant::now());
        }
        let mut stats = self.stats.lock().unwrap();
        stats.total_loaded += 1;
        stats.total_size_bytes += size_bytes;
    }

    /// 标记资源加载失败
    pub fn mark_failed(&self, key: &str) {
        let mut registry = self.registry.lock().unwrap();
        if let Some(meta) = registry.get_mut(key) {
            meta.state = ResourceState::Failed;
        }
    }

    /// 添加引用计数
    pub fn retain(&self, key: &str) -> bool {
        let mut registry = self.registry.lock().unwrap();
        if let Some(meta) = registry.get_mut(key) {
            meta.ref_count += 1;
            meta.last_access = Instant::now();
            self.stats.lock().unwrap().cache_hits += 1;
            true
        } else {
            self.stats.lock().unwrap().cache_misses += 1;
            false
        }
    }

    /// 减少引用计数
    pub fn release(&self, key: &str) -> usize {
        let mut registry = self.registry.lock().unwrap();
        if let Some(meta) = registry.get_mut(key) {
            if meta.ref_count > 0 {
                meta.ref_count -= 1;
            }
            meta.ref_count
        } else {
            0
        }
    }

    /// 获取资源元数据
    pub fn get_meta(&self, key: &str) -> Option<ResourceMeta> {
        self.registry.lock().unwrap().get(key).cloned()
    }

    /// 检查资源是否已加载
    pub fn is_loaded(&self, key: &str) -> bool {
        self.registry.lock().unwrap()
            .get(key)
            .map(|m| m.state == ResourceState::Loaded)
            .unwrap_or(false)
    }

    /// 获取已加载的资源数量
    pub fn loaded_count(&self) -> usize {
        self.registry.lock().unwrap()
            .values()
            .filter(|m| m.state == ResourceState::Loaded)
            .count()
    }

    /// 卸载未被引用的资源
    pub fn evict_unused(&self) -> usize {
        let mut registry = self.registry.lock().unwrap();
        let mut evicted = 0;
        let mut freed_bytes = 0;

        for meta in registry.values_mut() {
            if meta.ref_count == 0 && meta.state == ResourceState::Loaded {
                freed_bytes += meta.size_bytes;
                meta.state = ResourceState::Evicted;
                meta.size_bytes = 0;
                evicted += 1;
            }
        }

        let mut stats = self.stats.lock().unwrap();
        stats.evictions += evicted;
        stats.total_size_bytes -= freed_bytes;
        stats.total_loaded -= evicted;

        evicted
    }

    /// 添加到分组
    pub fn add_to_group(&self, key: &str, group: &str) {
        let mut groups = self.groups.lock().unwrap();
        groups.entry(group.to_string()).or_default().push(key.to_string());
        drop(groups);
        let mut registry = self.registry.lock().unwrap();
        if let Some(meta) = registry.get_mut(key) {
            meta.group = Some(group.to_string());
        }
    }

    /// 释放整个分组
    pub fn release_group(&self, group: &str) {
        let groups = self.groups.lock().unwrap();
        if let Some(keys) = groups.get(group) {
            let keys: Vec<String> = keys.clone();
            drop(groups);
            for key in keys {
                self.release(&key);
            }
        }
    }

    /// 获取分组的所有资源键
    pub fn get_group_keys(&self, group: &str) -> Vec<String> {
        self.groups.lock().unwrap()
            .get(group)
            .cloned()
            .unwrap_or_default()
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> ResourceStats {
        self.stats.lock().unwrap().clone()
    }

    /// 清空所有资源
    pub fn clear(&self) {
        self.registry.lock().unwrap().clear();
        self.groups.lock().unwrap().clear();
        *self.stats.lock().unwrap() = ResourceStats::default();
    }

    /// 生成报告
    pub fn generate_report(&self) -> String {
        let registry = self.registry.lock().unwrap();
        let stats = self.stats.lock().unwrap();

        format!(
            "=== ResourceManager Report ===\n\
             Total Resources: {}\n\
             Loaded: {}\n\
             Total Memory: {:.2} MB\n\
             Cache Hit Rate: {:.1}%\n\
             Evictions: {}",
            registry.len(),
            stats.total_loaded,
            stats.total_size_mb(),
            stats.hit_rate() * 100.0,
            stats.evictions
        )
    }
}

// ============================================================
// ObjectPool - 对象池
// ============================================================

/// 对象池 trait
pub trait Poolable: Default {
    fn reset(&mut self);
}

/// 泛型对象池
pub struct ObjectPool<T: Poolable> {
    pool: Mutex<Vec<T>>,
    capacity: usize,
    allocate_count: std::sync::atomic::AtomicUsize,
    reuse_count: std::sync::atomic::AtomicUsize,
}

impl<T: Poolable> ObjectPool<T> {
    /// 创建指定容量的对象池
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Mutex::new(Vec::with_capacity(capacity)),
            capacity,
            allocate_count: std::sync::atomic::AtomicUsize::new(0),
            reuse_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// 预热（预创建对象）
    pub fn prewarm(&self, count: usize) {
        let mut pool = self.pool.lock().unwrap();
        let count = count.min(self.capacity);
        for _ in 0..count {
            pool.push(T::default());
        }
    }

    /// 获取对象（优先从池中取）
    pub fn get(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        if let Some(mut obj) = pool.pop() {
            obj.reset();
            self.reuse_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            obj
        } else {
            self.allocate_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            T::default()
        }
    }

    /// 归还对象到池
    pub fn put(&self, obj: T) {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < self.capacity {
            pool.push(obj);
        }
        // 超出容量时丢弃（自动 drop）
    }

    /// 当前池中可用对象数
    pub fn available(&self) -> usize {
        self.pool.lock().unwrap().len()
    }

    /// 池容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 总分配次数
    pub fn allocate_count(&self) -> usize {
        self.allocate_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 复用次数
    pub fn reuse_count(&self) -> usize {
        self.reuse_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 复用率（0.0 ~ 1.0）
    pub fn reuse_rate(&self) -> f32 {
        let total = self.allocate_count() + self.reuse_count();
        if total == 0 { 0.0 } else { self.reuse_count() as f32 / total as f32 }
    }

    /// 清空池
    pub fn clear(&self) {
        self.pool.lock().unwrap().clear();
    }

    /// 生成报告
    pub fn report(&self) -> String {
        format!(
            "ObjectPool: available={}/{}, allocates={}, reuses={}, reuse_rate={:.1}%",
            self.available(),
            self.capacity,
            self.allocate_count(),
            self.reuse_count(),
            self.reuse_rate() * 100.0
        )
    }
}

/// 带归还功能的对象守卫
pub struct PoolGuard<'a, T: Poolable> {
    pool: &'a ObjectPool<T>,
    obj: Option<T>,
}

impl<'a, T: Poolable> PoolGuard<'a, T> {
    fn new(pool: &'a ObjectPool<T>) -> Self {
        let obj = pool.get();
        Self { pool, obj: Some(obj) }
    }
}

impl<'a, T: Poolable> std::ops::Deref for PoolGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.obj.as_ref().unwrap()
    }
}

impl<'a, T: Poolable> std::ops::DerefMut for PoolGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.obj.as_mut().unwrap()
    }
}

impl<'a, T: Poolable> Drop for PoolGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(obj) = self.obj.take() {
            self.pool.put(obj);
        }
    }
}

impl<T: Poolable> ObjectPool<T> {
    /// 从池中借用（作用域结束后自动归还）
    pub fn borrow(&self) -> PoolGuard<'_, T> {
        PoolGuard::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用的可池化对象
    #[derive(Default, Debug)]
    struct TestObj {
        pub value: i32,
        pub name: String,
    }

    impl Poolable for TestObj {
        fn reset(&mut self) {
            self.value = 0;
            self.name.clear();
        }
    }

    #[test]
    fn test_resource_meta() {
        let meta = ResourceMeta::new("texture/hero.png", ResourceType::Texture);
        assert_eq!(meta.key, "texture/hero.png");
        assert_eq!(meta.resource_type, ResourceType::Texture);
        assert!(!meta.is_loaded());
    }

    #[test]
    fn test_resource_manager_register() {
        let manager = ResourceManager::new();
        manager.register("tex/bg.png", ResourceType::Texture);
        assert!(manager.get_meta("tex/bg.png").is_some());
        assert!(!manager.is_loaded("tex/bg.png"));
    }

    #[test]
    fn test_resource_manager_mark_loaded() {
        let manager = ResourceManager::new();
        manager.register("tex/bg.png", ResourceType::Texture);
        manager.mark_loaded("tex/bg.png", 1024 * 1024);

        assert!(manager.is_loaded("tex/bg.png"));
        assert_eq!(manager.loaded_count(), 1);

        let stats = manager.get_stats();
        assert_eq!(stats.total_loaded, 1);
        assert_eq!(stats.total_size_bytes, 1024 * 1024);
    }

    #[test]
    fn test_resource_manager_ref_count() {
        let manager = ResourceManager::new();
        manager.register("audio/bgm.ogg", ResourceType::Audio);
        manager.mark_loaded("audio/bgm.ogg", 512);

        manager.retain("audio/bgm.ogg");
        manager.retain("audio/bgm.ogg");

        let meta = manager.get_meta("audio/bgm.ogg").unwrap();
        assert_eq!(meta.ref_count, 2);

        manager.release("audio/bgm.ogg");
        let meta = manager.get_meta("audio/bgm.ogg").unwrap();
        assert_eq!(meta.ref_count, 1);
    }

    #[test]
    fn test_resource_manager_evict_unused() {
        let manager = ResourceManager::new();
        manager.register("unused.png", ResourceType::Texture);
        manager.mark_loaded("unused.png", 100);

        manager.register("used.png", ResourceType::Texture);
        manager.mark_loaded("used.png", 200);
        manager.retain("used.png");

        let evicted = manager.evict_unused();
        assert_eq!(evicted, 1);

        let unused = manager.get_meta("unused.png").unwrap();
        assert_eq!(unused.state, ResourceState::Evicted);

        let used = manager.get_meta("used.png").unwrap();
        assert_eq!(used.state, ResourceState::Loaded);
    }

    #[test]
    fn test_resource_manager_groups() {
        let manager = ResourceManager::new();
        manager.register("level1/bg.png", ResourceType::Texture);
        manager.register("level1/music.ogg", ResourceType::Audio);

        manager.add_to_group("level1/bg.png", "level1");
        manager.add_to_group("level1/music.ogg", "level1");

        let keys = manager.get_group_keys("level1");
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_resource_manager_cache_stats() {
        let manager = ResourceManager::new();
        manager.register("test.png", ResourceType::Texture);
        manager.mark_loaded("test.png", 100);

        manager.retain("test.png"); // hit
        manager.retain("nonexistent"); // miss

        let stats = manager.get_stats();
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
        assert!((stats.hit_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_resource_manager_mark_failed() {
        let manager = ResourceManager::new();
        manager.register("missing.png", ResourceType::Texture);
        manager.mark_failed("missing.png");

        let meta = manager.get_meta("missing.png").unwrap();
        assert_eq!(meta.state, ResourceState::Failed);
    }

    #[test]
    fn test_resource_manager_clear() {
        let manager = ResourceManager::new();
        manager.register("a.png", ResourceType::Texture);
        manager.register("b.png", ResourceType::Texture);
        manager.clear();
        assert_eq!(manager.loaded_count(), 0);
    }

    #[test]
    fn test_resource_manager_report() {
        let manager = ResourceManager::new();
        manager.register("test.png", ResourceType::Texture);
        manager.mark_loaded("test.png", 1024);
        let report = manager.generate_report();
        assert!(report.contains("ResourceManager Report"));
    }

    #[test]
    fn test_object_pool_basic() {
        let pool: ObjectPool<TestObj> = ObjectPool::new(10);
        assert_eq!(pool.available(), 0);
        assert_eq!(pool.capacity(), 10);

        let obj = pool.get();
        assert_eq!(obj.value, 0);
        assert_eq!(pool.allocate_count(), 1);
        assert_eq!(pool.reuse_count(), 0);

        pool.put(obj);
        assert_eq!(pool.available(), 1);
    }

    #[test]
    fn test_object_pool_reuse() {
        let pool: ObjectPool<TestObj> = ObjectPool::new(5);

        let mut obj = pool.get();
        obj.value = 42;
        obj.name = "test".to_string();
        pool.put(obj);

        let obj2 = pool.get();
        // reset 后应为默认值
        assert_eq!(obj2.value, 0);
        assert!(obj2.name.is_empty());
        assert_eq!(pool.reuse_count(), 1);
    }

    #[test]
    fn test_object_pool_prewarm() {
        let pool: ObjectPool<TestObj> = ObjectPool::new(10);
        pool.prewarm(5);
        assert_eq!(pool.available(), 5);
    }

    #[test]
    fn test_object_pool_capacity_limit() {
        let pool: ObjectPool<TestObj> = ObjectPool::new(3);

        // 放入超过容量的对象
        for _ in 0..10 {
            pool.put(TestObj::default());
        }

        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_object_pool_borrow() {
        let pool: ObjectPool<TestObj> = ObjectPool::new(5);
        pool.prewarm(3);

        {
            let mut guard = pool.borrow();
            guard.value = 99;
            // guard 离开作用域时自动归还
        }

        assert_eq!(pool.available(), 3); // 归还后数量恢复
    }

    #[test]
    fn test_object_pool_reuse_rate() {
        let pool: ObjectPool<TestObj> = ObjectPool::new(5);

        let obj1 = pool.get(); // allocate
        let obj2 = pool.get(); // allocate
        pool.put(obj1);
        pool.put(obj2);

        let _obj3 = pool.get(); // reuse
        let _obj4 = pool.get(); // reuse

        assert!((pool.reuse_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_resource_stats_size() {
        let mut stats = ResourceStats::default();
        stats.total_size_bytes = 2 * 1024 * 1024; // 2 MB
        assert!((stats.total_size_mb() - 2.0).abs() < 0.01);
        assert!((stats.total_size_kb() - 2048.0).abs() < 0.1);
    }

    #[test]
    fn test_resource_type_display() {
        assert_eq!(ResourceType::Texture.to_string(), "Texture");
        assert_eq!(ResourceType::Audio.to_string(), "Audio");
        assert_eq!(ResourceType::Custom(42).to_string(), "Custom(42)");
    }

    #[test]
    fn test_resource_wrapper() {
        let mut res: Resource<Vec<u8>> = Resource::new("test.bin", ResourceType::Data);
        assert!(!res.is_loaded());

        res.set_data(vec![1, 2, 3, 4]);
        assert!(res.is_loaded());
        assert!(res.get().is_some());
        assert_eq!(res.get().unwrap().len(), 4);
    }
}
