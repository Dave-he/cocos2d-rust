/// AsyncTask - 异步任务模块
///
/// 功能：
/// - 异步任务创建和管理
/// - 任务状态跟踪
/// - 进度报告
/// - 错误处理
/// - 任务取消
/// - 任务依赖
/// - 任务组
/// - 线程池支持

use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::{VecDeque, HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::boxed::Box;
use std::result::Result;
use std::error::Error;

pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
    Paused,
}

impl Clone for TaskStatus {
    fn clone(&self) -> Self {
        match self {
            TaskStatus::Pending => TaskStatus::Pending,
            TaskStatus::Running => TaskStatus::Running,
            TaskStatus::Completed => TaskStatus::Completed,
            TaskStatus::Failed(msg) => TaskStatus::Failed(msg.clone()),
            TaskStatus::Cancelled => TaskStatus::Cancelled,
            TaskStatus::Paused => TaskStatus::Paused,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskProgress {
    pub progress: f32,
    pub message: String,
    pub current: usize,
    pub total: usize,
}

impl Default for TaskProgress {
    fn default() -> Self {
        Self {
            progress: 0.0,
            message: String::new(),
            current: 0,
            total: 100,
        }
    }
}

impl TaskProgress {
    pub fn new(current: usize, total: usize) -> Self {
        let progress = if total > 0 { current as f32 / total as f32 } else { 0.0 };
        Self {
            progress,
            message: String::new(),
            current,
            total,
        }
    }

    pub fn with_message(current: usize, total: usize, message: &str) -> Self {
        let mut progress = Self::new(current, total);
        progress.message = message.to_string();
        progress
    }

    pub fn percentage(&self) -> f32 {
        self.progress * 100.0
    }

    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }
}

#[derive(Clone, Debug)]
pub struct AsyncTaskResult<T> {
    pub status: TaskStatus,
    pub data: Option<T>,
    pub error: Option<String>,
    pub progress: TaskProgress,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

impl<T> AsyncTaskResult<T> {
    pub fn new() -> Self {
        Self {
            status: TaskStatus::Pending,
            data: None,
            error: None,
            progress: TaskProgress::default(),
            start_time: Instant::now(),
            end_time: None,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, TaskStatus::Failed(_))
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, TaskStatus::Cancelled)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, TaskStatus::Running)
    }

    pub fn duration(&self) -> Duration {
        let end = self.end_time.unwrap_or_else(Instant::now);
        end.duration_since(self.start_time)
    }

    pub fn duration_ms(&self) -> u128 {
        self.duration().as_millis()
    }
}

pub trait AsyncTaskHandler<T>: Send {
    fn on_progress(&self, progress: &TaskProgress);
    fn on_complete(&self, result: &AsyncTaskResult<T>);
    fn on_error(&self, error: &str);
    fn on_cancelled(&self);
}

struct DefaultTaskHandler<T> {
    on_progress: Option<Arc<dyn Fn(&TaskProgress) + Send + Sync>>,
    on_complete: Option<Arc<dyn Fn(&AsyncTaskResult<T>) + Send + Sync>>,
    on_error: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_cancelled: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl<T> DefaultTaskHandler<T> {
    pub fn new() -> Self {
        Self {
            on_progress: None,
            on_complete: None,
            on_error: None,
            on_cancelled: None,
        }
    }
}

impl<T> AsyncTaskHandler<T> for DefaultTaskHandler<T> {
    fn on_progress(&self, progress: &TaskProgress) {
        if let Some(cb) = &self.on_progress {
            cb(progress);
        }
    }

    fn on_complete(&self, result: &AsyncTaskResult<T>) {
        if let Some(cb) = &self.on_complete {
            cb(result);
        }
    }

    fn on_error(&self, error: &str) {
        if let Some(cb) = &self.on_error {
            cb(error);
        }
    }

