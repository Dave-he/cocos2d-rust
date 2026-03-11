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
/// - 真实线程池支持

use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::{VecDeque, HashMap, HashSet};
use std::sync::mpsc;

/// 任务 ID 生成器（线程安全）
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn new_task_id() -> usize {
    TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// 任务状态
#[derive(Debug)]
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

/// 任务进度
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

/// 任务执行结果
pub struct AsyncTaskResult<T> {
    pub status: TaskStatus,
    pub data: Option<T>,
    pub error: Option<String>,
    pub progress: TaskProgress,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

impl<T: Clone> Clone for AsyncTaskResult<T> {
    fn clone(&self) -> Self {
        Self {
            status: self.status.clone(),
            data: self.data.clone(),
            error: self.error.clone(),
            progress: self.progress.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
        }
    }
}

impl<T> Default for AsyncTaskResult<T> {
    fn default() -> Self {
        Self::new()
    }
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

impl<T: std::fmt::Debug> std::fmt::Debug for AsyncTaskResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncTaskResult")
            .field("status", &self.status)
            .field("data", &self.data)
            .field("error", &self.error)
            .field("progress", &self.progress)
            .finish()
    }
}

/// 异步任务处理器 trait
pub trait AsyncTaskHandler<T>: Send {
    fn on_progress(&self, progress: &TaskProgress);
    fn on_complete(&self, result: &AsyncTaskResult<T>);
    fn on_error(&self, error: &str);
    fn on_cancelled(&self);
}

/// 默认任务处理器（基于回调闭包）
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

impl<T: Send> AsyncTaskHandler<T> for DefaultTaskHandler<T> {
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

/// 异步任务
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
    pub fn new(name: &str) -> Self {
        Self {
            id: new_task_id(),
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

    /// 创建带任务函数的 AsyncTask（兼容旧接口）
    pub fn with_fn<F>(name: &str, _task_fn: F) -> Self
    where
        F: FnOnce(Arc<dyn Fn(TaskProgress) + Send + Sync>, Arc<dyn Fn() + Send + Sync>) -> Result<T, String>
            + Send
            + 'static,
    {
        Self::new(name)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_status(&self) -> TaskStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn get_progress(&self) -> TaskProgress {
        self.progress.lock().unwrap().clone()
    }

    pub fn get_result(&self) -> AsyncTaskResult<T>
    where
        T: Clone
    {
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
        let mut result = self.result.lock().unwrap();
        result.status = TaskStatus::Cancelled;
        result.end_time = Some(Instant::now());
        drop(result);
        self.notify_cancelled();
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

    /// 在新线程中执行任务函数
    pub fn execute<F>(&self, task_fn: F)
    where
        F: FnOnce(Arc<dyn Fn(TaskProgress) + Send + Sync>, Arc<dyn Fn() + Send + Sync>) -> Result<T, String>
            + Send
            + 'static,
    {
        let status = self.status.clone();
        let result = self.result.clone();
        let progress = self.progress.clone();
        let handler = self.handler.clone();
        let is_cancelled = self.is_cancelled.clone();

        thread::spawn(move || {
            // 标记为 Running
            *status.lock().unwrap() = TaskStatus::Running;
            {
                let mut r = result.lock().unwrap();
                r.status = TaskStatus::Running;
                r.start_time = Instant::now();
            }

            let progress_clone = progress.clone();
            let result_clone = result.clone();

            let progress_callback: Arc<dyn Fn(TaskProgress) + Send + Sync> = Arc::new(move |p: TaskProgress| {
                *progress_clone.lock().unwrap() = p.clone();
                result_clone.lock().unwrap().progress = p.clone();
                let h = handler.lock().unwrap();
                h.on_progress(&p);
            });

            let cancel_callback: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
                is_cancelled.store(true, Ordering::Relaxed);
            });

            match task_fn(progress_callback, cancel_callback) {
                Ok(data) => {
                    let mut r = result.lock().unwrap();
                    r.status = TaskStatus::Completed;
                    r.data = Some(data);
                    r.end_time = Some(Instant::now());
                    *status.lock().unwrap() = TaskStatus::Completed;
                }
                Err(err) => {
                    let mut r = result.lock().unwrap();
                    r.status = TaskStatus::Failed(err.clone());
                    r.error = Some(err.clone());
                    r.end_time = Some(Instant::now());
                    *status.lock().unwrap() = TaskStatus::Failed(err);
                }
            }
        });
    }

    /// 等待任务完成（阻塞当前线程）
    pub fn wait(&self) {
        loop {
            let status = self.status.lock().unwrap().clone();
            match status {
                TaskStatus::Pending | TaskStatus::Running => {
                    drop(status);
                    thread::sleep(Duration::from_millis(5));
                }
                _ => break,
            }
        }
    }

    /// 等待任务完成（超时版本）
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        loop {
            let status = self.status.lock().unwrap().clone();
            match status {
                TaskStatus::Pending | TaskStatus::Running => {
                    drop(status);
                    if start.elapsed() > timeout {
                        return false;
                    }
                    thread::sleep(Duration::from_millis(5));
                }
                _ => return true,
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

impl<T: std::fmt::Debug> std::fmt::Debug for AsyncTask<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncTask")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("priority", &self.priority.lock().unwrap())
            .field("is_cancelled", &self.is_cancelled.load(Ordering::Relaxed))
            .finish()
    }
}

/// 任务组 —— 管理一组相关任务
pub struct TaskGroup<T> {
    tasks: Arc<Mutex<Vec<AsyncTask<T>>>>,
    name: String,
    pub completed_count: Arc<AtomicUsize>,
    pub failed_count: Arc<AtomicUsize>,
    pub cancelled_count: Arc<AtomicUsize>,
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

    pub fn wait_all_timeout(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        loop {
            if self.is_all_complete() {
                return true;
            }
            if start.elapsed() > timeout {
                return false;
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

/// 线程池工作消息
enum WorkerMessage {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}

/// 简单线程池实现
pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<WorkerMessage>,
    size: usize,
    active_count: Arc<AtomicUsize>,
}

impl ThreadPool {
    /// 创建指定大小的线程池
    pub fn new(size: usize) -> Self {
        let size = size.max(1);
        let (sender, receiver) = mpsc::channel::<WorkerMessage>();
        let receiver = Arc::new(Mutex::new(receiver));
        let active_count = Arc::new(AtomicUsize::new(0));

        let mut workers = Vec::with_capacity(size);
        for _i in 0..size {
            let rx = receiver.clone();
            let active = active_count.clone();
            let handle = thread::spawn(move || {
                loop {
                    let msg = {
                        let lock = rx.lock().unwrap();
                        lock.recv()
                    };
                    match msg {
                        Ok(WorkerMessage::Job(job)) => {
                            active.fetch_add(1, Ordering::Relaxed);
                            job();
                            active.fetch_sub(1, Ordering::Relaxed);
                        }
                        Ok(WorkerMessage::Shutdown) | Err(_) => break,
                    }
                }
            });
            workers.push(handle);
        }

        Self {
            workers,
            sender,
            size,
            active_count,
        }
    }

    /// 提交任务到线程池
    pub fn execute<F>(&self, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(WorkerMessage::Job(Box::new(f))).is_ok()
    }

    /// 获取线程池大小
    pub fn size(&self) -> usize {
        self.size
    }

    /// 获取当前活跃任务数
    pub fn active_count(&self) -> usize {
        self.active_count.load(Ordering::Relaxed)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            let _ = self.sender.send(WorkerMessage::Shutdown);
        }
    }
}

/// 异步任务管理器 —— 基于线程池
pub struct AsyncTaskManager {
    thread_pool: Arc<ThreadPool>,
    running_tasks: Arc<Mutex<HashMap<usize, String>>>,
    completed_tasks: Arc<Mutex<VecDeque<(usize, Instant)>>>,
    active_count: Arc<AtomicUsize>,
    completed_count: Arc<AtomicUsize>,
    failed_count: Arc<AtomicUsize>,
    pending_queue: Arc<Mutex<VecDeque<usize>>>,
}

impl Default for AsyncTaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncTaskManager {
    pub fn new() -> Self {
        Self::with_max_concurrent(4)
    }

    pub fn with_max_concurrent(max: usize) -> Self {
        Self {
            thread_pool: Arc::new(ThreadPool::new(max)),
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            completed_tasks: Arc::new(Mutex::new(VecDeque::new())),
            active_count: Arc::new(AtomicUsize::new(0)),
            completed_count: Arc::new(AtomicUsize::new(0)),
            failed_count: Arc::new(AtomicUsize::new(0)),
            pending_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// 提交任务到线程池执行
    pub fn submit<T, F>(&self, task: &AsyncTask<T>, task_fn: F) -> bool
    where
        T: Send + 'static,
        F: FnOnce(Arc<dyn Fn(TaskProgress) + Send + Sync>, Arc<dyn Fn() + Send + Sync>) -> Result<T, String>
            + Send
            + 'static,
    {
        let task_id = task.id();
        let task_name = task.name().to_string();
        let running_tasks = self.running_tasks.clone();
        let active_count = self.active_count.clone();
        let completed_count = self.completed_count.clone();
        let failed_count = self.failed_count.clone();
        let completed_tasks = self.completed_tasks.clone();

        // 注册任务
        running_tasks.lock().unwrap().insert(task_id, task_name);
        active_count.fetch_add(1, Ordering::Relaxed);

        let status = task.status.clone();
        let result = task.result.clone();
        let progress = task.progress.clone();
        let handler = task.handler.clone();
        let is_cancelled = task.is_cancelled.clone();

        self.thread_pool.execute(move || {
            *status.lock().unwrap() = TaskStatus::Running;
            {
                let mut r = result.lock().unwrap();
                r.status = TaskStatus::Running;
                r.start_time = Instant::now();
            }

            let progress_clone = progress.clone();
            let result_clone = result.clone();
            let handler_clone = handler.clone();

            let progress_callback: Arc<dyn Fn(TaskProgress) + Send + Sync> = Arc::new(move |p: TaskProgress| {
                *progress_clone.lock().unwrap() = p.clone();
                result_clone.lock().unwrap().progress = p.clone();
                let h = handler_clone.lock().unwrap();
                h.on_progress(&p);
            });

            let cancel_callback: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
                is_cancelled.store(true, Ordering::Relaxed);
            });

            match task_fn(progress_callback, cancel_callback) {
                Ok(data) => {
                    let end_time = Instant::now();
                    let mut r = result.lock().unwrap();
                    r.status = TaskStatus::Completed;
                    r.data = Some(data);
                    r.end_time = Some(end_time);
                    *status.lock().unwrap() = TaskStatus::Completed;
                    let temp_result = AsyncTaskResult {
                        status: TaskStatus::Completed,
                        data: None::<T>,
                        error: None,
                        progress: r.progress.clone(),
                        start_time: r.start_time,
                        end_time: r.end_time,
                    };
                    drop(r);
                    handler.lock().unwrap().on_complete(&temp_result);
                    completed_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(err) => {
                    let mut r = result.lock().unwrap();
                    r.status = TaskStatus::Failed(err.clone());
                    r.error = Some(err.clone());
                    r.end_time = Some(Instant::now());
                    *status.lock().unwrap() = TaskStatus::Failed(err.clone());
                    drop(r);
                    handler.lock().unwrap().on_error(&err);
                    failed_count.fetch_add(1, Ordering::Relaxed);
                }
            }

            // 从活跃任务移除
            running_tasks.lock().unwrap().remove(&task_id);
            active_count.fetch_sub(1, Ordering::Relaxed);
            completed_tasks.lock().unwrap().push_back((task_id, Instant::now()));
        })
    }

    pub fn get_pool_size(&self) -> usize {
        self.thread_pool.size()
    }

    pub fn get_active_count(&self) -> usize {
        self.active_count.load(Ordering::Relaxed)
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed_count.load(Ordering::Relaxed)
    }

    pub fn get_failed_count(&self) -> usize {
        self.failed_count.load(Ordering::Relaxed)
    }

    pub fn get_pending_count(&self) -> usize {
        self.pending_queue.lock().unwrap().len()
    }

    pub fn cancel_all_running(&self) {
        // 清空等待队列
        self.pending_queue.lock().unwrap().clear();
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

    pub fn generate_report(&self) -> String {
        format!(
            "=== AsyncTaskManager Report ===\n\
             Pool Size: {}\n\
             Active Tasks: {}\n\
             Completed Tasks: {}\n\
             Failed Tasks: {}\n\
             Pending Tasks: {}",
            self.get_pool_size(),
            self.get_active_count(),
            self.get_completed_count(),
            self.get_failed_count(),
            self.get_pending_count()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicI32;

    #[test]
    fn test_task_id_unique() {
        let id1 = new_task_id();
        let id2 = new_task_id();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_task_result() {
        let result = AsyncTaskResult::<i32>::new();
        assert!(!result.is_success());
        assert!(!result.is_failed());
        assert!(!result.is_cancelled());
        assert!(matches!(result.status, TaskStatus::Pending));
    }

    #[test]
    fn test_task_status_clone() {
        assert!(matches!(TaskStatus::Pending.clone(), TaskStatus::Pending));
        assert!(matches!(TaskStatus::Running.clone(), TaskStatus::Running));
        assert!(matches!(TaskStatus::Completed.clone(), TaskStatus::Completed));
        assert!(matches!(TaskStatus::Cancelled.clone(), TaskStatus::Cancelled));
        let failed = TaskStatus::Failed("err".to_string());
        if let TaskStatus::Failed(msg) = failed.clone() {
            assert_eq!(msg, "err");
        }
    }

    #[test]
    fn test_task_creation() {
        let task = AsyncTask::<i32>::new("test_task");
        assert_eq!(task.name(), "test_task");
        assert!(matches!(task.get_status(), TaskStatus::Pending));
        assert!(task.id() > 0);
    }

    #[test]
    fn test_task_priority() {
        let mut task = AsyncTask::<i32>::new("test");
        task.set_priority(10);
        assert_eq!(task.get_priority(), 10);
    }

    #[test]
    fn test_task_dependencies() {
        let mut task = AsyncTask::<i32>::new("test");
        task.add_dependency(1);
        task.add_dependency(2);

        let deps = task.get_dependencies();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&1));
        assert!(deps.contains(&2));
    }

    #[test]
    fn test_task_cancellation() {
        let task = AsyncTask::<i32>::new("test");
        assert!(!task.is_cancelled());

        task.cancel();
        assert!(task.is_cancelled());
        assert!(matches!(task.get_status(), TaskStatus::Cancelled));
    }

    #[test]
    fn test_task_execute_success() {
        let task = AsyncTask::<i32>::new("success_task");
        let task_clone = task.clone();
        
        task.execute(|_progress, _cancel| Ok(42));

        // 等待完成
        let completed = task_clone.wait_timeout(Duration::from_secs(5));
        assert!(completed, "Task should complete within 5 seconds");
        assert!(matches!(task_clone.get_status(), TaskStatus::Completed));
    }

    #[test]
    fn test_task_execute_failure() {
        let task = AsyncTask::<i32>::new("fail_task");
        let task_clone = task.clone();
        
        task.execute(|_progress, _cancel| Err("test_error".to_string()));

        let completed = task_clone.wait_timeout(Duration::from_secs(5));
        assert!(completed);
        assert!(matches!(task_clone.get_status(), TaskStatus::Failed(_)));
    }

    #[test]
    fn test_task_progress_callback() {
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let task = AsyncTask::<i32>::new("progress_task");
        let task_clone = task.clone();

        task.execute(move |progress_cb, _| {
            progress_cb(TaskProgress::new(50, 100));
            received_clone.store(true, Ordering::Relaxed);
            Ok(0)
        });

        task_clone.wait_timeout(Duration::from_secs(5));
        assert!(received.load(Ordering::Relaxed));
    }

    #[test]
    fn test_task_group_basic() {
        let mut group = TaskGroup::<i32>::new("test");
        
        for i in 0..3 {
            let task = AsyncTask::<i32>::new(&format!("task{}", i));
            group.add_task(task);
        }

        assert_eq!(group.get_task_count(), 3);
        assert_eq!(group.get_progress(), 0.0);
    }

    #[test]
    fn test_task_group_progress() {
        let mut group = TaskGroup::<i32>::new("test");
        
        for i in 0..3 {
            let task = AsyncTask::<i32>::new(&format!("task{}", i));
            group.add_task(task);
        }

        // 模拟2个完成，1个失败
        group.completed_count.store(2, Ordering::Relaxed);
        group.failed_count.store(1, Ordering::Relaxed);

        let expected = (2.0 / 3.0) * 100.0;
        assert!((group.get_progress() - expected).abs() < 0.1);
    }

    #[test]
    fn test_task_group_report() {
        let group = TaskGroup::<i32>::new("test_group");
        let report = group.generate_report();
        assert!(report.contains("TaskGroup Report"));
        assert!(report.contains("test_group"));
    }

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.size(), 4);
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn test_thread_pool_min_size() {
        // size=0 时应至少有1个线程
        let pool = ThreadPool::new(0);
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_thread_pool_execute() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicI32::new(0));

        for _ in 0..5 {
            let c = counter.clone();
            pool.execute(move || {
                c.fetch_add(1, Ordering::Relaxed);
            });
        }

        thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn test_task_manager_creation() {
        let manager = AsyncTaskManager::new();
        assert_eq!(manager.get_active_count(), 0);
        assert_eq!(manager.get_completed_count(), 0);
        assert_eq!(manager.get_pending_count(), 0);
    }

    #[test]
    fn test_task_manager_max_concurrent() {
        let manager = AsyncTaskManager::with_max_concurrent(8);
        assert_eq!(manager.get_pool_size(), 8);
    }

    #[test]
    fn test_task_manager_submit() {
        let manager = AsyncTaskManager::new();
        let task = AsyncTask::<i32>::new("manager_task");
        let task_clone = task.clone();

        let submitted = manager.submit(&task, |_, _| Ok(100));
        assert!(submitted);

        let done = task_clone.wait_timeout(Duration::from_secs(5));
        assert!(done);
        assert_eq!(manager.get_completed_count(), 1);
    }

    #[test]
    fn test_task_manager_report() {
        let manager = AsyncTaskManager::new();
        let report = manager.generate_report();
        assert!(report.contains("AsyncTaskManager Report"));
    }

    #[test]
    fn test_notification_center_default() {
        let center = crate::base::notification_center::NotificationCenter::default();
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
    fn test_task_progress_with_message() {
        let progress = TaskProgress::with_message(50, 100, "Processing...");
        assert_eq!(progress.current, 50);
        assert_eq!(progress.total, 100);
        assert_eq!(progress.message, "Processing...");
    }

    #[test]
    fn test_task_progress_percentage() {
        let progress = TaskProgress::new(25, 100);
        assert!((progress.percentage() - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_task_progress_is_complete() {
        let progress = TaskProgress::new(100, 100);
        assert!(progress.is_complete());

        let progress2 = TaskProgress::new(50, 100);
        assert!(!progress2.is_complete());
    }

    #[test]
    fn test_multiple_tasks_concurrent() {
        let manager = AsyncTaskManager::with_max_concurrent(4);
        let counter = Arc::new(AtomicI32::new(0));
        let tasks: Vec<AsyncTask<i32>> = (0..8).map(|i| AsyncTask::new(&format!("task{}", i))).collect();
        let task_clones: Vec<AsyncTask<i32>> = tasks.iter().map(|t| t.clone()).collect();

        for task in &tasks {
            let c = counter.clone();
            manager.submit(task, move |_, _| {
                c.fetch_add(1, Ordering::Relaxed);
                Ok(1)
            });
        }

        // 等待所有完成
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let all_done = task_clones.iter().all(|t| {
                matches!(t.get_status(), TaskStatus::Completed | TaskStatus::Failed(_) | TaskStatus::Cancelled)
            });
            if all_done { break; }
            if Instant::now() > deadline { break; }
            thread::sleep(Duration::from_millis(20));
        }

        assert_eq!(counter.load(Ordering::Relaxed), 8);
    }
}
