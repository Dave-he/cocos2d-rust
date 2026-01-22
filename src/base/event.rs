use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::base::{Ref, RefPtr};
use crate::math::Vec2;

/// Event types supported by the engine
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Touch,
    Keyboard,
    Mouse,
    Acceleration,
    Custom,
}

/// Base event type
#[derive(Debug)]
pub struct Event {
    event_type: EventType,
    stopped: bool,
    #[allow(dead_code)]
    name: String,
}

impl Event {
    pub fn new(event_type: EventType) -> Event {
        Event {
            event_type,
            stopped: false,
            name: String::new(),
        }
    }

    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn reset(&mut self) {
        self.stopped = false;
    }
}

/// Touch event
#[derive(Debug)]
pub struct EventTouch {
    base: Event,
    touches: Vec<Vec2>,
    touch_id: i32,
}

impl EventTouch {
    pub fn new() -> EventTouch {
        EventTouch {
            base: Event::new(EventType::Touch),
            touches: Vec::new(),
            touch_id: 0,
        }
    }

    pub fn get_touches(&self) -> &Vec<Vec2> {
        &self.touches
    }

    pub fn add_touch(&mut self, touch: Vec2) {
        self.touches.push(touch);
    }

    pub fn get_touch_id(&self) -> i32 {
        self.touch_id
    }

    pub fn set_touch_id(&mut self, id: i32) {
        self.touch_id = id;
    }
}

/// Keyboard event
#[derive(Debug)]
pub struct EventKeyboard {
    base: Event,
    key_code: i32,
    is_pressed: bool,
}

impl EventKeyboard {
    pub fn new(key_code: i32, is_pressed: bool) -> EventKeyboard {
        EventKeyboard {
            base: Event::new(EventType::Keyboard),
            key_code,
            is_pressed,
        }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

/// Mouse event
#[derive(Debug)]
pub struct EventMouse {
    base: Event,
    x: f32,
    y: f32,
    mouse_type: MouseEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
    Scroll,
}

impl EventMouse {
    pub fn new() -> EventMouse {
        EventMouse {
            base: Event::new(EventType::Mouse),
            x: 0.0,
            y: 0.0,
            mouse_type: MouseEventType::Move,
        }
    }

    pub fn get_location(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn set_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_mouse_type(&self) -> &MouseEventType {
        &self.mouse_type
    }

    pub fn set_mouse_type(&mut self, mouse_type: MouseEventType) {
        self.mouse_type = mouse_type;
    }
}

/// Custom event
#[derive(Debug)]
pub struct EventCustom {
    base: Event,
    event_name: String,
    user_data: Option<Box<dyn std::any::Any>>,
}

impl EventCustom {
    pub fn new(event_name: &str) -> EventCustom {
        EventCustom {
            base: Event::new(EventType::Custom),
            event_name: event_name.to_string(),
            user_data: None,
        }
    }

    pub fn get_event_name(&self) -> &str {
        &self.event_name
    }

    pub fn set_user_data(&mut self, data: Box<dyn std::any::Any>) {
        self.user_data = Some(data);
    }

    pub fn get_user_data<T: std::any::Any>(&self) -> Option<&T> {
        self.user_data.as_ref().and_then(|d| d.downcast_ref())
    }
}

/// Event listener types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventListenerType {
    TouchOneByOne,
    TouchAllAtOnce,
    Keyboard,
    Mouse,
    Acceleration,
    Custom,
    Node,
}

/// Event listener
#[derive(Debug)]
pub struct EventListener {
    listener_type: EventListenerType,
    callback: Box<dyn FnMut(&mut Event)>,
    enabled: bool,
    paused: bool,
    #[allow(dead_code)]
    node: Option<Rc<dyn std::any::Any>>,
}

impl EventListener {
    pub fn new(listener_type: EventListenerType, callback: Box<dyn FnMut(&mut Event)>) -> EventListener {
        EventListener {
            listener_type,
            callback,
            enabled: true,
            paused: false,
            node: None,
        }
    }

    pub fn get_type(&self) -> &EventListenerType {
        &self.listener_type
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn on_event(&mut self, event: &mut Event) {
        (self.callback)(event);
    }
}

/// Event dispatcher
#[derive(Debug)]
pub struct EventDispatcher {
    listeners: Vec<RefPtr<RefCell<EventListener>>>,
    listeners_map: HashMap<EventListenerType, Vec<usize>>,
    to_removed_listeners: Vec<usize>,
    in_update: bool,
}

impl EventDispatcher {
    pub fn new() -> EventDispatcher {
        EventDispatcher {
            listeners: Vec::new(),
            listeners_map: HashMap::new(),
            to_removed_listeners: Vec::new(),
            in_update: false,
        }
    }

    /// Adds an event listener
    pub fn add_listener(&mut self, listener: RefPtr<RefCell<EventListener>>) {
        let index = self.listeners.len();
        self.listeners.push(listener.clone());

        let listener_type = listener.borrow().get_type().clone();
        self.listeners_map
            .entry(listener_type)
            .or_insert_with(Vec::new)
            .push(index);
    }

    /// Removes an event listener
    pub fn remove_listener(&mut self, index: usize) {
        if self.in_update {
            self.to_removed_listeners.push(index);
        } else {
            self.listeners.remove(index);
        }
    }

    /// Removes all event listeners
    pub fn remove_all_listeners(&mut self) {
        self.listeners.clear();
        self.listeners_map.clear();
    }

    /// Checks if an event listener is enabled
    pub fn is_enabled(&self, listener_type: EventListenerType) -> bool {
        if let Some(indices) = self.listeners_map.get(&listener_type) {
            for &index in indices {
                if index < self.listeners.len() && self.listeners[index].borrow().is_enabled() {
                    return true;
                }
            }
        }
        false
    }

    /// Sets event listener enabled
    pub fn set_enabled(&mut self, listener_type: EventListenerType, enabled: bool) {
        if let Some(indices) = self.listeners_map.get(&listener_type) {
            for &index in indices {
                if index < self.listeners.len() {
                    self.listeners[index].borrow_mut().set_enabled(enabled);
                }
            }
        }
    }

    /// Dispatches an event
    pub fn dispatch_event(&mut self, event: &mut Event) {
        let event_type = event.get_event_type().clone();

        let listener_type = match event_type {
            EventType::Touch => EventListenerType::TouchOneByOne,
            EventType::Keyboard => EventListenerType::Keyboard,
            EventType::Mouse => EventListenerType::Mouse,
            EventType::Acceleration => EventListenerType::Acceleration,
            EventType::Custom => EventListenerType::Custom,
        };

        if let Some(indices) = self.listeners_map.get(&listener_type) {
            self.in_update = true;

            for &index in indices {
                if index < self.listeners.len() {
                    let mut listener = self.listeners[index].borrow_mut();

                    if listener.is_enabled() && !listener.is_paused() {
                        listener.on_event(event);

                        if event.is_stopped() {
                            break;
                        }
                    }
                }
            }

            self.in_update = false;

            // Clean up removed listeners
            for index in &self.to_removed_listeners {
                self.listeners.remove(*index);
            }
            self.to_removed_listeners.clear();
        }
    }
}