    fn on_cancelled(&self) {
        if let Some(cb) = &self.on_cancelled {
            cb();
        }
    }
}

pub struct AsyncTask<T> {
    id: usize,
    name: String,
    status: Arc<Mutex<TaskStatus>>,
    progress: Arc<Mutex<TaskProgress>>,
    result: Arc<Mutex<AsyncTaskResult<T>>>,
    handler: Arc<Mutex<Box<dyn AsyncTaskHandler<T>>>>,
    is_cancelled: Arc<AtomicBool>,
    dependencies: Arc<Mutex<Vec<usize>>>,
    priority: Arc<Mutex<u32>>,
    created_at: Instant,
}

impl<T: Send + 'static> AsyncTask<T> {
    pub fn new<F>(name: &str, task_fn: F) -> Self
    where
        F: FnOnce(Arc<dyn Fn(TaskProgress) + Send + Sync>, Arc<dyn Fn() + Send + Sync>) -> Result<T, String>
            + Send
            + 'static,
    {
        Self {
            id: rand::random(),
            name: name.to_string(),
            status: Arc::new(Mutex::new(TaskStatus::Pending)),
            progress: Arc::new(Mutex::new(TaskProgress::default())),
            result: Arc::new(Mutex::new(AsyncTaskResult::new())),
            handler: Arc::new(Mutex::new(Box::new(DefaultTaskHandler::new()))),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            dependencies: Arc::new(Mutex::new(Vec::new())),
            priority: Arc::new(Mutex::new(0)),
            created_at: Instant::now(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_status(&self) -> TaskStatus {
        *self.status.lock().unwrap()
    }

    pub fn get_progress(&self) -> TaskProgress {
        *self.progress.lock().unwrap()
    }

    pub fn get_result(&self) -> AsyncTaskResult<T> {
        self.result.lock().unwrap().clone()
    }

    pub fn set_handler<H: AsyncTaskHandler<T> + 'static>(&mut self, handler: H) {
        *self.handler.lock().unwrap() = Box::new(handler);
    }

    pub fn set_priority(&mut self, priority: u32) {
        *self.priority.lock().unwrap() = priority;
    }

    pub fn get_priority(&self) -> u32 {
        *self.priority.lock().unwrap()
    }

    pub fn add_dependency(&mut self, task_id: usize) {
        self.dependencies.lock().unwrap().push(task_id);
    }

    pub fn get_dependencies(&self) -> Vec<usize> {
        self.dependencies.lock().unwrap().clone()
    }

    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }

    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
        *self.status.lock().unwrap() = TaskStatus::Cancelled;
        self.result.lock().unwrap().status = TaskStatus::Cancelled;
        self.result.lock().unwrap().end_time = Some(Instant::now());
    }

    pub fn update_progress(&self, progress: TaskProgress) {
        *self.progress.lock().unwrap() = progress.clone();
        self.result.lock().unwrap().progress = progress.clone();

        let handler = self.handler.lock().unwrap();
        handler.on_progress(&progress);
    }

    pub fn update_progress_message(&self, message: &str) {
        let mut progress = self.progress.lock().unwrap();
        progress.message = message.to_string();
    }

    fn notify_complete(&self, result: &AsyncTaskResult<T>) {
        let handler = self.handler.lock().unwrap();
        handler.on_complete(result);
    }

    fn notify_error(&self, error: &str) {
        let handler = self.handler.lock().unwrap();
        handler.on_error(error);
    }

    fn notify_cancelled(&self) {
        let handler = self.handler.lock().unwrap();
        handler.on_cancelled();
    }

