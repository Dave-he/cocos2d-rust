use std::collections::LinkedList;
use crate::base::{Ref, RefPtr};

#[derive(Debug, Default)]
pub struct AutoreleasePool {
    managed_objects: LinkedList<RefPtr<Ref>>,
    name: String,
}

impl AutoreleasePool {
    pub fn new() -> AutoreleasePool {
        AutoreleasePool {
            managed_objects: LinkedList::new(),
            name: String::from("autorelease"),
        }
    }

    pub fn with_name(name: &str) -> AutoreleasePool {
        AutoreleasePool {
            managed_objects: LinkedList::new(),
            name: name.to_string(),
        }
    }

    pub fn add_object(&mut self, obj: RefPtr<Ref>) {
        self.managed_objects.push_back(obj);
    }

    pub fn remove_object(&mut self, obj: &RefPtr<Ref>) {
    }

    pub fn clear(&mut self) {
        self.managed_objects.clear();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct PoolManager {
    pools: LinkedList<AutoreleasePool>,
}

impl PoolManager {
    pub fn get_instance() -> &'static mut PoolManager {
        static mut POOL_MANAGER: Option<PoolManager> = None;
        unsafe {
            if POOL_MANAGER.is_none() {
                POOL_MANAGER = Some(PoolManager::new());
            }
            POOL_MANAGER.as_mut().unwrap()
        }
    }

    pub fn new() -> PoolManager {
        PoolManager {
            pools: LinkedList::new(),
        }
    }

    pub fn get_current_pool(&mut self) -> &mut AutoreleasePool {
        if self.pools.is_empty() {
            self.pools.push_back(AutoreleasePool::new());
        }
        self.pools.back_mut().unwrap()
    }

    pub fn push_pool(&mut self, pool: AutoreleasePool) {
        self.pools.push_back(pool);
    }

    pub fn pop_pool(&mut self) {
        if let Some(pool) = self.pools.pop_back() {
        }
    }

    pub fn clear_all_pools(&mut self) {
        for pool in &mut self.pools {
            pool.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autorelease_pool_new() {
        let pool = AutoreleasePool::new();
        assert_eq!(pool.get_name(), "autorelease");
    }

    #[test]
    fn test_autorelease_pool_with_name() {
        let pool = AutoreleasePool::with_name("custom_pool");
        assert_eq!(pool.get_name(), "custom_pool");
    }

    #[test]
    fn test_autorelease_pool_add_object() {
        let mut pool = AutoreleasePool::new();
        let obj = RefPtr::new(Ref::new());
        pool.add_object(obj);
    }

    #[test]
    fn test_autorelease_pool_clear() {
        let mut pool = AutoreleasePool::new();
        let obj = RefPtr::new(Ref::new());
        pool.add_object(obj);
        pool.clear();
    }

    #[test]
    fn test_pool_manager_new() {
        let manager = PoolManager::new();
        assert!(manager.pools.is_empty());
    }

    #[test]
    fn test_pool_manager_get_current_pool() {
        let mut manager = PoolManager::new();
        let pool = manager.get_current_pool();
        assert_eq!(pool.get_name(), "autorelease");
    }

    #[test]
    fn test_pool_manager_push_pool() {
        let mut manager = PoolManager::new();
        let custom_pool = AutoreleasePool::with_name("custom");
        manager.push_pool(custom_pool);
        assert_eq!(manager.pools.len(), 1);
    }

    #[test]
    fn test_pool_manager_pop_pool() {
        let mut manager = PoolManager::new();
        let pool = AutoreleasePool::with_name("test");
        manager.push_pool(pool);
        assert_eq!(manager.pools.len(), 1);

        manager.pop_pool();
        assert_eq!(manager.pools.len(), 0);
    }

    #[test]
    fn test_pool_manager_clear_all_pools() {
        let mut manager = PoolManager::new();
        manager.push_pool(AutoreleasePool::with_name("pool1"));
        manager.push_pool(AutoreleasePool::with_name("pool2"));
        assert_eq!(manager.pools.len(), 2);

        manager.clear_all_pools();
        assert_eq!(manager.pools.len(), 2);
    }

    #[test]
    fn test_autorelease_pool_default() {
        let pool = AutoreleasePool::new();
        assert_eq!(pool.get_name(), "autorelease");
    }

    #[test]
    fn test_pool_manager_multiple_pools() {
        let mut manager = PoolManager::new();
        manager.push_pool(AutoreleasePool::with_name("pool1"));
        manager.push_pool(AutoreleasePool::with_name("pool2"));
        manager.push_pool(AutoreleasePool::with_name("pool3"));

        assert_eq!(manager.pools.len(), 3);

        manager.pop_pool();
        assert_eq!(manager.pools.len(), 2);
    }
}
