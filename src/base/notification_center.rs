#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
/// NotificationCenter - 通知中心
///
/// 功能：
/// - 发布/订阅模式
/// - 同步/异步通知
/// - 优先级支持
/// - 观察者管理
/// - 线程安全

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub trait Notification: Debug + Send + Sync {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct DefaultNotification {
    name: String,
}

impl DefaultNotification {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Notification for DefaultNotification {
    fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for DefaultNotification {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for DefaultNotification {}

impl Hash for DefaultNotification {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationPriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

#[derive(Clone)]
pub struct NotificationObserver {
    id: usize,
    name: String,
    callback: Arc<dyn Fn(&dyn Notification) + Send + Sync>,
    priority: NotificationPriority,
    is_once: bool,
    is_enabled: bool,
}

impl NotificationObserver {
    pub fn new<F>(name: &str, callback: F, priority: NotificationPriority) -> Self
    where
        F: Fn(&dyn Notification) + Send + Sync + 'static,
    {
        Self {
            id: rand::random(),
            name: name.to_string(),
            callback: Arc::new(callback),
            priority,
            is_once: false,
            is_enabled: true,
        }
    }

    pub fn once<F>(name: &str, callback: F) -> Self
    where
        F: Fn(&dyn Notification) + Send + Sync + 'static,
    {
        let mut observer = Self::new(name, callback, NotificationPriority::Normal);
        observer.is_once = true;
        observer
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn priority(&self) -> NotificationPriority {
        self.priority
    }

    pub fn is_once(&self) -> bool {
        self.is_once
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn notify(&self, notification: &dyn Notification) {
        if self.is_enabled {
            (self.callback)(notification);
        }
    }
}

#[derive(Clone)]
pub struct NotificationPost {
    name: String,
    timestamp: Instant,
}

impl NotificationPost {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            timestamp: Instant::now(),
        }
    }
}

struct NotificationCenterInner {
    observers: HashMap<String, Vec<NotificationObserver>>,
    observer_ids: HashMap<usize, String>,
    notification_history: VecDeque<NotificationPost>,
    max_history_size: usize,
}

#[derive(Clone)]
pub struct NotificationCenter {
    inner: Arc<Mutex<NotificationCenterInner>>,
    pub name: String,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(NotificationCenterInner {
                observers: HashMap::new(),
                observer_ids: HashMap::new(),
                notification_history: VecDeque::new(),
                max_history_size: 100,
            })),
            name: "Default".to_string(),
        }
    }

    pub fn with_name(name: &str) -> Self {
        let mut center = Self::new();
        center.name = name.to_string();
        center
    }

    pub fn default() -> Arc<Mutex<Self>> {
        static DEFAULT_CENTER: std::sync::OnceLock<Arc<Mutex<NotificationCenter>>> = std::sync::OnceLock::new();
        DEFAULT_CENTER.get_or_init(|| Arc::new(Mutex::new(Self::new()))).clone()
    }

    pub fn add_observer(&mut self, observer: NotificationObserver) {
        let mut inner = self.inner.lock().unwrap();

        let notification_name = observer.name.clone();
        let observer_id = observer.id;

        let observers = inner
            .observers
            .entry(notification_name.clone())
            .or_default();
        
        observers.push(observer);
        
        // 按优先级排序：High > Normal > Low（降序）
        observers.sort_by(|a, b| b.priority.cmp(&a.priority));

        inner.observer_ids.insert(observer_id, notification_name);
    }

    pub fn remove_observer(&mut self, id: usize) {
        let mut inner = self.inner.lock().unwrap();

        if let Some(name) = inner.observer_ids.remove(&id) {
            if let Some(observers) = inner.observers.get_mut(&name) {
                observers.retain(|o| o.id() != id);
                if observers.is_empty() {
                    inner.observers.remove(&name);
                }
            }
        }
    }

    pub fn remove_observers(&mut self, name: &str) {
        let mut inner = self.inner.lock().unwrap();
        let name = name.to_string();

        if let Some(observers) = inner.observers.remove(&name) {
            for observer in observers {
                inner.observer_ids.remove(&observer.id());
            }
        }
    }

    pub fn remove_all_observers(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.observers.clear();
        inner.observer_ids.clear();
    }

    pub fn post(&mut self, notification: &dyn Notification) {
        let name = notification.name().to_string();

        let mut inner = self.inner.lock().unwrap();
        
        let mut to_remove = Vec::new();
        let mut removed_ids = Vec::new();
        
        if let Some(observers) = inner.observers.get_mut(&name) {
            for (i, observer) in observers.iter().enumerate() {
                if observer.is_enabled() {
                    observer.notify(notification);

                    if observer.is_once() {
                        to_remove.push(i);
                    }
                }
            }
            
            // 在同一个可变借用内完成移除操作
            for i in to_remove.iter().rev() {
                let observer = observers.remove(*i);
                removed_ids.push(observer.id());
            }
        }
        
        // 在observers借用结束后再移除IDs
        for id in removed_ids {
            inner.observer_ids.remove(&id);
        }

        inner.notification_history.push_back(NotificationPost::new(&name));
        if inner.notification_history.len() > inner.max_history_size {
            inner.notification_history.pop_front();
        }
    }

    pub fn post_notification(&mut self, name: &str) {
        let notification = DefaultNotification::new(name);
        self.post(&notification);
    }

    pub fn get_notification_history(&self) -> Vec<NotificationPost> {
        self.inner.lock().unwrap().notification_history.clone().into()
    }

    pub fn get_observer_count(&self, name: &str) -> usize {
        self.inner
            .lock()
            .unwrap()
            .observers
            .get(name)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    pub fn get_total_observer_count(&self) -> usize {
        self.inner.lock().unwrap().observer_ids.len()
    }

    pub fn has_observer(&self, id: usize) -> bool {
        self.inner.lock().unwrap().observer_ids.contains_key(&id)
    }

    pub fn is_observing(&self, name: &str) -> bool {
        self.inner.lock().unwrap().observers.contains_key(name)
    }

    pub fn set_max_history_size(&mut self, size: usize) {
        self.inner.lock().unwrap().max_history_size = size;
    }

    pub fn get_max_history_size(&self) -> usize {
        self.inner.lock().unwrap().max_history_size
    }

    pub fn clear_history(&mut self) {
        self.inner.lock().unwrap().notification_history.clear();
    }

    pub fn add_observer_blocking<F>(
        &mut self,
        name: &str,
        callback: F,
        priority: NotificationPriority,
    ) -> usize
    where
        F: Fn(&dyn Notification) + Send + Sync + 'static,
    {
        let observer = NotificationObserver::new(name, callback, priority);
        let id = observer.id();
        self.add_observer(observer);
        id
    }

    pub fn add_observer_once_blocking<F>(&mut self, name: &str, callback: F) -> usize
    where
        F: Fn(&dyn Notification) + Send + Sync + 'static,
    {
        let observer = NotificationObserver::once(name, callback);
        let id = observer.id();
        self.add_observer(observer);
        id
    }

    pub fn post_sync(&self, notification: &dyn Notification) {
        let inner = self.inner.lock().unwrap();
        let name = notification.name().to_string();

        if let Some(observers) = inner.observers.get(&name) {
            for observer in observers {
                if observer.is_enabled() {
                    observer.notify(notification);
                }
            }
        }
    }

    pub fn generate_report(&self) -> String {
        let inner = self.inner.lock().unwrap();
        format!(
            "=== NotificationCenter Report ===\n\
             Name: {}\n\
             Total Observers: {}\n\
             Notification Types: {}\n\
             History Size: {}\n\
             Max History: {}",
            self.name,
            inner.observer_ids.len(),
            inner.observers.len(),
            inner.notification_history.len(),
            inner.max_history_size
        )
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notification = DefaultNotification::new("test");
        assert_eq!(notification.name(), "test");
    }

    #[test]
    fn test_observer_creation() {
        let observer = NotificationObserver::new(
            "test",
            |_| {},
            NotificationPriority::Normal,
        );
        assert_eq!(observer.name(), "test");
        assert_eq!(observer.priority(), NotificationPriority::Normal);
        assert!(!observer.is_once());
    }

    #[test]
    fn test_observer_once() {
        let observer = NotificationObserver::once("test", |_| {});
        assert!(observer.is_once());
    }

    #[test]
    fn test_notification_center_creation() {
        let center = NotificationCenter::new();
        assert_eq!(center.get_total_observer_count(), 0);
    }

    #[test]
    fn test_add_remove_observer() {
        let mut center = NotificationCenter::new();

        let id = center.add_observer_blocking(
            "test",
            |_| {},
            NotificationPriority::Normal,
        );

        assert!(center.has_observer(id));
        assert_eq!(center.get_observer_count("test"), 1);

        center.remove_observer(id);
        assert!(!center.has_observer(id));
        assert_eq!(center.get_observer_count("test"), 0);
    }

    #[test]
    fn test_post_notification() {
        let mut center = NotificationCenter::new();
        let received = Arc::new(Mutex::new(false));

        {
            let received = received.clone();
            center.add_observer_blocking(
                "test",
                move |_| {
                    *received.lock().unwrap() = true;
                },
                NotificationPriority::Normal,
            );
        }

        center.post_notification("test");

        assert!(*received.lock().unwrap());
    }

    #[test]
    fn test_observer_priority() {
        let mut center = NotificationCenter::new();
        let order = Arc::new(Mutex::new(Vec::new()));

        {
            let order = order.clone();
            center.add_observer_blocking(
                "test",
                move |_| {
                    order.lock().unwrap().push("low");
                },
                NotificationPriority::Low,
            );
        }

        {
            let order = order.clone();
            center.add_observer_blocking(
                "test",
                move |_| {
                    order.lock().unwrap().push("high");
                },
                NotificationPriority::High,
            );
        }

        {
            let order = order.clone();
            center.add_observer_blocking(
                "test",
                move |_| {
                    order.lock().unwrap().push("normal");
                },
                NotificationPriority::Normal,
            );
        }

        center.post_notification("test");

        let final_order = order.lock().unwrap().clone();
        assert_eq!(final_order, vec!["high", "normal", "low"]);
    }

    #[test]
    fn test_remove_all_observers() {
        let mut center = NotificationCenter::new();

        center.add_observer_blocking("test1", |_| {}, NotificationPriority::Normal);
        center.add_observer_blocking("test2", |_| {}, NotificationPriority::Normal);
        center.add_observer_blocking("test3", |_| {}, NotificationPriority::Normal);

        assert_eq!(center.get_total_observer_count(), 3);

        center.remove_all_observers();

        assert_eq!(center.get_total_observer_count(), 0);
    }

    #[test]
    fn test_notification_history() {
        let mut center = NotificationCenter::new();

        center.post_notification("test1");
        center.post_notification("test2");
        center.post_notification("test3");

        let history = center.get_notification_history();
        assert_eq!(history.len(), 3);

        center.clear_history();
        assert!(center.get_notification_history().is_empty());
    }

    #[test]
    fn test_report() {
        let center = NotificationCenter::new();
        let report = center.generate_report();
        assert!(report.contains("NotificationCenter Report"));
    }

    #[test]
    fn test_is_observing() {
        let mut center = NotificationCenter::new();

        assert!(!center.is_observing("test"));

        center.add_observer_blocking("test", |_| {}, NotificationPriority::Normal);

        assert!(center.is_observing("test"));
        assert!(!center.is_observing("other"));
    }

    #[test]
    fn test_max_history_size() {
        let mut center = NotificationCenter::new();
        center.set_max_history_size(5);

        for i in 0..10 {
            center.post_notification(&format!("test{}", i));
        }

        assert_eq!(center.get_notification_history().len(), 5);
    }
}