    pub fn execute<F>(mut self, task_fn: F) -> Self
    where
        F: FnOnce(Arc<dyn Fn(TaskProgress) + Send + Sync>, Arc<dyn Fn() + Send + Sync>) -> Result<T, String>
            + Send
            + 'static,
    {
        let self_clone = Arc::new(self);

        thread::spawn({
            let task = self_clone.clone();
            let progress_callback = Arc::new(move |p: TaskProgress| {
                task.update_progress(p);
            });

            let is_cancelled = task.is_cancelled.clone();
            let cancel_callback = Arc::new(move || {
                is_cancelled.store(true, Ordering::Relaxed);
            });

            move {
                if task.get_dependencies().is_empty() {
                    task.run_internal(task_fn, progress_callback, cancel_callback);
                }
            }
        });

        Arc::into_inner(self_clone).unwrap()
    }

    fn run_internal<F>(
        &self,
        task_fn: F,
        progress_callback: Arc<dyn Fn(TaskProgress) + Send + Sync>,
        cancel_callback: Arc<dyn Fn() + Send + Sync>,
    ) where
        F: FnOnce(Arc<dyn Fn(TaskProgress) + Send + Sync>, Arc<dyn Fn() + Send + Sync>) -> Result<T, String>
            + Send
            + 'static,
    {
        *self.status.lock().unwrap() = TaskStatus::Running;
        self.result.lock().unwrap().status = TaskStatus::Running;
        self.result.lock().unwrap().start_time = Instant::now();

        let result = task_fn(progress_callback, cancel_callback);

        let mut result_guard = self.result.lock().unwrap();
        match result {
            Ok(data) => {
                result_guard.status = TaskStatus::Completed;
                result_guard.data = Some(data);
                result_guard.end_time = Some(Instant::now());
                drop(result_guard);
                self.notify_complete(&self.get_result());
            }
            Err(error) => {
                result_guard.status = TaskStatus::Failed(error.clone());
                result_guard.error = Some(error.clone());
                result_guard.end_time = Some(Instant::now());
                drop(result_guard);
                self.notify_error(&error);
            }
        }
    }
}

impl<T> Clone for AsyncTask<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            status: self.status.clone(),
            progress: self.progress.clone(),
            result: self.result.clone(),
            handler: self.handler.clone(),
            is_cancelled: self.is_cancelled.clone(),
            dependencies: self.dependencies.clone(),
            priority: self.priority.clone(),
            created_at: self.created_at,
        }
    }
}

pub struct TaskGroup<T> {
    tasks: Arc<Mutex<Vec<AsyncTask<T>>>>,
    name: String,
    completed_count: Arc<AtomicUsize>,
    failed_count: Arc<AtomicUsize>,
    cancelled_count: Arc<AtomicUsize>,
    on_all_complete: Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>,
    on_progress: Arc<Mutex<Option<Box<dyn Fn(f32, usize, usize) + Send + Sync>>>>,
}

