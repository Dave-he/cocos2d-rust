use std::collections::LinkedList;
use std::rc::Rc;
use crate::base::Ref;

/// Autorelease pool manages objects that are autoreleased
///
/// In Rust, we use Rc for automatic reference counting, so this is primarily
/// for compatibility with the C++ API and for objects that need delayed cleanup.
#[derive(Debug)]
pub struct AutoreleasePool {
    managed_objects: LinkedList<Ref<Ref>>,
    name: String,
}

impl AutoreleasePool {
    /// Creates a new autorelease pool with a default name
    pub fn new() -> AutoreleasePool {
        AutoreleasePool {
            managed_objects: LinkedList::new(),
            name: String::from("autorelease"),
        }
    }

    /// Creates a new autorelease pool with a custom name
    pub fn with_name(name: &str) -> AutoreleasePool {
        AutoreleasePool {
            managed_objects: LinkedList::new(),
            name: name.to_string(),
        }
    }

    /// Adds an object to the pool
    pub fn add_object(&mut self, obj: Ref<Ref>) {
        self.managed_objects.push_back(obj);
    }

    /// Removes an object from the pool
    pub fn remove_object(&mut self, obj: &Ref<Ref>) {
    }

    /// Clears the pool
    pub fn clear(&mut self) {
        self.managed_objects.clear();
    }

    /// Gets the name of the pool
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// Pool manager manages all autorelease pools
#[derive(Debug)]
pub struct PoolManager {
    pools: LinkedList<AutoreleasePool>,
}

impl PoolManager {
    /// Gets the singleton instance
    pub fn get_instance() -> &'static mut PoolManager {
        static mut POOL_MANAGER: Option<PoolManager> = None;
        unsafe {
            if POOL_MANAGER.is_none() {
                POOL_MANAGER = Some(PoolManager::new());
            }
            POOL_MANAGER.as_mut().unwrap()
        }
    }

    /// Creates a new pool manager
    pub fn new() -> PoolManager {
        PoolManager {
            pools: LinkedList::new(),
        }
    }

    /// Gets the current pool
    pub fn get_current_pool(&mut self) -> &mut AutoreleasePool {
        if self.pools.is_empty() {
            self.pools.push_back(AutoreleasePool::new());
        }
        self.pools.back_mut().unwrap()
    }

    /// Pushes a new pool onto the stack
    pub fn push_pool(&mut self, pool: AutoreleasePool) {
        self.pools.push_back(pool);
    }

    /// Pops the current pool
    pub fn pop_pool(&mut self) {
        if let Some(pool) = self.pools.pop_back() {
        }
    }

    /// Clears all pools
    pub fn clear_all_pools(&mut self) {
        for pool in &mut self.pools {
            pool.clear();
        }
    }
}
