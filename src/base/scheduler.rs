use std::collections::HashMap;
use std::rc::Rc;
use crate::base::Ref;

/// Timer callback function type
pub type TimerCallback = Rc<dyn Fn(f32)>;

/// Schedule callback function type
pub type ScheduleCallback = Rc<dyn Fn(f32)>;

/// Scheduler is responsible for triggering the scheduled callbacks.
///
/// You should not invoke this manually. If you want to execute a function after
/// a delay, use `director.get_scheduler().schedule()`.
#[derive(Debug)]
pub struct Scheduler {
    timers: HashMap<String, Timer>,
    schedule_callbacks: HashMap<String, ScheduleCallback>,
    time_scale: f32,
    paused: bool,
    #[allow(dead_code)]
    update_hash: HashMap<usize, UpdateEntry>,
}

#[derive(Debug)]
struct Timer {
    callback: TimerCallback,
    interval: f32,
    elapsed: f32,
    repeat: i32,
    paused: bool,
}

#[derive(Debug)]
struct UpdateEntry {
    callback: Rc<dyn Fn(f32)>,
    paused: bool,
    priority: i32,
}

impl Scheduler {
    /// Creates a new scheduler
    pub fn new() -> Scheduler {
        Scheduler {
            timers: HashMap::new(),
            schedule_callbacks: HashMap::new(),
            time_scale: 1.0,
            paused: false,
            update_hash: HashMap::new(),
        }
    }

    /// Gets the time scale
    pub fn get_time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Sets the time scale
    pub fn set_time_scale(&mut self, time_scale: f32) {
        self.time_scale = time_scale;
    }

    /// Checks if the scheduler is paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Pauses the scheduler
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// Schedules a callback function with a given interval
    pub fn schedule(&mut self, key: &str, callback: ScheduleCallback, interval: f32, repeat: i32) {
        let timer = Timer {
            callback: Rc::clone(&callback),
            interval,
            elapsed: 0.0,
            repeat,
            paused: false,
        };
        self.timers.insert(key.to_string(), timer);
        self.schedule_callbacks.insert(key.to_string(), callback);
    }

    /// Schedules a callback function with a given interval (simplified version)
    pub fn schedule_simple(&mut self, key: &str, callback: ScheduleCallback, interval: f32) {
        self.schedule(key, callback, interval, 0xFFFFFF);
    }

    /// Unschedules a callback function
    pub fn unschedule(&mut self, key: &str) {
        self.timers.remove(key);
        self.schedule_callbacks.remove(key);
    }

    /// Unschedules all callback functions
    pub fn unschedule_all(&mut self) {
        self.timers.clear();
        self.schedule_callbacks.clear();
    }

    /// Updates the scheduler
    pub fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }

        let scaled_delta = delta_time * self.time_scale;

        // Update timers
        let mut timers_to_remove = Vec::new();

        for (key, timer) in &mut self.timers {
            if !timer.paused {
                timer.elapsed += scaled_delta;

                if timer.elapsed >= timer.interval {
                    timer.elapsed -= timer.interval;

                    // Execute callback
                    if let Some(callback) = self.schedule_callbacks.get(key) {
                        callback(timer.interval);
                    }

                    timer.repeat -= 1;
                    if timer.repeat == 0 {
                        timers_to_remove.push(key.clone());
                    }
                }
            }
        }

        // Remove finished timers
        for key in timers_to_remove {
            self.timers.remove(&key);
            self.schedule_callbacks.remove(&key);
        }
    }

    /// Performs a function in the main thread
    pub fn perform_function_in_main_thread(&self, _func: Box<dyn Fn()>) {
    }
}