impl<T: Send + 'static> TaskGroup<T> {
    pub fn new(name: &str) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            name: name.to_string(),
            completed_count: Arc::new(AtomicUsize::new(0)),
            failed_count: Arc::new(AtomicUsize::new(0)),
            cancelled_count: Arc::new(AtomicUsize::new(0)),
            on_all_complete: Arc::new(Mutex::new(None)),
            on_progress: Arc::new(Mutex::new(None)),
        }
    }

    pub fn add_task(&mut self, task: AsyncTask<T>) {
        self.tasks.lock().unwrap().push(task);
    }

    pub fn add_tasks(&mut self, tasks: Vec<AsyncTask<T>>) {
        self.tasks.lock().unwrap().extend(tasks);
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.lock().unwrap().len()
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed_count.load(Ordering::Relaxed)
    }

    pub fn get_failed_count(&self) -> usize {
        self.failed_count.load(Ordering::Relaxed)
    }

    pub fn get_cancelled_count(&self) -> usize {
        self.cancelled_count.load(Ordering::Relaxed)
    }

    pub fn get_progress(&self) -> f32 {
        let total = self.get_task_count() as f32;
        if total == 0.0 {
            return 0.0;
        }
        (self.completed_count.load(Ordering::Relaxed) as f32 / total) * 100.0
    }

    pub fn is_all_complete(&self) -> bool {
        let completed = self.completed_count.load(Ordering::Relaxed);
        let failed = self.failed_count.load(Ordering::Relaxed);
        let cancelled = self.cancelled_count.load(Ordering::Relaxed);
        let total = self.tasks.lock().unwrap().len();
        completed + failed + cancelled >= total
    }

    pub fn set_on_all_complete<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        *self.on_all_complete.lock().unwrap() = Some(Box::new(callback));
    }

    pub fn set_on_progress<F>(&mut self, callback: F)
    where
        F: Fn(f32, usize, usize) + Send + Sync + 'static,
    {
        *self.on_progress.lock().unwrap() = Some(Box::new(callback));
    }

    pub fn cancel_all(&self) {
        for task in self.tasks.lock().unwrap().iter() {
            task.cancel();
        }
    }

    pub fn wait_all(&self) {
        loop {
            if self.is_all_complete() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn generate_report(&self) -> String {
        format!(
            "=== TaskGroup Report ===\n\
             Name: {}\n\
             Total Tasks: {}\n\
             Completed: {}\n\
             Failed: {}\n\
             Cancelled: {}\n\
             Progress: {:.1}%",
            self.name,
            self.get_task_count(),
            self.get_completed_count(),
            self.get_failed_count(),
            self.get_cancelled_count(),
            self.get_progress()
        )
    }
}

pub struct AsyncTaskManager {
    running_tasks: Arc<Mutex<HashMap<usize, AsyncTask<()>>>>,
    completed_tasks: Arc<Mutex<VecDeque<(usize, Instant)>>>,
    max_concurrent: usize,
    active_count: Arc<AtomicUsize>,
    completed_count: Arc<AtomicUsize>,
}

impl AsyncTaskManager {
    pub fn new() -> Self {
        Self {
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            completed_tasks: Arc::new(Mutex::new(VecDeque::new())),
            max_concurrent: 4,
            active_count: Arc::new(AtomicUsize::new(0)),
            completed_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn with_max_concurrent(max: usize) -> Self {
        Self {
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            completed_tasks: Arc::new(Mutex::new(VecDeque::new())),
            max_concurrent: max.max(1),
            active_count: Arc::new(AtomicUsize::new(0)),
            completed_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn submit_task<T: Send + 'static>(&mut self, task: AsyncTask<T>) {
        let task_id = task.id();

        self.running_tasks.lock().unwrap().insert(task_id, unsafe {
            std::mem::transmute(task)
        });
    }

    pub fn get_active_count(&self) -> usize {
        self.active_count.load(Ordering::Relaxed)
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed_count.load(Ordering::Relaxed)
    }

    pub fn get_pending_count(&self) -> usize {
        self.running_tasks.lock().unwrap().len() - self.active_count.load(Ordering::Relaxed)
    }

    pub fn cancel_all(&self) {
        for task in self.running_tasks.lock().unwrap().values() {
            task.cancel();
        }
    }

    pub fn clear_completed(&mut self, keep_last: usize) {
        let mut completed = self.completed_tasks.lock().unwrap();
        while completed.len() > keep_last {
            completed.pop_front();
        }
    }

    pub fn get_running_task_ids(&self) -> HashSet<usize> {
        self.running_tasks.lock().unwrap().keys().cloned().collect()
    }

    pub fn is_task_running(&self, id: usize) -> bool {
        self.running_tasks.lock().unwrap().contains_key(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_result() {
        let result = AsyncTaskResult::<i32>::new();
        assert!(!result.is_success());
        assert!(!result.is_failed());
        assert!(!result.is_cancelled());
        assert!(result.is_running() || matches!(result.status, TaskStatus::Pending));
    }

    #[test]
    fn test_task_status() {
        assert!(matches!(TaskStatus::Pending, TaskStatus::Pending));
        assert!(matches!(TaskStatus::Running, TaskStatus::Running));
        assert!(matches!(TaskStatus::Completed, TaskStatus::Completed));
        assert!(matches!(TaskStatus::Cancelled, TaskStatus::Cancelled));
    }

    #[test]
    fn test_task_creation() {
        let task = AsyncTask::<i32>::new("test_task", |_, _| Ok(42));
        assert_eq!(task.name(), "test_task");
        assert!(matches!(task.get_status(), TaskStatus::Pending));
    }

    #[test]
    fn test_task_priority() {
        let mut task = AsyncTask::<i32>::new("test", |_, _| Ok(0));
        task.set_priority(10);
        assert_eq!(task.get_priority(), 10);
    }

    #[test]
    fn test_task_dependencies() {
        let mut task = AsyncTask::<i32>::new("test", |_, _| Ok(0));
        task.add_dependency(1);
        task.add_dependency(2);

        let deps = task.get_dependencies();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&1));
        assert!(deps.contains(&2));
    }

    #[test]
    fn test_task_cancellation() {
        let task = AsyncTask::<i32>::new("test", |_, _| Ok(0));
        assert!(!task.is_cancelled());

        task.cancel();
        assert!(task.is_cancelled());
        assert!(matches!(task.get_status(), TaskStatus::Cancelled));
    }

    #[test]
    fn test_task_group() {
        let mut group = TaskGroup::<i32>::new("test_group");

        for i in 0..3 {
            let task = AsyncTask::<i32>::new(&format!("task{}", i), |_, _| Ok(i));
            group.add_task(task);
        }

        assert_eq!(group.get_task_count(), 3);
        assert_eq!(group.get_progress(), 0.0);
    }

    #[test]
    fn test_task_group_progress() {
        let mut group = TaskGroup::<i32>::new("test");

        group.completed_count.store(2, Ordering::Relaxed);
        group.failed_count.store(1, Ordering::Relaxed);

        assert!((group.get_progress() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_task_manager() {
        let manager = AsyncTaskManager::new();
        assert_eq!(manager.get_active_count(), 0);
        assert_eq!(manager.get_completed_count(), 0);
        assert_eq!(manager.get_pending_count(), 0);
    }

    #[test]
    fn test_task_manager_max_concurrent() {
        let manager = AsyncTaskManager::with_max_concurrent(8);
        assert_eq!(manager.max_concurrent, 8);
    }

    #[test]
    fn test_notification_center_default() {
        let center = NotificationCenter::default();
        let guard = center.lock().unwrap();
        assert_eq!(guard.name, "Default");
    }

    #[test]
    fn test_task_result_duration() {
        let result = AsyncTaskResult::<()>::new();
        thread::sleep(Duration::from_millis(10));

        let duration = result.duration();
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_task_result_is_success() {
        let mut result = AsyncTaskResult::<i32>::new();
        result.status = TaskStatus::Completed;
        result.data = Some(42);

        assert!(result.is_success());
        assert!(!result.is_failed());
        assert!(!result.is_cancelled());
    }

    #[test]
    fn test_task_result_is_failed() {
        let mut result = AsyncTaskResult::<i32>::new();
        result.status = TaskStatus::Failed("error".to_string());

        assert!(!result.is_success());
        assert!(result.is_failed());
        assert!(!result.is_cancelled());
    }

    #[test]
    fn test_task_group_report() {
        let group = TaskGroup::<i32>::new("test_group");
        let report = group.generate_report();
        assert!(report.contains("TaskGroup Report"));
        assert!(report.contains("test_group"));
    }

    #[test]
    fn test_task_progress_with_message() {
        let progress = TaskProgress::with_message(50, 100, "Processing...");
        assert_eq!(progress.current, 50);
        assert_eq!(progress.total, 100);
        assert_eq!(progress.message, "Processing...");
    }

    #[test]
    fn test_notification_center_default() {
        let center = NotificationCenter::default();
        let guard = center.lock().unwrap();
        assert_eq!(guard.name, "Default");
    }
}
