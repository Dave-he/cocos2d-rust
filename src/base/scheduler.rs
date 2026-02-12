#[allow(unused_imports)]
use crate::base::{Ref, RefPtr};
use std::collections::HashMap;
use std::rc::Rc;

pub type TimerCallback = Rc<dyn Fn(f32)>;

pub type ScheduleCallback = Rc<dyn Fn(f32)>;

impl std::fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scheduler")
            .field("time_scale", &self.time_scale)
            .field("paused", &self.paused)
            .field("timers_count", &self.timers.len())
            .field("callbacks_count", &self.schedule_callbacks.len())
            .finish()
    }
}

pub struct Scheduler {
    timers: HashMap<String, Timer>,
    schedule_callbacks: HashMap<String, ScheduleCallback>,
    time_scale: f32,
    paused: bool,
    #[allow(dead_code)]
    update_hash: HashMap<usize, UpdateEntry>,
}

struct Timer {
    callback: TimerCallback,
    interval: f32,
    elapsed: f32,
    repeat: i32,
    paused: bool,
}

struct UpdateEntry {
    callback: Rc<dyn Fn(f32)>,
    paused: bool,
    priority: i32,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            timers: HashMap::new(),
            schedule_callbacks: HashMap::new(),
            time_scale: 1.0,
            paused: false,
            update_hash: HashMap::new(),
        }
    }

    pub fn get_time_scale(&self) -> f32 {
        self.time_scale
    }

    pub fn set_time_scale(&mut self, time_scale: f32) {
        self.time_scale = time_scale;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

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

    pub fn schedule_simple(&mut self, key: &str, callback: ScheduleCallback, interval: f32) {
        self.schedule(key, callback, interval, 0xFFFFFF);
    }

    pub fn unschedule(&mut self, key: &str) {
        self.timers.remove(key);
        self.schedule_callbacks.remove(key);
    }

    pub fn unschedule_all(&mut self) {
        self.timers.clear();
        self.schedule_callbacks.clear();
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }

        let scaled_delta = delta_time * self.time_scale;

        let mut timers_to_remove = Vec::new();

        for (key, timer) in &mut self.timers {
            if !timer.paused {
                timer.elapsed += scaled_delta;

                if timer.elapsed >= timer.interval {
                    timer.elapsed -= timer.interval;

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

        for key in timers_to_remove {
            self.timers.remove(&key);
            self.schedule_callbacks.remove(&key);
        }
    }

    /// Performs a function in the main thread
    pub fn perform_function_in_main_thread(&self, _func: Box<dyn Fn()>) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_scheduler_new() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.get_time_scale(), 1.0);
        assert!(!scheduler.is_paused());
        assert_eq!(scheduler.timers.len(), 0);
    }

    #[test]
    fn test_scheduler_time_scale() {
        let mut scheduler = Scheduler::new();
        assert_eq!(scheduler.get_time_scale(), 1.0);

        scheduler.set_time_scale(2.0);
        assert_eq!(scheduler.get_time_scale(), 2.0);

        scheduler.set_time_scale(0.5);
        assert_eq!(scheduler.get_time_scale(), 0.5);
    }

    #[test]
    fn test_scheduler_paused() {
        let mut scheduler = Scheduler::new();
        assert!(!scheduler.is_paused());

        scheduler.set_paused(true);
        assert!(scheduler.is_paused());

        scheduler.set_paused(false);
        assert!(!scheduler.is_paused());
    }

    #[test]
    fn test_scheduler_schedule() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);
        assert_eq!(scheduler.timers.len(), 1);
        assert_eq!(scheduler.schedule_callbacks.len(), 1);
    }

    #[test]
    fn test_scheduler_schedule_simple() {
        let mut scheduler = Scheduler::new();
        let callback: ScheduleCallback = Rc::new(|_delta: f32| {});
        scheduler.schedule_simple("simple_timer", callback, 0.5);
        assert_eq!(scheduler.timers.len(), 1);
    }

    #[test]
    fn test_scheduler_update() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);

        scheduler.update(1.0);
        assert_eq!(callback_count.get(), 1);
    }

    #[test]
    fn test_scheduler_update_multiple_calls() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 3);

        scheduler.update(1.0);
        assert_eq!(callback_count.get(), 1);

        scheduler.update(1.0);
        assert_eq!(callback_count.get(), 2);

        scheduler.update(1.0);
        assert_eq!(callback_count.get(), 3);
    }

    #[test]
    fn test_scheduler_update_partial_elapsed() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);

        scheduler.update(0.5);
        assert_eq!(callback_count.get(), 0);

        scheduler.update(0.6);
        assert_eq!(callback_count.get(), 1);
    }

    #[test]
    fn test_scheduler_update_accumulates_time() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);

        scheduler.update(0.3);
        scheduler.update(0.3);
        scheduler.update(0.3);
        scheduler.update(0.3);
        assert_eq!(callback_count.get(), 1);
    }

    #[test]
    fn test_scheduler_paused_blocks_update() {
        let mut scheduler = Scheduler::new();
        scheduler.set_paused(true);

        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);

        scheduler.update(2.0);
        assert_eq!(callback_count.get(), 0);
    }

    #[test]
    fn test_scheduler_time_scale_effect() {
        let mut scheduler = Scheduler::new();
        scheduler.set_time_scale(2.0);

        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);

        scheduler.update(0.5);
        assert_eq!(callback_count.get(), 1);
    }

    #[test]
    fn test_scheduler_unschedule() {
        let mut scheduler = Scheduler::new();
        let callback: ScheduleCallback = Rc::new(|_delta: f32| {});
        scheduler.schedule("test_timer", callback, 1.0, 1);
        assert_eq!(scheduler.timers.len(), 1);

        scheduler.unschedule("test_timer");
        assert_eq!(scheduler.timers.len(), 0);
        assert_eq!(scheduler.schedule_callbacks.len(), 0);
    }

    #[test]
    fn test_scheduler_unschedule_all() {
        let mut scheduler = Scheduler::new();
        let callback: ScheduleCallback = Rc::new(|_delta: f32| {});

        scheduler.schedule("timer1", callback.clone(), 1.0, 1);
        scheduler.schedule("timer2", callback.clone(), 2.0, 1);
        scheduler.schedule("timer3", callback.clone(), 3.0, 1);
        assert_eq!(scheduler.timers.len(), 3);

        scheduler.unschedule_all();
        assert_eq!(scheduler.timers.len(), 0);
        assert_eq!(scheduler.schedule_callbacks.len(), 0);
    }

    #[test]
    fn test_scheduler_multiple_timers() {
        let mut scheduler = Scheduler::new();
        let count1 = Rc::new(Cell::new(0));
        let count2 = Rc::new(Cell::new(0));
        let count1_clone = count1.clone();
        let count2_clone = count2.clone();

        let callback1: ScheduleCallback = Rc::new(move |_delta: f32| {
            count1_clone.set(count1_clone.get() + 1);
        });
        let callback2: ScheduleCallback = Rc::new(move |_delta: f32| {
            count2_clone.set(count2_clone.get() + 1);
        });

        scheduler.schedule("timer1", callback1, 1.0, 1);
        scheduler.schedule("timer2", callback2, 2.0, 1);

        scheduler.update(1.0);
        assert_eq!(count1.get(), 1);
        assert_eq!(count2.get(), 0);

        scheduler.update(1.0);
        assert_eq!(count1.get(), 1);
        assert_eq!(count2.get(), 1);
    }

    #[test]
    fn test_scheduler_get_callback() {
        let mut scheduler = Scheduler::new();
        let last_delta = Rc::new(Cell::new(0.0));
        let last_delta_clone = last_delta.clone();

        let callback: ScheduleCallback = Rc::new(move |delta: f32| {
            last_delta_clone.set(delta);
        });

        scheduler.schedule("test_timer", callback, 2.0, 1);
        scheduler.update(1.0);
        scheduler.update(1.0);
        assert!((last_delta.get() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_scheduler_repeat_count() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 5);

        for _ in 0..5 {
            scheduler.update(1.0);
        }
        assert_eq!(callback_count.get(), 5);
        assert_eq!(scheduler.timers.len(), 0);
    }

    #[test]
    fn test_scheduler_zero_interval() {
        let mut scheduler = Scheduler::new();
        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 0.0, 3);
        scheduler.update(1.0);
        assert_eq!(callback_count.get(), 1);
    }

    #[test]
    fn test_scheduler_zero_time_scale() {
        let mut scheduler = Scheduler::new();
        scheduler.set_time_scale(0.0);

        let callback_count = Rc::new(Cell::new(0));
        let callback_count_clone = callback_count.clone();

        let callback: ScheduleCallback = Rc::new(move |_delta: f32| {
            let count = callback_count_clone.get();
            callback_count_clone.set(count + 1);
        });

        scheduler.schedule("test_timer", callback, 1.0, 1);
        scheduler.update(2.0);
        assert_eq!(callback_count.get(), 0);
    }
}
